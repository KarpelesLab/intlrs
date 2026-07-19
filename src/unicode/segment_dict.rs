//! Dictionary-based word segmentation for Thai (feature `segmentation-dict`).
//!
//! Space-less scripts such as Thai are not segmented into words by the UAX #29
//! rules alone; a language dictionary is required. This module is a faithful,
//! `no_std`, allocation-free port of ICU's `ThaiBreakEngine` /
//! `DictionaryBreakEngine` (`icu4c/source/common/dictbe.cpp`), operating on a
//! UTF-8 `&str` run and driven by the committed `segment_dict.bin` DAWG built
//! from ICU's `thaidict.txt`.
//!
//! The engine is invoked from [`super::segment::words`] over each maximal run of
//! Thai dictionary characters; everything else keeps its exact UAX #29 behavior.
//! ICU's algorithm uses only bounded lookahead (at most three candidate words),
//! so it fits fixed-size stack buffers with no heap allocation.

/// Minimum word size (code points), per ICU `THAI_MIN_WORD`.
const THAI_MIN_WORD: usize = 2;
/// Minimum number of code points for two words. A run shorter than this is left
/// whole (matches ICU's `divideUpDictionaryRange` early-out).
pub(crate) const THAI_MIN_WORD_SPAN: usize = THAI_MIN_WORD * 2;
// ICU's THAI_LOOKAHEAD (3) is realized directly as the three `PossibleWord`
// slots `w0`/`w1`/`w2` in `thai_next_boundary` rather than a modular array.
/// Won't combine a non-word with a preceding dictionary word longer than this.
const THAI_ROOT_COMBINE_THRESHOLD: usize = 3;
/// Won't combine a non-word sharing at least this long a prefix with a
/// dictionary word onto a preceding word.
const THAI_PREFIX_COMBINE_THRESHOLD: usize = 3;
/// Elision character THAI CHARACTER PAIYANNOI.
const THAI_PAIYANNOI: char = '\u{0E2F}';
/// Repeat character THAI CHARACTER MAIYAMOK.
const THAI_MAIYAMOK: char = '\u{0E46}';

/// Max candidate list length, per ICU `POSSIBLE_WORD_LIST_MAX`.
const POSSIBLE_WORD_LIST_MAX: usize = 20;

/// `[[:Thai:] & [:LineBreak=SA:]]` — the characters the Thai engine segments.
#[inline]
pub(crate) fn is_thai_dict_char(c: char) -> bool {
    matches!(c as u32, 0x0E01..=0x0E3A | 0x0E40..=0x0E4E)
}

/// `fMarkSet`: Thai combining marks (plus SPACE), never a stopping point.
#[inline]
fn is_mark(c: char) -> bool {
    matches!(c as u32, 0x0020 | 0x0E31 | 0x0E34..=0x0E3A | 0x0E47..=0x0E4E)
}

/// `fEndWordSet`: dictionary chars that may end a word.
#[inline]
fn is_end_word(c: char) -> bool {
    matches!(c as u32, 0x0E01..=0x0E30 | 0x0E32..=0x0E3A | 0x0E45..=0x0E4E)
}

/// `fBeginWordSet`: dictionary chars that may begin a word.
#[inline]
fn is_begin_word(c: char) -> bool {
    matches!(c as u32, 0x0E01..=0x0E2E | 0x0E40..=0x0E44)
}

/// `fSuffixSet`: the two Thai suffix characters.
#[inline]
fn is_suffix(c: char) -> bool {
    c == THAI_PAIYANNOI || c == THAI_MAIYAMOK
}

// ---- DAWG dictionary matcher ----

/// The committed, minimized Thai dictionary DAWG. Always embedded; only read
/// when the `segmentation-dict` feature is on. See `codegen::emit_segment_dict`
/// for the byte layout.
const DICT: &[u8] = include_bytes!("segment_dict.bin");

