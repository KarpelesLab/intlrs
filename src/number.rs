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
use alloc::vec::Vec;

pub use crate::cldr::{NumberSpec, Pattern};

/// The kind of a [`NumberPart`] produced by [`format_to_parts`], matching the
/// ECMA-402 `Intl.NumberFormat.prototype.formatToParts` part `type` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberPartType {
    /// An integer-digit run (between grouping separators).
    Integer,
    /// A grouping separator.
    Group,
    /// The decimal separator.
    Decimal,
    /// A fraction-digit run.
    Fraction,
    /// Literal text from the pattern (prefix/suffix glue, spaces).
    Literal,
    /// The minus sign.
    MinusSign,
    /// The plus sign.
    PlusSign,
    /// The percent sign.
    PercentSign,
    /// A currency symbol/code/name.
    Currency,
    /// A measurement-unit symbol/name.
    Unit,
    /// The compact-notation suffix (e.g. `K`, `M`).
    Compact,
    /// The exponent separator (e.g. `E`).
    ExponentSeparator,
    /// The exponent's minus sign.
    ExponentMinusSign,
    /// The exponent's integer digits.
    ExponentInteger,
    /// The `NaN` placeholder.
    Nan,
    /// The infinity placeholder.
    Infinity,
}

impl NumberPartType {
    /// The ECMA-402 part `type` string for this kind.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            NumberPartType::Integer => "integer",
            NumberPartType::Group => "group",
            NumberPartType::Decimal => "decimal",
            NumberPartType::Fraction => "fraction",
            NumberPartType::Literal => "literal",
            NumberPartType::MinusSign => "minusSign",
            NumberPartType::PlusSign => "plusSign",
            NumberPartType::PercentSign => "percentSign",
            NumberPartType::Currency => "currency",
            NumberPartType::Unit => "unit",
            NumberPartType::Compact => "compact",
            NumberPartType::ExponentSeparator => "exponentSeparator",
            NumberPartType::ExponentMinusSign => "exponentMinusSign",
            NumberPartType::ExponentInteger => "exponentInteger",
            NumberPartType::Nan => "nan",
            NumberPartType::Infinity => "infinity",
        }
    }
}

/// One tagged segment of a formatted number (see [`format_to_parts`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberPart {
    /// What this segment represents.
    pub kind: NumberPartType,
    /// The literal text of this segment.
    pub value: String,
}

impl NumberPart {
    fn new(kind: NumberPartType, value: impl Into<String>) -> NumberPart {
        NumberPart {
            kind,
            value: value.into(),
        }
    }
}

/// Concatenate a part list's values into the final string.
fn join_parts(parts: &[NumberPart]) -> String {
    let mut out = String::new();
    for p in parts {
        out.push_str(&p.value);
    }
    out
}

/// The kind of quantity being formatted (ECMA-402 `style`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NumberStyle {
    /// A plain decimal number.
    #[default]
    Decimal,
    /// A percent (the value is a ratio, so `0.5` → `50%`).
    Percent,
    /// A currency amount (requires [`NumberFormatOptions::currency`]).
    Currency,
    /// A measurement unit (requires [`NumberFormatOptions::unit`]).
    Unit,
}

/// Notation (ECMA-402 `notation`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Notation {
    /// Plain positional notation.
    #[default]
    Standard,
    /// Scientific notation (mantissa in `[1, 10)` × 10ⁿ).
    Scientific,
    /// Engineering notation (mantissa in `[1, 1000)`, exponent a multiple of 3).
    Engineering,
    /// Compact notation (e.g. `1.2K`).
    Compact,
}

/// Whether compact notation uses short or long suffixes (ECMA-402
/// `compactDisplay`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompactDisplay {
    /// Short suffixes (`1.5K`).
    #[default]
    Short,
    /// Long suffixes (`1.5 thousand`).
    Long,
}

/// How a currency is shown (ECMA-402 `currencyDisplay`). `Symbol` and
/// `NarrowSymbol` use the localized symbol; `Code` uses the ISO code; `Name`
/// falls back to the code (no display-name data yet).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrencyDisplay {
    /// The localized currency symbol (e.g. `$`).
    #[default]
    Symbol,
    /// The narrow symbol; falls back to `Symbol`.
    NarrowSymbol,
    /// The ISO 4217 code (e.g. `USD`).
    Code,
    /// The currency display name; falls back to the code.
    Name,
}

/// How a unit is shown (ECMA-402 `unitDisplay`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnitDisplay {
    /// Short form.
    #[default]
    Short,
    /// Narrow form; falls back to `Short`.
    Narrow,
    /// Long form.
    Long,
}

/// Grouping-separator strategy (ECMA-402 `useGrouping`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UseGrouping {
    /// Locale default (group per the pattern).
    #[default]
    Auto,
    /// Always group.
    Always,
    /// Group only when the integer part has at least two groups.
    Min2,
    /// Never group.
    Never,
}

/// When to show a sign (ECMA-402 `signDisplay`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SignDisplay {
    /// Sign for negative numbers only.
    #[default]
    Auto,
    /// Always show a sign, including `+` for positive and zero.
    Always,
    /// Show a sign except for zero.
    ExceptZero,
    /// Show a sign for negative numbers only (alias of `Auto` here).
    Negative,
    /// Never show a sign.
    Never,
}

