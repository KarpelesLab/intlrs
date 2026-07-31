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
///
/// The enum is `#[non_exhaustive]`: ECMA-402 keeps adding part types (the range
/// formatters brought [`NumberPartType::ApproximatelySign`]), and a formatter can
/// always emit a kind the caller does not know, so match with a fallback arm.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    /// The "approximately" marker a collapsed range carries (see
    /// [`format_range_to_parts`]).
    ApproximatelySign,
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
            NumberPartType::ApproximatelySign => "approximatelySign",
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
    /// Narrow form (`"5km"`). Needs the `units-narrow` cargo feature; without it
    /// the narrow patterns are not compiled in and this falls back to `Short`.
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
///
/// The struct is `#[non_exhaustive]` (so new options can be added without a
/// breaking change): construct it from [`Default`] and set the fields you need,
/// e.g. `let mut o = NumberFormatOptions::default(); o.style = …;`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
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
    /// Numbering system override (e.g. `"arab"`, or `"native"` for the locale's
    /// `otherNumberingSystems.native`). Selects the digits *and* that system's
    /// CLDR separators/sign symbols for this locale, falling back to the `latn`
    /// ones where CLDR ships no block. `None` uses the tag's `-u-nu-` keyword if
    /// present, else the locale's own CLDR `defaultNumberingSystem` (`latn` for
    /// most locales, `arab` for `ar-EG`, `arabext` for `fa`) — the ECMA-402
    /// default. Per ECMA-402 `ResolveLocale`, this option outranks the tag.
    pub numbering_system: Option<&'static str>,
}

/// Lowercase `lang` and normalize `_` to `-`, the form the CLDR tables are keyed
/// in and the fallback chain is walked over.
fn normalize(lang: &str) -> String {
    lang.chars()
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

/// A locale resolved against a numbering system: the CLDR symbols/patterns of
/// the effective system plus its digit glyphs (`None` for `latn`, and for any
/// system with no positional digits — algorithmic systems like `jpan` degrade to
/// Latin digits rather than mis-rendering).
struct Resolved {
    spec: NumberSpec,
    digits: Option<&'static str>,
}

/// Split a normalized tag into its language part and the `-u-nu-` numbering
/// system, if any. BCP-47 §2.2.6: an extension begins at a singleton subtag, so
/// the language part ends there; inside the `u` extension a two-letter subtag is
/// a key and the subtag after it is its value (UTS #35 `nu`). `number` does not
/// depend on the `locale` feature, so this is a self-contained scan rather than
/// a `Locale` accessor.
fn split_nu(norm: &str) -> (&str, Option<&str>) {
    let mut lang_end = None;
    let mut nu = None;
    let mut in_u = false;
    let mut want_value = false;
    let mut off = 0usize;
    for sub in norm.split('-') {
        let start = off;
        off += sub.len() + 1;
        if sub.len() == 1 {
            if lang_end.is_none() {
                lang_end = Some(start.saturating_sub(1));
            }
            in_u = sub == "u";
            want_value = false;
        } else if in_u {
            if want_value {
                nu = Some(sub);
                in_u = false;
            } else if sub.len() == 2 {
                // Two-letter subtags are keys; longer ones before the first key
                // are `u`-extension attributes and carry no value.
                want_value = sub == "nu";
            }
        }
    }
    (&norm[..lang_end.unwrap_or(norm.len())], nu)
}

/// Resolve `lang` (and an explicit `system` override) to the symbols, patterns
/// and digits to format with.
///
/// The requested system comes from `system` if set, else from the tag's
/// `-u-nu-` keyword, else the locale's own CLDR `defaultNumberingSystem` —
/// ECMA-402 `InitializeNumberFormat` step 12, which is why `ar-EG` formats in
/// Arabic-Indic digits from a bare tag while `ar` stays Latin. Per ECMA-402
/// `ResolveLocale` an explicit option outranks the `-u-` keyword, which in turn
/// outranks the locale default. `"native"` is the UTS #35 alias for the locale's
/// `otherNumberingSystems` native system.
fn resolve(lang: &str, system: Option<&str>) -> Resolved {
    use crate::cldr::{number_spec, numbering_systems};
    let norm = normalize(lang);
    let (base, tag_nu) = split_nu(&norm);

    // Walk the fallback chain once; the first locale with data answers the
    // `native` alias, the locale default and the symbol lookup alike, matching
    // ICU's bundle inheritance.
    let mut end = base.len();
    let key = loop {
        if numbering_systems(&base[..end]).is_some() {
            break &base[..end];
        }
        match base[..end].rfind('-') {
            Some(i) => end = i,
            None => break "en",
        }
    };
    let system = match system.or(tag_nu) {
        Some("native") => numbering_systems(key).map_or("latn", |(_, native)| native),
        Some(other) => other,
        None => numbering_systems(key).map_or("latn", |(default, _)| default),
    };
    Resolved {
        spec: number_spec(key, system)
            .or_else(|| number_spec("en", system))
            .expect("root spec present"),
        digits: (system != "latn")
            .then(|| crate::cldr::numbering_digits(system))
            .flatten(),
    }
}

/// Transliterate the ASCII digits of a formatted run into `digits`.
fn map_digits(value: &str, digits: Option<&'static str>) -> String {
    let Some(glyphs) = digits else {
        return String::from(value);
    };
    let table: Vec<char> = glyphs.chars().collect();
    if table.len() != 10 {
        return String::from(value);
    }
    value
        .chars()
        .map(|c| {
            if c.is_ascii_digit() {
                table[(c as u8 - b'0') as usize]
            } else {
                c
            }
        })
        .collect()
}

/// The `defaultNumberingSystem` of `lang` (UTS #35): the system CLDR formats
/// numbers with by default, which for most locales — including `ar` and `hi` in
/// CLDR 48 — is `"latn"`.
///
/// It is a per-*locale* setting, not a per-language one: CLDR gives 22 region
/// locales a default their base language does not have, and they are read here
/// even though everything else about them is inherited.
///
/// ```
/// use intl::number::default_numbering_system as d;
/// assert_eq!(d("en"), "latn");
/// assert_eq!(d("ar"), "latn"); // matches `Intl.NumberFormat('ar')`
/// assert_eq!(d("ar-EG"), "arab"); // …and so does the Egyptian default
/// assert_eq!(d("fa"), "arabext");
/// ```
#[must_use]
pub fn default_numbering_system(lang: &str) -> &'static str {
    resolve_systems(lang).0
}

/// The `otherNumberingSystems.native` system of `lang` (UTS #35): the locale's
/// *native* digits, which are often not the default — `ar` defaults to `latn`
/// but its native system is `arab`.
///
/// Format with it by requesting `-u-nu-native` (or the resolved id) through
/// [`NumberFormatOptions::numbering_system`] or the tag itself.
///
/// ```
/// use intl::number::{format_decimal, native_numbering_system as n};
/// assert_eq!(n("en"), "latn");
/// assert_eq!(n("ar"), "arab");
/// assert_eq!(n("hi"), "deva");
/// assert_eq!(format_decimal("hi-u-nu-native", 1234.0), "१,२३४");
/// ```
#[must_use]
pub fn native_numbering_system(lang: &str) -> &'static str {
    resolve_systems(lang).1
}

/// `(default, native)` numbering systems for `lang`, through the fallback chain.
fn resolve_systems(lang: &str) -> (&'static str, &'static str) {
    let norm = normalize(lang);
    let (base, _) = split_nu(&norm);
    let mut end = base.len();
    loop {
        if let Some(pair) = crate::cldr::numbering_systems(&base[..end]) {
            return pair;
        }
        match base[..end].rfind('-') {
            Some(i) => end = i,
            None => return ("latn", "latn"),
        }
    }
}

