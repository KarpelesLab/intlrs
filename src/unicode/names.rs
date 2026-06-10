//! Character names (`alloc`). The *algorithmic* names ([`char_name`],
//! [`hangul_syllable_name`]) are always available and need no data. The full
//! tabulated [`name`] database (every explicitly-named codepoint) is behind the
//! **`names`** feature, which embeds a ~1.3 MB table (`names.bin`).

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

#[cfg(feature = "names")]
const NAMES: &[u8] = include_bytes!("names.bin");

/// Look up the tabulated `Name` of a codepoint in `names.bin` (binary search).
///
/// `names.bin` is a trusted compile-time constant, but every read here is bounds-
/// checked anyway: a truncated, regenerated, or otherwise internally inconsistent
/// blob fails gracefully with `None` rather than panicking. The untrusted `cp`
/// argument only drives binary-search comparisons and never indexes the blob.
#[cfg(feature = "names")]
fn tabulated_name(cp: u32) -> Option<&'static str> {
    // Read a little-endian u32 at byte offset `o`, returning `None` if the
    // 4-byte window falls outside the blob.
    let rd = |o: usize| -> Option<u32> {
        let end = o.checked_add(4)?;
        let bytes = NAMES.get(o..end)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    };

    // Header: [count][count cp keys][count+1 offsets][name bytes].
    let count = rd(0)? as usize;
    let cp_base = 4usize;
    // off_base = cp_base + count * 4
    let off_base = count.checked_mul(4)?.checked_add(cp_base)?;
    // data_base = off_base + (count + 1) * 4
    let data_base = count
        .checked_add(1)?
        .checked_mul(4)?
        .checked_add(off_base)?;
    // Validate that the entire offset table lies within the blob before searching.
    if data_base > NAMES.len() {
        return None;
    }

    let (mut lo, mut hi) = (0usize, count);
    while lo < hi {
        let mid = (lo + hi) / 2;
        let key = rd(cp_base + mid * 4)?;
        match key.cmp(&cp) {
            core::cmp::Ordering::Less => lo = mid + 1,
            core::cmp::Ordering::Greater => hi = mid,
            core::cmp::Ordering::Equal => {
                let o0 = rd(off_base + mid * 4)? as usize;
                let o1 = rd(off_base + (mid + 1) * 4)? as usize;
                // The name spans data_base + o0 .. data_base + o1; reject any
                // inconsistent range so the slice below cannot panic.
                if o0 > o1 {
                    return None;
                }
                let start = data_base.checked_add(o0)?;
                let stop = data_base.checked_add(o1)?;
                let slice = NAMES.get(start..stop)?;
                return core::str::from_utf8(slice).ok();
            }
        }
    }
    None
}

/// The full Unicode `Name` of `c` — the algorithmic names ([`char_name`]) plus
/// the tabulated database for every explicitly-named codepoint. Requires the
/// **`names`** feature (which embeds a ~1.3 MB table). Returns `None` only for
/// unnamed codepoints (control characters, unassigned, private use).
///
/// ```
/// use intl::unicode::name;
/// assert_eq!(name('A').as_deref(), Some("LATIN CAPITAL LETTER A"));
/// assert_eq!(name('€').as_deref(), Some("EURO SIGN"));
/// assert_eq!(name('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
/// assert_eq!(name('一').as_deref(), Some("CJK UNIFIED IDEOGRAPH-4E00"));
/// ```
#[cfg(feature = "names")]
#[must_use]
pub fn name(c: char) -> Option<String> {
    if let Some(n) = char_name(c) {
        return Some(n);
    }
    tabulated_name(c as u32).map(String::from)
}

#[cfg(all(test, feature = "names"))]
mod tests {
    use super::name;

    #[test]
    fn known_names_resolve() {
        // Tabulated (blob) names.
        assert_eq!(name('A').as_deref(), Some("LATIN CAPITAL LETTER A"));
        assert_eq!(name('€').as_deref(), Some("EURO SIGN"));
        // Algorithmic names (CJK ideograph + Hangul syllable) must be unaffected.
        assert_eq!(name('一').as_deref(), Some("CJK UNIFIED IDEOGRAPH-4E00"));
        assert_eq!(name('한').as_deref(), Some("HANGUL SYLLABLE HAN"));
        // Unnamed codepoint (control char) yields None.
        assert_eq!(name('\u{0007}'), None);
    }
}
