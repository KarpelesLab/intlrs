//! Unicode Normalization Forms (UAX #15): NFD, NFC, NFKD, NFKC.
//!
//! Normalization is exposed as streaming iterator adaptors over
//! `Iterator<Item = char>`, so it works in `no_std` without allocation:
//!
//! ```
//! use intl::unicode::{nfc, nfd};
//!
//! // "e" + combining acute  ->  precomposed "é"
//! assert_eq!(nfc("e\u{0301}".chars()).collect::<String>(), "é");
//! // precomposed "é"  ->  "e" + combining acute
//! assert_eq!(nfd("é".chars()).collect::<String>(), "e\u{0301}");
//! ```
//!
//! Canonical reordering buffers one combining sequence at a time in a
//! fixed-size array. The bound ([`MAX_COMBINING`]) comfortably covers the
//! Stream-Safe Text Format limit (UAX #15); pathological input with a longer
//! run of combining marks is split at the bound (each part is still correctly
//! ordered internally).

use super::generated::normalization as gen;

/// Maximum number of entries buffered for one combining sequence. Exceeds the
/// Stream-Safe limit of 30 trailing non-starters.
pub const MAX_COMBINING: usize = 64;

/// Maximum length of a single codepoint's full (compatibility) decomposition.
const MAX_DECOMP: usize = 32;

// Hangul algorithmic composition/decomposition constants (UAX #15, §10.2).
const S_BASE: u32 = 0xAC00;
const L_BASE: u32 = 0x1100;
const V_BASE: u32 = 0x1161;
const T_BASE: u32 = 0x11A7;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = V_COUNT * T_COUNT; // 588
const S_COUNT: u32 = L_COUNT * N_COUNT; // 11172

/// The Canonical_Combining_Class of `c`.
#[inline]
#[must_use]
pub const fn canonical_combining_class(c: char) -> u8 {
    gen::canonical_combining_class(c as u32)
}

/// The Canonical_Combining_Class of an arbitrary Unicode scalar value.
#[inline]
#[must_use]
pub const fn canonical_combining_class_u32(cp: u32) -> u8 {
    gen::canonical_combining_class(cp)
}

/// Compose a starter `a` with a following character `b`, if they form a
/// primary composite (canonical composition or Hangul).
fn compose(a: char, b: char) -> Option<char> {
    let (ua, ub) = (a as u32, b as u32);
    // Hangul L + V.
    if (L_BASE..L_BASE + L_COUNT).contains(&ua) && (V_BASE..V_BASE + V_COUNT).contains(&ub) {
        let li = ua - L_BASE;
        let vi = ub - V_BASE;
        return char::from_u32(S_BASE + (li * V_COUNT + vi) * T_COUNT);
    }
    // Hangul LV + T.
    if (S_BASE..S_BASE + S_COUNT).contains(&ua)
        && (ua - S_BASE) % T_COUNT == 0
        && (T_BASE + 1..T_BASE + T_COUNT).contains(&ub)
    {
        return char::from_u32(ua + (ub - T_BASE));
    }
    // Table-driven canonical composition.
    let pairs = gen::compose_pairs(ua)?;
    let mut i = 0;
    while i < pairs.len() {
        if pairs[i].0 == b {
            return Some(pairs[i].1);
        }
        i += 1;
    }
    None
}

/// Iterator yielding the NFD/NFKD form of an input char iterator.
///
/// `COMPAT == false` produces NFD (canonical), `true` produces NFKD
/// (compatibility). Constructed via [`nfd`] / [`nfkd`].
#[derive(Clone)]
pub struct Decompositions<I> {
    iter: I,
    compat: bool,
    // Decomposition of the current input char, drained one at a time.
    pend: [(u8, char); MAX_DECOMP],
    pend_len: u8,
    pend_pos: u8,
    // The current combining sequence (starter + trailing marks), sorted lazily.
    seq: [(u8, char); MAX_COMBINING],
    seq_len: usize,
    pos: usize,
    ready: usize,
    // A char pulled that begins the next sequence.
    carry: Option<(u8, char)>,
}

impl<I: Iterator<Item = char>> Decompositions<I> {
    fn new(iter: I, compat: bool) -> Self {
        Decompositions {
            iter,
            compat,
            pend: [(0, '\0'); MAX_DECOMP],
            pend_len: 0,
            pend_pos: 0,
            seq: [(0, '\0'); MAX_COMBINING],
            seq_len: 0,
            pos: 0,
            ready: 0,
            carry: None,
        }
    }

