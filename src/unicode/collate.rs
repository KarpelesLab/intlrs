//! Unicode Collation Algorithm (UTS #10) — DUCET root collation.
//!
//! Compares strings in the Unicode default collation order. Requires the
//! `alloc` feature (collation buffers the whole string while generating
//! collation elements and sort keys).
//!
//! ```
//! # #[cfg(feature = "alloc")] {
//! use intl::unicode::collate::{compare, AlternateHandling};
//! use core::cmp::Ordering;
//!
//! // Accented letters sort just after their base letter, not by code point.
//! assert_eq!(compare("a", "z"), Ordering::Less);
//! assert_eq!(compare("café", "cafz"), Ordering::Less);
//! # }
//! ```

use super::generated::collation as gen;
use super::normalize::{canonical_combining_class as ccc, nfd};
use alloc::vec::Vec;
use core::cmp::Ordering;

// ---- Collation element (packed u64) accessors ----

#[inline]
fn primary(ce: u64) -> u16 {
    ((ce >> 32) & 0xFFFF) as u16
}
#[inline]
fn secondary(ce: u64) -> u16 {
    ((ce >> 16) & 0xFFFF) as u16
}
#[inline]
fn tertiary(ce: u64) -> u16 {
    (ce & 0xFFFF) as u16
}
#[inline]
fn is_variable(ce: u64) -> bool {
    (ce >> 48) & 1 != 0
}
#[inline]
fn pack(p: u32, s: u32, t: u32) -> u64 {
    (p as u64) << 32 | (s as u64) << 16 | t as u64
}

/// How variable collation elements (spaces, punctuation, symbols) are handled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlternateHandling {
    /// Variable elements keep their primary weight (punctuation is significant).
    NonIgnorable,
    /// Variable elements are moved to a quaternary level (punctuation is ignored
    /// at the primary level). This is the CLDR/ICU default.
    Shifted,
}

// `@implicitweights` ranges from allkeys.txt: (first, last, base, origin).
// `origin` is the start of the *first* range declaring `base`, so ranges that
// share a base (e.g. Tangut + Tangut Supplement) get a continuous BBBB offset.
const IMPLICIT_RANGES: &[(u32, u32, u32, u32)] = &[
    (0x17000, 0x187FF, 0xFB00, 0x17000),
    (0x18800, 0x18AFF, 0xFB01, 0x18800),
    (0x18D00, 0x18D7F, 0xFB00, 0x17000),
    (0x18D80, 0x18DFF, 0xFB01, 0x18800),
    (0x1B170, 0x1B2FF, 0xFB02, 0x1B170),
    (0x18B00, 0x18CFF, 0xFB03, 0x18B00),
];

/// Append the two derived (implicit) collation elements for `cp`.
fn push_implicit(cp: u32, out: &mut Vec<u64>) {
    let (aaaa, bbbb) = implicit_primaries(cp);
    out.push(pack(aaaa, 0x0020, 0x0002));
    out.push(pack(bbbb, 0x0000, 0x0000));
}

fn implicit_primaries(cp: u32) -> (u32, u32) {
    for &(first, last, base, origin) in IMPLICIT_RANGES {
        if cp >= first && cp <= last {
            return (base, (cp - origin) | 0x8000);
        }
    }
    let base = if gen::unified_ideograph(cp) {
        if (0x4E00..=0x9FFF).contains(&cp) || (0xF900..=0xFAFF).contains(&cp) {
            0xFB40
        } else {
            0xFB80
        }
    } else {
        0xFBC0
    };
    (base + (cp >> 15), (cp & 0x7FFF) | 0x8000)
}

/// Look up the collation elements for the contraction `first` + `suffix`.
fn lookup_contraction(first: u32, suffix: &[char]) -> Option<&'static [u64]> {
    for (suf, ces) in gen::contractions(first)? {
        if *suf == suffix {
            return Some(ces);
        }
    }
    None
}

