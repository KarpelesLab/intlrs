//! Dictionary-based word segmentation for Thai (feature `segmentation-dict`),
//! Lao (`segmentation-dict-lao`), Khmer (`segmentation-dict-km`) and Burmese /
//! Myanmar (`segmentation-dict-my`).
//!
//! Space-less scripts such as Thai, Lao, Khmer and Burmese are not segmented into
//! words by the UAX #29 rules alone; a language dictionary is required. This
//! module is a faithful, `no_std`, allocation-free port of ICU's
//! `ThaiBreakEngine` / `LaoBreakEngine` / `KhmerBreakEngine` /
//! `BurmeseBreakEngine` / `DictionaryBreakEngine`
//! (`icu4c/source/common/dictbe.cpp`), operating on a UTF-8 `&str` run and driven
//! by the committed `segment_dict*.bin` DAWGs built from ICU's `thaidict.txt` /
//! `laodict.txt` / `khmerdict.txt` / `burmesedict.txt`. All four share one engine
//! ([`next_boundary`]) parameterized by a [`Lang`]: only Thai uses the extra
//! suffix handling; the others are the same algorithm without it.
//!
//! Each script's DAWG edge symbol is a `u8` offset from that script's *own* block
//! base ([`Lang::base`]): U+0E00 for Thai/Lao, U+1780 for Khmer, U+1000 for the
//! main Myanmar block. (Myanmar Extended-A/B code points lie outside a `u8` of
//! U+1000; ICU's `burmesedict.txt` contains none of them, so the engine — and the
//! run detector — is restricted to the main block, and any supplementary Myanmar
//! characters fall back to single UAX #29 runs.)
//!
//! The engine is invoked from [`super::segment::words`] over each maximal run of a
//! script's dictionary characters; everything else keeps its exact UAX #29
//! behavior. ICU's algorithm uses only bounded lookahead (at most three candidate
//! words), so it fits fixed-size stack buffers with no heap allocation.

/// Minimum word size (code points), per ICU `THAI_MIN_WORD`.
const THAI_MIN_WORD: usize = 2;
/// Minimum number of code points for two words. A run shorter than this is left
/// whole (matches ICU's `divideUpDictionaryRange` early-out).
pub(crate) const THAI_MIN_WORD_SPAN: usize = THAI_MIN_WORD * 2;
// ICU's THAI_LOOKAHEAD (3) is realized directly as the three `PossibleWord`
// slots `w0`/`w1`/`w2` in `next_boundary` rather than a modular array.
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

// ---- Lao character classes (feature `segmentation-dict-lao`) ----
//
// ICU's `LaoBreakEngine` uses the same algorithm as Thai (minus the suffix
// handling, "NOT CURRENTLY APPLICABLE TO LAO") with these Lao sets. The dict /
// mark sets are `[[:Laoo:]&[:LineBreak=SA:]]` and its `[:M:]` subset; the
// begin/end sets are the explicit ranges from the ICU constructor.

/// `[[:Laoo:]&[:LineBreak=SA:]]` — the characters the Lao engine segments.
#[cfg(feature = "segmentation-dict-lao")]
#[inline]
pub(crate) fn is_lao_dict_char(c: char) -> bool {
    matches!(c as u32,
        0x0E81..=0x0E82 | 0x0E84 | 0x0E86..=0x0E8A | 0x0E8C..=0x0EA3 | 0x0EA5
        | 0x0EA7..=0x0EBD | 0x0EC0..=0x0EC4 | 0x0EC6 | 0x0EC8..=0x0ECE | 0x0EDC..=0x0EDF)
}

/// `fMarkSet` (Lao): `[[:Laoo:]&[:LineBreak=SA:]&[:M:]]` plus SPACE.
#[cfg(feature = "segmentation-dict-lao")]
#[inline]
fn is_mark_lao(c: char) -> bool {
    matches!(c as u32, 0x0020 | 0x0EB1 | 0x0EB4..=0x0EBC | 0x0EC8..=0x0ECE)
}

/// `fEndWordSet` (Lao): the dict set minus the prefix vowels U+0EC0..=U+0EC4.
#[cfg(feature = "segmentation-dict-lao")]
#[inline]
fn is_end_word_lao(c: char) -> bool {
    matches!(c as u32,
        0x0E81..=0x0E82 | 0x0E84 | 0x0E86..=0x0E8A | 0x0E8C..=0x0EA3 | 0x0EA5
        | 0x0EA7..=0x0EBD | 0x0EC6 | 0x0EC8..=0x0ECE | 0x0EDC..=0x0EDF)
}

