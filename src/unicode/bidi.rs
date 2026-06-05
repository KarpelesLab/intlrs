//! Bidirectional text (UAX #9).
//!
//! Provides the `Bidi_Class` property, paragraph base-direction detection
//! (rules P2–P3), and — with the `alloc` feature — the full reordering algorithm
//! ([`process`], rules X1–X10 / W1–W7 / N0–N2 / I1–I2 / L1–L2) resolving
//! embedding levels and visual order. Conformance: **100% on the exhaustive
//! `BidiTest.txt`** (all bidi-class combinations) and 99.996% on
//! `BidiCharacterTest.txt` (four deeply-nested isolate-boundary lines aside).

use super::generated::bidi as gen;

/// The `Bidi_Class` of a codepoint (UAX #9). Order matches the generated table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum BidiClass {
    /// Left-to-Right
    L,
    /// Right-to-Left
    R,
    /// Right-to-Left Arabic
    AL,
    /// European Number
    EN,
    /// European Separator
    ES,
    /// European Terminator
    ET,
    /// Arabic Number
    AN,
    /// Common Separator
    CS,
    /// Nonspacing Mark
    NSM,
    /// Boundary Neutral
    BN,
    /// Paragraph Separator
    B,
    /// Segment Separator
    S,
    /// White Space
    WS,
    /// Other Neutral
    ON,
    /// Left-to-Right Embedding
    LRE,
    /// Left-to-Right Override
    LRO,
    /// Right-to-Left Embedding
    RLE,
    /// Right-to-Left Override
    RLO,
    /// Pop Directional Format
    PDF,
    /// Left-to-Right Isolate
    LRI,
    /// Right-to-Left Isolate
    RLI,
    /// First Strong Isolate
    FSI,
    /// Pop Directional Isolate
    PDI,
}

impl BidiClass {
    /// `true` if this is a strong right-to-left class (`R` or `AL`).
    #[inline]
    #[must_use]
    pub const fn is_rtl(self) -> bool {
        matches!(self, BidiClass::R | BidiClass::AL)
    }
}

/// The [`BidiClass`] of `c`.
#[inline]
#[must_use]
pub const fn bidi_class(c: char) -> BidiClass {
    gen::bidi_class(c as u32)
}

/// The [`BidiClass`] of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn bidi_class_u32(cp: u32) -> BidiClass {
    gen::bidi_class(cp)
}

/// A paragraph direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Left-to-right (base embedding level 0).
    LeftToRight,
    /// Right-to-left (base embedding level 1).
    RightToLeft,
}

/// The base paragraph direction of `s` per UAX #9 rules P2–P3: the direction of
/// the first strong character (`L`, `R`, or `AL`), skipping over isolated
/// sequences. Defaults to left-to-right when there is no strong character.
#[must_use]
pub fn base_direction(s: &str) -> Direction {
    let mut isolate_depth = 0u32;
    for c in s.chars() {
        match bidi_class(c) {
            BidiClass::LRI | BidiClass::RLI | BidiClass::FSI => isolate_depth += 1,
            BidiClass::PDI => isolate_depth = isolate_depth.saturating_sub(1),
            BidiClass::L if isolate_depth == 0 => return Direction::LeftToRight,
            BidiClass::R | BidiClass::AL if isolate_depth == 0 => return Direction::RightToLeft,
            _ => {}
        }
    }
    Direction::LeftToRight
}

/// `true` if `s` has a right-to-left base direction.
#[must_use]
pub fn is_rtl(s: &str) -> bool {
    base_direction(s) == Direction::RightToLeft
}

#[cfg(feature = "alloc")]
pub use resolve::{process, BidiInfo};

/// The full UAX #9 algorithm: resolve embedding levels and visual order.
#[cfg(feature = "alloc")]
mod resolve {
    use super::bidi_class;
    use super::BidiClass::{self, *};
    use super::Direction;
    use crate::unicode::generated::bidi::bidi_bracket;
    use alloc::vec;
    use alloc::vec::Vec;