/// Rounding mode (ECMA-402 `roundingMode`), applied at the rounding boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoundingMode {
    /// Toward +∞.
    Ceil,
    /// Toward −∞.
    Floor,
    /// Away from zero.
    Expand,
    /// Toward zero.
    Trunc,
    /// Nearest; ties toward +∞.
    HalfCeil,
    /// Nearest; ties toward −∞.
    HalfFloor,
    /// Nearest; ties away from zero.
    HalfExpand,
    /// Nearest; ties toward zero.
    HalfTrunc,
    /// Nearest; ties to even (the default).
    #[default]
    HalfEven,
}

/// Options for [`format`] / [`format_to_parts`], modeled on the ECMA-402
/// `Intl.NumberFormat` options. [`Default`] is plain decimal formatting with the
/// locale's pattern precision and half-even rounding.
///
/// Currency/unit codes are `&'static str` (so the struct stays `Copy` and borrows
/// from compile-time string literals, matching the crate's data model).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NumberFormatOptions {
    /// The kind of quantity.
    pub style: NumberStyle,
    /// Notation.
    pub notation: Notation,
    /// Compact suffix length (only relevant for [`Notation::Compact`]).
    pub compact_display: CompactDisplay,
    /// Grouping strategy.
    pub use_grouping: UseGrouping,
    /// When to show a sign.
    pub sign_display: SignDisplay,
    /// Rounding mode.
    pub rounding_mode: RoundingMode,
    /// Minimum integer digits (default 1).
    pub minimum_integer_digits: u8,
    /// Minimum fraction digits (`None` → style/locale default).
    pub minimum_fraction_digits: Option<u8>,
    /// Maximum fraction digits (`None` → style/locale default).
    pub maximum_fraction_digits: Option<u8>,
    /// Minimum significant digits (`None` → unused).
    pub minimum_significant_digits: Option<u8>,
    /// Maximum significant digits (`None` → unused; takes precedence over the
    /// fraction-digit settings when set, per ECMA-402 `roundingPriority: auto`).
    pub maximum_significant_digits: Option<u8>,
    /// ISO 4217 currency code (required when `style` is [`NumberStyle::Currency`]).
    pub currency: Option<&'static str>,
    /// How the currency is displayed.
    pub currency_display: CurrencyDisplay,
    /// CLDR unit identifier (required when `style` is [`NumberStyle::Unit`]).
    pub unit: Option<&'static str>,
    /// How the unit is displayed.
    pub unit_display: UnitDisplay,
    /// Numbering system override (e.g. `"arab"`); `None` uses the locale default
    /// for [`format`]'s plain path (Latin digits).
    pub numbering_system: Option<&'static str>,
}

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
    use crate::plural::{PluralOperands, ordinal_category};
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
        if !got_pat && let Some(p) = cur::currency_pattern(&norm[..end]) {
            pat = p;
            got_pat = true;
        }
        if !got_sym && let Some(sym) = cur::currency_symbol(&norm[..end], code) {
            symbol = sym;
            got_sym = true;
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
    if !got_sym && let Some(sym) = cur::currency_symbol("en", code) {
        symbol = sym;
    }

    let digits = cur::currency_digits(code);
    pat.min_frac = digits;
    pat.max_frac = digits;

    let formatted = format_with(&pat, value, s.decimal, s.group, s.minus);
    // The pattern carries the ¤ placeholder; replace it with the symbol.
    formatted.replace('\u{a4}', symbol)
}

fn format_with(p: &Pattern, value: f64, decimal: &str, group: &str, minus: &str) -> String {
    join_parts(&format_with_parts(p, value, decimal, group, minus))
}

/// Behavior-preserving parts core of [`format_with`]: produces the same output
/// as the historical string builder, but as tagged [`NumberPart`]s. Joining the
/// values reproduces the legacy string exactly.
fn format_with_parts(
    p: &Pattern,
    value: f64,
    decimal: &str,
    group: &str,
    minus: &str,
) -> Vec<NumberPart> {
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

    let mut parts = Vec::new();
    if neg {
        parts.push(NumberPart::new(NumberPartType::MinusSign, minus));
    }
    if !p.prefix.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Literal, p.prefix));
    }
    parts.extend(group_parts(
        int_str,
        p.primary_group,
        p.secondary_group,
        group,
    ));
    if !frac.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Decimal, decimal));
        parts.push(NumberPart::new(NumberPartType::Fraction, frac));
    }
    if !p.suffix.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Literal, p.suffix));
    }
    parts
}

/// Split an integer-digit string into `Integer` runs separated by `Group` parts,
/// per the primary/secondary grouping sizes. Concatenating the values reproduces
/// the historical `group_digits` output.
fn group_parts(digits: &str, primary: u8, secondary: u8, sep: &str) -> Vec<NumberPart> {
    if primary == 0 || digits.len() <= primary as usize {
        return alloc::vec![NumberPart::new(NumberPartType::Integer, digits)];
    }
    let chars: Vec<char> = digits.chars().collect();
    let n = chars.len();
    // Cut positions (from the left) where a separator is inserted: the rightmost
    // group is `primary` wide, then `secondary` repeats. `secondary == 0` means
    // only the single primary group is separated (rest stays one run).
    let mut cuts: Vec<usize> = Vec::new();
    let mut pos = n - primary as usize;
    cuts.push(pos);
    if secondary > 0 {
        while pos > secondary as usize {
            pos -= secondary as usize;
            cuts.push(pos);
        }
    }
    cuts.sort_unstable();

    let mut parts = Vec::new();
    let mut prev = 0;
    for &cut in &cuts {
        parts.push(NumberPart::new(
            NumberPartType::Integer,
            chars[prev..cut].iter().collect::<String>(),
        ));
        parts.push(NumberPart::new(NumberPartType::Group, sep));
        prev = cut;
    }
    parts.push(NumberPart::new(
        NumberPartType::Integer,
        chars[prev..n].iter().collect::<String>(),
    ));
    parts
}