/// Format `value` as a decimal number in the conventions of `lang`.
///
/// The digits are the locale's own: `resolve` applies CLDR's
/// `defaultNumberingSystem`, as ECMA-402 does, so `"ar-EG"` renders
/// `"١٬٢٣٤٫٥"` and `"en"` renders `"1,234.5"`. A `-u-nu-` keyword on the tag
/// (`"hi-u-nu-deva"`, `"ar-u-nu-native"`) overrides it, selecting both the
/// digits and that system's separators.
///
/// ```
/// use intl::number::format_decimal;
/// assert_eq!(format_decimal("en", 1234.5), "1,234.5");
/// assert_eq!(format_decimal("ar", 1234.5), "1,234.5"); // ar defaults to latn
// The non-`latn` separators need the per-system blocks (`number-numsys`).
#[cfg_attr(
    feature = "number-numsys",
    doc = r#"assert_eq!(format_decimal("ar-EG", 1234.5), "١٬٢٣٤٫٥"); // ar-EG defaults to arab"#
)]
/// ```
#[must_use]
pub fn format_decimal(lang: &str, value: f64) -> String {
    let r = resolve(lang, None);
    format_with(&r.spec.dec, value, &r, NumberStyle::Decimal, "")
}

/// Format `value` (a ratio, so `0.5` → `50%`) as a percent in `lang`.
#[must_use]
pub fn format_percent(lang: &str, value: f64) -> String {
    let r = resolve(lang, None);
    format_with(&r.spec.pct, value * 100.0, &r, NumberStyle::Percent, "")
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
    let r = resolve(lang, None);
    let s = r.spec;
    // Guard before the mantissa normalization below: `inf / 10.0` is still
    // `inf`, so the loop would never terminate (and `exp` would overflow).
    if !value.is_finite() {
        if value.is_nan() {
            return String::from(s.nan);
        }
        return if value < 0.0 {
            alloc::format!("{}{}", s.minus, s.infinity)
        } else {
            String::from(s.infinity)
        };
    }
    if value == 0.0 {
        return map_digits("0", r.digits);
    }
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
    out.push_str(&map_digits(int_part, r.digits));
    if !frac.is_empty() {
        out.push_str(s.decimal);
        out.push_str(&map_digits(frac, r.digits));
    }
    out.push('E');
    if exp < 0 {
        out.push_str(s.minus);
    }
    out.push_str(&map_digits(
        &alloc::format!("{}", exp.unsigned_abs()),
        r.digits,
    ));
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
/// **Digits only, by design.** Separators are deliberately untouched, even
/// though `arab` conventionally pairs with U+066B/U+066C rather than `.`/`,`:
/// CLDR has no locale-independent symbol table for a numbering system. Symbols
/// live per locale under `symbols-numberSystem-<ns>` and disagree between
/// locales for the *same* system — `ar`'s `arab` decimal separator is U+066B
/// while `sd`'s is `.`, and `arabext`'s percent and minus differ across `fa`,
/// `ps` and `ur`. Picking one locale's symbols here would silently impose it on
/// every caller. Use the locale-aware path instead — a `-u-nu-` tag or
/// [`NumberFormatOptions::numbering_system`] — which resolves symbols against
/// the locale, as ICU does.
///
/// ```
/// use intl::number::{format_decimal, to_numbering_system};
/// assert_eq!(to_numbering_system("2024", "arab"), "٢٠٢٤");
/// assert_eq!(to_numbering_system("3.14", "deva"), "३.१४");
/// // Separators stay put; the locale-aware path supplies them.
/// assert_eq!(to_numbering_system("1.5", "arab"), "١.٥");
// The contrast only holds with the per-system blocks compiled in; without
// `number-numsys` the locale-aware path also keeps the `latn` separators.
#[cfg_attr(
    feature = "number-numsys",
    doc = r#"assert_eq!(format_decimal("ar-u-nu-arab", 1.5), "١٫٥");"#
)]
#[cfg_attr(
    feature = "number-numsys",
    doc = r#"assert_eq!(format_decimal("sd-u-nu-arab", 1.5), "١.٥"); // same system, other symbols"#
)]
/// ```
#[must_use]
pub fn to_numbering_system(s: &str, system: &str) -> String {
    map_digits(s, crate::cldr::numbering_digits(system))
}

/// Format `value` as a decimal in `lang` using the locale's CLDR
/// `defaultNumberingSystem` — digits *and* that system's separators (so Persian
/// renders `"۱٬۲۳۴٫۵"`, not Latin separators with Persian digits).
///
/// **Now a synonym of [`format_decimal`].** Since 0.7.0 the ordinary entry
/// points apply the locale's `defaultNumberingSystem` themselves, as ECMA-402
/// does, so this function no longer selects anything they would not.
///
/// ```
/// use intl::number::format_decimal_default_numbering as f;
/// assert_eq!(f("en", 1234.5), "1,234.5");
/// assert_eq!(f("ar", 1234.5), "1,234.5");   // ar defaults to latn in CLDR 48
// The arabext/arab separators need the per-system blocks (`number-numsys`).
#[cfg_attr(
    feature = "number-numsys",
    doc = r#"assert_eq!(f("fa", 1234.5), "۱٬۲۳۴٫۵");   // fa defaults to arabext"#
)]
#[cfg_attr(
    feature = "number-numsys",
    doc = r#"assert_eq!(f("ar-EG", 1234.5), "١٬٢٣٤٫٥"); // but ar-EG defaults to arab"#
)]
/// ```
#[must_use]
#[deprecated(
    since = "0.7.0",
    note = "the default numbering system is now the default: this is a synonym of `format_decimal`"
)]
pub fn format_decimal_default_numbering(lang: &str, value: f64) -> String {
    format_decimal(lang, value)
}

/// Format `value` as a decimal in `lang` using the locale's default numbering
/// system.
#[must_use]
#[deprecated(
    since = "0.6.0",
    note = "misnamed: it reads `defaultNumberingSystem`, not `otherNumberingSystems.native`. \
            Use `format_decimal_default_numbering`, or `format_decimal(\"<lang>-u-nu-native\", …)` \
            for the native system."
)]
pub fn format_decimal_native(lang: &str, value: f64) -> String {
    format_decimal(lang, value)
}

/// Format `value` in compact (short) form in `lang`, e.g.
/// `format_compact("en", 1500.0)` → `"1.5K"`, `format_compact("en", 2_300_000.0)`
/// → `"2.3M"`. Values below 1000 (or magnitudes the locale does not abbreviate)
/// are written out in full.
///
/// The precision is compact notation's own (ECMA-402 `roundingPriority:
/// "morePrecision"` over `maximumFractionDigits: 0` and 2 significant digits),
/// so `1500` keeps its tenth but `123456789` does not: `"123M"`. This is
/// [`format`] with [`Notation::Compact`], and stays in step with it.
///
/// ```
/// use intl::number::format_compact;
/// assert_eq!(format_compact("en", 1500.0), "1.5K");
/// assert_eq!(format_compact("en", 2_300_000.0), "2.3M");
/// assert_eq!(format_compact("en", 123_456_789.0), "123M");
/// assert_eq!(format_compact("en", 999.0), "999");
/// ```
#[must_use]
pub fn format_compact(lang: &str, value: f64) -> String {
    format(
        lang,
        value,
        &NumberFormatOptions {
            notation: Notation::Compact,
            ..NumberFormatOptions::default()
        },
    )
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
    parse_decimal_with(&resolve(lang, None).spec, input)
}