#[inline]
fn rd_u32(o: usize) -> Option<u32> {
    let b = DICT.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
#[inline]
fn rd_u16(o: usize) -> Option<u16> {
    let b = DICT.get(o..o.checked_add(2)?)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

/// A parsed view over the embedded DAWG. Every access is bounds-checked so a
/// truncated or inconsistent blob degrades to "no match" rather than panicking.
struct Dict {
    n: usize,
    root: usize,
    bitmap_base: usize,
    off_base: usize,
    edges_base: usize,
}

impl Dict {
    fn load() -> Option<Dict> {
        let n = rd_u32(0)? as usize;
        let e = rd_u32(4)? as usize;
        let root = rd_u32(8)? as usize;
        let bitmap_base = 12;
        let off_base = bitmap_base + n.div_ceil(8);
        let edges_base = off_base.checked_add((n.checked_add(1)?).checked_mul(2)?)?;
        // Validate the whole blob is present before any hot-path reads.
        if edges_base.checked_add(e.checked_mul(3)?)? > DICT.len() || root >= n {
            return None;
        }
        Some(Dict {
            n,
            root,
            bitmap_base,
            off_base,
            edges_base,
        })
    }

    #[inline]
    fn is_final(&self, node: usize) -> bool {
        if node >= self.n {
            return false;
        }
        DICT.get(self.bitmap_base + node / 8)
            .is_some_and(|byte| (byte >> (node % 8)) & 1 != 0)
    }

    /// The `[start, end)` edge index range owned by `node`.
    #[inline]
    fn edge_range(&self, node: usize) -> (usize, usize) {
        let a = rd_u16(self.off_base + node * 2).unwrap_or(0) as usize;
        let b = rd_u16(self.off_base + (node + 1) * 2).unwrap_or(0) as usize;
        (a, b)
    }

    #[inline]
    fn has_children(&self, node: usize) -> bool {
        let (a, b) = self.edge_range(node);
        a < b
    }

    /// Follow the edge labelled `sym` out of `node`, if any (binary search).
    #[inline]
    fn child(&self, node: usize, sym: u8) -> Option<usize> {
        let (mut lo, mut hi) = self.edge_range(node);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let rec = self.edges_base + mid * 3;
            let s = *DICT.get(rec)?;
            match s.cmp(&sym) {
                core::cmp::Ordering::Less => lo = mid + 1,
                core::cmp::Ordering::Greater => hi = mid,
                core::cmp::Ordering::Equal => return Some(rd_u16(rec + 1)? as usize),
            }
        }
        None
    }
}

/// The DAWG edge symbol for a codepoint (offset from U+0E00), or `None` if it is
/// outside the dictionary's byte range (which forces a NO_MATCH like ICU).
#[inline]
fn sym(c: char) -> Option<u8> {
    let cp = c as u32;
    (0x0E00..=0x0EFF)
        .contains(&cp)
        .then_some((cp - 0x0E00) as u8)
}

// ---- UText-style cursor helpers over a UTF-8 run ----
//
// These mirror `utext_next32` / `utext_previous32` / `utext_current32`, which
// the ICU algorithm drives by side-effecting a shared cursor. Positions are byte
// offsets into `run`.

/// Char at `pos`, or `'\0'` (our stand-in for `U_SENTINEL`) at/after the end.
#[inline]
fn current32(run: &str, pos: usize) -> char {
    run.get(pos..)
        .and_then(|r| r.chars().next())
        .unwrap_or('\0')
}

/// `utext_next32`: read the char at `*pos` and advance past it.
#[inline]
fn next32(run: &str, pos: &mut usize) -> char {
    match run.get(*pos..).and_then(|r| r.chars().next()) {
        Some(c) => {
            *pos += c.len_utf8();
            c
        }
        None => '\0',
    }
}

/// `utext_previous32`: move back one char and return it.
#[inline]
fn previous32(run: &str, pos: &mut usize) -> char {
    if *pos == 0 {
        return '\0';
    }
    let mut i = *pos - 1;
    while i > 0 && !run.is_char_boundary(i) {
        i -= 1;
    }
    *pos = i;
    current32(run, i)
}

// ---- PossibleWord (bounded candidate list) ----

/// Port of ICU's `PossibleWord` helper: the dictionary words that start at one
/// text offset, in increasing length order, plus a "marked" preferred choice.
struct PossibleWord {
    count: usize,
    prefix: usize,  // longest shared prefix with a dictionary word, in code points
    offset: usize,  // byte offset these candidates were computed at
    mark: usize,    // preferred candidate index
    current: usize, // candidate currently under consideration
    cu: [usize; POSSIBLE_WORD_LIST_MAX], // word lengths in bytes (code units)
    cp: [usize; POSSIBLE_WORD_LIST_MAX], // word lengths in code points
}

impl PossibleWord {
    fn new() -> Self {
        PossibleWord {
            count: 0,
            prefix: 0,
            offset: usize::MAX, // != any real offset, forces first fill
            mark: 0,
            current: 0,
            cu: [0; POSSIBLE_WORD_LIST_MAX],
            cp: [0; POSSIBLE_WORD_LIST_MAX],
        }
    }

    /// Fill the candidate list for the current cursor position (if not already),
    /// leave the cursor after the longest candidate, and return the count.
    fn candidates(
        &mut self,
        dict: &Dict,
        run: &str,
        range_end: usize,
        cursor: &mut usize,
    ) -> usize {
        let start = *cursor;
        if start != self.offset {
            self.offset = start;
            self.fill(dict, run, start, range_end);
            // Dictionary leaves the cursor after the longest *prefix*, not the
            // longest *word*; back up when nothing matched.
            if self.count == 0 {
                *cursor = start;
            }
        }
        if self.count > 0 {
            *cursor = start + self.cu[self.count - 1];
        }
        self.current = self.count.saturating_sub(1);
        self.mark = self.current;
        self.count
    }

    /// Scan the DAWG from `start`, recording every word end (bounded by
    /// `range_end`). Mirrors `UCharsDictionaryMatcher::matches`, including its
    /// `prefix` (code points consumed along the trie, counting the char that
    /// caused the final NO_MATCH / FINAL_VALUE).
    fn fill(&mut self, dict: &Dict, run: &str, start: usize, range_end: usize) {
        self.count = 0;
        let mut node = dict.root;
        let mut cp_matched = 0usize;
        let mut consumed = 0usize;
        let end = range_end.min(run.len());
        let max_bytes = end - start;
        for c in run[start..end].chars() {
            let child = sym(c).and_then(|s| dict.child(node, s));
            consumed += c.len_utf8();
            cp_matched += 1;
            match child {
                None => break, // NO_MATCH (counts this code point in prefix)
                Some(nx) => {
                    node = nx;
                    if dict.is_final(node) {
                        if self.count < POSSIBLE_WORD_LIST_MAX {
                            self.cu[self.count] = consumed;
                            self.cp[self.count] = cp_matched;
                            self.count += 1;
                        }
                        if !dict.has_children(node) {
                            break; // FINAL_VALUE: leaf, stop
                        }
                    }
                }
            }
            if consumed >= max_bytes {
                break;
            }
        }
        self.prefix = cp_matched;
    }

    /// Select the marked candidate, point the cursor after it, return its bytes.
    fn accept_marked(&self, cursor: &mut usize) -> usize {
        *cursor = self.offset + self.cu[self.mark];
        self.cu[self.mark]
    }

    /// Back up to the next shorter candidate; return whether one existed.
    fn back_up(&mut self, cursor: &mut usize) -> bool {
        if self.current > 0 {
            self.current -= 1;
            *cursor = self.offset + self.cu[self.current];
            true
        } else {
            false
        }
    }

    #[inline]
    fn mark_current(&mut self) {
        self.mark = self.current;
    }
    #[inline]
    fn marked_cp_length(&self) -> usize {
        self.cp[self.mark]
    }
    #[inline]
    fn longest_prefix(&self) -> usize {
        self.prefix
    }
}

/// Byte offset of the next dictionary word boundary at or after `start` within
/// the Thai run `run` (whose whole length is the dictionary range end).
///
/// This is exactly one iteration of ICU's
/// `ThaiBreakEngine::divideUpDictionaryRange` outer loop. That loop carries no
/// state between iterations other than the text position (each iteration refills
/// its `PossibleWord` scratch from the current offset), so it is a pure function
/// of `(run, start)` and can be resumed here one word at a time — no allocation,
/// no buffering of the whole run's breaks. The returned boundary is always
/// `> start` and `<= run.len()`; a return of `run.len()` marks the final word.
pub(crate) fn thai_next_boundary(run: &str, start: usize) -> usize {
    let range_end = run.len();
    let Some(dict) = Dict::load() else {
        return range_end;
    };

    let mut w0 = PossibleWord::new();
    let mut w1 = PossibleWord::new();
    let mut w2 = PossibleWord::new();

    let current = start;
    let mut cursor = start;
    let mut cu_word_length = 0usize;
    let mut cp_word_length = 0usize;

    // Look for candidate words at the current position.
    let candidates = w0.candidates(&dict, run, range_end, &mut cursor);

    if candidates == 1 {
        cu_word_length = w0.accept_marked(&mut cursor);
        cp_word_length = w0.marked_cp_length();
    } else if candidates > 1 {
        // More than one: pick the one that lets the most following words parse.
        if cursor < range_end {
            'search: loop {
                if w1.candidates(&dict, run, range_end, &mut cursor) > 0 {
                    w0.mark_current();
                    if cursor >= range_end {
                        break 'search;
                    }
                    loop {
                        if w2.candidates(&dict, run, range_end, &mut cursor) > 0 {
                            w0.mark_current();
                            break 'search;
                        }
                        if !w1.back_up(&mut cursor) {
                            break;
                        }
                    }
                }
                if !w0.back_up(&mut cursor) {
                    break 'search;
                }
            }
        }
        cu_word_length = w0.accept_marked(&mut cursor);
        cp_word_length = w0.marked_cp_length();
    }

    // Look ahead to the next word. If it is not a dictionary word, fold it into
    // the word just found — unless that word is already long enough (root
    // threshold) or the non-word shares enough of a dictionary prefix.
    let mut uc: char;
    if cursor < range_end && cp_word_length < THAI_ROOT_COMBINE_THRESHOLD {
        if w0.candidates(&dict, run, range_end, &mut cursor) == 0
            && (cu_word_length == 0 || w0.longest_prefix() < THAI_PREFIX_COMBINE_THRESHOLD)
        {
            // Scan forward for a plausible boundary and resynchronize.
            let mut remaining = (range_end - (current + cu_word_length)) as isize;
            let mut chars = 0usize;
            loop {
                let pc_index = cursor;
                let pc = next32(run, &mut cursor);
                let pc_size = cursor - pc_index;
                chars += pc_size;
                remaining -= pc_size as isize;
                if remaining <= 0 {
                    break;
                }
                uc = current32(run, cursor);
                if is_end_word(pc) && is_begin_word(uc) {
                    let num = w1.candidates(&dict, run, range_end, &mut cursor);
                    cursor = current + cu_word_length + chars;
                    if num > 0 {
                        break;
                    }
                }
            }
            cu_word_length += chars;
        } else {
            // Back up to where we were for the next iteration.
            cursor = current + cu_word_length;
        }
    }

    // Never stop before a combining mark.
    while cursor < range_end && is_mark(current32(run, cursor)) {
        let curr_pos = cursor;
        next32(run, &mut cursor);
        cu_word_length += cursor - curr_pos;
    }

    // Look ahead for possible suffixes if a dictionary word does not follow. Done
    // in code (not via a rule) so the heuristic resync keeps working, e.g. when a
    // suffix character is a typo mid-word.
    if cursor < range_end && cu_word_length > 0 {
        let no_dict_follows = w0.candidates(&dict, run, range_end, &mut cursor) == 0;
        uc = current32(run, cursor);
        if no_dict_follows && is_suffix(uc) {
            if uc == THAI_PAIYANNOI {
                if !is_suffix(previous32(run, &mut cursor)) {
                    next32(run, &mut cursor); // skip over the previous end
                    let paiyannoi_index = cursor;
                    next32(run, &mut cursor); // and PAIYANNOI
                    cu_word_length += cursor - paiyannoi_index;
                    uc = current32(run, cursor);
                } else {
                    next32(run, &mut cursor); // restore prior position
                }
            }
            if uc == THAI_MAIYAMOK {
                if previous32(run, &mut cursor) != THAI_MAIYAMOK {
                    next32(run, &mut cursor);
                    let maiyamok_index = cursor;
                    next32(run, &mut cursor);
                    cu_word_length += cursor - maiyamok_index;
                } else {
                    next32(run, &mut cursor);
                }
            }
        }
    }

    // The accepted word ends here; hand back its boundary. (ICU keeps mutating
    // the cursor for its next loop iteration; we recompute from this boundary
    // instead.)
    (current + cu_word_length).min(range_end)
}