/// Decide whether the magnitude is rounded up, given the kept-digit count `cut`,
/// the rounding `mode`, and the value's sign. `digits` holds 0–9 values.
fn should_round_up(digits: &[u8], cut: usize, mode: RoundingMode, negative: bool) -> bool {
    if cut >= digits.len() {
        return false; // nothing discarded
    }
    let first = digits[cut];
    let rest_nonzero = digits[cut + 1..].iter().any(|&d| d != 0);
    let any_discarded = first != 0 || rest_nonzero;
    let gt_half = first > 5 || (first == 5 && rest_nonzero);
    let eq_half = first == 5 && !rest_nonzero;
    let kept_last_odd = cut > 0 && digits[cut - 1] % 2 == 1;
    use RoundingMode::*;
    match mode {
        Trunc => false,
        Expand => any_discarded,
        Ceil => any_discarded && !negative,
        Floor => any_discarded && negative,
        HalfExpand => gt_half || eq_half,
        HalfTrunc => gt_half,
        HalfEven => gt_half || (eq_half && kept_last_odd),
        HalfCeil => gt_half || (eq_half && !negative),
        HalfFloor => gt_half || (eq_half && negative),
    }
}

/// Round a non-negative finite `abs` to a decimal digit string per the resolved
/// precision and mode, returning `(integer_digits, fraction_digits)` (no
/// separators; `integer_digits` is never empty — `"0"` for a zero magnitude).
///
/// When `max_sig` is set, significant-digit precision is used and the
/// fraction-digit limits are ignored (ECMA-402 `roundingPriority: auto`).
///
/// Uses the f64's decimal expansion, so binary-inexact values may round at the
/// last ulp differently from a true decimal type (ICU). Precision beyond ~17
/// significant digits is not meaningful for f64.
#[allow(clippy::too_many_arguments)]
fn round_digits(
    abs: f64,
    min_int: usize,
    min_frac: usize,
    max_frac: usize,
    min_sig: Option<usize>,
    max_sig: Option<usize>,
    mode: RoundingMode,
    negative: bool,
) -> (String, String) {
    // Expand to a working decimal with enough fraction digits to see the cut.
    let work = max_frac.max(min_frac).saturating_add(2).clamp(40, 320);
    let s = alloc::format!("{abs:.work$}");
    let (ip, fp) = s.split_once('.').unwrap_or((s.as_str(), ""));
    let mut digits: Vec<u8> = ip.bytes().chain(fp.bytes()).map(|b| b - b'0').collect();
    let mut point = ip.len(); // number of integer digits

    // Rounding boundary (number of leading digits kept).
    let cut = if let Some(ms) = max_sig {
        match digits.iter().position(|&d| d != 0) {
            Some(first_nz) => (first_nz + ms).min(digits.len()),
            None => point, // all zero
        }
    } else {
        (point + max_frac).min(digits.len())
    };

    let up = should_round_up(&digits, cut, mode, negative);
    // Discarded integer positions (when cut < point) become zeros; fraction
    // beyond the cut is dropped.
    for d in digits.iter_mut().skip(cut).take(point.saturating_sub(cut)) {
        *d = 0;
    }
    let keep = cut.max(point);
    digits.truncate(keep);
    if up {
        let mut i = cut;
        loop {
            if i == 0 {
                digits.insert(0, 1);
                point += 1;
                break;
            }
            i -= 1;
            if digits[i] == 9 {
                digits[i] = 0;
            } else {
                digits[i] += 1;
                break;
            }
        }
    }

    let mut int_digits: String = digits[..point]
        .iter()
        .map(|&d| (b'0' + d) as char)
        .collect();
    let mut frac_digits: String = digits[point..]
        .iter()
        .map(|&d| (b'0' + d) as char)
        .collect();

    // Finalize precision constraints.
    if max_sig.is_some() {
        let min_s = min_sig.unwrap_or(1);
        let combined: Vec<u8> = int_digits
            .bytes()
            .chain(frac_digits.bytes())
            .map(|b| b - b'0')
            .collect();
        match combined.iter().position(|&d| d != 0) {
            None => {
                // Zero: the leading "0" is the first significant position.
                for _ in 0..min_s.saturating_sub(1) {
                    frac_digits.push('0');
                }
            }
            Some(first_nz) => {
                let mut sig = combined.len() - first_nz;
                while sig < min_s {
                    frac_digits.push('0');
                    sig += 1;
                }
                while sig > min_s && frac_digits.ends_with('0') {
                    frac_digits.pop();
                    sig -= 1;
                }
            }
        }
    } else {
        while frac_digits.len() > min_frac && frac_digits.ends_with('0') {
            frac_digits.pop();
        }
    }

    while int_digits.len() < min_int {
        int_digits.insert(0, '0');
    }
    (int_digits, frac_digits)
}