/// Produce the collation element array for an NFD codepoint buffer (UCA S2.1).
fn collation_elements(mut cv: Vec<char>) -> Vec<u64> {
    let mut cea = Vec::new();
    let mut i = 0;
    while i < cv.len() {
        let s0 = cv[i] as u32;
        let mut end = i + 1;
        let mut matched: Option<&'static [u64]> = gen::ce_singles(s0);
        let mut suffix: Vec<char> = Vec::new();

        // Longest contiguous contraction (entries are sorted longest-first).
        if let Some(entries) = gen::contractions(s0) {
            for (suf, ces) in entries {
                let stop = i + 1 + suf.len();
                if stop <= cv.len() && cv[i + 1..stop] == **suf {
                    matched = Some(ces);
                    suffix = suf.to_vec();
                    end = stop;
                    break;
                }
            }
        }

        // Discontiguous extension: pull in unblocked non-starters (S2.1.1–S2.1.3).
        loop {
            let mut last_ccc = 0u8;
            let mut j = end;
            let mut hit = None;
            while j < cv.len() {
                let cc = ccc(cv[j]);
                if cc == 0 {
                    break; // starter: stop
                }
                if last_ccc < cc {
                    let mut trial = suffix.clone();
                    trial.push(cv[j]);
                    if let Some(ces) = lookup_contraction(s0, &trial) {
                        hit = Some((j, ces, trial));
                        break;
                    }
                    last_ccc = cc;
                } else {
                    break; // blocked non-starter: stop
                }
                j += 1;
            }
            match hit {
                Some((j, ces, trial)) => {
                    matched = Some(ces);
                    suffix = trial;
                    cv.remove(j);
                }
                None => break,
            }
        }

        match matched {
            Some(ces) => cea.extend_from_slice(ces),
            None => push_implicit(s0, &mut cea),
        }
        i = end;
    }
    cea
}

/// Emit synthetic collation elements for an ASCII-digit run so that numeric
/// values sort by magnitude (`"file2" < "file10"`). Encoding: a fixed marker
/// primary (placing numbers where digits sort), then the significant-digit count
/// (so shorter numbers sort first), then one primary per significant digit.
fn emit_number(digits: &[char], cea: &mut Vec<u64>) {
    // The marker = the DUCET primary of '0', so numbers keep the digit position.
    let marker = primary(collation_elements(alloc::vec!['0'])[0]) as u32;
    // Significant digits (drop leading zeros, but keep a single zero for "0").
    let first_sig = digits
        .iter()
        .position(|&c| c != '0')
        .unwrap_or(digits.len() - 1);
    let sig = &digits[first_sig..];
    cea.push(pack(marker, 0, 0));
    cea.push(pack(sig.len() as u32 + 1, 0, 0));
    for &d in sig {
        cea.push(pack((d as u32 - '0' as u32) + 1, 0, 0));
    }
}

/// Collation element array with numeric ordering: ASCII-digit runs become a
/// single magnitude-ordered element, the rest uses the normal UCA algorithm.
fn collation_elements_numeric(cv: Vec<char>) -> Vec<u64> {
    let mut cea = Vec::new();
    let mut i = 0;
    while i < cv.len() {
        if cv[i].is_ascii_digit() {
            let start = i;
            while i < cv.len() && cv[i].is_ascii_digit() {
                i += 1;
            }
            emit_number(&cv[start..i], &mut cea);
        } else {
            let start = i;
            while i < cv.len() && !cv[i].is_ascii_digit() {
                i += 1;
            }
            cea.extend(collation_elements(cv[start..i].to_vec()));
        }
    }
    cea
}

/// Collation strength — the most significant weight level that is compared.
/// Lower strengths ignore finer distinctions: [`Primary`](Strength::Primary)
/// ignores accents and case, [`Secondary`](Strength::Secondary) ignores case
/// (but not accents), [`Tertiary`](Strength::Tertiary) (the default) compares
/// everything.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    /// Level 1 only: base letters (`a` = `A` = `á`).
    Primary,
    /// Levels 1–2: accents matter, case does not (`a` = `A`, `á` ≠ `a`).
    Secondary,
    /// Levels 1–3 (default): accents and case both matter.
    Tertiary,
    /// Levels 1–4: also distinguishes shifted variable elements.
    Quaternary,
}

