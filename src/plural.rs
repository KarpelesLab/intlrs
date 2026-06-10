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
    /// (operands use the absolute value). A compact/scientific exponent is
    /// accepted (`"1.2c6"` or `"1.2e6"` = 1 200 000, setting `c` = `e` = 6).
    /// Returns `None` if not a number.
    ///
    /// ```
    /// use intl::plural::PluralOperands;
    /// let c = PluralOperands::parse("1.2c6").unwrap();
    /// assert_eq!(c.i, 1_200_000);
    /// assert_eq!(c.c, 6);
    /// assert_eq!(c.v, 0); // the expanded value is an integer
    /// ```
    #[must_use]
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        let s = s.strip_prefix(['-', '+']).unwrap_or(s);
        // Compact/scientific exponent: expand the mantissa into a stack buffer
        // (this module is `no_std`/no-alloc), then parse the plain decimal.
        if let Some((mantissa, exp_str)) = s.split_once(['c', 'e', 'C', 'E']) {
            let exp: u32 = exp_str.parse().ok()?;
            if exp > 30 {
                return None;
            }
            let mut buf = [0u8; 48];
            let expanded = expand_compact(&mut buf, mantissa, exp as usize)?;
            let mut ops = Self::parse_plain(expanded)?;
            ops.c = exp;
            return Some(ops);
        }
        Self::parse_plain(s)
    }

    fn parse_plain(s: &str) -> Option<Self> {
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

/// Write the plain decimal form of `mantissa × 10^exp` into `buf` and return it
/// as a `&str` (no allocation). Shifts the decimal point right by `exp` digits.
fn expand_compact<'a>(buf: &'a mut [u8], mantissa: &str, exp: usize) -> Option<&'a str> {
    let (int_str, frac_str) = match mantissa.split_once('.') {
        Some((a, b)) => (a, b),
        None => (mantissa, ""),
    };
    if int_str.is_empty() || !int_str.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    if !frac_str.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let point = int_str.len() + exp; // digit count before the decimal afterwards
    let mut pos = 0usize;
    let mut idx = 0usize;
    for &b in int_str.as_bytes().iter().chain(frac_str.as_bytes()) {
        if idx == point {
            *buf.get_mut(pos)? = b'.';
            pos += 1;
        }
        *buf.get_mut(pos)? = b;
        pos += 1;
        idx += 1;
    }
    while idx < point {
        *buf.get_mut(pos)? = b'0'; // pad integer part with trailing zeros
        pos += 1;
        idx += 1;
    }
    core::str::from_utf8(&buf[..pos]).ok()
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
    // Truncate on a char boundary so an over-long, multibyte-prefixed tag keeps
    // a valid (shorter) prefix instead of failing UTF-8 validation and falling
    // back to "" (which would silently resolve to the wrong/root locale).
    let mut buf = [0u8; 40];
    let mut len = 0;
    for ch in lang.chars() {
        let w = ch.len_utf8();
        if len + w > buf.len() {
            break;
        }
        ch.encode_utf8(&mut buf[len..len + w]);
        if ch == '_' {
            buf[len] = b'-';
        } else {
            buf[len..len + w].make_ascii_lowercase();
        }
        len += w;
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