/// The sign part for a value, given its sign/zero-ness and the sign-display mode.
fn sign_part(
    negative: bool,
    is_zero: bool,
    opts: &NumberFormatOptions,
    s: &NumberSpec,
) -> Option<NumberPart> {
    let (show, plus) = match opts.sign_display {
        SignDisplay::Auto | SignDisplay::Negative => (negative, false),
        SignDisplay::Always => (true, !negative),
        SignDisplay::ExceptZero => (!is_zero, !negative),
        SignDisplay::Never => (false, false),
    };
    if !show {
        return None;
    }
    if plus {
        Some(NumberPart::new(NumberPartType::PlusSign, s.plus))
    } else {
        Some(NumberPart::new(NumberPartType::MinusSign, s.minus))
    }
}

/// Effective grouping sizes after applying the `useGrouping` strategy.
fn effective_grouping(opts: &NumberFormatOptions, pattern: &Pattern, int_len: usize) -> (u8, u8) {
    match opts.use_grouping {
        UseGrouping::Never => (0, 0),
        UseGrouping::Min2 => {
            // Group only when the leftmost group would have ≥2 digits.
            if int_len > pattern.primary_group as usize + 1 {
                (pattern.primary_group, pattern.secondary_group)
            } else {
                (0, 0)
            }
        }
        UseGrouping::Auto | UseGrouping::Always => (pattern.primary_group, pattern.secondary_group),
    }
}

/// Apply a numbering-system transliteration to a digit run's value, if requested.
fn map_digits(value: &str, opts: &NumberFormatOptions) -> String {
    match opts.numbering_system {
        Some(sys) if sys != "latn" => to_numbering_system(value, sys),
        _ => String::from(value),
    }
}

/// Build the numeric core (sign, grouped integer, decimal + fraction) common to
/// all standard-notation styles. The numbering system is applied to the digit
/// runs only (separators keep the locale symbols).
#[allow(clippy::too_many_arguments)]
fn core_parts(
    int_digits: &str,
    frac_digits: &str,
    negative: bool,
    is_zero: bool,
    primary: u8,
    secondary: u8,
    opts: &NumberFormatOptions,
    s: &NumberSpec,
) -> Vec<NumberPart> {
    let mut parts = Vec::new();
    if let Some(sign) = sign_part(negative, is_zero, opts, s) {
        parts.push(sign);
    }
    for mut p in group_parts(int_digits, primary, secondary, s.group) {
        if p.kind == NumberPartType::Integer {
            p.value = map_digits(&p.value, opts);
        }
        parts.push(p);
    }
    if !frac_digits.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Decimal, s.decimal));
        parts.push(NumberPart::new(
            NumberPartType::Fraction,
            map_digits(frac_digits, opts),
        ));
    }
    parts
}

/// Split a pattern affix into parts, tagging the percent symbol / currency
/// placeholder; everything else is literal glue.
fn affix_parts(text: &str, style: NumberStyle, s: &NumberSpec, currency: &str) -> Vec<NumberPart> {
    let mut parts = Vec::new();
    if text.is_empty() {
        return parts;
    }
    match style {
        NumberStyle::Percent => {
            for (i, seg) in text.split(s.percent).enumerate() {
                if i > 0 {
                    parts.push(NumberPart::new(NumberPartType::PercentSign, s.percent));
                }
                if !seg.is_empty() {
                    parts.push(NumberPart::new(NumberPartType::Literal, seg));
                }
            }
        }
        NumberStyle::Currency => {
            for (i, seg) in text.split('\u{a4}').enumerate() {
                if i > 0 {
                    parts.push(NumberPart::new(NumberPartType::Currency, currency));
                }
                if !seg.is_empty() {
                    parts.push(NumberPart::new(NumberPartType::Literal, seg));
                }
            }
        }
        _ => parts.push(NumberPart::new(NumberPartType::Literal, text)),
    }
    parts
}

/// Map an ECMA-402 sanctioned unit identifier to the embedded unit-table index
/// (the order of `crate::unit::Unit`). Returns `None` for unsupported units.
fn unit_index(id: &str) -> Option<usize> {
    Some(match id {
        "second" => 0,
        "minute" => 1,
        "hour" => 2,
        "day" => 3,
        "week" => 4,
        "month" => 5,
        "year" => 6,
        "millimeter" => 7,
        "centimeter" => 8,
        "meter" => 9,
        "kilometer" => 10,
        "inch" => 11,
        "foot" => 12,
        "mile" => 13,
        "gram" => 14,
        "kilogram" => 15,
        "ounce" => 16,
        "pound" => 17,
        "byte" => 18,
        "kilobyte" => 19,
        "megabyte" => 20,
        "gigabyte" => 21,
        "celsius" => 22,
        "fahrenheit" => 23,
        "kilometer-per-hour" => 24,
        "mile-per-hour" => 25,
        "liter" => 26,
        "milliliter" => 27,
        _ => return None,
    })
}