/// `fBeginWordSet` (Lao): basic + digraph consonants and the prefix vowels.
#[cfg(feature = "segmentation-dict-lao")]
#[inline]
fn is_begin_word_lao(c: char) -> bool {
    matches!(c as u32, 0x0E81..=0x0EAE | 0x0EC0..=0x0EC4 | 0x0EDC..=0x0EDD)
}

// ---- Khmer character classes (feature `segmentation-dict-km`) ----
//
// ICU's `KhmerBreakEngine` uses the Thai/Lao algorithm (minus the suffix
// handling — the `fSuffixSet` block is commented out in ICU) with these Khmer
// sets. The dict / mark sets are `[[:Khmr:]&[:LineBreak=SA:]]` and its `[:M:]`
// subset; the whole Khmer dict block fits in a `u8` offset from U+1780.

/// `[[:Khmr:]&[:LineBreak=SA:]]` — the characters the Khmer engine segments.
#[cfg(feature = "segmentation-dict-km")]
#[inline]
pub(crate) fn is_khmer_dict_char(c: char) -> bool {
    matches!(c as u32, 0x1780..=0x17D3 | 0x17D7 | 0x17DC..=0x17DD)
}

/// `fMarkSet` (Khmer): `[[:Khmr:]&[:LineBreak=SA:]&[:M:]]` plus SPACE.
#[cfg(feature = "segmentation-dict-km")]
#[inline]
fn is_mark_khmer(c: char) -> bool {
    matches!(c as u32, 0x0020 | 0x17B4..=0x17D3 | 0x17DD)
}

/// `fEndWordSet` (Khmer): the dict set minus U+17D2 KHMER SIGN COENG.
#[cfg(feature = "segmentation-dict-km")]
#[inline]
fn is_end_word_khmer(c: char) -> bool {
    matches!(c as u32, 0x1780..=0x17D1 | 0x17D3 | 0x17D7 | 0x17DC..=0x17DD)
}

/// `fBeginWordSet` (Khmer): U+1780..=U+17B3 (consonants and independent vowels).
#[cfg(feature = "segmentation-dict-km")]
#[inline]
fn is_begin_word_khmer(c: char) -> bool {
    matches!(c as u32, 0x1780..=0x17B3)
}

// ---- Burmese (Myanmar) character classes (feature `segmentation-dict-my`) ----
//
// ICU's `BurmeseBreakEngine` uses the Thai/Lao algorithm (no suffix handling)
// with these Myanmar sets. The dict / mark sets are `[[:Mymr:]&[:LineBreak=SA:]]`
// and its `[:M:]` subset, restricted here to the *main* U+1000 block so the edge
// symbol fits a `u8` offset from U+1000. Myanmar Extended-A (U+AA60..) and
// Extended-B (U+A9E0..) are excluded — ICU's `burmesedict.txt` contains no such
// code points, so nothing is lost from the dictionary; any such characters in
// input simply fall back to single UAX #29 runs.

/// `[[:Mymr:]&[:LineBreak=SA:]]` restricted to the main U+1000 block — the
/// characters the Burmese engine segments.
#[cfg(feature = "segmentation-dict-my")]
#[inline]
pub(crate) fn is_burmese_dict_char(c: char) -> bool {
    matches!(c as u32, 0x1000..=0x103F | 0x1050..=0x108F | 0x109A..=0x109F)
}

/// `fMarkSet` (Burmese): `[[:Mymr:]&[:LineBreak=SA:]&[:M:]]` (main block) plus SPACE.
#[cfg(feature = "segmentation-dict-my")]
#[inline]
fn is_mark_burmese(c: char) -> bool {
    matches!(c as u32,
        0x0020 | 0x102B..=0x103E | 0x1056..=0x1059 | 0x105E..=0x1060 | 0x1062..=0x1064
        | 0x1067..=0x106D | 0x1071..=0x1074 | 0x1082..=0x108D | 0x108F | 0x109A..=0x109D)
}

/// `fEndWordSet` (Burmese): the same as the dict set (`setCharacters(fEndWordSet)`).
#[cfg(feature = "segmentation-dict-my")]
#[inline]
fn is_end_word_burmese(c: char) -> bool {
    is_burmese_dict_char(c)
}

/// `fBeginWordSet` (Burmese): U+1000..=U+102A (basic consonants and independent vowels).
#[cfg(feature = "segmentation-dict-my")]
#[inline]
fn is_begin_word_burmese(c: char) -> bool {
    matches!(c as u32, 0x1000..=0x102A)
}