/// Build the sort key (a sequence of 16-bit weights) for a collation element
/// array under the given variable handling, truncated at `strength`.
fn build_sort_key(cea: &[u64], alternate: AlternateHandling, strength: Strength) -> Vec<u16> {
    let mut key = Vec::new();
    match alternate {
        AlternateHandling::NonIgnorable => {
            for &ce in cea {
                let p = primary(ce);
                if p != 0 {
                    key.push(p);
                }
            }
            if strength == Strength::Primary {
                return key;
            }
            key.push(0);
            for &ce in cea {
                let s = secondary(ce);
                if s != 0 {
                    key.push(s);
                }
            }
            if strength == Strength::Secondary {
                return key;
            }
            key.push(0);
            for &ce in cea {
                let t = tertiary(ce);
                if t != 0 {
                    key.push(t);
                }
            }
        }
        AlternateHandling::Shifted => {
            // Transform each element, tracking whether we follow a shifted
            // variable; collect (p, s, t, quaternary).
            let mut rows: Vec<(u16, u16, u16, u16)> = Vec::with_capacity(cea.len());
            let mut after_variable = false;
            for &ce in cea {
                let (p, s, t) = (primary(ce), secondary(ce), tertiary(ce));
                if is_variable(ce) && p != 0 {
                    rows.push((0, 0, 0, p)); // shifted to the quaternary level
                    after_variable = true;
                } else if p == 0 && s == 0 && t == 0 {
                    rows.push((0, 0, 0, 0)); // completely ignorable: transparent
                } else if p == 0 {
                    // Primary-ignorable (combining mark): fully shifted if it
                    // trails a variable, otherwise significant with L4 = FFFF.
                    if after_variable {
                        rows.push((0, 0, 0, 0));
                    } else {
                        rows.push((0, s, t, 0xFFFF));
                    }
                } else {
                    rows.push((p, s, t, 0xFFFF));
                    after_variable = false;
                }
            }
            for &(p, ..) in &rows {
                if p != 0 {
                    key.push(p);
                }
            }
            if strength == Strength::Primary {
                return key;
            }
            key.push(0);
            for &(_, s, ..) in &rows {
                if s != 0 {
                    key.push(s);
                }
            }
            if strength == Strength::Secondary {
                return key;
            }
            key.push(0);
            for &(_, _, t, _) in &rows {
                if t != 0 {
                    key.push(t);
                }
            }
            if strength == Strength::Tertiary {
                return key;
            }
            key.push(0);
            for &(.., q) in &rows {
                if q != 0 {
                    key.push(q);
                }
            }
        }
    }
    key
}

/// A configured collator.
#[derive(Debug, Clone, Copy)]
pub struct Collator {
    alternate: AlternateHandling,
    strength: Strength,
    numeric: bool,
}

impl Default for Collator {
    fn default() -> Self {
        Collator {
            alternate: AlternateHandling::Shifted,
            strength: Strength::Tertiary,
            numeric: false,
        }
    }
}

impl Collator {
    /// A collator with the given variable handling (and tertiary strength).
    #[must_use]
    pub fn new(alternate: AlternateHandling) -> Self {
        Collator {
            alternate,
            strength: Strength::Tertiary,
            numeric: false,
        }
    }

    /// Set the comparison [`Strength`] (e.g. [`Strength::Primary`] for
    /// accent- and case-insensitive comparison).
    #[must_use]
    pub fn with_strength(mut self, strength: Strength) -> Self {
        self.strength = strength;
        self
    }

    /// Enable **numeric** ordering (CLDR `kn`): runs of ASCII digits compare by
    /// numeric value, so `"item2"` sorts before `"item10"`.
    #[must_use]
    pub fn with_numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }

    /// The DUCET sort key for `s`: comparing two sort keys lexicographically
    /// yields the same order as [`compare`](Self::compare).
    #[must_use]
    pub fn sort_key(&self, s: &str) -> Vec<u16> {
        let cv: Vec<char> = nfd(s.chars()).collect();
        let cea = if self.numeric {
            collation_elements_numeric(cv)
        } else {
            collation_elements(cv)
        };
        build_sort_key(&cea, self.alternate, self.strength)
    }

    /// Compare two strings in DUCET collation order.
    #[must_use]
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.sort_key(a).cmp(&self.sort_key(b))
    }
}

/// Compare two strings in DUCET collation order using the default collator
/// (variable elements [shifted](AlternateHandling::Shifted)).
#[must_use]
pub fn compare(a: &str, b: &str) -> Ordering {
    Collator::default().compare(a, b)
}

/// The DUCET sort key for `s` using the default collator.
#[must_use]
pub fn sort_key(s: &str) -> Vec<u16> {
    Collator::default().sort_key(s)
}