/// Split a unit-pattern affix into `Unit` (non-whitespace) and `Literal`
/// (whitespace) runs, matching ECMA-402's tagging of `"1.5 m"` as
/// `… literal(" ") unit("m")`.
fn unit_affix(text: &str) -> Vec<NumberPart> {
    let mut parts = Vec::new();
    let mut buf = String::new();
    let mut in_ws = false;
    for ch in text.chars() {
        let ws = ch.is_whitespace();
        if buf.is_empty() {
            in_ws = ws;
        } else if ws != in_ws {
            let kind = if in_ws {
                NumberPartType::Literal
            } else {
                NumberPartType::Unit
            };
            parts.push(NumberPart::new(kind, core::mem::take(&mut buf)));
            in_ws = ws;
        }
        buf.push(ch);
    }
    if !buf.is_empty() {
        let kind = if in_ws {
            NumberPartType::Literal
        } else {
            NumberPartType::Unit
        };
        parts.push(NumberPart::new(kind, buf));
    }
    parts
}

/// Wrap the numeric `core` parts with the locale's CLDR unit pattern (e.g.
/// `"{0} km"`), choosing the plural-correct wording. An unknown/missing unit
/// degrades to the bare number.
fn unit_wrap(
    lang: &str,
    value: f64,
    core: Vec<NumberPart>,
    opts: &NumberFormatOptions,
) -> Vec<NumberPart> {
    let Some(uidx) = opts.unit.and_then(unit_index) else {
        return core;
    };
    // Long form when requested; short/narrow share the short table (no narrow data).
    let width = usize::from(opts.unit_display != UnitDisplay::Long);
    let ops = if value % 1.0 == 0.0 && value > -1e15 && value < 1e15 {
        crate::plural::PluralOperands::from_int(value as i64)
    } else {
        crate::plural::PluralOperands::parse(&alloc::format!("{value}"))
            .unwrap_or_else(|| crate::plural::PluralOperands::from_int(value as i64))
    };
    let cat = crate::plural::plural_category(lang, &ops) as usize;

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
    let mut pattern = "{0}";
    let mut end = norm.len();
    loop {
        if let Some(p) = crate::cldr::unit_pattern(&norm[..end], width, uidx, cat) {
            pattern = p;
            break;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => {
                if let Some(p) = crate::cldr::unit_pattern("en", width, uidx, cat) {
                    pattern = p;
                }
                break;
            }
        }
    }

    let (pre, post) = pattern.split_once("{0}").unwrap_or(("", pattern));
    let mut parts = unit_affix(pre);
    parts.extend(core);
    parts.extend(unit_affix(post));
    parts
}

/// Resolve the base pattern, scaled value, and currency symbol for `style`.
fn resolve_style(
    lang: &str,
    value: f64,
    s: &NumberSpec,
    opts: &NumberFormatOptions,
) -> (Pattern, f64, String) {
    match opts.style {
        NumberStyle::Decimal | NumberStyle::Unit => (s.dec, value, String::new()),
        NumberStyle::Percent => (s.pct, value * 100.0, String::new()),
        NumberStyle::Currency => {
            let code = opts.currency.unwrap_or("XXX");
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
            let mut pat = crate::cldr::currency_pattern("en").expect("root currency pattern");
            let mut symbol = String::from(code);
            let mut end = norm.len();
            let (mut got_pat, mut got_sym) = (false, false);
            loop {
                if !got_pat && let Some(p) = crate::cldr::currency_pattern(&norm[..end]) {
                    pat = p;
                    got_pat = true;
                }
                if !got_sym && let Some(sym) = crate::cldr::currency_symbol(&norm[..end], code) {
                    symbol = String::from(sym);
                    got_sym = true;
                }
                if got_pat && got_sym {
                    break;
                }
                match norm[..end].rfind('-') {
                    Some(i) => end = i,
                    None => break,
                }
            }
            if !got_sym && let Some(sym) = crate::cldr::currency_symbol("en", code) {
                symbol = String::from(sym);
            }
            // currencyDisplay: Code/Name show the code rather than the symbol.
            let shown = match opts.currency_display {
                CurrencyDisplay::Code | CurrencyDisplay::Name => String::from(code),
                CurrencyDisplay::Symbol | CurrencyDisplay::NarrowSymbol => symbol,
            };
            let digits = crate::cldr::currency_digits(code);
            pat.min_frac = digits;
            pat.max_frac = digits;
            (pat, value, shown)
        }
    }
}

/// Standard (positional) notation.
fn standard_parts(
    lang: &str,
    value: f64,
    s: &NumberSpec,
    opts: &NumberFormatOptions,
) -> Vec<NumberPart> {
    let (pattern, scaled, currency) = resolve_style(lang, value, s, opts);
    let min_frac = opts
        .minimum_fraction_digits
        .map_or(pattern.min_frac as usize, usize::from);
    let max_frac = opts
        .maximum_fraction_digits
        .map_or(pattern.max_frac as usize, usize::from)
        .max(min_frac);
    let min_int = (opts.minimum_integer_digits.max(1)) as usize;
    let min_sig = opts.minimum_significant_digits.map(usize::from);
    let max_sig = opts.maximum_significant_digits.map(usize::from);

    let negative = scaled.is_sign_negative() && scaled != 0.0;
    let abs = if scaled < 0.0 { -scaled } else { scaled };
    let (int_d, frac_d) = round_digits(
        abs,
        min_int,
        min_frac,
        max_frac,
        min_sig,
        max_sig,
        opts.rounding_mode,
        negative,
    );
    let is_zero = int_d.bytes().all(|b| b == b'0') && frac_d.bytes().all(|b| b == b'0');
    let (pri, sec) = effective_grouping(opts, &pattern, int_d.len());

    let mut parts = Vec::new();
    let core = core_parts(&int_d, &frac_d, negative, is_zero, pri, sec, opts, s);
    // Unit style wraps the numeric core in the locale's unit pattern.
    if opts.style == NumberStyle::Unit {
        return unit_wrap(lang, scaled, core, opts);
    }
    // The sign (first core part, if any) precedes the prefix affix.
    let mut core_iter = core.into_iter().peekable();
    if let Some(first) = core_iter.peek()
        && matches!(
            first.kind,
            NumberPartType::MinusSign | NumberPartType::PlusSign
        )
    {
        parts.push(core_iter.next().unwrap());
    }
    parts.extend(affix_parts(pattern.prefix, opts.style, s, &currency));
    parts.extend(core_iter);
    parts.extend(affix_parts(pattern.suffix, opts.style, s, &currency));
    parts
}