    const MAX_DEPTH: u8 = 125;

    /// The result of running the bidi algorithm over one paragraph. Indices are
    /// Unicode scalar (`char`) positions in the input.
    #[derive(Debug, Clone)]
    pub struct BidiInfo {
        /// The paragraph embedding level (0 = LTR, 1 = RTL).
        pub paragraph_level: u8,
        /// The resolved embedding level of each character, or `None` for a
        /// character removed by rule X9 (explicit formatting and boundary
        /// neutrals).
        pub levels: Vec<Option<u8>>,
        /// Original character indices in left-to-right visual order, excluding
        /// the X9-removed characters.
        pub visual_order: Vec<usize>,
    }

    fn next_odd(level: u8) -> u8 {
        if level % 2 == 0 {
            level + 1
        } else {
            level + 2
        }
    }
    fn next_even(level: u8) -> u8 {
        if level % 2 == 0 {
            level + 2
        } else {
            level + 1
        }
    }
    fn is_isolate_init(c: BidiClass) -> bool {
        matches!(c, LRI | RLI | FSI)
    }
    /// Neutral or Isolate formatting (the `NI` set in the N rules).
    fn is_ni(c: BidiClass) -> bool {
        matches!(c, B | S | WS | ON | FSI | LRI | RLI | PDI)
    }
    /// The directional contribution of a (resolved) class for the N rules:
    /// `EN`/`AN` count as `R`.
    fn neutral_dir(c: BidiClass) -> Option<BidiClass> {
        match c {
            L => Some(L),
            R | EN | AN => Some(R),
            _ => None,
        }
    }
    fn canon_bracket(cp: u32) -> u32 {
        match cp {
            0x2329 => 0x3008,
            0x232A => 0x3009,
            other => other,
        }
    }

    /// First strong direction (0 = L, 1 = R/AL) in `[start, end)`, skipping the
    /// contents of nested isolates. Defaults to 0.
    fn first_strong(classes: &[BidiClass], start: usize, end: usize) -> u8 {
        let mut depth = 0u32;
        for &c in &classes[start..end.min(classes.len())] {
            match c {
                _ if is_isolate_init(c) => depth += 1,
                PDI => depth = depth.saturating_sub(1),
                L if depth == 0 => return 0,
                R | AL if depth == 0 => return 1,
                _ => {}
            }
        }
        0
    }

    /// BD9: match each isolate initiator with its PDI (indices, or `len` if none).
    fn match_isolates(classes: &[BidiClass]) -> Vec<usize> {
        let n = classes.len();
        let mut match_pdi = vec![n; n];
        let mut stack: Vec<usize> = Vec::new();
        for (i, &c) in classes.iter().enumerate() {
            if is_isolate_init(c) {
                stack.push(i);
            } else if c == PDI {
                if let Some(o) = stack.pop() {
                    match_pdi[o] = i;
                }
            }
        }
        match_pdi
    }

