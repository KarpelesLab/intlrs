//! Locale-aware decimal and percent number formatting (CLDR / UTS #35).
//! Requires the `alloc` feature.
//!
//! Driven by CLDR number symbols and patterns compiled into a table by the
//! offline codegen (a curated set of locales; unknown locales fall back to the
//! root convention, which matches English).
//!
//! ```
//! use intl::number::{format_decimal, format_percent};
//! assert_eq!(format_decimal("en", 1234.5), "1,234.5");
//! assert_eq!(format_decimal("de", 1234.5), "1.234,5");
//! assert_eq!(format_decimal("hi", 1234567.0), "12,34,567"); // Indian grouping
//! assert_eq!(format_percent("en", 0.5), "50%");
//! assert_eq!(format_percent("de", 0.5), "50\u{a0}%");
//! ```

use alloc::string::String;

pub use crate::cldr::{NumberSpec, Pattern};

/// Resolve the [`NumberSpec`] for `lang`, walking up the locale fallback chain
/// and finally to the root (English) convention.
fn spec(lang: &str) -> NumberSpec {
    use crate::cldr::number_spec;
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    loop {
        if let Some(s) = number_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return number_spec("en").expect("root spec present"),
        }
    }
}

/// Format `value` as a decimal number in the conventions of `lang`.
#[must_use]
pub fn format_decimal(lang: &str, value: f64) -> String {
    let s = spec(lang);
    format_with(&s.dec, value, s.decimal, s.group, s.minus)
}

/// Format `value` (a ratio, so `0.5` → `50%`) as a percent in `lang`.
#[must_use]
pub fn format_percent(lang: &str, value: f64) -> String {
    let s = spec(lang);
    format_with(&s.pct, value * 100.0, s.decimal, s.group, s.minus)
}

/// Format `value` in scientific notation (mantissa × 10ⁿ) in `lang`, e.g.
/// `format_scientific("en", 12345.0)` → `"1.2345E4"`. The mantissa uses the
/// locale decimal separator and is rounded to at most `1 + sig_after` digits
/// (trailing zeros trimmed); `0` is rendered as `"0"`.
///
/// ```
/// use intl::number::format_scientific;
/// assert_eq!(format_scientific("en", 12345.0, 6), "1.2345E4");
/// assert_eq!(format_scientific("de", 0.00042, 6), "4,2E-4");
/// assert_eq!(format_scientific("en", 0.0, 6), "0");
/// ```
#[must_use]
pub fn format_scientific(lang: &str, value: f64, sig_after: usize) -> String {
    if value == 0.0 {
        return String::from("0");
    }
    let s = spec(lang);
    let neg = value < 0.0;
    let mut m = if neg { -value } else { value };
    // Normalize the mantissa to 1 ≤ m < 10 without `std::f64::log10`.
    let mut exp = 0i32;
    while m >= 10.0 {
        m /= 10.0;
        exp += 1;
    }
    while m < 1.0 {
        m *= 10.0;
        exp -= 1;
    }
    let mantissa = alloc::format!("{:.*}", sig_after, m);
    let (int_part, frac_full) = mantissa.split_once('.').unwrap_or((&mantissa, ""));
    let frac = frac_full.trim_end_matches('0');

    let mut out = String::new();
    if neg {
        out.push_str(s.minus);
    }
    out.push_str(int_part);
    if !frac.is_empty() {
        out.push_str(s.decimal);
        out.push_str(frac);
    }
    out.push('E');
    if exp < 0 {
        out.push_str(s.minus);
    }
    out.push_str(&alloc::format!("{}", exp.unsigned_abs()));
    out
}

/// Format `n` as an ordinal in `lang`, e.g. `format_ordinal("en", 21)` →
/// `"21st"`, `format_ordinal("fr", 1)` → `"1er"`, `format_ordinal("de", 2)` →
/// `"2."`. The suffix is chosen by the CLDR **ordinal** plural category of `n`.
///
/// ```
/// use intl::number::format_ordinal;
/// assert_eq!(format_ordinal("en", 1), "1st");
/// assert_eq!(format_ordinal("en", 2), "2nd");
/// assert_eq!(format_ordinal("en", 3), "3rd");
/// assert_eq!(format_ordinal("en", 4), "4th");
/// assert_eq!(format_ordinal("en", 21), "21st");
/// ```
#[must_use]
pub fn format_ordinal(lang: &str, n: i64) -> String {
    use crate::plural::{ordinal_category, PluralOperands};
    let cat = ordinal_category(lang, &PluralOperands::from_int(n)) as usize;
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let suffix = loop {
        if let Some(s) = crate::cldr::ordinal_suffix(&norm[..end], cat) {
            break s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::ordinal_suffix("en", cat).unwrap_or(""),
        }
    };
    let mut out = format_decimal(lang, n as f64);
    out.push_str(suffix);
    out
}