/// Scientific (`base = 1`) or engineering (`base = 3`) notation.
fn exponent_parts(
    value: f64,
    s: &NumberSpec,
    opts: &NumberFormatOptions,
    base: i32,
) -> Vec<NumberPart> {
    let negative = value.is_sign_negative() && value != 0.0;
    let abs = if value < 0.0 { -value } else { value };
    let mut exp = 0i32;
    let mut m = abs;
    if abs != 0.0 {
        while m >= 10.0 {
            m /= 10.0;
            exp += 1;
        }
        while m < 1.0 {
            m *= 10.0;
            exp -= 1;
        }
    }
    if base > 1 {
        let rem = exp.rem_euclid(base);
        for _ in 0..rem {
            m *= 10.0;
        }
        exp -= rem;
    }

    let min_frac = opts.minimum_fraction_digits.map_or(0usize, usize::from);
    let max_frac = opts
        .maximum_fraction_digits
        .map_or(6usize, usize::from)
        .max(min_frac);
    let min_sig = opts.minimum_significant_digits.map(usize::from);
    let max_sig = opts.maximum_significant_digits.map(usize::from);
    let (mut int_d, mut frac_d) = round_digits(
        m,
        1,
        min_frac,
        max_frac,
        min_sig,
        max_sig,
        opts.rounding_mode,
        negative,
    );

    // A rounding carry can push the mantissa to ≥ 10^base (e.g. 9.99 → 10);
    // shift the point back so the integer part is the expected width.
    let want = base.max(1) as usize;
    if int_d.len() > want {
        let shift = (int_d.len() - want) as i32;
        exp += shift;
        let combined = alloc::format!("{int_d}{frac_d}");
        int_d = String::from(&combined[..want]);
        frac_d = String::from(combined[want..].trim_end_matches('0'));
    }

    let is_zero = abs == 0.0;
    let mut parts = Vec::new();
    if let Some(sign) = sign_part(negative, is_zero, opts, s) {
        parts.push(sign);
    }
    parts.push(NumberPart::new(
        NumberPartType::Integer,
        map_digits(&int_d, opts),
    ));
    if !frac_d.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Decimal, s.decimal));
        parts.push(NumberPart::new(
            NumberPartType::Fraction,
            map_digits(&frac_d, opts),
        ));
    }
    parts.push(NumberPart::new(NumberPartType::ExponentSeparator, "E"));
    if exp < 0 {
        parts.push(NumberPart::new(NumberPartType::ExponentMinusSign, s.minus));
    }
    parts.push(NumberPart::new(
        NumberPartType::ExponentInteger,
        map_digits(&alloc::format!("{}", exp.unsigned_abs()), opts),
    ));
    parts
}

/// Compact notation (short suffixes), for the decimal style.
fn compact_parts(
    lang: &str,
    value: f64,
    s: &NumberSpec,
    opts: &NumberFormatOptions,
) -> Vec<NumberPart> {
    let abs = if value < 0.0 { -value } else { value };
    if !abs.is_finite() || abs < 1000.0 {
        return standard_parts(lang, value, s, opts);
    }
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
    let mut exp = 0usize;
    let mut t = abs;
    while t >= 10.0 && exp < 14 {
        t /= 10.0;
        exp += 1;
    }
    // compact.bin holds 12 short patterns then 12 long; pick the band.
    let base = if opts.compact_display == CompactDisplay::Long {
        12
    } else {
        0
    };
    let pattern = table[base + (exp - 3).min(11)];
    let zeros = pattern.chars().filter(|&c| c == '0').count();
    let has_suffix = pattern
        .chars()
        .any(|c| c != '0' && c != '\'' && !c.is_whitespace());
    if zeros == 0 || !has_suffix {
        return standard_parts(lang, value, s, opts);
    }
    let mut divisor = 1.0f64;
    for _ in 0..(exp + 1).saturating_sub(zeros) {
        divisor *= 10.0;
    }
    let mantissa = value / divisor;
    let negative = mantissa.is_sign_negative() && mantissa != 0.0;
    let mabs = if mantissa < 0.0 { -mantissa } else { mantissa };
    let min_frac = opts.minimum_fraction_digits.map_or(0usize, usize::from);
    let max_frac = opts
        .maximum_fraction_digits
        .map_or(1usize, usize::from)
        .max(min_frac);
    let (int_d, frac_d) = round_digits(
        mabs,
        1,
        min_frac,
        max_frac,
        None,
        None,
        opts.rounding_mode,
        negative,
    );
    let is_zero = false;

    // Render the pattern, substituting the numeric core for the `0`-run and
    // tagging the literal magnitude suffix as `compact`.
    let mut parts = Vec::new();
    if let Some(sign) = sign_part(negative, is_zero, opts, s) {
        parts.push(sign);
    }
    let mut wrote = false;
    let mut chars = pattern.chars().peekable();
    let mut lit = String::new();
    let flush_lit = |lit: &mut String, parts: &mut Vec<NumberPart>| {
        if !lit.is_empty() {
            parts.push(NumberPart::new(
                NumberPartType::Compact,
                core::mem::take(lit),
            ));
        }
    };
    while let Some(c) = chars.next() {
        match c {
            '0' => {
                while chars.peek() == Some(&'0') {
                    chars.next();
                }
                if !wrote {
                    flush_lit(&mut lit, &mut parts);
                    parts.push(NumberPart::new(
                        NumberPartType::Integer,
                        map_digits(&int_d, opts),
                    ));
                    if !frac_d.is_empty() {
                        parts.push(NumberPart::new(NumberPartType::Decimal, s.decimal));
                        parts.push(NumberPart::new(
                            NumberPartType::Fraction,
                            map_digits(&frac_d, opts),
                        ));
                    }
                    wrote = true;
                }
            }
            '\'' => {
                for q in chars.by_ref() {
                    if q == '\'' {
                        break;
                    }
                    lit.push(q);
                }
            }
            other => lit.push(other),
        }
    }
    flush_lit(&mut lit, &mut parts);
    parts
}