    /// Run the bidi algorithm over `text` as a single paragraph. `base` forces
    /// the paragraph direction; `None` auto-detects it (rules P2–P3).
    #[must_use]
    pub fn process(text: &str, base: Option<Direction>) -> BidiInfo {
        let chars: Vec<char> = text.chars().collect();
        let raw: Vec<BidiClass> = chars.iter().map(|&c| bidi_class(c)).collect();
        let n = chars.len();

        let para_level = match base {
            Some(Direction::LeftToRight) => 0,
            Some(Direction::RightToLeft) => 1,
            None => first_strong(&raw, 0, n),
        };

        let match_pdi = match_isolates(&raw);
        // O(1) "is this index a matched PDI?" lookup for X10 below (a matched
        // PDI is the target of some isolate initiator in `match_pdi`). Avoids a
        // per-run linear `match_pdi.contains(..)` scan (latent O(n^2)).
        let mut is_matched_pdi = vec![false; n];
        for &p in &match_pdi {
            if p < n {
                is_matched_pdi[p] = true;
            }
        }

        // ---- X1–X8: explicit levels and overrides. ----
        let mut classes = raw.clone();
        let mut levels = vec![para_level; n];
        let mut removed = vec![false; n];
        struct Entry {
            level: u8,
            ov: Option<BidiClass>,
            iso: bool,
        }
        let mut stack = vec![Entry {
            level: para_level,
            ov: None,
            iso: false,
        }];
        let (mut oic, mut oec, mut vic) = (0u32, 0u32, 0u32);
        for i in 0..n {
            let c = raw[i];
            match c {
                RLE | LRE | RLO | LRO => {
                    levels[i] = stack.last().unwrap().level;
                    removed[i] = true;
                    let new = if matches!(c, RLE | RLO) {
                        next_odd(stack.last().unwrap().level)
                    } else {
                        next_even(stack.last().unwrap().level)
                    };
                    if new <= MAX_DEPTH && oic == 0 && oec == 0 {
                        let ov = match c {
                            RLO => Some(R),
                            LRO => Some(L),
                            _ => None,
                        };
                        stack.push(Entry {
                            level: new,
                            ov,
                            iso: false,
                        });
                    } else if oic == 0 {
                        oec += 1;
                    }
                }
                RLI | LRI | FSI => {
                    levels[i] = stack.last().unwrap().level;
                    let rtl = match c {
                        RLI => true,
                        LRI => false,
                        _ => first_strong(&raw, i + 1, match_pdi[i]) == 1,
                    };
                    let new = if rtl {
                        next_odd(stack.last().unwrap().level)
                    } else {
                        next_even(stack.last().unwrap().level)
                    };
                    if new <= MAX_DEPTH && oic == 0 && oec == 0 {
                        vic += 1;
                        stack.push(Entry {
                            level: new,
                            ov: None,
                            iso: true,
                        });
                    } else {
                        oic += 1;
                    }
                }
                PDI => {
                    if oic > 0 {
                        oic -= 1;
                    } else if vic != 0 {
                        oec = 0;
                        while !stack.last().unwrap().iso {
                            stack.pop();
                        }
                        stack.pop();
                        vic -= 1;
                    }
                    levels[i] = stack.last().unwrap().level;
                }
                PDF => {
                    if oic > 0 {
                    } else if oec > 0 {
                        oec -= 1;
                    } else if !stack.last().unwrap().iso && stack.len() >= 2 {
                        stack.pop();
                    }
                    levels[i] = stack.last().unwrap().level;
                    removed[i] = true;
                }
                B => {
                    levels[i] = para_level;
                }
                BN => {
                    levels[i] = stack.last().unwrap().level;
                    removed[i] = true;
                }
                _ => {
                    let top = stack.last().unwrap();
                    levels[i] = top.level;
                    if let Some(ov) = top.ov {
                        classes[i] = ov;
                    }
                }
            }
        }

        // ---- X10: build isolating run sequences over the non-removed chars. ----
        let reduced: Vec<usize> = (0..n).filter(|&i| !removed[i]).collect();
        let mut runs: Vec<Vec<usize>> = Vec::new();
        for &i in &reduced {
            match runs.last() {
                Some(r) if levels[*r.last().unwrap()] == levels[i] => {
                    runs.last_mut().unwrap().push(i);
                }
                _ => runs.push(vec![i]),
            }
        }
        // Map a run's first index -> run position.
        let mut run_of_start = vec![usize::MAX; n];
        for (ri, r) in runs.iter().enumerate() {
            run_of_start[r[0]] = ri;
        }
        let orig = classes.clone(); // post-X classes, before W mutations
        let mut used = vec![false; runs.len()];
        let mut sequences: Vec<Vec<usize>> = Vec::new();
        for r in 0..runs.len() {
            if used[r] {
                continue;
            }
            // A continuation run (starts with a matched PDI) is appended, not started.
            let first = runs[r][0];
            if classes[first] == PDI && is_matched_pdi[first] {
                continue;
            }
            let mut seq = Vec::new();
            let mut cur = r;
            loop {
                used[cur] = true;
                seq.extend_from_slice(&runs[cur]);
                let last = *runs[cur].last().unwrap();
                if is_isolate_init(classes[last]) && match_pdi[last] < n {
                    let nr = run_of_start[match_pdi[last]];
                    if nr != usize::MAX {
                        cur = nr;
                        continue;
                    }
                }
                break;
            }
            sequences.push(seq);
        }

        // ---- W, N, I rules per isolating run sequence. ----
        // The I rules overwrite `levels` with resolved levels, so sos/eos must
        // read the embedding levels from this snapshot, not the live array.
        let elevels = levels.clone();
        for seq in &sequences {
            resolve_sequence(
                seq,
                &chars,
                &orig,
                &mut classes,
                &mut levels,
                &elevels,
                para_level,
                n,
                &removed,
            );
        }

        // ---- L1: reset separators and trailing whitespace to the paragraph level. ----
        let mut reset_from = n;
        for i in 0..n {
            if removed[i] {
                continue;
            }
            match raw[i] {
                S | B => {
                    levels[i] = para_level;
                    for j in reset_from..i {
                        if !removed[j] {
                            levels[j] = para_level;
                        }
                    }
                    reset_from = n;
                }
                WS | FSI | LRI | RLI | PDI => {
                    if reset_from == n {
                        reset_from = i;
                    }
                }
                _ => reset_from = n,
            }
        }
        for j in reset_from..n {
            if !removed[j] {
                levels[j] = para_level;
            }
        }

        // ---- L2: reverse contiguous runs to produce visual order. ----
        let visible: Vec<usize> = (0..n).filter(|&i| !removed[i]).collect();
        let lv: Vec<u8> = visible.iter().map(|&i| levels[i]).collect();
        let mut order: Vec<usize> = (0..visible.len()).collect();
        let max_level = lv.iter().copied().max().unwrap_or(0);
        let min_odd = lv
            .iter()
            .copied()
            .filter(|l| l % 2 == 1)
            .min()
            .unwrap_or(max_level + 1);
        let mut level = max_level;
        while level >= min_odd {
            let mut i = 0;
            while i < order.len() {
                if lv[order[i]] >= level {
                    let start = i;
                    while i < order.len() && lv[order[i]] >= level {
                        i += 1;
                    }
                    order[start..i].reverse();
                } else {
                    i += 1;
                }
            }
            if level == 0 {
                break;
            }
            level -= 1;
        }
        let visual_order: Vec<usize> = order.iter().map(|&p| visible[p]).collect();

        let out_levels: Vec<Option<u8>> = (0..n)
            .map(|i| if removed[i] { None } else { Some(levels[i]) })
            .collect();
        BidiInfo {
            paragraph_level: para_level,
            levels: out_levels,
            visual_order,
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn resolve_sequence(
        seq: &[usize],
        chars: &[char],
        orig: &[BidiClass],
        classes: &mut [BidiClass],
        levels: &mut [u8],
        elevels: &[u8],
        para_level: u8,
        n: usize,
        removed: &[bool],
    ) {
        let len = seq.len();
        if len == 0 {
            return;
        }
        let seq_level = elevels[seq[0]];
        let e = if seq_level % 2 == 0 { L } else { R };

        // sos / eos (X10): compare the sequence level with the adjacent embedding
        // levels (from the snapshot, since the live levels are being resolved).
        // X9-removed characters are skipped on both sides; the immediate adjacent
        // non-removed character's level is used per the standard. (This passes the
        // exhaustive BidiTest.txt 100%; four very deeply-nested
        // isolate+embedding+override lines in BidiCharacterTest remain off by a
        // level — a documented residual, not worth a heuristic that regresses the
        // exhaustive suite.)
        let first = seq[0];
        let prev_level = (0..first)
            .rev()
            .find(|&j| !removed[j])
            .map_or(para_level, |j| elevels[j]);
        let sos = if seq_level.max(prev_level) % 2 == 1 {
            R
        } else {
            L
        };
        let last = seq[len - 1];
        let next_level = if is_isolate_init(classes[last]) {
            para_level // an isolate initiator with no matching PDI
        } else {
            (last + 1..n)
                .find(|&j| !removed[j])
                .map_or(para_level, |j| elevels[j])
        };
        let eos = if seq_level.max(next_level) % 2 == 1 {
            R
        } else {
            L
        };

        // Working classes for this sequence.
        let mut cls: Vec<BidiClass> = seq.iter().map(|&i| classes[i]).collect();

        // W1: NSM -> type of previous char (sos at start; ON after isolates).
        let mut prev = sos;
        for c in cls.iter_mut() {
            if *c == NSM {
                *c = if matches!(prev, LRI | RLI | FSI | PDI) {
                    ON
                } else {
                    prev
                };
            }
            prev = *c;
        }
        // W2: EN -> AN if the last strong type is AL.
        let mut strong = sos;
        for c in cls.iter_mut() {
            match *c {
                R | L | AL => strong = *c,
                EN if strong == AL => *c = AN,
                _ => {}
            }
        }
        // W3: AL -> R.
        for c in cls.iter_mut() {
            if *c == AL {
                *c = R;
            }
        }
        // W4: a single ES between EN, or CS between EN/AN, joins the numbers.
        for k in 1..len.saturating_sub(1) {
            if cls[k] == ES && cls[k - 1] == EN && cls[k + 1] == EN {
                cls[k] = EN;
            } else if cls[k] == CS {
                if cls[k - 1] == EN && cls[k + 1] == EN {
                    cls[k] = EN;
                } else if cls[k - 1] == AN && cls[k + 1] == AN {
                    cls[k] = AN;
                }
            }
        }
        // W5: a run of ET adjacent to EN becomes EN.
        let mut k = 0;
        while k < len {
            if cls[k] == ET {
                let start = k;
                while k < len && cls[k] == ET {
                    k += 1;
                }
                let before = start > 0 && cls[start - 1] == EN;
                let after = k < len && cls[k] == EN;
                if before || after {
                    for c in cls.iter_mut().take(k).skip(start) {
                        *c = EN;
                    }
                }
            } else {
                k += 1;
            }
        }
        // W6: remaining separators / terminators become ON.
        for c in cls.iter_mut() {
            if matches!(*c, ES | ET | CS) {
                *c = ON;
            }
        }
        // W7: EN -> L if the last strong type is L.
        let mut strong = sos;
        for c in cls.iter_mut() {
            match *c {
                R | L => strong = *c,
                EN if strong == L => *c = L,
                _ => {}
            }
        }

        // N0: paired brackets (BD16).
        resolve_brackets(seq, chars, orig, &mut cls, e, sos);

        // N1 / N2: resolve runs of neutrals.
        let mut k = 0;
        while k < len {
            if is_ni(cls[k]) {
                let start = k;
                while k < len && is_ni(cls[k]) {
                    k += 1;
                }
                let before = if start == 0 {
                    sos
                } else {
                    neutral_dir(cls[start - 1]).unwrap_or(sos)
                };
                let after = if k == len {
                    eos
                } else {
                    neutral_dir(cls[k]).unwrap_or(eos)
                };
                let set = if before == after { before } else { e };
                for c in cls.iter_mut().take(k).skip(start) {
                    *c = set;
                }
            } else {
                k += 1;
            }
        }

        // I1 / I2: implicit levels, then write back.
        for (k, &i) in seq.iter().enumerate() {
            let add = if seq_level % 2 == 0 {
                match cls[k] {
                    R => 1,
                    AN | EN => 2,
                    _ => 0,
                }
            } else {
                match cls[k] {
                    L | EN | AN => 1,
                    _ => 0,
                }
            };
            levels[i] = seq_level + add;
            classes[i] = cls[k];
        }
    }

    /// N0: resolve the direction of paired brackets within a sequence.
    fn resolve_brackets(
        seq: &[usize],
        chars: &[char],
        orig: &[BidiClass],
        cls: &mut [BidiClass],
        e: BidiClass,
        sos: BidiClass,
    ) {
        let len = seq.len();
        let mut stack: Vec<(u32, usize)> = Vec::new();
        let mut pairs: Vec<(usize, usize)> = Vec::new();
        for k in 0..len {
            if cls[k] != ON {
                continue;
            }
            let (paired, ty) = bidi_bracket(chars[seq[k]] as u32);
            if ty == 1 {
                if stack.len() == 63 {
                    break;
                }
                stack.push((canon_bracket(paired), k));
            } else if ty == 2 {
                let cc = canon_bracket(chars[seq[k]] as u32);
                if let Some(si) = (0..stack.len()).rev().find(|&si| stack[si].0 == cc) {
                    pairs.push((stack[si].1, k));
                    stack.truncate(si);
                }
            }
        }
        pairs.sort_unstable_by_key(|p| p.0);

        let o = if e == L { R } else { L };
        for (open_k, close_k) in pairs {
            let mut found_e = false;
            let mut found_o = false;
            for c in cls.iter().take(close_k).skip(open_k + 1) {
                if let Some(d) = neutral_dir(*c) {
                    if d == e {
                        found_e = true;
                        break;
                    }
                    found_o = true;
                }
            }
            let set = if found_e {
                Some(e)
            } else if found_o {
                let before = (0..open_k)
                    .rev()
                    .find_map(|m| neutral_dir(cls[m]))
                    .unwrap_or(sos);
                Some(if before == o { o } else { e })
            } else {
                None
            };
            if let Some(dir) = set {
                cls[open_k] = dir;
                cls[close_k] = dir;
                // Trailing characters originally NSM take the bracket's direction.
                for &bk in &[open_k, close_k] {
                    let mut m = bk + 1;
                    while m < len && orig[seq[m]] == NSM {
                        cls[m] = dir;
                        m += 1;
                    }
                }
            }
        }
    }
}

#[cfg(all(test, feature = "alloc", feature = "bmp"))]
mod tests {
    use super::process;

    /// Regression for the X10 matched-PDI membership test (formerly an O(n)
    /// `match_pdi.contains(..)` scan, now an O(1) `is_matched_pdi[..]` lookup).
    /// An input with many isolate initiators and their matching PDIs exercises
    /// the continuation-run assembly path that the lookup guards; the result
    /// must be unchanged (correct levels and a valid visual permutation).
    #[test]
    fn matched_pdi_lookup_assembles_isolate_runs() {
        // LRI Hebrew PDI repeated: each isolate initiator is matched to a PDI,
        // so the per-run `is_matched_pdi[first]` test must hold for every PDI
        // run, which previously hit the linear scan.
        let mut s = alloc::string::String::new();
        for _ in 0..200 {
            s.push('\u{2066}'); // LRI
            s.push('\u{05D0}'); // Hebrew alef (R)
            s.push('\u{2069}'); // PDI
        }
        let info = process(&s, None);
        let n = s.chars().count();

        // LTR paragraph (no strong char outside the isolates).
        assert_eq!(info.paragraph_level, 0);
        // The visual order is a permutation of the non-removed indices: same
        // length, no duplicates, all in range.
        let mut seen = alloc::vec![false; n];
        for &i in &info.visual_order {
            assert!(i < n);
            assert!(!seen[i], "duplicate index in visual order");
            seen[i] = true;
        }
        // The Hebrew letters resolve to an odd (RTL) level inside their isolate.
        let chars: alloc::vec::Vec<char> = s.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            if c == '\u{05D0}' {
                assert_eq!(info.levels[i].map(|l| l % 2), Some(1));
            }
        }
    }
}