    /// Decompose one input char into `pend` (Hangul or table-driven; a char
    /// with no decomposition maps to itself).
    fn decompose_into(&mut self, ch: char) {
        self.pend_len = 0;
        self.pend_pos = 0;
        let u = ch as u32;
        if (S_BASE..S_BASE + S_COUNT).contains(&u) {
            let si = u - S_BASE;
            let l = L_BASE + si / N_COUNT;
            let v = V_BASE + (si % N_COUNT) / T_COUNT;
            let t = si % T_COUNT;
            self.push_pend(l);
            self.push_pend(v);
            if t != 0 {
                self.push_pend(T_BASE + t);
            }
            return;
        }
        let table = if self.compat {
            gen::decompose_compatible(u)
        } else {
            gen::decompose_canonical(u)
        };
        match table {
            Some(seq) => {
                let mut i = 0;
                while i < seq.len() {
                    self.push_pend(seq[i] as u32);
                    i += 1;
                }
            }
            None => self.push_pend(u),
        }
    }

    fn push_pend(&mut self, cp: u32) {
        let c = char::from_u32(cp).unwrap_or('\u{FFFD}');
        self.pend[self.pend_len as usize] = (gen::canonical_combining_class(cp), c);
        self.pend_len += 1;
    }

    /// Pull the next decomposed `(ccc, char)`.
    fn pull(&mut self) -> Option<(u8, char)> {
        loop {
            if self.pend_pos < self.pend_len {
                let r = self.pend[self.pend_pos as usize];
                self.pend_pos += 1;
                return Some(r);
            }
            let ch = self.iter.next()?;
            self.decompose_into(ch);
        }
    }

    /// Build and canonically order the next combining sequence. Returns `false`
    /// when the input is exhausted.
    fn build_sequence(&mut self) -> bool {
        self.seq_len = 0;
        if let Some(c) = self.carry.take() {
            self.seq[0] = c;
            self.seq_len = 1;
        }
        while let Some(c) = self.pull() {
            if c.0 == 0 {
                if self.seq_len == 0 {
                    self.seq[0] = c;
                    self.seq_len = 1;
                } else {
                    self.carry = Some(c);
                    break;
                }
            } else if self.seq_len < MAX_COMBINING {
                self.seq[self.seq_len] = c;
                self.seq_len += 1;
            } else {
                self.carry = Some(c);
                break;
            }
        }
        if self.seq_len == 0 {
            return false;
        }
        // Stable insertion sort of the trailing non-starter run by CCC.
        let start = if self.seq[0].0 == 0 { 1 } else { 0 };
        let mut i = start + 1;
        while i < self.seq_len {
            let mut j = i;
            while j > start && self.seq[j - 1].0 > self.seq[j].0 {
                self.seq.swap(j - 1, j);
                j -= 1;
            }
            i += 1;
        }
        self.pos = 0;
        self.ready = self.seq_len;
        true
    }
}

