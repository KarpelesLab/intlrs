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
        }
    }

    fn push_back(&mut self, c: char) {
        if self.len < MAX_COMBINING {
            self.ring[(self.head + self.len) % MAX_COMBINING] = c;
            self.len += 1;
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
                                    self.push_back(ch);
                                    self.last_ccc = Some(ch_class);
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
                                    self.push_back(ch);
                                    self.last_ccc = Some(ch_class);
                                } else {
                                    match compose(k, ch) {
                                        Some(r) => {
                                            self.composee = Some(r);
                                            continue;
                                        }
                                        None => {
                                            self.push_back(ch);
                                            self.last_ccc = Some(ch_class);
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