/// A **locale-tailored** collator: the DUCET order with per-locale primary
/// reordering applied (CLDR tailoring rules). Built from a rule string such as
/// `"&z < å < ä < ö"` (Swedish), which places `å`/`ä`/`ö` immediately after `z`.
///
/// Supported relations: `<` (primary — a new sort position) and `=` (identical
/// to the previous element). Each tailored letter is given a primary weight in
/// the reserved gap just above its reset anchor; upper-case forms are added
/// automatically. Characters not mentioned keep their DUCET order.
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use intl::unicode::collate::Tailoring;
/// use core::cmp::Ordering;
/// let sv = Tailoring::parse("&z < å < ä < ö").unwrap();
/// // In Swedish, å/ä/ö sort *after* z, not near a/o.
/// assert_eq!(sv.compare("z", "å"), Ordering::Less);
/// assert_eq!(sv.compare("ä", "ö"), Ordering::Less);
/// assert_eq!(sv.compare("z", "ö"), Ordering::Less);
/// # }
/// ```
pub struct Tailoring {
    /// Tailored NFD sequences → synthetic collation element, sorted longest-first.
    entries: Vec<(Vec<char>, u64)>,
}

impl Tailoring {
    /// Parse a CLDR-style tailoring rule string (the `<` and `=` relations).
    /// Returns `None` if a reset anchor or target is malformed.
    #[must_use]
    pub fn parse(rules: &str) -> Option<Tailoring> {
        let chars: Vec<char> = rules.chars().filter(|c| !c.is_whitespace()).collect();
        let mut entries: Vec<(Vec<char>, u64)> = Vec::new();
        let mut anchor_primary = 0u32;
        let mut offset = 0u32;
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '&' => {
                    i += 1;
                    let anchor = *chars.get(i)?;
                    anchor_primary =
                        primary(collation_elements(alloc::vec![anchor]).first().copied()?) as u32;
                    offset = 0;
                    i += 1;
                }
                '<' | '=' => {
                    let primary_rel = chars[i] == '<';
                    // Consume a run of the same operator (`<<`, `<<<` → one step).
                    while i < chars.len() && (chars[i] == '<' || chars[i] == '=') {
                        i += 1;
                    }
                    let target = *chars.get(i)?;
                    i += 1;
                    if primary_rel {
                        offset += 1;
                    }
                    if anchor_primary == 0 {
                        return None; // relation before a reset
                    }
                    let p = anchor_primary + offset;
                    Self::push_letter(&mut entries, target, p);
                }
                _ => i += 1, // ignore anything else (comments, options)
            }
        }
        if entries.is_empty() {
            return None;
        }
        // Longest sequences first so contractions win during matching.
        entries.sort_by_key(|e| core::cmp::Reverse(e.0.len()));
        Some(Tailoring { entries })
    }

    /// Add tailored entries for `target` (lower form) and its upper-case form,
    /// each as its NFD sequence mapped to a primary-`p` collation element.
    fn push_letter(entries: &mut Vec<(Vec<char>, u64)>, target: char, p: u32) {
        for (ch, tert) in [(target, 0x0002u32), (upper(target), 0x0008)] {
            let seq: Vec<char> = nfd(core::iter::once(ch)).collect();
            if !seq.is_empty() {
                entries.push((seq, pack(p, 0x0020, tert)));
            }
        }
    }

    fn match_at(&self, rest: &[char]) -> Option<(usize, u64)> {
        for (seq, ce) in &self.entries {
            if rest.len() >= seq.len() && rest[..seq.len()] == seq[..] {
                return Some((seq.len(), *ce));
            }
        }
        None
    }

    /// The tailored sort key for `s`.
    #[must_use]
    pub fn sort_key(&self, s: &str) -> Vec<u16> {
        let cv: Vec<char> = nfd(s.chars()).collect();
        let mut cea = Vec::new();
        let mut buf: Vec<char> = Vec::new();
        let mut i = 0;
        while i < cv.len() {
            if let Some((len, ce)) = self.match_at(&cv[i..]) {
                if !buf.is_empty() {
                    cea.extend(collation_elements(core::mem::take(&mut buf)));
                }
                cea.push(ce);
                i += len;
            } else {
                buf.push(cv[i]);
                i += 1;
            }
        }
        if !buf.is_empty() {
            cea.extend(collation_elements(buf));
        }
        build_sort_key(&cea, AlternateHandling::Shifted, Strength::Tertiary)
    }

    /// Compare two strings in this tailored order.
    #[must_use]
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.sort_key(a).cmp(&self.sort_key(b))
    }
}

/// The upper-case form of `c` (first char of its full mapping), or `c` itself.
fn upper(c: char) -> char {
    super::case::to_uppercase(c).next().unwrap_or(c)
}