/// Per-language parameters for the shared `DictionaryBreakEngine` loop: which
/// DAWG to consult, the DAWG edge-symbol block base, the character classes, and
/// whether the Thai suffix (PAIYANNOI/MAIYAMOK) heuristic applies.
pub(crate) struct Lang {
    dict: &'static [u8],
    /// Block base for the DAWG edge symbol (`sym`): a code point maps to the byte
    /// `c - base`, valid only for `base..=base + 0xFF`. U+0E00 (Thai/Lao),
    /// U+1780 (Khmer) or U+1000 (Burmese main block).
    base: u32,
    is_mark: fn(char) -> bool,
    is_end_word: fn(char) -> bool,
    is_begin_word: fn(char) -> bool,
    thai_suffix: bool,
}

/// The Thai engine parameters (`ThaiBreakEngine`).
pub(crate) const THAI: Lang = Lang {
    dict: THAI_DICT,
    base: 0x0E00,
    is_mark,
    is_end_word,
    is_begin_word,
    thai_suffix: true,
};

/// The Lao engine parameters (`LaoBreakEngine`).
#[cfg(feature = "segmentation-dict-lao")]
pub(crate) const LAO: Lang = Lang {
    dict: LAO_DICT,
    base: 0x0E00,
    is_mark: is_mark_lao,
    is_end_word: is_end_word_lao,
    is_begin_word: is_begin_word_lao,
    thai_suffix: false,
};

/// The Khmer engine parameters (`KhmerBreakEngine`).
#[cfg(feature = "segmentation-dict-km")]
pub(crate) const KHMER: Lang = Lang {
    dict: KHMER_DICT,
    base: 0x1780,
    is_mark: is_mark_khmer,
    is_end_word: is_end_word_khmer,
    is_begin_word: is_begin_word_khmer,
    thai_suffix: false,
};

/// The Burmese engine parameters (`BurmeseBreakEngine`).
#[cfg(feature = "segmentation-dict-my")]
pub(crate) const BURMESE: Lang = Lang {
    dict: BURMESE_DICT,
    base: 0x1000,
    is_mark: is_mark_burmese,
    is_end_word: is_end_word_burmese,
    is_begin_word: is_begin_word_burmese,
    thai_suffix: false,
};

// ---- DAWG dictionary matcher ----

/// The committed, minimized Thai dictionary DAWG. Always embedded; only read
/// when the `segmentation-dict` feature is on. See `codegen::emit_segment_dict`
/// for the byte layout.
const THAI_DICT: &[u8] = include_bytes!("segment_dict.bin");
/// The committed, minimized Lao dictionary DAWG (feature `segmentation-dict-lao`;
/// same byte layout and U+0E00-relative edge symbols as the Thai DAWG).
#[cfg(feature = "segmentation-dict-lao")]
const LAO_DICT: &[u8] = include_bytes!("segment_dict_lao.bin");
/// The committed, minimized Khmer dictionary DAWG (feature `segmentation-dict-km`;
/// same byte layout, but edge symbols are U+1780-relative).
#[cfg(feature = "segmentation-dict-km")]
const KHMER_DICT: &[u8] = include_bytes!("segment_dict_km.bin");
/// The committed, minimized Burmese dictionary DAWG (feature `segmentation-dict-my`;
/// same byte layout, but edge symbols are U+1000-relative, main block only).
#[cfg(feature = "segmentation-dict-my")]
const BURMESE_DICT: &[u8] = include_bytes!("segment_dict_my.bin");

