//! Algorithmically-derived character names. Requires the `alloc` feature.
//!
//! The full Unicode `Name` database is large and not embedded; this module
//! provides the names that are *computed* rather than tabulated — currently the
//! Hangul syllables, whose names follow directly from their jamo decomposition.

use alloc::string::String;

// Hangul syllable composition constants (UAX #15 / The Unicode Standard §3.12).
const S_BASE: u32 = 0xAC00;
const L_COUNT: u32 = 19;
const V_COUNT: u32 = 21;
const T_COUNT: u32 = 28;
const N_COUNT: u32 = V_COUNT * T_COUNT; // 588
const S_COUNT: u32 = L_COUNT * N_COUNT; // 11172

const JAMO_L: [&str; 19] = [
    "G", "GG", "N", "D", "DD", "R", "M", "B", "BB", "S", "SS", "", "J", "JJ", "C", "K", "T", "P",
    "H",
];
const JAMO_V: [&str; 21] = [
    "A", "AE", "YA", "YAE", "EO", "E", "YEO", "YE", "O", "WA", "WAE", "OE", "YO", "U", "WEO", "WE",
    "WI", "YU", "EU", "YI", "I",
];
const JAMO_T: [&str; 28] = [
    "", "G", "GG", "GS", "N", "NJ", "NH", "D", "L", "LG", "LM", "LB", "LS", "LT", "LP", "LH", "M",
    "B", "BS", "S", "SS", "NG", "J", "C", "K", "T", "P", "H",
];

/// The Unicode `Name` of a precomposed Hangul syllable (`U+AC00`–`U+D7A3`), e.g.
/// `hangul_syllable_name('한')` → `"HANGUL SYLLABLE HAN"`. Returns `None` for any
/// other character.
///
/// ```
/// use intl::unicode::hangul_syllable_name;
/// assert_eq!(hangul_syllable_name('가').as_deref(), Some("HANGUL SYLLABLE GA"));
/// assert_eq!(hangul_syllable_name('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
/// assert_eq!(hangul_syllable_name('A'), None);
/// ```
#[must_use]
pub fn hangul_syllable_name(c: char) -> Option<String> {
    let s_index = (c as u32).checked_sub(S_BASE)?;
    if s_index >= S_COUNT {
        return None;
    }
    let l = (s_index / N_COUNT) as usize;
    let v = (s_index % N_COUNT / T_COUNT) as usize;
    let t = (s_index % T_COUNT) as usize;
    let mut name = String::from("HANGUL SYLLABLE ");
    name.push_str(JAMO_L[l]);
    name.push_str(JAMO_V[v]);
    name.push_str(JAMO_T[t]);
    Some(name)
}

/// The Unicode `Name` of `c` **when it is derived algorithmically** — Hangul
/// syllables and the unified-ideograph ranges (CJK, Tangut, Khitan, Nüshu),
/// whose names are computed from the codepoint rather than tabulated. Returns
/// `None` for characters whose name lives in the (unembedded) Name database,
/// such as Latin letters or punctuation.
///
/// ```
/// use intl::unicode::char_name;
/// assert_eq!(char_name('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
/// assert_eq!(char_name('一').as_deref(), Some("CJK UNIFIED IDEOGRAPH-4E00"));
/// assert_eq!(char_name('A'), None); // tabulated name, not embedded
/// ```
#[must_use]
pub fn char_name(c: char) -> Option<String> {
    if let Some(n) = hangul_syllable_name(c) {
        return Some(n);
    }
    let prefix = crate::unicode::generated::properties::ideograph_name_prefix(c as u32)?;
    let mut name = String::from(prefix);
    name.push_str(&alloc::format!("{:04X}", c as u32));
    Some(name)
}