/// Transliterate the ASCII digits `0`–`9` in `s` to the glyphs of the named
/// numbering `system` (e.g. `"arab"`, `"deva"`). Non-digit characters and
/// unknown systems are left unchanged.
///
/// ```
/// use intl::number::to_numbering_system;
/// assert_eq!(to_numbering_system("2024", "arab"), "٢٠٢٤");
/// assert_eq!(to_numbering_system("3.14", "deva"), "३.१४");
/// ```
#[must_use]
pub fn to_numbering_system(s: &str, system: &str) -> String {
    let Some(glyphs) = crate::cldr::numbering_digits(system) else {
        return String::from(s);
    };
    let table: alloc::vec::Vec<char> = glyphs.chars().collect();
    if table.len() != 10 {
        return String::from(s);
    }
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() {
                table[(c as u8 - b'0') as usize]
            } else {
                c
            }
        })
        .collect()
}

/// Format `value` as a decimal in `lang`, using the locale's default numbering
/// system (so e.g. Persian renders with Extended Arabic-Indic digits). Most
/// locales default to Latin digits, where this matches [`format_decimal`].
#[must_use]
pub fn format_decimal_native(lang: &str, value: f64) -> String {
    let formatted = format_decimal(lang, value);
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let system = loop {
        if let Some(s) = crate::cldr::default_numbering(&norm[..end]) {
            break s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break "latn",
        }
    };
    if system == "latn" {
        formatted
    } else {
        to_numbering_system(&formatted, system)
    }
}

/// Format `value` in compact (short) form in `lang`, e.g.
/// `format_compact("en", 1500.0)` → `"1.5K"`, `format_compact("en", 2_300_000.0)`
/// → `"2.3M"`. Values below 1000 (or magnitudes the locale does not abbreviate)
/// fall back to [`format_decimal`].
///
/// ```
/// use intl::number::format_compact;
/// assert_eq!(format_compact("en", 1500.0), "1.5K");
/// assert_eq!(format_compact("en", 2_300_000.0), "2.3M");
/// assert_eq!(format_compact("en", 999.0), "999");
/// ```
#[must_use]
pub fn format_compact(lang: &str, value: f64) -> String {
    let abs = if value < 0.0 { -value } else { value };
    // Below 1000, non-finite (NaN/∞), so the magnitude exponent is well-defined
    // and the `exp - 3` index below cannot underflow.
    if !abs.is_finite() || abs < 1000.0 {
        return format_decimal(lang, value);
    }
    let s = spec(lang);
    // Resolve the compact pattern table through the locale fallback chain.
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut end = norm.len();
    let table = loop {
        if let Some(t) = crate::cldr::compact_patterns(&norm[..end]) {
            break t;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break crate::cldr::compact_patterns("en").expect("root compact present"),
        }
    };

    // Magnitude exponent (3..=14) without `std::f64::log10`.
    let mut exp = 0usize;
    let mut t = abs;
    while t >= 10.0 && exp < 14 {
        t /= 10.0;
        exp += 1;
    }
    let pattern = table[(exp - 3).min(11)];
    let zeros = pattern.chars().filter(|&c| c == '0').count();
    // A pattern of only `0`s (no magnitude suffix) means "do not abbreviate".
    let has_suffix = pattern
        .chars()
        .any(|c| c != '0' && c != '\'' && !c.is_whitespace());
    if zeros == 0 || !has_suffix {
        return format_decimal(lang, value);
    }
    let mut divisor = 1.0f64; // 10^(exp + 1 - zeros), without std::f64::powi
    for _ in 0..(exp + 1).saturating_sub(zeros) {
        divisor *= 10.0;
    }
    let mantissa = value / divisor;
    // One fraction digit, trailing zero trimmed.
    let m = alloc::format!("{mantissa:.1}");
    let (mi, mf) = m.split_once('.').unwrap_or((&m, ""));
    let mf = mf.trim_end_matches('0');

    // Render the pattern: replace the `0`-run with the number; `'…'` is literal.
    let mut out = String::new();
    let mut wrote_num = false;
    let mut chars = pattern.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '0' => {
                while chars.peek() == Some(&'0') {
                    chars.next();
                }
                if !wrote_num {
                    out.push_str(mi);
                    if !mf.is_empty() {
                        out.push_str(s.decimal);
                        out.push_str(mf);
                    }
                    wrote_num = true;
                }
            }
            '\'' => {
                for q in chars.by_ref() {
                    if q == '\'' {
                        break;
                    }
                    out.push(q);
                }
            }
            other => out.push(other),
        }
    }
    out
}