/// The locale's compact-notation pattern table, through the fallback chain.
fn compact_table(lang: &str) -> [&'static str; 24] {
    let norm = normalize(lang);
    let (base, _) = split_nu(&norm);
    let mut end = base.len();
    loop {
        if let Some(t) = crate::cldr::compact_patterns(&base[..end]) {
            return t;
        }
        match base[..end].rfind('-') {
            Some(i) => end = i,
            None => return crate::cldr::compact_patterns("en").expect("root compact present"),
        }
    }
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
#[cfg(feature = "currency")]
pub fn format_currency(lang: &str, value: f64, code: &str) -> String {
    use crate::cldr as cur;
    let r = resolve(lang, None);

    // Resolve the currency pattern and symbol through the locale fallback chain.
    let norm = normalize(lang);
    let norm = String::from(split_nu(&norm).0);
    let mut pat = cur::currency_pattern("en").expect("root currency pattern");
    let mut symbol = code;
    let mut end = norm.len();
    let (mut got_pat, mut got_sym) = (false, false);
    loop {
        if !got_pat && let Some(p) = cur::currency_pattern(&norm[..end]) {
            pat = p;
            got_pat = true;
        }
        if !got_sym && let Some((sym, _, _)) = cur::currency_forms(&norm[..end], code) {
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
    if !got_sym && let Some((sym, _, _)) = cur::currency_forms("en", code) {
        symbol = sym;
    }

    let digits = cur::currency_digits(code);
    pat.min_frac = digits;
    pat.max_frac = digits;

    // The pattern carries the ¤ placeholder; `format_with` substitutes the
    // symbol for it (and applies UTS #35 currency spacing around it).
    format_with(&pat, value, &r, NumberStyle::Currency, symbol)
}

fn format_with(
    p: &Pattern,
    value: f64,
    r: &Resolved,
    style: NumberStyle,
    currency: &str,
) -> String {
    join_parts(&format_with_parts(p, value, r, style, currency))
}

/// The free-function path ([`format_decimal`] / [`format_percent`] /
/// [`format_currency`]): the pattern's own fixed-width rounding, wrapped by the
/// same [`wrap`] the [`NumberFormat`-style](format_to_parts) path uses. Sharing
/// the wrapper is what keeps the sign, the affixes and the non-finite spelling
/// from drifting apart between the two entry points.
fn format_with_parts(
    p: &Pattern,
    value: f64,
    r: &Resolved,
    style: NumberStyle,
    currency: &str,
) -> Vec<NumberPart> {
    let opts = NumberFormatOptions {
        style,
        ..NumberFormatOptions::default()
    };
    let body = if value.is_finite() {
        fixed_body(p, value, r)
    } else {
        non_finite_body(value, &r.spec)
    };
    wrap(body, p, &opts, &r.spec, currency).parts
}

/// Round `value` to the pattern's fixed fraction width and group it — the
/// historical [`format_decimal`] rounding (the float formatter's, not
/// [`round_digits`]'s), which the free functions keep.
fn fixed_body(p: &Pattern, value: f64, r: &Resolved) -> Body {
    let negative = value.is_sign_negative() && value != 0.0;
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

    let is_zero = int_str.bytes().all(|b| b == b'0') && frac.bytes().all(|b| b == b'0');
    Body {
        parts: digit_parts(int_str, frac, p.primary_group, p.secondary_group, r),
        inner: (0, 0),
        negative,
        is_zero,
    }
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
///
/// ECMA-402 `InitializeNumberFormat` step 21 gives compact notation a default of
/// `"min2"` rather than `"auto"`, which is why `de` compacts `1234` to `"1234"`
/// but `12345` to `"12.345"`.
fn effective_grouping(opts: &NumberFormatOptions, pattern: &Pattern, int_len: usize) -> (u8, u8) {
    let strategy = match (opts.use_grouping, opts.notation) {
        (UseGrouping::Auto, Notation::Compact) => UseGrouping::Min2,
        (other, _) => other,
    };
    match strategy {
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

/// The digit run: the grouped integer, then the decimal separator and fraction.
/// The numbering system supplies both the glyphs and — through [`resolve`] —
/// the separators, as ICU's `NumberElements` does.
fn digit_parts(
    int_digits: &str,
    frac_digits: &str,
    primary: u8,
    secondary: u8,
    r: &Resolved,
) -> Vec<NumberPart> {
    let s = &r.spec;
    let mut parts = Vec::new();
    for mut p in group_parts(int_digits, primary, secondary, s.group) {
        if p.kind == NumberPartType::Integer {
            p.value = map_digits(&p.value, r.digits);
        }
        parts.push(p);
    }
    if !frac_digits.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Decimal, s.decimal));
        parts.push(NumberPart::new(
            NumberPartType::Fraction,
            map_digits(frac_digits, r.digits),
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

/// ECMA-402 `SetNumberFormatDigitOptions` step 16.a: the resolved
/// `(minimum, maximum)` fraction digits, given the style's defaults.
///
/// Supplying one of the pair moves the other rather than being clamped by it:
/// `{maximumFractionDigits: 0}` on a currency sets the *minimum* to
/// `min(mnfdDefault, mxfd)` — so `$3`, not the `$3.00` a plain clamp would keep.
fn fraction_digits(
    opts: &NumberFormatOptions,
    min_default: usize,
    max_default: usize,
) -> (usize, usize) {
    match (opts.minimum_fraction_digits, opts.maximum_fraction_digits) {
        (None, None) => (min_default, max_default),
        (None, Some(mx)) => (min_default.min(usize::from(mx)), usize::from(mx)),
        (Some(mn), None) => (usize::from(mn), max_default.max(usize::from(mn))),
        // The spec throws when mnfd > mxfd; with no error channel here, widen.
        (Some(mn), Some(mx)) => (usize::from(mn), usize::from(mx).max(usize::from(mn))),
    }
}

/// How many digits a value keeps: ECMA-402 `SetNumberFormatDigitOptions`'
/// resolved rounding type. Normally one of the two limits is live —
/// significant digits when the caller asked for them, fraction digits
/// otherwise (`roundingPriority: "auto"`). `more_precision` is the third case,
/// which the spec creates on its own for compact notation: *both* limits apply
/// and the more precise one wins.
struct Precision {
    min_frac: usize,
    max_frac: usize,
    min_sig: Option<usize>,
    max_sig: Option<usize>,
    more_precision: bool,
}

/// ECMA-402 `SetNumberFormatDigitOptions`: the single place that decides a
/// request's precision, from the notation and the style's `mnfdDefault` /
/// `mxfdDefault` — which are the style pattern's own fraction bounds (0–3 for
/// decimal, 0–0 for percent, the currency's digits for currency), so a
/// scientific mantissa is bounded by the style like everything else rather than
/// by a count of its own.
///
/// The notation enters at step 15.b, which gives **compact** a default no other
/// notation has: asked for neither fraction nor significant digits, it resolves
/// to `roundingPriority: "morePrecision"` with `maximumFractionDigits: 0` and
/// 2 significant digits. That is why `123456789` compacts to `123M` while
/// `1500` keeps its tenth as `1.5K`.
fn digit_options(opts: &NumberFormatOptions, min_default: usize, max_default: usize) -> Precision {
    let has_sd =
        opts.minimum_significant_digits.is_some() || opts.maximum_significant_digits.is_some();
    let has_fd = opts.minimum_fraction_digits.is_some() || opts.maximum_fraction_digits.is_some();
    if has_sd {
        // Step 17: either bound alone selects significant-digit rounding, the
        // other taking the spec's own default (1 and 21).
        let min_sig = opts.minimum_significant_digits.map_or(1, usize::from);
        let max_sig = opts
            .maximum_significant_digits
            .map_or(21, usize::from)
            .max(min_sig);
        Precision {
            min_frac: 0,
            max_frac: 0,
            min_sig: Some(min_sig),
            max_sig: Some(max_sig),
            more_precision: false,
        }
    } else if has_fd || opts.notation != Notation::Compact {
        let (min_frac, max_frac) = fraction_digits(opts, min_default, max_default);
        Precision {
            min_frac,
            max_frac,
            min_sig: None,
            max_sig: None,
            more_precision: false,
        }
    } else {
        // Step 15.d.iii: compact's implicit `morePrecision` over 0 fraction and
        // 2 significant digits.
        Precision {
            min_frac: 0,
            max_frac: 0,
            min_sig: Some(1),
            max_sig: Some(2),
            more_precision: true,
        }
    }
}

/// The decimal exponent of a rounded digit string — the power of ten of its
/// leading significant digit, and `0` for a zero magnitude (ECMA-402
/// `ToRawPrecision` step 2's `e` for `x = 0`).
fn magnitude_exp(int_digits: &str, frac_digits: &str) -> i32 {
    if let Some(i) = int_digits.bytes().position(|b| b != b'0') {
        return (int_digits.len() - i - 1) as i32;
    }
    match frac_digits.bytes().position(|b| b != b'0') {
        Some(i) => -(i as i32) - 1,
        None => 0,
    }
}

/// Round `abs` to a digit string per `p`: [`round_digits`] plus ECMA-402
/// `FormatNumericToString` step 7's `morePrecision` tie-break, which rounds the
/// value both ways and keeps whichever lands on the smaller — more precise —
/// `[[RoundingMagnitude]]` (`e - p + 1` for significant digits, `-mxfd` for
/// fraction digits).
fn round_to(
    abs: f64,
    min_int: usize,
    p: &Precision,
    mode: RoundingMode,
    negative: bool,
) -> (String, String) {
    if !p.more_precision {
        return round_digits(
            abs, min_int, p.min_frac, p.max_frac, p.min_sig, p.max_sig, mode, negative,
        );
    }
    let sig = round_digits(abs, min_int, 0, 0, p.min_sig, p.max_sig, mode, negative);
    let sig_magnitude = magnitude_exp(&sig.0, &sig.1) - p.max_sig.unwrap_or(1) as i32 + 1;
    if sig_magnitude <= -(p.max_frac as i32) {
        sig
    } else {
        round_digits(
            abs, min_int, p.min_frac, p.max_frac, None, None, mode, negative,
        )
    }
}

/// The plural category of `value` in `lang`, as a [`crate::plural::PluralCategory`]
/// discriminant: what a unit phrase's wording agrees with. Without the unit
/// tables there is no such wording, and nothing reads the result.
fn plural_of(lang: &str, value: f64) -> usize {
    #[cfg(feature = "units")]
    {
        crate::unit::category(lang, value)
    }
    #[cfg(not(feature = "units"))]
    {
        let _ = (lang, value);
        crate::plural::PluralCategory::Other as usize
    }
}

/// A formatted number, plus the boundaries of the two modifier layers that sit
/// *inside* the unit wrapper. They are ICU's `modInner` (the notation — a
/// compact suffix, a scientific exponent) and `modMiddle` (the style affixes —
/// currency symbol, percent sign, sign), and `parts` is laid out
///
/// ```text
/// middle.0 | inner.0 | digits | inner.1 | middle.1
/// ```
///
/// with each field a count of *parts*, not characters. Only
/// [`format_range_to_parts`] reads the split: `NumberRangeFormatterImpl::
/// formatRange` decides layer by layer what may be factored out of both ends.
#[cfg_attr(not(feature = "number-range"), allow(dead_code))]
struct Formatted {
    parts: Vec<NumberPart>,
    middle: (usize, usize),
    inner: (usize, usize),
}

#[cfg(feature = "number-range")]
impl Formatted {
    /// The `modMiddle` prefix and suffix.
    fn middle_mod(&self) -> (&[NumberPart], &[NumberPart]) {
        (
            &self.parts[..self.middle.0],
            &self.parts[self.parts.len() - self.middle.1..],
        )
    }

    /// Everything inside `modMiddle`: the digits with the notation modifier
    /// still attached, which is what a collapsed range repeats per end.
    fn middle_body(&self) -> &[NumberPart] {
        &self.parts[self.middle.0..self.parts.len() - self.middle.1]
    }

    /// Code points in `modMiddle`. ICU's AUTO collapse heuristic factors out
    /// only modifiers longer than one code point, which is why `+$` collapses
    /// but a lone `$` or `+` does not.
    fn middle_len(&self) -> usize {
        let (pre, post) = self.middle_mod();
        pre.iter()
            .chain(post)
            .map(|p| p.value.chars().count())
            .sum()
    }

    /// Code points in `modInner`.
    fn inner_len(&self) -> usize {
        let end = self.parts.len() - self.middle.1;
        self.parts[self.middle.0..self.middle.0 + self.inner.0]
            .iter()
            .chain(&self.parts[end - self.inner.1..end])
            .map(|p| p.value.chars().count())
            .sum()
    }
}

/// The digits of a number together with the notation's own modifier — ICU's
/// `modInner`, a compact magnitude suffix or a scientific exponent — plus what
/// the sign depends on. [`wrap`] turns one into a [`Formatted`] by applying the
/// style's affixes around it, which is the only place they are applied.
struct Body {
    parts: Vec<NumberPart>,
    /// Parts of the notation modifier at each end of `parts`.
    inner: (usize, usize),
    negative: bool,
    /// Zero *or* NaN: what ECMA-402's `exceptZero` sign display suppresses.
    is_zero: bool,
}

/// Apply the sign and the pattern's affixes — ICU's `modMiddle` — around a
/// [`Body`]. The sign precedes the prefix affix and belongs to the same
/// modifier as it: `+$` is one two-code-point `modMiddle`.
fn wrap(
    body: Body,
    pattern: &Pattern,
    opts: &NumberFormatOptions,
    s: &NumberSpec,
    currency: &str,
) -> Formatted {
    let mut parts = Vec::new();
    if let Some(sign) = sign_part(body.negative, body.is_zero, opts, s) {
        parts.push(sign);
    }
    let mut prefix = affix_parts(pattern.prefix, opts.style, s, currency);
    let mut suffix = affix_parts(pattern.suffix, opts.style, s, currency);
    currency_spacing(&mut prefix, &mut suffix, &body.parts, opts.style);
    let middle = (parts.len() + prefix.len(), suffix.len());
    parts.extend(prefix);
    parts.extend(body.parts);
    parts.extend(suffix);
    Formatted {
        parts,
        middle,
        inner: body.inner,
    }
}

/// UTS #35 §3.5 `currencySpacing`: insert a no-break space between the currency
/// and the number when the currency's adjacent character is neither a symbol
/// nor a space (`currencyMatch` `[[:^S:]&[:^Z:]]` — an alphabetic code like
/// `USD` or `SEK`) and the number's adjacent character is a digit
/// (`surroundingMatch` `[:digit:]`). Hence `en` writes `"USD 3.00"` and
/// `"SEK 3.00"` but `"$3.00"` (a symbol) and `"USDNaN"` (no digit), and `de`
/// writes `"3,00 USD"` with no second space, its pattern already ending in one.
///
/// The rule is applied directly rather than carried as per-locale data because
/// there is nothing per-locale about it: all 173 `currencyFormats` blocks in the
/// vendored CLDR 48 (125 locales × numbering system) declare root's rule
/// verbatim, `insertBetween` U+00A0 included.
fn currency_spacing(
    prefix: &mut Vec<NumberPart>,
    suffix: &mut Vec<NumberPart>,
    body: &[NumberPart],
    style: NumberStyle,
) {
    if style != NumberStyle::Currency {
        return;
    }
    use crate::unicode::{is_decimal_digit, is_separator, is_symbol};
    // Like ICU's `applyCurrencySpacingAffix`, the test is on the affix part that
    // abuts the number: it must be the currency itself, so `he`'s pattern —
    // whose prefix is a lone RLM, the ¤ being in the suffix — inserts nothing.
    let edge = |part: Option<&NumberPart>, last: bool| {
        part.filter(|p| p.kind == NumberPartType::Currency)
            .and_then(|p| {
                if last {
                    p.value.chars().next_back()
                } else {
                    p.value.chars().next()
                }
            })
    };
    let joins = |currency: Option<char>, number: Option<char>| match (currency, number) {
        (Some(c), Some(n)) => !is_symbol(c) && !is_separator(c) && is_decimal_digit(n),
        _ => false,
    };
    let first_digit = body.iter().flat_map(|p| p.value.chars()).next();
    let last_digit = body.iter().rev().flat_map(|p| p.value.chars().rev()).next();
    if joins(edge(prefix.last(), true), first_digit) {
        prefix.push(NumberPart::new(NumberPartType::Literal, "\u{a0}"));
    }
    if joins(edge(suffix.first(), false), last_digit) {
        suffix.insert(0, NumberPart::new(NumberPartType::Literal, "\u{a0}"));
    }
}

/// Split a unit-pattern affix into the whitespace `Literal` that separates it
/// from the number and the `Unit` remainder. ICU tags the whole unit phrase —
/// interior spaces included — as one `unit` field, so `"1.5 m"` is
/// `… literal(" ") unit("m")` and `"5 meters per second"` is
/// `… literal(" ") unit("meters per second")`. `prefix` says which end abuts the
/// number: a prefix affix is separated on its right, a suffix affix on its left.
#[cfg(feature = "units")]
fn unit_affix(text: &str, prefix: bool) -> Vec<NumberPart> {
    let mut parts = Vec::new();
    let (unit, sep) = if prefix {
        let unit = text.trim_end_matches(char::is_whitespace);
        (unit, &text[unit.len()..])
    } else {
        let unit = text.trim_start_matches(char::is_whitespace);
        (unit, &text[..text.len() - unit.len()])
    };
    let mut push = |kind, s: &str| {
        if !s.is_empty() {
            parts.push(NumberPart::new(kind, String::from(s)));
        }
    };
    if prefix {
        push(NumberPartType::Unit, unit);
        push(NumberPartType::Literal, sep);
    } else {
        push(NumberPartType::Literal, sep);
        push(NumberPartType::Unit, unit);
    }
    parts
}

/// Wrap the numeric `core` parts with the locale's CLDR unit pattern (e.g.
/// `"{0} km"`, `"{0} meters per second"`), worded for plural category `plural`.
/// Resolution — locale fallback, width, compound assembly — is shared with
/// [`crate::unit`]. An unknown/missing unit degrades to the bare number.
/// Returns the wrapped parts and how many of them are the wrapper itself.
#[cfg(feature = "units")]
fn unit_wrap(
    lang: &str,
    opts: &NumberFormatOptions,
    plural: usize,
    core: Vec<NumberPart>,
) -> (Vec<NumberPart>, (usize, usize)) {
    let width = match opts.unit_display {
        UnitDisplay::Short => crate::unit::UnitWidth::Short,
        UnitDisplay::Narrow => crate::unit::UnitWidth::Narrow,
        UnitDisplay::Long => crate::unit::UnitWidth::Long,
    };
    let Some(pattern) = opts
        .unit
        .and_then(|id| crate::unit::pattern_for_id(lang, id, width, plural))
    else {
        return (core, (0, 0));
    };

    let (pre, post) = pattern.split_once("{0}").unwrap_or(("", &pattern));
    let head = unit_affix(pre, true);
    let tail = unit_affix(post, false);
    let counts = (head.len(), tail.len());
    let mut parts = head;
    parts.extend(core);
    parts.extend(tail);
    (parts, counts)
}

/// ICU's `modOuter`: the phrase wrapped around a formatted number that is not
/// part of the number pattern — a measurement unit, or the currency code/name
/// spliced into the locale's currency unit pattern. Applied last, and (unlike
/// `modMiddle`) parameterized by plural category rather than by sign, which is
/// what lets a range factor it out and re-word it once.
fn outer_wrap(
    lang: &str,
    opts: &NumberFormatOptions,
    plural: usize,
    parts: Vec<NumberPart>,
) -> (Vec<NumberPart>, (usize, usize)) {
    #[cfg(feature = "units")]
    if opts.style == NumberStyle::Unit {
        return unit_wrap(lang, opts, plural, parts);
    }
    #[cfg(feature = "currency")]
    if opts.style == NumberStyle::Currency && opts.currency_display == CurrencyDisplay::Name {
        return currency_unit_wrap(lang, opts, parts);
    }
    let _ = (lang, opts, plural);
    (parts, (0, 0))
}

/// Render a currency amount with `currencyDisplay: name`: the numeric `core`
/// spliced into the locale's currency unit pattern (`"{0} {1}"`) with the display
/// name tagged `Currency`. (The base display name is used; plural name forms are
/// not applied.) Returns the wrapped parts and how many of them are the wrapper
/// itself.
///
/// `code` does *not* come through here — ECMA-402 renders it through the ¤
/// pattern like a symbol.
#[cfg(feature = "currency")]
fn currency_unit_wrap(
    lang: &str,
    opts: &NumberFormatOptions,
    core: Vec<NumberPart>,
) -> (Vec<NumberPart>, (usize, usize)) {
    let code = opts.currency.unwrap_or("XXX");
    let norm = normalize(lang);
    let norm = String::from(split_nu(&norm).0);
    let mut forms: Option<(&str, &str, &str)> = None;
    let mut unit = "{0} {1}";
    let mut end = norm.len();
    let mut got_unit = false;
    loop {
        if forms.is_none() {
            forms = crate::cldr::currency_forms(&norm[..end], code);
        }
        if !got_unit && let Some(u) = crate::cldr::currency_unit_pattern(&norm[..end]) {
            unit = u;
            got_unit = true;
        }
        if forms.is_some() && got_unit {
            break;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => break,
        }
    }
    forms = forms.or_else(|| crate::cldr::currency_forms("en", code));
    let (_, _, text) = forms.unwrap_or((code, code, code));

    // Splice into the two-placeholder unit pattern ({0} number, {1} currency).
    let mut parts = Vec::new();
    let mut before = 0usize;
    let mut seen_core = false;
    let mut rest = unit;
    while !rest.is_empty() {
        if let Some(i) = rest.find('{') {
            if i > 0 {
                parts.push(NumberPart::new(NumberPartType::Literal, &rest[..i]));
            }
            if rest[i..].starts_with("{0}") {
                before = parts.len();
                seen_core = true;
                parts.extend(core.iter().cloned());
                rest = &rest[i + 3..];
            } else if rest[i..].starts_with("{1}") {
                parts.push(NumberPart::new(NumberPartType::Currency, text));
                rest = &rest[i + 3..];
            } else {
                parts.push(NumberPart::new(NumberPartType::Literal, "{"));
                rest = &rest[i + 1..];
            }
        } else {
            parts.push(NumberPart::new(NumberPartType::Literal, rest));
            break;
        }
    }
    // A pattern with no `{0}` swallows the number; treat all of it as wrapper.
    let after = if seen_core {
        parts.len() - before - core.len()
    } else {
        parts.len()
    };
    (parts, (before, after))
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
        // Without the `currency` feature, currency style degrades to decimal.
        #[cfg(not(feature = "currency"))]
        NumberStyle::Currency => (s.dec, value, String::new()),
        #[cfg(feature = "currency")]
        NumberStyle::Currency => {
            let code = opts.currency.unwrap_or("XXX");
            let norm = normalize(lang);
            let norm = String::from(split_nu(&norm).0);
            let mut pat = crate::cldr::currency_pattern("en").expect("root currency pattern");
            // (symbol, narrow symbol, display name) for the requested currency.
            let mut forms: Option<(&str, &str, &str)> = None;
            let mut end = norm.len();
            let mut got_pat = false;
            loop {
                if !got_pat && let Some(p) = crate::cldr::currency_pattern(&norm[..end]) {
                    pat = p;
                    got_pat = true;
                }
                if forms.is_none() {
                    forms = crate::cldr::currency_forms(&norm[..end], code);
                }
                if got_pat && forms.is_some() {
                    break;
                }
                match norm[..end].rfind('-') {
                    Some(i) => end = i,
                    None => break,
                }
            }
            if forms.is_none() {
                forms = crate::cldr::currency_forms("en", code);
            }
            let (sym, narrow, name) = forms.unwrap_or((code, code, code));
            let shown = match opts.currency_display {
                CurrencyDisplay::Symbol => String::from(sym),
                CurrencyDisplay::NarrowSymbol => String::from(narrow),
                CurrencyDisplay::Code => String::from(code),
                CurrencyDisplay::Name => String::from(name),
            };
            let digits = crate::cldr::currency_digits(code);
            pat.min_frac = digits;
            pat.max_frac = digits;
            (pat, value, shown)
        }
    }
}

/// Standard (positional) notation: the digits, grouped.
fn standard_body(
    scaled: f64,
    pattern: &Pattern,
    p: &Precision,
    opts: &NumberFormatOptions,
    r: &Resolved,
) -> Body {
    let min_int = (opts.minimum_integer_digits.max(1)) as usize;
    let negative = scaled.is_sign_negative() && scaled != 0.0;
    let abs = if scaled < 0.0 { -scaled } else { scaled };
    let (int_d, frac_d) = round_to(abs, min_int, p, opts.rounding_mode, negative);
    let is_zero = int_d.bytes().all(|b| b == b'0') && frac_d.bytes().all(|b| b == b'0');
    let (pri, sec) = effective_grouping(opts, pattern, int_d.len());
    Body {
        parts: digit_parts(&int_d, &frac_d, pri, sec, r),
        inner: (0, 0),
        negative,
        is_zero,
    }
}

/// Scientific (`base = 1`) or engineering (`base = 3`) notation. The exponent is
/// ICU's `modInner` — the innermost modifier, which the AUTO collapse level
/// never factors out of a range.
fn exponent_body(
    value: f64,
    p: &Precision,
    opts: &NumberFormatOptions,
    r: &Resolved,
    base: i32,
) -> Body {
    let s = &r.spec;
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

    let (mut int_d, mut frac_d) = round_to(m, 1, p, opts.rounding_mode, negative);

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

    let mut parts = Vec::new();
    parts.push(NumberPart::new(
        NumberPartType::Integer,
        map_digits(&int_d, r.digits),
    ));
    if !frac_d.is_empty() {
        parts.push(NumberPart::new(NumberPartType::Decimal, s.decimal));
        parts.push(NumberPart::new(
            NumberPartType::Fraction,
            map_digits(&frac_d, r.digits),
        ));
    }
    let digits_end = parts.len();
    parts.push(NumberPart::new(NumberPartType::ExponentSeparator, "E"));
    if exp < 0 {
        parts.push(NumberPart::new(NumberPartType::ExponentMinusSign, s.minus));
    }
    parts.push(NumberPart::new(
        NumberPartType::ExponentInteger,
        map_digits(&alloc::format!("{}", exp.unsigned_abs()), r.digits),
    ));
    Body {
        inner: (0, parts.len() - digits_end),
        parts,
        negative,
        is_zero: abs == 0.0,
    }
}

/// Compact notation: the mantissa spliced into the locale's magnitude pattern.
/// A value the locale does not abbreviate at this magnitude — or one below the
/// smallest band — is written out in full, but still at compact's precision,
/// which is why `{notation: "compact"}` renders `123.456` as `"123"`.
fn compact_body(
    lang: &str,
    scaled: f64,
    pattern: &Pattern,
    p: &Precision,
    opts: &NumberFormatOptions,
    r: &Resolved,
) -> Body {
    let abs = if scaled < 0.0 { -scaled } else { scaled };
    // Below 1000 the magnitude exponent is not one of the table's bands, so the
    // `exp - 3` index below cannot underflow.
    if abs < 1000.0 {
        return standard_body(scaled, pattern, p, opts, r);
    }
    let table = compact_table(lang);
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
    // The band, and the mantissa rounded in it. Rounding can carry the mantissa
    // into the *next* band (999_999 rounds to 1000 thousands), so re-select the
    // magnitude and round again rather than printing "1000K" — ICU's
    // `CompactHandler::processQuantity` does the same second pass, giving "1M".
    let (magnitude, int_d, frac_d, negative) = loop {
        let magnitude = table[base + (exp - 3).min(11)];
        let zeros = magnitude.chars().filter(|&c| c == '0').count();
        // A pattern of only `0`s (no magnitude suffix) means "do not abbreviate".
        let has_suffix = magnitude
            .chars()
            .any(|c| c != '0' && c != '\'' && !c.is_whitespace());
        if zeros == 0 || !has_suffix {
            return standard_body(scaled, pattern, p, opts, r);
        }
        let mut divisor = 1.0f64;
        for _ in 0..(exp + 1).saturating_sub(zeros) {
            divisor *= 10.0;
        }
        let mantissa = scaled / divisor;
        let negative = mantissa.is_sign_negative() && mantissa != 0.0;
        let mabs = if mantissa < 0.0 { -mantissa } else { mantissa };
        let (int_d, frac_d) = round_to(mabs, 1, p, opts.rounding_mode, negative);
        if int_d.len() > zeros && exp < 14 {
            exp += 1;
            continue;
        }
        break (magnitude, int_d, frac_d, negative);
    };

    // Render the magnitude pattern, substituting the digits for the `0`-run and
    // tagging its literal text as `compact`. Those literals are ICU's
    // `modInner`, so they sit inside the style's affixes.
    let mut parts = Vec::new();
    let (mut digits_start, mut digits_end) = (0, 0);
    let mut wrote = false;
    let mut chars = magnitude.chars().peekable();
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
                    digits_start = parts.len();
                    // The mantissa groups like any other integer, which only
                    // shows past the largest band, where it runs long: ICU
                    // writes 10^18 as "1,000,000T".
                    let (pri, sec) = effective_grouping(opts, pattern, int_d.len());
                    parts.extend(digit_parts(&int_d, &frac_d, pri, sec, r));
                    digits_end = parts.len();
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
    Body {
        inner: (digits_start, parts.len() - digits_end),
        parts,
        negative,
        is_zero: false,
    }
}

/// A non-finite value: the ECMA-402 `∞` / `NaN` placeholder in place of the
/// digits. It is a body like any other, so the style's affixes and the sign
/// wrap it as usual — `"$NaN"`, `"∞%"`.
fn non_finite_body(value: f64, s: &NumberSpec) -> Body {
    let nan = value.is_nan();
    Body {
        parts: alloc::vec![if nan {
            NumberPart::new(NumberPartType::Nan, s.nan)
        } else {
            NumberPart::new(NumberPartType::Infinity, s.infinity)
        }],
        inner: (0, 0),
        negative: !nan && value < 0.0,
        // ECMA-402 `PartitionNumberPattern` step 4 lumps NaN in with zero, so
        // `signDisplay: "exceptZero"` leaves it unsigned while `"always"` does
        // sign it (`"+NaN"`).
        is_zero: nan,
    }
}

/// The number itself — everything except the outer unit/currency-name wrapper,
/// which ICU applies last and which a range factors out of both ends.
///
/// One place resolves the style (its pattern, its scaling and its currency
/// text), one place resolves the precision, and one place wraps the result in
/// the affixes, so every notation and the non-finite forms alike keep the
/// style: `"1.23E4"` in a currency is `"$1.23E4"`, and `{style: "unit"}` renders
/// `"NaN km"` rather than a bare `"NaN"`.
fn format_number(lang: &str, value: f64, r: &Resolved, opts: &NumberFormatOptions) -> Formatted {
    let s = &r.spec;
    #[allow(unused_mut)]
    let (mut pattern, scaled, currency) = resolve_style(lang, value, s, opts);
    // `currencyDisplay: name` keeps the currency pattern's digits and grouping
    // but drops its ¤ affixes: the display name is applied outside, through the
    // locale's currency unit pattern ("{0} {1}"). `code` is *not* one of those —
    // ECMA-402 substitutes the code into the ¤ pattern as it does a symbol, so
    // `en` writes "USD 3.00" rather than "3.00 USD".
    #[cfg(feature = "currency")]
    if opts.style == NumberStyle::Currency && opts.currency_display == CurrencyDisplay::Name {
        pattern.prefix = "";
        pattern.suffix = "";
    }
    let body = if scaled.is_finite() {
        let p = digit_options(opts, pattern.min_frac as usize, pattern.max_frac as usize);
        match opts.notation {
            Notation::Standard => standard_body(scaled, &pattern, &p, opts, r),
            Notation::Scientific => exponent_body(scaled, &p, opts, r, 1),
            Notation::Engineering => exponent_body(scaled, &p, opts, r, 3),
            Notation::Compact => compact_body(lang, scaled, &pattern, &p, opts, r),
        }
    } else {
        non_finite_body(scaled, s)
    };
    wrap(body, &pattern, opts, s, &currency)
}

/// Format `value` in `lang` per ECMA-402-style `opts`, returning the tagged
/// parts (`Intl.NumberFormat.prototype.formatToParts`).
///
/// ```
/// use intl::number::{format_to_parts, NumberFormatOptions, NumberPartType, UseGrouping};
/// let mut opts = NumberFormatOptions::default();
/// opts.use_grouping = UseGrouping::Never;
/// let parts = format_to_parts("en", 1234.5, &opts);
/// assert_eq!(parts[0].kind, NumberPartType::Integer);
/// assert_eq!(parts.iter().map(|p| p.value.as_str()).collect::<String>(), "1234.5");
/// ```
#[must_use]
pub fn format_to_parts(lang: &str, value: f64, opts: &NumberFormatOptions) -> Vec<NumberPart> {
    let r = resolve(lang, opts.numbering_system);
    let f = format_number(lang, value, &r, opts);
    outer_wrap(lang, opts, plural_of(lang, value), f.parts).0
}

/// Format `value` in `lang` per ECMA-402-style `opts` (`Intl.NumberFormat`).
///
/// ```
/// use intl::number::{format, NumberFormatOptions, SignDisplay};
/// let mut opts = NumberFormatOptions::default();
/// opts.sign_display = SignDisplay::Always;
/// assert_eq!(format("en", 5.0, &opts), "+5");
/// assert_eq!(format("en", 1234.5, &Default::default()), "1,234.5");
/// ```
#[must_use]
pub fn format(lang: &str, value: f64, opts: &NumberFormatOptions) -> String {
    join_parts(&format_to_parts(lang, value, opts))
}

/// Which end of a range a [`NumberRangePart`] came from (the ECMA-402
/// `formatRangeToParts` `source` field).
#[cfg(feature = "number-range")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberRangeSource {
    /// From the formatted start value.
    StartRange,
    /// From the formatted end value.
    EndRange,
    /// Glue that belongs to neither end (the separator, or everything in a
    /// collapsed range).
    Shared,
}

#[cfg(feature = "number-range")]
impl NumberRangeSource {
    /// The ECMA-402 `source` string for this origin.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            NumberRangeSource::StartRange => "startRange",
            NumberRangeSource::EndRange => "endRange",
            NumberRangeSource::Shared => "shared",
        }
    }
}

/// One tagged segment of a formatted range (see [`format_range_to_parts`]).
#[cfg(feature = "number-range")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberRangePart {
    /// What this segment represents.
    pub kind: NumberPartType,
    /// Which end it came from.
    pub source: NumberRangeSource,
    /// The literal text of this segment.
    pub value: String,
}

/// The locale's CLDR `miscPatterns` `(approximately, range)`, through the
/// fallback chain (`en` last).
#[cfg(feature = "number-range")]
fn misc_patterns(lang: &str) -> (&'static str, &'static str) {
    let norm = normalize(lang);
    let (base, _) = split_nu(&norm);
    let mut end = base.len();
    loop {
        if let Some(p) = crate::cldr::misc_patterns(&base[..end]) {
            return p;
        }
        match base[..end].rfind('-') {
            Some(i) => end = i,
            None => return crate::cldr::misc_patterns("en").expect("root misc patterns"),
        }
    }
}

/// Split the literal text around a `miscPatterns` placeholder into an
/// `ApproximatelySign` part and the whitespace that separates it from the
/// number. ICU tags only the sign itself, so `ja`'s `"約 {0}"` yields
/// `approximatelySign("約") literal(" ")`.
#[cfg(feature = "number-range")]
fn approx_literal(text: &str, before: bool, out: &mut Vec<NumberRangePart>) {
    let (sign, space) = if before {
        let sign = text.trim_end_matches(char::is_whitespace);
        (sign, &text[sign.len()..])
    } else {
        let sign = text.trim_start_matches(char::is_whitespace);
        (sign, &text[..text.len() - sign.len()])
    };
    let mut push = |kind, s: &str| {
        if !s.is_empty() {
            out.push(NumberRangePart {
                kind,
                source: NumberRangeSource::Shared,
                value: String::from(s),
            });
        }
    };
    if before {
        push(NumberPartType::ApproximatelySign, sign);
        push(NumberPartType::Literal, space);
    } else {
        push(NumberPartType::Literal, space);
        push(NumberPartType::ApproximatelySign, sign);
    }
}

/// The `range` pattern split at its two placeholders: `(prefix, infix, suffix)`
/// and whether `{1}` comes first (so the *end* is written on the left).
#[cfg(feature = "number-range")]
fn split_range_pattern(pattern: &str) -> (&str, &str, &str, bool) {
    let (Some(a), Some(b)) = (pattern.find("{0}"), pattern.find("{1}")) else {
        return ("", pattern, "", false);
    };
    let (first, second) = if a < b { (a, b) } else { (b, a) };
    (
        &pattern[..first],
        &pattern[first + 3..second],
        &pattern[second + 3..],
        b < a,
    )
}

/// The plural category a range's shared unit wording agrees with: CLDR's
/// `pluralRanges` combination of the two ends' categories (`en` `one` + `other`
/// is `other`, so 1–2 kilometres reads "1–2 kilometers"). CLDR keys the table by
/// language alone.
#[cfg(feature = "number-range")]
fn range_plural(lang: &str, start: f64, end: f64) -> usize {
    let norm = normalize(lang);
    let (base, _) = split_nu(&norm);
    let language = base.split('-').next().unwrap_or(base);
    crate::cldr::plural_range(language, plural_of(lang, start), plural_of(lang, end))
}

/// Format the range `start`–`end` in `lang` per `opts`, returning the tagged
/// parts (`Intl.NumberFormat.prototype.formatRangeToParts`).
///
/// Both ends are formatted and spliced into the CLDR `miscPatterns` `range` form
/// (`en` `"{0}–{1}"`, `zh` an ASCII hyphen). Per ECMA-402
/// `PartitionNumberRangePattern` step 5, when the two ends format *identically*
/// the result is not a range but the `approximately` form (`en` `"~{0}"`,
/// `de`/`fr` `"≈{0}"`), with every part marked `Shared`.
///
/// ECMA-402 leaves what `CollapseNumberRange` removes to the implementation.
/// This follows ICU's `NumberRangeFormatter` at its default `AUTO` collapse
/// level, which the `Intl` API always uses:
///
/// * The **outer** modifier — a unit phrase, a currency display name — is the
///   same for both ends by construction, so it is always factored out, and
///   re-worded for the range's own plural category: `"1–2 kilometers"`.
/// * The **middle** modifier — the currency symbol, percent sign and sign — is
///   factored out when both ends render it alike *and* it is longer than one
///   code point. That heuristic is why `"+$2.90–3.10"` shares its `+$` but
///   `"$3.00 – $5.00"` repeats its `$`.
/// * The **inner** modifier — a compact suffix, a scientific exponent — is never
///   factored out at this level (`"1.2K – 5K"`).
///
/// A space is then added on each side of the separator unless every modifier was
/// factored out, matching ICU's spacing heuristic.
///
/// Unlike ECMA-402 this cannot throw on a NaN endpoint; a NaN simply formats as
/// the locale's `nan` string and flows through the same rules.
///
/// ```
/// use intl::number::{format_range_to_parts, NumberPartType, NumberRangeSource};
/// let parts = format_range_to_parts("en", 3.0, 5.0, &Default::default());
/// assert_eq!(parts[0].source, NumberRangeSource::StartRange);
/// assert_eq!(parts[1].kind, NumberPartType::Literal); // the en dash
/// assert_eq!(parts[2].source, NumberRangeSource::EndRange);
/// ```
#[cfg(feature = "number-range")]
#[must_use]
pub fn format_range_to_parts(
    lang: &str,
    start: f64,
    end: f64,
    opts: &NumberFormatOptions,
) -> Vec<NumberRangePart> {
    let r = resolve(lang, opts.numbering_system);
    let sf = format_number(lang, start, &r, opts);
    let ef = format_number(lang, end, &r, opts);
    let (approx, range) = misc_patterns(lang);
    let tag = |parts: &[NumberPart], source| -> Vec<NumberRangePart> {
        parts
            .iter()
            .map(|p| NumberRangePart {
                kind: p.kind,
                source,
                value: p.value.clone(),
            })
            .collect()
    };
    let literal = |text: &str, out: &mut Vec<NumberRangePart>| {
        if !text.is_empty() {
            out.push(NumberRangePart {
                kind: NumberPartType::Literal,
                source: NumberRangeSource::Shared,
                value: String::from(text),
            });
        }
    };

    // The two ends are identical: the `approximately` sign replaces the range,
    // inside the outer modifier (`"~3 km"`, not `"~3" + " km"`), and everything
    // is worded for the single value.
    if join_parts(&sf.parts) == join_parts(&ef.parts) {
        let mut out = Vec::new();
        let (pre, post) = approx.split_once("{0}").unwrap_or(("", approx));
        approx_literal(pre, true, &mut out);
        out.extend(tag(&sf.parts, NumberRangeSource::Shared));
        approx_literal(post, false, &mut out);
        return wrap_range(lang, opts, plural_of(lang, start), out);
    }

    // ICU `NumberRangeFormatterImpl::formatRange` at collapse level AUTO.
    let (pre, infix, post, reversed) = split_range_pattern(range);
    let (first, second) = if reversed { (&ef, &sf) } else { (&sf, &ef) };
    let (first_src, second_src) = if reversed {
        (NumberRangeSource::EndRange, NumberRangeSource::StartRange)
    } else {
        (NumberRangeSource::StartRange, NumberRangeSource::EndRange)
    };
    let collapse_middle = sf.middle_mod() == ef.middle_mod() && sf.middle_len() > 1;

    // Spacing heuristic: pad the separator unless every modifier collapsed.
    // ICU tests only the *first* end's modifiers, which is why an `exceptZero`
    // range from 0 (no sign at the low end) stays tight: "0–+5".
    let mut sep = String::from(infix);
    if sf.inner_len() > 0 || (!collapse_middle && sf.middle_len() > 0) {
        if !sep.starts_with(char::is_whitespace) {
            sep.insert(0, ' ');
        }
        if !sep.ends_with(char::is_whitespace) {
            sep.push(' ');
        }
    }

    let mut out = Vec::new();
    literal(pre, &mut out);
    if collapse_middle {
        // The two modifiers are equal by now, so either end's text will do;
        // ICU takes the start's (`resolveModifierPlurals` returns `first`).
        out.extend(tag(sf.middle_mod().0, NumberRangeSource::Shared));
        out.extend(tag(first.middle_body(), first_src));
        literal(&sep, &mut out);
        out.extend(tag(second.middle_body(), second_src));
        out.extend(tag(sf.middle_mod().1, NumberRangeSource::Shared));
    } else {
        out.extend(tag(&first.parts, first_src));
        literal(&sep, &mut out);
        out.extend(tag(&second.parts, second_src));
    }
    literal(post, &mut out);
    wrap_range(lang, opts, range_plural(lang, start, end), out)
}

/// Wrap a formatted range in the outer modifier both ends share — the unit
/// phrase or currency display name — tagged `Shared`.
#[cfg(feature = "number-range")]
fn wrap_range(
    lang: &str,
    opts: &NumberFormatOptions,
    plural: usize,
    body: Vec<NumberRangePart>,
) -> Vec<NumberRangePart> {
    // Applying the wrapper to an empty number yields the modifier alone.
    let (affixes, (head, _)) = outer_wrap(lang, opts, plural, Vec::new());
    if affixes.is_empty() {
        return body;
    }
    let shared = |parts: &[NumberPart]| -> Vec<NumberRangePart> {
        parts
            .iter()
            .map(|p| NumberRangePart {
                kind: p.kind,
                source: NumberRangeSource::Shared,
                value: p.value.clone(),
            })
            .collect()
    };
    let mut out = shared(&affixes[..head]);
    out.extend(body);
    out.extend(shared(&affixes[head..]));
    out
}

/// Format the range `start`–`end` in `lang` per `opts`
/// (`Intl.NumberFormat.prototype.formatRange`).
///
/// ```
/// use intl::number::{format_range, NumberFormatOptions, NumberStyle};
/// assert_eq!(format_range("en", 3.0, 5.0, &Default::default()), "3\u{2013}5");
/// assert_eq!(format_range("zh", 3.0, 5.0, &Default::default()), "3-5"); // zh uses a hyphen
/// // Equal-formatting ends collapse to the `approximately` form.
/// assert_eq!(format_range("en", 3.0, 3.0, &Default::default()), "~3");
/// assert_eq!(format_range("de", 3.0, 3.0, &Default::default()), "\u{2248}3");
/// ```
#[cfg(feature = "number-range")]
#[must_use]
pub fn format_range(lang: &str, start: f64, end: f64, opts: &NumberFormatOptions) -> String {
    let mut out = String::new();
    for p in format_range_to_parts(lang, start, end, opts) {
        out.push_str(&p.value);
    }
    out
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
            nan: "NaN",
            infinity: "∞",
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
    fn split_nu_finds_the_keyword() {
        // The language part ends at the first singleton; inside `u`, two-letter
        // subtags are keys and the one after `nu` is its value (BCP-47 §2.2.6).
        assert_eq!(split_nu("ar"), ("ar", None));
        assert_eq!(split_nu("ar-eg"), ("ar-eg", None));
        assert_eq!(split_nu("ar-u-nu-arab"), ("ar", Some("arab")));
        assert_eq!(
            split_nu("zh-hant-hk-u-nu-hanidec"),
            ("zh-hant-hk", Some("hanidec"))
        );
        assert_eq!(split_nu("en-u-ca-islamic-nu-arab"), ("en", Some("arab")));
        assert_eq!(split_nu("en-u-nu-arab-ca-islamic"), ("en", Some("arab")));
        // Attributes (3-8 chars) before the first key are not keys.
        assert_eq!(split_nu("en-u-attr-nu-thai"), ("en", Some("thai")));
        // `nu` outside the `u` extension, and other singletons, are ignored.
        assert_eq!(split_nu("en-t-nu-arab"), ("en", None));
        assert_eq!(split_nu("en-x-nu"), ("en", None));
        assert_eq!(split_nu("en-u-ca-islamic"), ("en", None));
        // A dangling key has no value.
        assert_eq!(split_nu("en-u-nu"), ("en", None));
        // Degenerate input must not panic or slice mid-boundary.
        assert_eq!(split_nu(""), ("", None));
        assert_eq!(split_nu("u-nu-arab"), ("", Some("arab")));
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
    fn options_percent_parts() {
        let pct = NumberFormatOptions {
            style: NumberStyle::Percent,
            ..opt()
        };
        let parts = format_to_parts("en", 0.5, &pct);
        assert_eq!(parts.last().unwrap().kind, NumberPartType::PercentSign);
        assert_eq!(format("en", 0.5, &pct), "50%");
    }

    #[cfg(feature = "currency")]
    #[test]
    fn options_currency_parts() {
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
        // currencyDisplay: code goes through the ¤ pattern, like the symbol,
        // with UTS #35 currency spacing between the code and the digits.
        let code = NumberFormatOptions {
            currency_display: CurrencyDisplay::Code,
            ..cur
        };
        assert_eq!(format("en", 5.0, &code), "USD\u{a0}5.00");
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
        // The mantissa takes the style's `maximumFractionDigits` default (3 for
        // decimal), not a notation-specific width.
        assert_eq!(format("en", 12345.0, &sci), "1.235E4");
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
