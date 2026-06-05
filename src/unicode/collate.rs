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

// Tailoring sub-weight: a second primary component for tailored letters, stored
// in the otherwise-unused high bits of the CE (bits 49–63, above the variable
// bit). It lets the tailored sort key emit a `(base, sub)` pair per element, so
// arbitrarily many letters can be inserted immediately after a reset anchor
// (`&z < a < b < c < …`) without exhausting the DUCET inter-letter gap. Plain
// DUCET CEs have `sub == 0`.
#[inline]
fn sub_weight(ce: u64) -> u16 {
    ((ce >> 49) & 0x7FFF) as u16
}
#[inline]
fn pack_tailored(base: u32, sub: u32, s: u32, t: u32) -> u64 {
    ((sub as u64) << 49) | (base as u64) << 32 | (s as u64) << 16 | t as u64
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
fn collation_elements(cv: Vec<char>) -> Vec<u64> {
    let mut cea = Vec::new();
    each_collation_element(&cv, |ces, opt, _start| match ces {
        Some(ces) => cea.extend_from_slice(ces),
        None => push_implicit(opt, &mut cea),
    });
    cea
}

/// Core of [`collation_elements`]: walk the NFD buffer `cv` (UCA S2.1) and, for
/// each collation step, invoke `emit(Some(ces), 0, start)` with the matched
/// element slice, or `emit(None, s0, start)` when no mapping exists (caller
/// derives the implicit weights for code point `s0`). `start` is the index in
/// `cv` of the starter that began the step.
///
/// Discontiguous matching (S2.1.1–S2.1.3) consumes unblocked non-starters that
/// lie *after* the contiguous match. The previous implementation removed each
/// consumed char from `cv` with `Vec::remove` — an O(n) shift inside the loop,
/// quadratic on long combining-mark runs. Here consumed positions are marked in
/// a `consumed` bitmask instead (no shifting), and the discontiguous lookahead
/// is capped at the longest registered contraction suffix for the starter, so a
/// run of marks that forms no contraction is not rescanned repeatedly.
fn each_collation_element<F: FnMut(Option<&'static [u64]>, u32, usize)>(cv: &[char], mut emit: F) {
    let mut consumed = alloc::vec![false; cv.len()];
    let mut suffix: Vec<char> = Vec::new(); // reused buffer (no per-step clone)
    let mut i = 0;
    while i < cv.len() {
        if consumed[i] {
            i += 1;
            continue;
        }
        let s0 = cv[i] as u32;
        let mut end = i + 1;
        let mut matched: Option<&'static [u64]> = gen::ce_singles(s0);
        suffix.clear();

        // The longest registered contraction suffix for `s0` bounds how far the
        // discontiguous scan can usefully look ahead.
        let mut max_suf = 0usize;

        // Longest contiguous contraction (entries are sorted longest-first).
        if let Some(entries) = gen::contractions(s0) {
            for (suf, ces) in entries {
                if suf.len() > max_suf {
                    max_suf = suf.len();
                }
                let stop = i + 1 + suf.len();
                if stop <= cv.len() && cv[i + 1..stop] == **suf {
                    matched = Some(ces);
                    suffix.clear();
                    suffix.extend_from_slice(suf);
                    end = stop;
                    break;
                }
            }
        }

        // Discontiguous extension: pull in unblocked non-starters (S2.1.1–S2.1.3).
        // Each successful pass appends one consumable non-starter to `suffix`; we
        // cap the suffix length by `max_suf` (the longest registered contraction)
        // so a long mark run that can't form a longer contraction stops at once
        // instead of being rescanned.
        if max_suf > suffix.len() {
            loop {
                let mut last_ccc = 0u8;
                let mut j = end;
                let mut hit: Option<usize> = None;
                while j < cv.len() {
                    if consumed[j] {
                        j += 1;
                        continue;
                    }
                    let cc = ccc(cv[j]);
                    if cc == 0 {
                        break; // starter: stop
                    }
                    if last_ccc < cc {
                        suffix.push(cv[j]);
                        if let Some(ces) = lookup_contraction(s0, &suffix) {
                            matched = Some(ces);
                            hit = Some(j); // keep the pushed char in `suffix`
                            break;
                        }
                        suffix.pop();
                        last_ccc = cc;
                    } else {
                        break; // blocked non-starter: stop
                    }
                    j += 1;
                }
                match hit {
                    Some(j) => {
                        consumed[j] = true;
                        if suffix.len() >= max_suf {
                            break; // no longer contraction possible
                        }
                    }
                    None => break,
                }
            }
        }

        match matched {
            Some(ces) => emit(Some(ces), 0, i),
            None => emit(None, s0, i),
        }
        i = end;
    }
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

/// The non-ignorable primary weights of `s` (root DUCET) — the sequence used
/// for primary-strength (case- and accent-insensitive) matching.
fn primaries(s: &str) -> Vec<u16> {
    collation_elements(nfd(s.chars()).collect())
        .into_iter()
        .map(primary)
        .filter(|&p| p != 0)
        .collect()
}

/// Find the first substring of `text` that matches `pattern` at **primary
/// strength** — i.e. case- and accent-insensitively (`"CAFÉ"` matches `"cafe"`)
/// — and return its byte range, or `None`. Uses root (DUCET) collation. An empty
/// pattern matches at `0..0`. This is the collation analog of `str::find`.
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use intl::unicode::collate::find;
/// assert_eq!(find("Hello, CAFÉ!", "cafe"), Some(7..12));
/// assert_eq!(find("a naïve approach", "naive"), Some(2..8));
/// assert_eq!(find("abc", "xyz"), None);
/// # }
/// ```
#[must_use]
pub fn find(text: &str, pattern: &str) -> Option<core::ops::Range<usize>> {
    let pat = primaries(pattern);
    if pat.is_empty() {
        return Some(0..0);
    }
    // Char-boundary byte offsets, plus the end (so `bounds[k]..bounds[m]` is a
    // valid slice for any k <= m).
    let bounds: Vec<usize> = text
        .char_indices()
        .map(|(i, _)| i)
        .chain(core::iter::once(text.len()))
        .collect();
    for a in 0..bounds.len() - 1 {
        for b in a + 1..bounds.len() {
            let pr = primaries(&text[bounds[a]..bounds[b]]);
            if pr.len() < pat.len() {
                continue; // not enough weights yet — extend the candidate
            }
            if pr == pat {
                return Some(bounds[a]..bounds[b]);
            }
            break; // reached/passed the needed length without a match; advance start
        }
    }
    None
}

/// `true` if `text` contains `pattern` at primary strength (see [`find`]).
#[must_use]
pub fn contains(text: &str, pattern: &str) -> bool {
    find(text, pattern).is_some()
}

/// The first primary collation weight of `s` under `tail`, as a `(base, sub)`
/// pair packed into a `u32` (the pair-encoded primary), or `0` if `s` starts
/// with an ignorable/variable element — the value used to place a string in an
/// index bucket. Both the tailored and root (identity) tailorings emit the pair,
/// so a tailored letter (`å` → `(z, sub)`) is distinguished from its anchor.
fn first_primary(tail: &Tailoring, s: &str) -> u32 {
    let key = tail.sort_key(s);
    let base = key.first().copied().unwrap_or(0) as u32;
    if base == 0 {
        return 0;
    }
    let sub = key.get(1).copied().unwrap_or(0) as u32;
    (base << 16) | sub
}

/// The alphabetic-index bucket **labels** for `lang` — the headings under which
/// strings are grouped, in collation order: `A`–`Z` plus the locale's extra
/// letters (Swedish `Å Ä Ö`, Czech `Ch`, …). Latin-script locales only; an
/// unknown or non-Latin locale yields just `A`–`Z`.
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use intl::unicode::collate::index_labels;
/// assert_eq!(index_labels("sv").last().map(String::as_str), Some("Ö"));
/// assert_eq!(index_labels("en").len(), 26);
/// # }
/// ```
#[must_use]
pub fn index_labels(lang: &str) -> Vec<alloc::string::String> {
    use alloc::string::ToString;
    let extra: &[&str] = match lang.split(['-', '_']).next().unwrap_or(lang) {
        "sv" | "fi" => &["Å", "Ä", "Ö"],
        "da" | "nb" | "nn" | "no" => &["Æ", "Ø", "Å"],
        "is" => &["Þ", "Æ", "Ö"],
        "es" | "gl" => &["Ñ"],
        "et" => &["Š", "Ž", "Õ", "Ä", "Ö", "Ü"],
        "cs" | "sk" => &["Č", "Ch", "Ř", "Š", "Ž"],
        "pl" => &["Ą", "Ć", "Ę", "Ł", "Ń", "Ó", "Ś", "Ź", "Ż"],
        "hu" => &["Cs", "Dz", "Dzs", "Gy", "Ly", "Ny", "Sz", "Ty", "Zs"],
        "tr" | "az" => &["Ç", "Ğ", "Ö", "Ş", "Ü"],
        "ro" => &["Ă", "Â", "Î", "Ș", "Ț"],
        "sq" => &[
            "Ç", "Dh", "Ë", "Gj", "Ll", "Nj", "Rr", "Sh", "Th", "Xh", "Zh",
        ],
        "cy" => &["Ch", "Dd", "Ff", "Ng", "Ll", "Ph", "Rh", "Th"],
        _ => &[],
    };
    let tail = Tailoring::for_locale(lang).unwrap_or_else(Tailoring::identity);
    let mut labels: Vec<alloc::string::String> = ('A'..='Z')
        .map(|c| c.to_string())
        .chain(extra.iter().map(|s| s.to_string()))
        .collect();
    // Order the labels by collation, so an inserted letter lands in its real
    // place (Spanish `Ñ` after `N`, Swedish `Å Ä Ö` after `Z`).
    labels.sort_by_key(|l| first_primary(&tail, l));
    labels
}

/// The alphabetic-index bucket that `s` sorts into for `lang` (the ICU
/// `AlphabeticIndex` operation): one of [`index_labels`], or `"#"` for a string
/// that sorts before `A` (digits, symbols) or past the last label (other
/// scripts). A diacritic letter that sorts *between* two labels (like `á`) is
/// grouped under the earlier one (`A`), matching ICU.
///
/// ```
/// # #[cfg(feature = "alloc")] {
/// use intl::unicode::collate::index_bucket;
/// assert_eq!(index_bucket("en", "Apple"), "A");
/// assert_eq!(index_bucket("en", "Ångström"), "A"); // root: å ≈ a
/// assert_eq!(index_bucket("sv", "Ångström"), "Å"); // Swedish: å is its own letter
/// assert_eq!(index_bucket("en", "123"), "#");
/// # }
/// ```
#[must_use]
pub fn index_bucket(lang: &str, s: &str) -> alloc::string::String {
    use alloc::string::ToString;
    let tail = Tailoring::for_locale(lang).unwrap_or_else(Tailoring::identity);
    let sp = first_primary(&tail, s);
    if sp == 0 {
        return "#".to_string(); // sorts before A (variable/ignorable lead)
    }
    let labels = index_labels(lang);
    let mut chosen: Option<usize> = None;
    for (i, label) in labels.iter().enumerate() {
        if first_primary(&tail, label) <= sp {
            chosen = Some(i);
        } else {
            break;
        }
    }
    match chosen {
        // Past the last label and not equal to it → a later script: overflow.
        Some(i) if i == labels.len() - 1 && first_primary(&tail, &labels[i]) < sp => {
            "#".to_string()
        }
        Some(i) => labels[i].clone(),
        None => "#".to_string(),
    }
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
/// **Capacity:** tailored letters share their anchor's DUCET primary as a *base*
/// and are ordered by a second *sub-weight* component (the sort key emits a
/// `(base, sub)` pair per element), so a single reset can be followed by an
/// effectively unbounded number of consecutive primary (`<`) reorderings —
/// `&a < x₁ < x₂ < … < x₅₀` sorts correctly, between the anchor and the next
/// base letter, with no gap-exhaustion. This pair encoding is used only by the
/// tailored sort key; the root [`compare`]/[`sort_key`] path is unchanged.
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
    /// Tailored NFD sequences → collation-element sequence, sorted longest-first.
    entries: Vec<(Vec<char>, Vec<u64>)>,
}

impl Tailoring {
    /// A built-in tailoring for a locale, or `None` if none is bundled. Two
    /// sources are consulted, in order: the **official CLDR collation rules**
    /// (generated into a committed table for every locale whose rule uses the
    /// supported relations — the bulk of the coverage), then a small set of
    /// hand-written rules for locales whose CLDR rules need syntax the parser
    /// doesn't implement (`[before]`/`[import]`/extensions), e.g. the Nordic
    /// `&z < å < ä < ö`.
    ///
    /// A self-consistency gate (`tests/collation_data_consistency`) excludes from
    /// the generated table any locale whose rule the parser would mis-order
    /// (decomposing-letter anchors, chained multi-char expansions), so this never
    /// returns a tailoring that sorts against its own rule — it falls back to a
    /// hand-written rule or to root DUCET instead.
    ///
    /// ```
    /// # #[cfg(feature = "alloc")] {
    /// use intl::unicode::collate::Tailoring;
    /// use core::cmp::Ordering;
    /// let sv = Tailoring::for_locale("sv").unwrap();
    /// assert_eq!(sv.compare("z", "å"), Ordering::Less);
    /// let es = Tailoring::for_locale("es").unwrap(); // from CLDR data
    /// assert_eq!(es.compare("n", "ñ"), Ordering::Less);
    /// # }
    /// ```
    #[must_use]
    pub fn for_locale(lang: &str) -> Option<Tailoring> {
        // Normalize, then try the official CLDR rule table — first the full tag
        // (`ff-adlm`), then the primary subtag (`es`) — before the hand-written
        // fallbacks. A plain `[..2]` truncation would alias "fil" to "fi".
        let full = lang.replace('_', "-").to_ascii_lowercase();
        let primary = full.split('-').next().unwrap_or(&full);
        for key in [full.as_str(), primary] {
            if let Some(rule) = crate::cldr::collation_rule(key) {
                if let Some(t) = Tailoring::parse(rule) {
                    return Some(t);
                }
            }
        }
        let lc = primary;
        let rules = match lc {
            "sv" | "fi" => "&z < å < ä < ö",               // Swedish, Finnish
            "da" | "nb" | "nn" | "no" => "&z < æ < ø < å", // Danish, Norwegian
            "is" => "&y < ð < þ < æ < ö",                  // Icelandic
            "et" => "&s < š < z < ž < õ < ä < ö < ü",      // Estonian
            "de" => "&ae = ä &oe = ö &ue = ü &ss = ß",     // German phonebook (expansions)
            "pl" => "&a < ą &c < ć &e < ę &l < ł &n < ń &o < ó &s < ś &z < ź < ż", // Polish
            "cs" | "sk" => "&c < č &h < ch &r < ř &s < š &z < ž", // Czech/Slovak (ch digraph)
            "tr" | "az" => "&c < ç &g < ğ &h < ı &i < i̇ &o < ö &s < ş &u < ü", // Turkish/Azeri
            "lv" => "&c < č &g < ģ &i < ī &k < ķ &l < ļ &n < ņ &s < š &z < ž", // Latvian
            "lt" => "&c < č &s < š &z < ž",                // Lithuanian
            "hr" | "sr" | "bs" => "&c < č < ć &d < dž < đ &l < lj &n < nj &s < š &z < ž", // Serbo-Croatian
            "es" => "&n < ñ",                              // Spanish (ñ after n)
            // Hungarian digraphs (dzs/dz longest-match first via the engine sort).
            "hu" => "&c < cs &d < dz < dzs &g < gy &l < ly &n < ny &s < sz &t < ty &z < zs",
            "ro" => "&a < ă < â &i < î &s < ș &t < ț",     // Romanian
            "sq" => "&c < ç &d < dh &e < ë &g < gj &l < ll &n < nj &r < rr &s < sh &t < th &x < xh &z < zh", // Albanian
            "uk" => "&г < ґ &е < є &и < і < ї",            // Ukrainian (Cyrillic)
            "vi" => "&a < ă < â &d < đ &e < ê &o < ô < ơ &u < ư", // Vietnamese (base letters)
            // Welsh digraphs (ch/dd/ff/ng/ll/ph/rh/th), each after its base letter.
            "cy" => "&c < ch &d < dd &f < ff &g < ng &l < ll &p < ph &r < rh &t < th",
            "fil" | "tl" => "&n < ñ < ng",                 // Filipino/Tagalog (ng digraph)
            "fo" => "&a < á &d < ð &i < í &o < ó &u < ú &y < ý &z < æ < ø", // Faroese
            "kl" => "&z < æ < ø < å",                      // Greenlandic (Danish-style)
            "gl" => "&n < ñ",                              // Galician (ñ after n)
            "ga" => "&a < á &e < é &i < í &o < ó &u < ú",  // Irish (long-vowel accents)
            "ha" => "&b < ɓ &d < ɗ &k < ƙ &s < sh &t < ts &y < ƴ", // Hausa (hooked letters)
            _ => return None,
        };
        Tailoring::parse(rules)
    }

    /// Parse a CLDR-style tailoring rule string. Supports the `<` (primary),
    /// `<<` (secondary), `<<<` (tertiary), and `=` (identity) relations, and
    /// **expansions** when the reset anchor is a multi-character string
    /// (`"&ae = ä"` makes `ä` collate as `"ae"`). Returns `None` if a reset
    /// anchor or target is malformed.
    #[must_use]
    pub fn parse(rules: &str) -> Option<Tailoring> {
        let chars: Vec<char> = rules.chars().filter(|c| !c.is_whitespace()).collect();
        let mut entries: Vec<(Vec<char>, Vec<u64>)> = Vec::new();
        let mut anchor: Vec<char> = Vec::new();
        let mut anchor_primary = 0u32;
        // Running offsets within each level relative to the reset anchor.
        let (mut p_off, mut s_off, mut t_off) = (0u32, 0u32, 0u32);
        let mut i = 0;
        while i < chars.len() {
            match chars[i] {
                '&' => {
                    i += 1;
                    let start = i;
                    while i < chars.len() && !matches!(chars[i], '<' | '=' | '&') {
                        i += 1;
                    }
                    anchor = chars[start..i].to_vec();
                    anchor_primary = primary(*collation_elements(anchor.clone()).first()?) as u32;
                    (p_off, s_off, t_off) = (0, 0, 0);
                }
                '<' | '=' => {
                    // The number of `<`s is the relation level: 1 = primary,
                    // 2 = secondary, 3 = tertiary; `=` is identity (no change).
                    let mut level = 0u32;
                    while i < chars.len() && (chars[i] == '<' || chars[i] == '=') {
                        if chars[i] == '<' {
                            level += 1;
                        }
                        i += 1;
                    }
                    // The target may be a multi-character contraction (e.g.
                    // Czech "ch"); read up to the next operator/reset.
                    let tstart = i;
                    while i < chars.len() && !matches!(chars[i], '<' | '=' | '&') {
                        i += 1;
                    }
                    let target = &chars[tstart..i];
                    if target.is_empty() || anchor_primary == 0 {
                        return None;
                    }
                    if level == 0 {
                        // `=` identity / expansion: target collates as the anchor.
                        Self::push_expansion(&mut entries, target, &anchor);
                    } else {
                        match level {
                            1 => (p_off, s_off, t_off) = (p_off + 1, 0, 0),
                            2 => (s_off, t_off) = (s_off + 1, 0),
                            _ => t_off += 1,
                        }
                        // Tailored letters share the anchor's DUCET primary as
                        // their base and are ordered by the sub-weight `p_off`,
                        // so any number of them fit after the anchor.
                        Self::push_letter(
                            &mut entries,
                            target,
                            anchor_primary,
                            p_off,
                            s_off,
                            t_off,
                        );
                    }
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

    /// Case variants of a tailored `target`: the lower form (tertiary `0x02`),
    /// the all-upper form (`0x08`), and — for a multi-char target — the
    /// title-case form (`0x08`), so `ch`/`CH`/`Ch` all match.
    fn case_variants(target: &[char]) -> Vec<(Vec<char>, u32)> {
        let upper_all: Vec<char> = target.iter().map(|&c| upper(c)).collect();
        let mut v = alloc::vec![(target.to_vec(), 0x0002u32), (upper_all, 0x0008u32)];
        if target.len() > 1 {
            let mut title = target.to_vec();
            title[0] = upper(target[0]);
            v.push((title, 0x0008));
        }
        v
    }

    /// Map `target` (in each case form) to a single synthetic collation element
    /// at primary `p`, secondary/tertiary bumped by `s_off`/`t_off`.
    fn push_letter(
        entries: &mut Vec<(Vec<char>, Vec<u64>)>,
        target: &[char],
        base: u32,
        sub: u32,
        s_off: u32,
        t_off: u32,
    ) {
        for (form, case_t) in Self::case_variants(target) {
            let seq: Vec<char> = nfd(form.into_iter()).collect();
            if !seq.is_empty() {
                let ce = pack_tailored(base, sub, 0x0020 + s_off, case_t + t_off);
                entries.push((seq, alloc::vec![ce]));
            }
        }
    }

    /// Map `target` (lower and upper forms) to the full collation-element
    /// sequence of the `anchor` string — an expansion (`ä` → CEs of `"ae"`).
    fn push_expansion(entries: &mut Vec<(Vec<char>, Vec<u64>)>, target: &[char], anchor: &[char]) {
        let forms = [
            (target.to_vec(), anchor.to_vec()),
            (
                target.iter().map(|&c| upper(c)).collect::<Vec<_>>(),
                anchor.iter().map(|&c| upper(c)).collect::<Vec<_>>(),
            ),
        ];
        for (t_form, a_form) in forms {
            let seq: Vec<char> = nfd(t_form.into_iter()).collect();
            let ces = collation_elements(nfd(a_form.into_iter()).collect());
            if !seq.is_empty() && !ces.is_empty() {
                entries.push((seq, ces));
            }
        }
    }

    fn match_at(&self, rest: &[char]) -> Option<(usize, &[u64])> {
        for (seq, ces) in &self.entries {
            if rest.len() >= seq.len() && rest[..seq.len()] == seq[..] {
                return Some((seq.len(), ces));
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
            if let Some((len, ces)) = self.match_at(&cv[i..]) {
                if !buf.is_empty() {
                    cea.extend(collation_elements(core::mem::take(&mut buf)));
                }
                cea.extend_from_slice(ces);
                i += len;
            } else {
                buf.push(cv[i]);
                i += 1;
            }
        }
        if !buf.is_empty() {
            cea.extend(collation_elements(buf));
        }
        build_tailored_sort_key(&cea)
    }

    /// Compare two strings in this tailored order.
    #[must_use]
    pub fn compare(&self, a: &str, b: &str) -> Ordering {
        self.sort_key(a).cmp(&self.sort_key(b))
    }

    /// An empty tailoring (DUCET root order), whose [`sort_key`](Self::sort_key)
    /// uses the same pair-encoded primary level as any other tailoring — so the
    /// alphabetic index can treat root and tailored locales uniformly.
    #[must_use]
    pub fn identity() -> Tailoring {
        Tailoring {
            entries: Vec::new(),
        }
    }
}

/// Build a tailored sort key (UCA `Shifted`, tertiary strength) whose **primary
/// level emits a `(base, sub)` pair per element** — `sub` is the tailoring
/// sub-weight (0 for plain DUCET letters). This places a tailored letter
/// immediately after its anchor and after every word that merely *starts* with
/// the anchor, with no bound on how many letters share one anchor.
fn build_tailored_sort_key(cea: &[u64]) -> Vec<u16> {
    let mut rows: Vec<(u16, u16, u16, u16, u16)> = Vec::with_capacity(cea.len());
    let mut after_variable = false;
    for &ce in cea {
        let (p, sub, s, t) = (primary(ce), sub_weight(ce), secondary(ce), tertiary(ce));
        if is_variable(ce) && p != 0 {
            rows.push((0, 0, 0, 0, p)); // shifted to the quaternary level
            after_variable = true;
        } else if p == 0 && s == 0 && t == 0 {
            rows.push((0, 0, 0, 0, 0)); // completely ignorable
        } else if p == 0 {
            if after_variable {
                rows.push((0, 0, 0, 0, 0));
            } else {
                rows.push((0, 0, s, t, 0xFFFF));
            }
        } else {
            rows.push((p, sub, s, t, 0xFFFF));
            after_variable = false;
        }
    }
    let mut key = Vec::new();
    for &(p, sub, ..) in &rows {
        if p != 0 {
            key.push(p);
            key.push(sub);
        }
    }
    key.push(0);
    for &(_, _, s, ..) in &rows {
        if s != 0 {
            key.push(s);
        }
    }
    key.push(0);
    for &(_, _, _, t, _) in &rows {
        if t != 0 {
            key.push(t);
        }
    }
    key.push(0);
    for &(.., q) in &rows {
        if q != 0 {
            key.push(q);
        }
    }
    key
}

/// The upper-case form of `c` (first char of its full mapping), or `c` itself.
fn upper(c: char) -> char {
    super::case::to_uppercase(c).next().unwrap_or(c)
}

#[cfg(test)]
mod dos_fix_tests {
    use super::*;
    use alloc::string::String;

    /// Reference (pre-fix) collation-element generator, using `Vec::remove`, to
    /// confirm the bitmask rewrite is byte-identical.
    fn collation_elements_reference(mut cv: Vec<char>) -> Vec<u64> {
        let mut cea = Vec::new();
        let mut i = 0;
        while i < cv.len() {
            let s0 = cv[i] as u32;
            let mut end = i + 1;
            let mut matched: Option<&'static [u64]> = gen::ce_singles(s0);
            let mut suffix: Vec<char> = Vec::new();
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
            loop {
                let mut last_ccc = 0u8;
                let mut j = end;
                let mut hit = None;
                while j < cv.len() {
                    let cc = ccc(cv[j]);
                    if cc == 0 {
                        break;
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
                        break;
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

    // Tiny deterministic PRNG (xorshift) — no external deps.
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            self.0 = x;
            x
        }
        fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
            &xs[(self.next() as usize) % xs.len()]
        }
    }

    /// Characters chosen to exercise contractions (`l·`, `ch`), discontiguous
    /// non-starters (combining marks of varying ccc), expansions, CJK (implicit
    /// weights), digits, and ASCII.
    const ALPHABET: &[char] = &[
        'a', 'b', 'c', 'e', 'h', 'l', 'z', 'A', 'C', 'H', 'L', 'ñ', 'Ñ', 'å', 'Ç', 'ç', '·',
        '\u{0301}', '\u{0300}', '\u{0327}', '\u{0323}', '\u{0308}', 'é', 'É', '0', '1', '2', '中',
        ' ', '!', '\u{00C6}', '\u{0153}',
    ];

    fn random_string(rng: &mut Rng, len: usize) -> String {
        (0..len).map(|_| *rng.pick(ALPHABET)).collect()
    }

    #[test]
    fn collation_elements_matches_reference_fuzz() {
        let mut rng = Rng(0xD1B54A32D192ED03);
        for _ in 0..4000 {
            let len = (rng.next() as usize) % 16;
            let s = random_string(&mut rng, len);
            let cv: Vec<char> = nfd(s.chars()).collect();
            assert_eq!(
                collation_elements(cv.clone()),
                collation_elements_reference(cv),
                "CE mismatch: s={s:?}"
            );
        }
    }

    #[test]
    fn perf_smoke_long_combining_run() {
        // Previously O(n^2) in `collation_elements` via `Vec::remove`: a long run
        // of combining marks after a starter must collate quickly.
        let mut s = String::from("e");
        for _ in 0..200_000 {
            s.push('\u{0301}');
        }
        let key = sort_key(&s);
        assert!(!key.is_empty());
    }
}