/// Parse a number written in `lang`'s conventions back to an `f64` — the inverse
/// of [`format_decimal`]: grouping separators are removed and the locale decimal
/// separator is accepted. A leading minus sign (ASCII `-` or the locale's) is
/// honored. Returns `None` if the remaining text is not a number.
///
/// ```
/// use intl::number::parse_decimal;
/// assert_eq!(parse_decimal("en", "1,234.5"), Some(1234.5));
/// assert_eq!(parse_decimal("de", "1.234,5"), Some(1234.5));
/// assert_eq!(parse_decimal("fr", "-1\u{202f}234,5"), Some(-1234.5));
/// assert_eq!(parse_decimal("en", "abc"), None);
/// ```
#[must_use]
pub fn parse_decimal(lang: &str, input: &str) -> Option<f64> {
    parse_decimal_with(&spec(lang), input)
}

/// Inner parser for [`parse_decimal`], split out so the separator-progress guard
/// can be exercised against a synthetic [`NumberSpec`] in unit tests.
fn parse_decimal_with(s: &NumberSpec, input: &str) -> Option<f64> {
    let mut out = String::with_capacity(input.len());
    let mut rest = input.trim();
    if let Some(r) = rest
        .strip_prefix(s.minus)
        .or_else(|| rest.strip_prefix('-'))
    {
        out.push('-');
        rest = r;
    }
    // Walk the rest, dropping group separators and normalizing the decimal point.
    let mut seen_point = false;
    while !rest.is_empty() {
        // Guard against empty separators: `str::strip_prefix("")` returns
        // `Some` without consuming input, which would stall the loop forever.
        if let Some(r) = (!s.group.is_empty())
            .then(|| rest.strip_prefix(s.group))
            .flatten()
        {
            rest = r;
        } else if !seen_point {
            if let Some(r) = (!s.decimal.is_empty())
                .then(|| rest.strip_prefix(s.decimal))
                .flatten()
            {
                out.push('.');
                seen_point = true;
                rest = r;
                continue;
            } else {
                let c = rest.chars().next()?;
                if !c.is_ascii_digit() {
                    return None;
                }
                out.push(c);
                rest = &rest[c.len_utf8()..];
            }
        } else {
            let c = rest.chars().next()?;
            if !c.is_ascii_digit() {
                return None;
            }
            out.push(c);
            rest = &rest[c.len_utf8()..];
        }
    }
    out.parse().ok()
}

/// Format `value` as an amount in the currency `code` (ISO 4217, e.g. `"USD"`)
/// using the conventions of `lang`. The fraction-digit count follows the
/// currency (e.g. `JPY` has none), and the currency symbol is localized.
///
/// ```
/// use intl::number::format_currency;
/// assert_eq!(format_currency("en", 1234.5, "USD"), "$1,234.50");
/// assert_eq!(format_currency("de", 1234.5, "EUR"), "1.234,50\u{a0}€");
/// assert_eq!(format_currency("ja", 1234.0, "JPY"), "￥1,234"); // no fraction digits
/// ```
#[must_use]
pub fn format_currency(lang: &str, value: f64, code: &str) -> String {
    use crate::cldr as cur;
    let s = spec(lang);

    // Resolve the currency pattern and symbol through the locale fallback chain.
    let norm: String = lang
        .chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut pat = cur::currency_pattern("en").expect("root currency pattern");
    let mut symbol = code;
    let mut end = norm.len();
    let (mut got_pat, mut got_sym) = (false, false);
    loop {
        if !got_pat {
            if let Some(p) = cur::currency_pattern(&norm[..end]) {
                pat = p;
                got_pat = true;
            }
        }
        if !got_sym {
            if let Some(sym) = cur::currency_symbol(&norm[..end], code) {
                symbol = sym;
                got_sym = true;
            }
        }
        if got_pat && got_sym {
            break;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break,
        }
    }
    // Root fallback (English) for anything the locale chain didn't supply.
    if !got_sym {
        if let Some(sym) = cur::currency_symbol("en", code) {
            symbol = sym;
        }
    }

    let digits = cur::currency_digits(code);
    pat.min_frac = digits;
    pat.max_frac = digits;

    let formatted = format_with(&pat, value, s.decimal, s.group, s.minus);
    // The pattern carries the ¤ placeholder; replace it with the symbol.
    formatted.replace('\u{a4}', symbol)
}

