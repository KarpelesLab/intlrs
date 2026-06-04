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

    // Left-pad the integer to the minimum digit count.
    let mut int_owned;
    let int_str: &str = if (int_str.len() as u8) < p.min_int {
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