/// Format `value` in `lang` per ECMA-402-style `opts`, returning the tagged
/// parts (`Intl.NumberFormat.prototype.formatToParts`).
///
/// ```
/// use intl::number::{format_to_parts, NumberFormatOptions, NumberPartType, UseGrouping};
/// let opts = NumberFormatOptions { use_grouping: UseGrouping::Never, ..Default::default() };
/// let parts = format_to_parts("en", 1234.5, &opts);
/// assert_eq!(parts[0].kind, NumberPartType::Integer);
/// assert_eq!(parts.iter().map(|p| p.value.as_str()).collect::<String>(), "1234.5");
/// ```
#[must_use]
pub fn format_to_parts(lang: &str, value: f64, opts: &NumberFormatOptions) -> Vec<NumberPart> {
    let s = spec(lang);
    if value.is_nan() {
        return alloc::vec![NumberPart::new(NumberPartType::Nan, "NaN")];
    }
    if value.is_infinite() {
        let mut parts = Vec::new();
        if let Some(sign) = sign_part(value < 0.0, false, opts, &s) {
            parts.push(sign);
        }
        parts.push(NumberPart::new(NumberPartType::Infinity, "∞"));
        return parts;
    }
    match opts.notation {
        Notation::Standard => standard_parts(lang, value, &s, opts),
        Notation::Scientific => exponent_parts(value, &s, opts, 1),
        Notation::Engineering => exponent_parts(value, &s, opts, 3),
        Notation::Compact => compact_parts(lang, value, &s, opts),
    }
}