impl<I: Iterator<Item = char>> Iterator for Decompositions<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            if self.pos < self.ready {
                let c = self.seq[self.pos].1;
                self.pos += 1;
                return Some(c);
            }
            if !self.build_sequence() {
                return None;
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum RecompState {
    Composing,
    Purging,
    Finished,
    // The ring filled mid-run: emit `composee` (if any) then drain the ring,
    // re-buffering the stashed `overflow` mark once a slot frees, so a
    // pathological run of >MAX_COMBINING blocked marks loses nothing.
    Overflow,
}

/// Iterator yielding the NFC/NFKC form of an input char iterator.
///
/// Composes the output of a [`Decompositions`] stream. Constructed via
/// [`nfc`] / [`nfkc`].
#[derive(Clone)]
pub struct Recompositions<I> {
    iter: Decompositions<I>,
    state: RecompState,
    // FIFO ring buffer of non-composed combining marks awaiting output.
    ring: [char; MAX_COMBINING],
    head: usize,
    len: usize,
    composee: Option<char>,
    last_ccc: Option<u8>,
    // A blocked combining mark that did not fit in `ring` (ring was full). It is
    // held here, not dropped, and pushed once the ring has drained one slot.
    // Within a single non-starter run the marks reaching the ring arrive in
    // non-decreasing CCC order (the decomposition stream pre-orders them), so
    // draining the front and appending this at the back preserves canonical
    // order, and no later mark can compose with `composee` across the boundary.
    overflow: Option<char>,
}

impl<I: Iterator<Item = char>> Recompositions<I> {
    fn new(iter: Decompositions<I>) -> Self {
        Recompositions {
            iter,
            state: RecompState::Composing,
            ring: ['\0'; MAX_COMBINING],
            head: 0,
            len: 0,
            composee: None,
            last_ccc: None,
            overflow: None,
        }
    }

    /// Append `c` to the FIFO ring of pending combining marks. Returns `false`
    /// if the ring is full (the caller must drain before retrying) so that no
    /// mark is ever silently dropped.
    #[must_use]
    fn push_back(&mut self, c: char) -> bool {
        if self.len < MAX_COMBINING {
            self.ring[(self.head + self.len) % MAX_COMBINING] = c;
            self.len += 1;
            true
        } else {
            false
        }
    }

    fn pop_front(&mut self) -> Option<char> {
        if self.len == 0 {
            return None;
        }
        let c = self.ring[self.head];
        self.head = (self.head + 1) % MAX_COMBINING;
        self.len -= 1;
        Some(c)
    }

    /// Buffer a blocked combining mark `ch` (CCC `ch_class`), tracking it as the
    /// new last non-starter. If the ring is full, stash `ch` in `overflow` and
    /// switch to the `Overflow` drain state instead of dropping it. Returns the
    /// first char to emit when an overflow drain is started (the held
    /// `composee`, or the oldest ring mark if there is no starter), else `None`.
    fn block(&mut self, ch: char, ch_class: u8) -> Option<char> {
        self.last_ccc = Some(ch_class);
        if self.push_back(ch) {
            return None;
        }
        // Ring full: begin draining. Emit the starter first (canonical order),
        // then the ring will be drained by the `Overflow` state. The starter is
        // leaving the composition window, so reset `last_ccc`: any later starter
        // begins a fresh window. (Within this run the decomposition stream
        // delivers marks in non-decreasing CCC order, so no already-passed mark
        // could still have composed with this starter.)
        self.overflow = Some(ch);
        self.state = RecompState::Overflow;
        self.last_ccc = None;
        if let Some(k) = self.composee.take() {
            return Some(k);
        }
        // No starter held (a run of marks with no leading starter): emit the
        // oldest ring mark to free a slot; the stashed mark is pushed in the
        // `Overflow` state once room exists.
        self.pop_front()
    }
}

impl<I: Iterator<Item = char>> Iterator for Recompositions<I> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        loop {
            match self.state {
                RecompState::Composing => {
                    while let Some(ch) = self.iter.next() {
                        let ch_class = gen::canonical_combining_class(ch as u32);
                        let k = match self.composee {
                            None => {
                                if ch_class != 0 {
                                    return Some(ch);
                                }
                                self.composee = Some(ch);
                                continue;
                            }
                            Some(k) => k,
                        };
                        match self.last_ccc {
                            None => match compose(k, ch) {
                                Some(r) => {
                                    self.composee = Some(r);
                                    continue;
                                }
                                None => {
                                    if ch_class == 0 {
                                        self.composee = Some(ch);
                                        return Some(k);
                                    }
                                    if let Some(out) = self.block(ch, ch_class) {
                                        return Some(out);
                                    }
                                }
                            },
                            Some(l_class) => {
                                if l_class >= ch_class {
                                    // `ch` is blocked from `composee`.
                                    if ch_class == 0 {
                                        self.composee = Some(ch);
                                        self.last_ccc = None;
                                        self.state = RecompState::Purging;
                                        return Some(k);
                                    }
                                    if let Some(out) = self.block(ch, ch_class) {
                                        return Some(out);
                                    }
                                } else {
                                    match compose(k, ch) {
                                        Some(r) => {
                                            self.composee = Some(r);
                                            continue;
                                        }
                                        None => {
                                            if let Some(out) = self.block(ch, ch_class) {
                                                return Some(out);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self.state = RecompState::Finished;
                    if let Some(k) = self.composee.take() {
                        return Some(k);
                    }
                }
                RecompState::Purging => match self.pop_front() {
                    None => self.state = RecompState::Composing,
                    s => return s,
                },
                RecompState::Overflow => {
                    // Re-buffer the stashed mark once the ring has room; it
                    // arrived last in this non-decreasing-CCC run, so appending
                    // it at the tail keeps canonical order.
                    if self.overflow.is_some() && self.len < MAX_COMBINING {
                        let m = self.overflow.take().unwrap();
                        let _ = self.push_back(m);
                    }
                    match self.pop_front() {
                        // Ring drained and the stashed mark re-buffered: the run
                        // continues, so resume composing (composee already
                        // emitted; `last_ccc` still reflects the run).
                        None => self.state = RecompState::Composing,
                        s => return s,
                    }
                }
                RecompState::Finished => match self.pop_front() {
                    None => return self.composee.take(),
                    s => return s,
                },
            }
        }
    }
}

/// Canonical decomposition (NFD) of an input character stream.
#[inline]
pub fn nfd<I: Iterator<Item = char>>(iter: I) -> Decompositions<I> {
    Decompositions::new(iter, false)
}

/// Compatibility decomposition (NFKD) of an input character stream.
#[inline]
pub fn nfkd<I: Iterator<Item = char>>(iter: I) -> Decompositions<I> {
    Decompositions::new(iter, true)
}

/// Canonical composition (NFC) of an input character stream.
#[inline]
pub fn nfc<I: Iterator<Item = char>>(iter: I) -> Recompositions<I> {
    Recompositions::new(Decompositions::new(iter, false))
}

/// Compatibility composition (NFKC) of an input character stream.
#[inline]
pub fn nfkc<I: Iterator<Item = char>>(iter: I) -> Recompositions<I> {
    Recompositions::new(Decompositions::new(iter, true))
}

/// Result of a normalization quick check (UAX #15, §9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsNormalized {
    /// The text is definitely in the normalization form.
    Yes,
    /// The text is definitely not in the normalization form.
    No,
    /// The quick check is inconclusive; a full normalization is required to
    /// decide (only ever returned for the composed forms NFC/NFKC).
    Maybe,
}

/// Run the quick-check algorithm using the per-codepoint QC value
/// (0 = No, 1 = Maybe, 2 = Yes) returned by `qc`.
fn quick_check<I: Iterator<Item = char>>(iter: I, qc: fn(u32) -> u8) -> IsNormalized {
    let mut last_ccc = 0u8;
    let mut result = IsNormalized::Yes;
    for ch in iter {
        let c = gen::canonical_combining_class(ch as u32);
        if c != 0 && last_ccc > c {
            return IsNormalized::No; // canonical ordering violated
        }
        match qc(ch as u32) {
            0 => return IsNormalized::No,
            1 => result = IsNormalized::Maybe,
            _ => {}
        }
        last_ccc = c;
    }
    result
}

/// Quick-check whether a character stream is already in NFC.
#[inline]
pub fn quick_check_nfc<I: Iterator<Item = char>>(iter: I) -> IsNormalized {
    quick_check(iter, gen::nfc_qc)
}

/// Quick-check whether a character stream is already in NFD.
#[inline]
pub fn quick_check_nfd<I: Iterator<Item = char>>(iter: I) -> IsNormalized {
    quick_check(iter, gen::nfd_qc)
}

/// Quick-check whether a character stream is already in NFKC.
#[inline]
pub fn quick_check_nfkc<I: Iterator<Item = char>>(iter: I) -> IsNormalized {
    quick_check(iter, gen::nfkc_qc)
}

/// Quick-check whether a character stream is already in NFKD.
#[inline]
pub fn quick_check_nfkd<I: Iterator<Item = char>>(iter: I) -> IsNormalized {
    quick_check(iter, gen::nfkd_qc)
}

/// `true` if the stream is in NFC. A `Maybe` quick-check result is resolved by
/// comparing against the fully normalized form, so the input iterator must be
/// `Clone`.
#[inline]
pub fn is_nfc<I: Iterator<Item = char> + Clone>(iter: I) -> bool {
    match quick_check(iter.clone(), gen::nfc_qc) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => iter.clone().eq(nfc(iter)),
    }
}

/// `true` if the stream is in NFD.
#[inline]
pub fn is_nfd<I: Iterator<Item = char> + Clone>(iter: I) -> bool {
    match quick_check(iter.clone(), gen::nfd_qc) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => iter.clone().eq(nfd(iter)),
    }
}

/// `true` if the stream is in NFKC.
#[inline]
pub fn is_nfkc<I: Iterator<Item = char> + Clone>(iter: I) -> bool {
    match quick_check(iter.clone(), gen::nfkc_qc) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => iter.clone().eq(nfkc(iter)),
    }
}

/// `true` if the stream is in NFKD.
#[inline]
pub fn is_nfkd<I: Iterator<Item = char> + Clone>(iter: I) -> bool {
    match quick_check(iter.clone(), gen::nfkd_qc) {
        IsNormalized::Yes => true,
        IsNormalized::No => false,
        IsNormalized::Maybe => iter.clone().eq(nfkd(iter)),
    }
}

#[cfg(all(test, feature = "alloc", feature = "bmp"))]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec::Vec;

    /// Regression for the recomposition ring overflow: a starter followed by a
    /// run of *blocked* combining marks longer than `MAX_COMBINING` must NOT
    /// silently drop the overflow marks (a normalization-stability / security
    /// bug, since NFC underpins IDNA and identifier comparison). The run far
    /// exceeds the Stream-Safe limit (30), so conformant text is unaffected, but
    /// the output must still be correct NFC: all marks preserved, canonical
    /// order. Uses a CJK starter (no composition with combining marks) so every
    /// mark is blocked and reaches the ring.
    #[test]
    fn nfc_does_not_drop_marks_past_max_combining() {
        // `中` (U+4E2D) + 65 combining acute accents (CCC 230, all blocked).
        let mut input = String::from("\u{4E2D}");
        for _ in 0..65 {
            input.push('\u{0301}');
        }
        let out: Vec<char> = nfc(input.chars()).collect();

        // Nothing dropped: 1 starter + all 65 marks.
        assert_eq!(out.len(), 66, "overflow marks were dropped");
        assert_eq!(out[0], '\u{4E2D}');
        assert!(out[1..].iter().all(|&c| c == '\u{0301}'));
        assert_eq!(out[1..].len(), 65);

        // Idempotent and canonically ordered (equal CCC, so already ordered).
        let out2: Vec<char> = nfc(out.iter().copied()).collect();
        assert_eq!(out, out2);

        // NFKC must behave identically here (no compatibility decomposition).
        let outk: Vec<char> = nfkc(input.chars()).collect();
        assert_eq!(outk, out);

        // Also exercise the task's exact `a` + 65×acute case: the first acute
        // composes into `á`, the remaining 64 are blocked; nothing is dropped.
        let mut a_input = String::from("a");
        for _ in 0..65 {
            a_input.push('\u{0301}');
        }
        let a_out: Vec<char> = nfc(a_input.chars()).collect();
        assert_eq!(a_out[0], '\u{00E1}'); // á (a + first acute)
        assert_eq!(a_out[1..].iter().filter(|&&c| c == '\u{0301}').count(), 64);
        assert_eq!(a_out.len(), 65); // no marks lost
    }

    /// Overflow with mixed CCC marks that do not compose: the marks must come
    /// out in canonical (CCC-ascending) order with none lost. Uses 40 dot-below
    /// (CCC 220) followed by 40 acute (CCC 230) on a CJK starter, exceeding the
    /// ring.
    #[test]
    fn nfc_overflow_preserves_canonical_order() {
        let mut input = String::from("\u{4E2D}");
        for _ in 0..40 {
            input.push('\u{0323}'); // CCC 220
        }
        for _ in 0..40 {
            input.push('\u{0301}'); // CCC 230
        }
        let out: Vec<char> = nfc(input.chars()).collect();
        assert_eq!(out.len(), 81); // starter + 80 marks, none dropped

        // CCC is non-decreasing across the whole output (canonical order).
        let mut prev = 0u8;
        for &c in &out {
            let cc = canonical_combining_class(c);
            if cc != 0 {
                assert!(cc >= prev, "canonical order violated");
                prev = cc;
            }
        }
        // Counts preserved per mark.
        assert_eq!(out.iter().filter(|&&c| c == '\u{0323}').count(), 40);
        assert_eq!(out.iter().filter(|&&c| c == '\u{0301}').count(), 40);
    }
}
