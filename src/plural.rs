//! CLDR plural rules (UTS #35): selecting the [`PluralCategory`] for a number in
//! a given language. The rules are compiled from CLDR into a `match` by the
//! offline codegen, so selection allocates nothing and needs no runtime parsing.
//!
//! ```
//! use intl::plural::{plural_category, PluralCategory, PluralOperands};
//!
//! // English: 1 is "one", everything else "other".
//! assert_eq!(plural_category("en", &PluralOperands::from_int(1)), PluralCategory::One);
//! assert_eq!(plural_category("en", &PluralOperands::from_int(2)), PluralCategory::Other);
//! // "1.0" is "other" in English (it has a visible fraction digit).
//! assert_eq!(
//!     plural_category("en", &PluralOperands::parse("1.0").unwrap()),
//!     PluralCategory::Other
//! );
//! ```

/// A CLDR plural category. `Other` is the catch-all (and the only category every
/// language defines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluralCategory {
    /// `zero`
    Zero,
    /// `one`
    One,
    /// `two`
    Two,
    /// `few`
    Few,
    /// `many`
    Many,
    /// `other`
    Other,
}

/// The UTS #35 plural operands of a number: `n` (value), `i` (integer digits),
/// `v`/`w` (count of fraction digits with / without trailing zeros), `f`/`t`
/// (those fraction digits as an integer, with / without trailing zeros), and `c`
/// (the compact decimal exponent).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PluralOperands {
    /// Absolute numeric value.
    pub n: f64,
    /// Integer digits.
    pub i: u64,
    /// Number of visible fraction digits, with trailing zeros.
    pub v: u32,
    /// Number of visible fraction digits, without trailing zeros.
    pub w: u32,
    /// Visible fraction digits, with trailing zeros, as an integer.
    pub f: u64,
    /// Visible fraction digits, without trailing zeros, as an integer.
    pub t: u64,
    /// The compact decimal exponent (0 if none).
    pub c: u32,
}

impl PluralOperands {
    /// Operands for an integer value (no fraction digits).
    #[must_use]
    pub const fn from_int(value: i64) -> Self {
        let i = value.unsigned_abs();
        PluralOperands {
            n: i as f64,
            i,
            v: 0,
            w: 0,
            f: 0,
            t: 0,
            c: 0,
        }
    }

    /// Parse operands from a decimal string such as `"1.230"` (the trailing
    /// zeros matter: they set `v`/`f`). An optional leading `-` is ignored
    /// (operands use the absolute value). Returns `None` if not a decimal
    /// number. Compact exponents are not parsed (`c` is always 0).
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let s = s.strip_prefix(['-', '+']).unwrap_or(s);
        let (int_str, frac_str) = match s.split_once('.') {
            Some((a, b)) => (a, b),
            None => (s, ""),
        };
        if int_str.is_empty() || !int_str.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        if !frac_str.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let i: u64 = int_str.parse().ok()?;
        let v = frac_str.len() as u32;
        let f: u64 = if frac_str.is_empty() {
            0
        } else {
            frac_str.parse().ok()?
        };
        let trimmed = frac_str.trim_end_matches('0');
        let w = trimmed.len() as u32;
        let t: u64 = if trimmed.is_empty() {
            0
        } else {
            trimmed.parse().ok()?
        };
        let n: f64 = s.parse().ok()?;
        Some(PluralOperands {
            n,
            i,
            v,
            w,
            f,
            t,
            c: 0,
        })
    }
}

/// `true` if `x` is an integer that lies in one of the inclusive `ranges`. (Used
/// by the generated rule code; plural relations only match integer values.)
#[doc(hidden)]
#[must_use]
pub fn in_set(x: f64, ranges: &[(f64, f64)]) -> bool {
    // `f64::fract` lives in `std`; `% 1.0` is a core operator, so use it to test
    // integrality without pulling in `std`.
    x % 1.0 == 0.0 && ranges.iter().any(|&(a, b)| x >= a && x <= b)
}

/// Select the [`PluralCategory`] (cardinal) for `operands` in the language of
/// `lang` (a BCP-47 tag). The most specific known locale is used: the full tag
/// is tried first, then progressively shorter prefixes (so `"pt-BR"` falls back
/// to `"pt"`). Unknown languages resolve to [`PluralCategory::Other`].
#[must_use]
pub fn plural_category(lang: &str, operands: &PluralOperands) -> PluralCategory {
    select(
        operands,
        lang,
        crate::unicode::generated::plurals::plural_category,
    )
}

/// Select the [`PluralCategory`] (ordinal — "1st", "2nd", "3rd") for `operands`
/// in the language of `lang`, with the same locale fallback as
/// [`plural_category`]. Languages without ordinal rules resolve to
/// [`PluralCategory::Other`].
#[must_use]
pub fn ordinal_category(lang: &str, operands: &PluralOperands) -> PluralCategory {
    select(
        operands,
        lang,
        crate::unicode::generated::plurals::ordinal_category,
    )
}

/// Shared locale-fallback dispatch for the cardinal/ordinal generated tables.
fn select(
    operands: &PluralOperands,
    lang: &str,
    lookup: fn(&str, &PluralOperands) -> Option<PluralCategory>,
) -> PluralCategory {
    // Case-fold and normalize separators into a small stack buffer (no alloc).
    let mut buf = [0u8; 40];
    let bytes = lang.as_bytes();
    let len = bytes.len().min(buf.len());
    for k in 0..len {
        let b = bytes[k].to_ascii_lowercase();
        buf[k] = if b == b'_' { b'-' } else { b };
    }
    let norm = core::str::from_utf8(&buf[..len]).unwrap_or("");

    let mut end = norm.len();
    loop {
        if let Some(cat) = lookup(&norm[..end], operands) {
            return cat;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return PluralCategory::Other,
        }
    }
}