/// Format `value` in `lang` per ECMA-402-style `opts` (`Intl.NumberFormat`).
///
/// ```
/// use intl::number::{format, NumberFormatOptions, SignDisplay};
/// let opts = NumberFormatOptions { sign_display: SignDisplay::Always, ..Default::default() };
/// assert_eq!(format("en", 5.0, &opts), "+5");
/// assert_eq!(format("en", 1234.5, &Default::default()), "1,234.5");
/// ```
#[must_use]
pub fn format(lang: &str, value: f64, opts: &NumberFormatOptions) -> String {
    join_parts(&format_to_parts(lang, value, opts))
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

    fn opt() -> NumberFormatOptions {
        NumberFormatOptions::default()
    }

    #[test]
    fn options_default_matches_decimal() {
        assert_eq!(format("en", 1234.5, &opt()), "1,234.5");
        assert_eq!(format("de", 1234.5, &opt()), "1.234,5");
        assert_eq!(format("hi", 1234567.0, &opt()), "12,34,567");
    }

    #[test]
    fn options_grouping_and_min_int() {
        let ng = NumberFormatOptions {
            use_grouping: UseGrouping::Never,
            ..opt()
        };
        assert_eq!(format("en", 1234567.0, &ng), "1234567");
        let mi = NumberFormatOptions {
            minimum_integer_digits: 3,
            ..opt()
        };
        assert_eq!(format("en", 5.0, &mi), "005");
    }

    #[test]
    fn options_sign_display() {
        let always = NumberFormatOptions {
            sign_display: SignDisplay::Always,
            ..opt()
        };
        assert_eq!(format("en", 5.0, &always), "+5");
        assert_eq!(format("en", -5.0, &always), "-5");
        assert_eq!(format("en", 0.0, &always), "+0");
        let ez = NumberFormatOptions {
            sign_display: SignDisplay::ExceptZero,
            ..opt()
        };
        assert_eq!(format("en", 0.0, &ez), "0");
        assert_eq!(format("en", 3.0, &ez), "+3");
        let never = NumberFormatOptions {
            sign_display: SignDisplay::Never,
            ..opt()
        };
        assert_eq!(format("en", -5.0, &never), "5");
    }

    #[test]
    fn options_fraction_digits() {
        let f = NumberFormatOptions {
            minimum_fraction_digits: Some(2),
            maximum_fraction_digits: Some(2),
            ..opt()
        };
        assert_eq!(format("en", 1.5, &f), "1.50");
        assert_eq!(format("en", 1.005, &f), "1.00"); // f64: true value < 1.005
    }

    #[test]
    fn options_rounding_modes() {
        let mk = |mode, mx| NumberFormatOptions {
            rounding_mode: mode,
            maximum_fraction_digits: Some(mx),
            ..opt()
        };
        assert_eq!(format("en", 1.001, &mk(RoundingMode::Ceil, 2)), "1.01");
        assert_eq!(format("en", 1.999, &mk(RoundingMode::Trunc, 2)), "1.99");
        assert_eq!(format("en", -1.001, &mk(RoundingMode::Floor, 2)), "-1.01");
        assert_eq!(format("en", 2.5, &mk(RoundingMode::HalfEven, 0)), "2");
        assert_eq!(format("en", 3.5, &mk(RoundingMode::HalfEven, 0)), "4");
        assert_eq!(format("en", 2.5, &mk(RoundingMode::HalfExpand, 0)), "3");
    }

    #[test]
    fn options_significant_digits() {
        let mx = NumberFormatOptions {
            maximum_significant_digits: Some(3),
            ..opt()
        };
        assert_eq!(format("en", 1234.0, &mx), "1,230");
        let mn = NumberFormatOptions {
            minimum_significant_digits: Some(4),
            maximum_significant_digits: Some(6),
            ..opt()
        };
        assert_eq!(format("en", 1.5, &mn), "1.500");
    }

    #[test]
    fn options_percent_and_currency_parts() {
        let pct = NumberFormatOptions {
            style: NumberStyle::Percent,
            ..opt()
        };
        let parts = format_to_parts("en", 0.5, &pct);
        assert_eq!(parts.last().unwrap().kind, NumberPartType::PercentSign);
        assert_eq!(format("en", 0.5, &pct), "50%");

        let cur = NumberFormatOptions {
            style: NumberStyle::Currency,
            currency: Some("USD"),
            ..opt()
        };
        assert_eq!(format("en", 1234.5, &cur), "$1,234.50");
        let parts = format_to_parts("en", 1234.5, &cur);
        assert!(
            parts
                .iter()
                .any(|p| p.kind == NumberPartType::Currency && p.value == "$")
        );
        // currencyDisplay: code
        let code = NumberFormatOptions {
            currency_display: CurrencyDisplay::Code,
            ..cur
        };
        assert_eq!(format("en", 5.0, &code), "USD5.00");
    }

    #[test]
    fn options_notation() {
        let sci = NumberFormatOptions {
            notation: Notation::Scientific,
            ..opt()
        };
        let parts = format_to_parts("en", 12345.0, &sci);
        let kinds: Vec<_> = parts.iter().map(|p| p.kind).collect();
        assert_eq!(
            kinds,
            alloc::vec![
                NumberPartType::Integer,
                NumberPartType::Decimal,
                NumberPartType::Fraction,
                NumberPartType::ExponentSeparator,
                NumberPartType::ExponentInteger,
            ]
        );
        assert_eq!(format("en", 12345.0, &sci), "1.2345E4");
        assert!(
            format_to_parts("en", 0.00042, &sci)
                .iter()
                .any(|p| p.kind == NumberPartType::ExponentMinusSign)
        );

        let eng = NumberFormatOptions {
            notation: Notation::Engineering,
            ..opt()
        };
        assert_eq!(format("en", 12345.0, &eng), "12.345E3");

        let comp = NumberFormatOptions {
            notation: Notation::Compact,
            ..opt()
        };
        assert_eq!(format("en", 1500.0, &comp), "1.5K");
        assert!(
            format_to_parts("en", 1500.0, &comp)
                .iter()
                .any(|p| p.kind == NumberPartType::Compact && p.value == "K")
        );
    }

    #[test]
    fn options_numbering_system() {
        let ns = NumberFormatOptions {
            numbering_system: Some("arab"),
            ..opt()
        };
        assert_eq!(format("en", 123.0, &ns), "١٢٣");
    }

    #[test]
    fn non_finite() {
        assert_eq!(format("en", f64::NAN, &opt()), "NaN");
        assert_eq!(format("en", f64::INFINITY, &opt()), "∞");
        assert_eq!(format("en", f64::NEG_INFINITY, &opt()), "-∞");
    }

    #[test]
    fn parts_join_round_trips() {
        // join(format_to_parts) == format across a style/notation matrix.
        let cases = [
            NumberFormatOptions::default(),
            NumberFormatOptions {
                style: NumberStyle::Percent,
                ..opt()
            },
            NumberFormatOptions {
                style: NumberStyle::Currency,
                currency: Some("EUR"),
                ..opt()
            },
            NumberFormatOptions {
                notation: Notation::Scientific,
                ..opt()
            },
            NumberFormatOptions {
                notation: Notation::Compact,
                ..opt()
            },
        ];
        for o in cases {
            for v in [0.0, 1234.5, -9999.99, 0.001] {
                let joined = join_parts(&format_to_parts("en", v, &o));
                assert_eq!(joined, format("en", v, &o));
            }
        }
    }
}