#[inline]
fn rd_u32(blob: &[u8], o: usize) -> Option<u32> {
    let b = blob.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
#[inline]
fn rd_u16(blob: &[u8], o: usize) -> Option<u16> {
    let b = blob.get(o..o.checked_add(2)?)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

/// A parsed view over an embedded DAWG. Every access is bounds-checked so a
/// truncated or inconsistent blob degrades to "no match" rather than panicking.
struct Dict {
    blob: &'static [u8],
    n: usize,
    root: usize,
    bitmap_base: usize,
    off_base: usize,
    edges_base: usize,
    /// Block base for edge symbols (see [`Lang::base`]).
    base: u32,
    /// `true` when node ids / edge indices are stored as `u32` rather than `u16`
    /// (a large dictionary such as Khmer whose `n`/`e` exceed `u16::MAX`). The
    /// offset table then uses 4-byte entries and edge records are 5 bytes.
    wide: bool,
}

impl Dict {
    fn load(blob: &'static [u8], base: u32) -> Option<Dict> {
        let n = rd_u32(blob, 0)? as usize;
        let e = rd_u32(blob, 4)? as usize;
        let root = rd_u32(blob, 8)? as usize;
        let wide = n > u16::MAX as usize || e > u16::MAX as usize;
        let idx = if wide { 4 } else { 2 }; // offset-table / target width
        let rec = 1 + idx; // edge record width
        let bitmap_base = 12;
        let off_base = bitmap_base + n.div_ceil(8);
        let edges_base = off_base.checked_add((n.checked_add(1)?).checked_mul(idx)?)?;
        // Validate the whole blob is present before any hot-path reads.
        if edges_base.checked_add(e.checked_mul(rec)?)? > blob.len() || root >= n {
            return None;
        }
        Some(Dict {
            blob,
            n,
            root,
            bitmap_base,
            off_base,
            edges_base,
            base,
            wide,
        })
    }

    /// Read an offset-table / edge-target index at byte offset `o` (u16 or u32).
    #[inline]
    fn rd_idx(&self, o: usize) -> Option<usize> {
        if self.wide {
            rd_u32(self.blob, o).map(|v| v as usize)
        } else {
            rd_u16(self.blob, o).map(|v| v as usize)
        }
    }

    #[inline]
    fn is_final(&self, node: usize) -> bool {
        if node >= self.n {
            return false;
        }
        self.blob
            .get(self.bitmap_base + node / 8)
            .is_some_and(|byte| (byte >> (node % 8)) & 1 != 0)
    }

    /// The `[start, end)` edge index range owned by `node`.
    #[inline]
    fn edge_range(&self, node: usize) -> (usize, usize) {
        let w = if self.wide { 4 } else { 2 };
        let a = self.rd_idx(self.off_base + node * w).unwrap_or(0);
        let b = self.rd_idx(self.off_base + (node + 1) * w).unwrap_or(0);
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
        let recw = if self.wide { 5 } else { 3 };
        let (mut lo, mut hi) = self.edge_range(node);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let rec = self.edges_base + mid * recw;
            let s = *self.blob.get(rec)?;
            match s.cmp(&sym) {
                core::cmp::Ordering::Less => lo = mid + 1,
                core::cmp::Ordering::Greater => hi = mid,
                core::cmp::Ordering::Equal => return self.rd_idx(rec + 1),
            }
        }
        None
    }
}

/// The DAWG edge symbol for a codepoint (offset from the script's block `base`),
/// or `None` if it is outside the dictionary's byte range (which forces a
/// NO_MATCH like ICU).
#[inline]
fn sym(base: u32, c: char) -> Option<u8> {
    (c as u32)
        .checked_sub(base)
        .filter(|d| *d <= 0xFF)
        .map(|d| d as u8)
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
            let child = sym(dict.base, c).and_then(|s| dict.child(node, s));
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
/// the Thai/Lao run `run` (whose whole length is the dictionary range end), using
/// the [`Lang`] `lang`'s dictionary and character classes.
///
/// This is exactly one iteration of ICU's
/// `ThaiBreakEngine`/`LaoBreakEngine::divideUpDictionaryRange` outer loop. That loop carries no
/// state between iterations other than the text position (each iteration refills
/// its `PossibleWord` scratch from the current offset), so it is a pure function
/// of `(run, start)` and can be resumed here one word at a time — no allocation,
/// no buffering of the whole run's breaks. The returned boundary is always
/// `> start` and `<= run.len()`; a return of `run.len()` marks the final word.
pub(crate) fn next_boundary(lang: &Lang, run: &str, start: usize) -> usize {
    let range_end = run.len();
    let Some(dict) = Dict::load(lang.dict, lang.base) else {
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
                if (lang.is_end_word)(pc) && (lang.is_begin_word)(uc) {
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
    while cursor < range_end && (lang.is_mark)(current32(run, cursor)) {
        let curr_pos = cursor;
        next32(run, &mut cursor);
        cu_word_length += cursor - curr_pos;
    }

    // Look ahead for possible suffixes if a dictionary word does not follow. Done
    // in code (not via a rule) so the heuristic resync keeps working, e.g. when a
    // suffix character is a typo mid-word. (Thai only — ICU's Lao engine omits
    // this: "NOT CURRENTLY APPLICABLE TO LAO".)
    if lang.thai_suffix && cursor < range_end && cu_word_length > 0 {
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