fn format_with(p: &Pattern, value: f64, decimal: &str, group: &str, minus: &str) -> String {
    let neg = value.is_sign_negative() && value != 0.0;
    let abs = if value < 0.0 { -value } else { value };

    // Round to max_frac fixed decimals via the float formatter.
    let formatted = alloc::format!("{:.*}", p.max_frac as usize, abs);
    let (int_str, frac_full) = match formatted.split_once('.') {
        Some((a, b)) => (a, b),
        None => (formatted.as_str(), ""),
    };

    // Left-pad the integer to the minimum digit count. Compare in `usize` (not
    // `as u8`, which would truncate for >255-digit values and could underflow
    // the subtraction below).
    let mut int_owned;
    let int_str: &str = if int_str.len() < p.min_int as usize {
        int_owned = String::new();
        for _ in 0..(p.min_int as usize - int_str.len()) {
            int_owned.push('0');
        }
        int_owned.push_str(int_str);
        &int_owned
    } else {
        int_str
    };

    // Trim trailing zeros from the fraction down to the minimum count.
    let mut frac = frac_full;
    while frac.len() > p.min_frac as usize && frac.ends_with('0') {
        frac = &frac[..frac.len() - 1];
    }

    let grouped = group_digits(int_str, p.primary_group, p.secondary_group, group);

    let mut out = String::new();
    if neg {
        out.push_str(minus);
    }
    out.push_str(p.prefix);
    out.push_str(&grouped);
    if !frac.is_empty() {
        out.push_str(decimal);
        out.push_str(frac);
    }
    out.push_str(p.suffix);
    out
}

/// Insert `sep` into `digits` per the primary/secondary grouping sizes.
fn group_digits(digits: &str, primary: u8, secondary: u8, sep: &str) -> String {
    if primary == 0 || digits.len() <= primary as usize {
        return String::from(digits);
    }
    let chars: alloc::vec::Vec<char> = digits.chars().collect();
    let mut rev: alloc::vec::Vec<char> = alloc::vec::Vec::new();
    let mut count = 0u8;
    let mut limit = primary;
    for &c in chars.iter().rev() {
        if count == limit {
            for sc in sep.chars().rev() {
                rev.push(sc);
            }
            count = 0;
            limit = secondary;
        }
        rev.push(c);
        count += 1;
    }
    rev.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A `NumberSpec` with **empty** group/decimal separators — not producible by
    /// the real (curated, non-empty-separator) data, used only to prove the
    /// `parse_decimal` loop always makes progress and cannot hang.
    fn empty_sep_spec() -> NumberSpec {
        let pat = Pattern {
            prefix: "",
            suffix: "",
            min_int: 1,
            min_frac: 0,
            max_frac: 3,
            primary_group: 3,
            secondary_group: 3,
        };
        NumberSpec {
            decimal: "",
            group: "",
            minus: "-",
            plus: "+",
            percent: "%",
            dec: pat,
            pct: pat,
        }
    }

    #[test]
    fn parse_empty_separators_does_not_hang() {
        // With empty separators the guard skips the (otherwise non-advancing)
        // `strip_prefix("")` and consumes input one digit at a time. This must
        // terminate and parse the bare digits.
        let s = empty_sep_spec();
        assert_eq!(parse_decimal_with(&s, "1234"), Some(1234.0));
        assert_eq!(parse_decimal_with(&s, "-42"), Some(-42.0));
        // A separator/non-digit it can't normalize: still terminates, returns None.
        assert_eq!(parse_decimal_with(&s, "1.5"), None);
        assert_eq!(parse_decimal_with(&s, "abc"), None);
    }

    #[test]
    fn parse_real_locales_unchanged() {
        // Real (non-empty-separator) behavior is preserved by the guard.
        assert_eq!(parse_decimal("en", "1,234.5"), Some(1234.5));
        assert_eq!(parse_decimal("de", "1.234,5"), Some(1234.5));
        assert_eq!(parse_decimal("en", "-7.0"), Some(-7.0));
        assert_eq!(parse_decimal("en", "abc"), None);
    }

    #[test]
    fn compact_width_saturates() {
        // Well-formed data: compact formatting is unchanged by the saturating sub.
        assert_eq!(format_compact("en", 1500.0), "1.5K");
        assert_eq!(format_compact("en", 2_300_000.0), "2.3M");
        assert_eq!(format_compact("en", 999.0), "999");
    }
}
