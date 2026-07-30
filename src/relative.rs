//! Locale-aware relative time formatting (CLDR / UTS #35 §8.2 `<fields>`,
//! ECMA-402 `Intl.RelativeTimeFormat`): "in 3 days", "2 hours ago",
//! "yesterday". Requires the `alloc` feature.
//!
//! Two options shape the output, both in [`RelativeTimeFormatOptions`]:
//!
//! * [`RelativeNumeric`] — `Always` (the ECMA-402 default) always spells the
//!   number, so `(1, Day)` is "in 1 day"; `Auto` prefers the locale's literal
//!   where it has one, giving "tomorrow". CLDR carries literals for offsets
//!   −2..=+3, so `Auto` also reaches "vorgestern" and "après-demain".
//! * [`RelativeWidth`] — `Long` / `Short` / `Narrow`, selecting CLDR's `day` /
//!   `day-short` / `day-narrow` blocks: "in 3 days" / "in 3 days" / "in 3d".
//!
//! The count-specific wording is chosen with the CLDR plural rules
//! ([`crate::plural`]) and the number is rendered with [`crate::number`], so it
//! is grouped and localized like any other decimal.
//!
//! `value` is an `f64` because ECMA-402's is a Number, and the sign of zero is
//! load-bearing: `-0.0` is in the past ("0 days ago") and `0.0` in the future
//! ("in 0 days").
//!
//! ```
//! use intl::relative::{RelativeNumeric, RelativeTimeFormatOptions, RelativeUnit, format_relative};
//! let always = RelativeTimeFormatOptions::default();
//! assert_eq!(format_relative("en", -1.0, RelativeUnit::Day, &always), "1 day ago");
//! assert_eq!(format_relative("en", 3.0, RelativeUnit::Day, &always), "in 3 days");
//! assert_eq!(format_relative("pl", 1.0, RelativeUnit::Day, &always), "za 1 dzień");
//!
//! let auto: RelativeTimeFormatOptions = RelativeNumeric::Auto.into();
//! assert_eq!(format_relative("en", -1.0, RelativeUnit::Day, &auto), "yesterday");
//! assert_eq!(format_relative("de", -2.0, RelativeUnit::Day, &auto), "vorgestern");
//! ```

use crate::number::{NumberFormatOptions, NumberPartType, format_decimal, format_to_parts};
use crate::plural::{PluralCategory, PluralOperands, plural_category};
use alloc::string::String;
use alloc::vec::Vec;

pub use crate::cldr::RelUnit;

/// A relative time unit — ECMA-402's eight `SingularRelativeTimeUnit` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeUnit {
    /// Years.
    Year,
    /// Quarters (three months).
    Quarter,
    /// Months.
    Month,
    /// Weeks.
    Week,
    /// Days.
    Day,
    /// Hours.
    Hour,
    /// Minutes.
    Minute,
    /// Seconds.
    Second,
}

impl RelativeUnit {
    /// The ECMA-402 singular unit identifier (`"day"`, `"quarter"`), which is
    /// also the CLDR `<fields>` key.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            RelativeUnit::Year => "year",
            RelativeUnit::Quarter => "quarter",
            RelativeUnit::Month => "month",
            RelativeUnit::Week => "week",
            RelativeUnit::Day => "day",
            RelativeUnit::Hour => "hour",
            RelativeUnit::Minute => "minute",
            RelativeUnit::Second => "second",
        }
    }

    /// The unit for an ECMA-402 identifier. Both the singular and the plural
    /// spelling are accepted, as in `SingularRelativeTimeUnit`, which is what
    /// `Intl.RelativeTimeFormat.prototype.format` takes.
    #[must_use]
    pub fn from_ecma_id(id: &str) -> Option<RelativeUnit> {
        Some(match id {
            "year" | "years" => RelativeUnit::Year,
            "quarter" | "quarters" => RelativeUnit::Quarter,
            "month" | "months" => RelativeUnit::Month,
            "week" | "weeks" => RelativeUnit::Week,
            "day" | "days" => RelativeUnit::Day,
            "hour" | "hours" => RelativeUnit::Hour,
            "minute" | "minutes" => RelativeUnit::Minute,
            "second" | "seconds" => RelativeUnit::Second,
            _ => return None,
        })
    }
}

/// Whether the count is always spelled out — ECMA-402
/// `Intl.RelativeTimeFormat`'s `numeric` option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeNumeric {
    /// Always use the numeric pattern: `(−1, Day)` is "1 day ago". The
    /// ECMA-402 default.
    #[default]
    Always,
    /// Prefer the locale's literal where CLDR defines one for the offset:
    /// `(−1, Day)` is "yesterday", `(2, Day)` in German "übermorgen".
    Auto,
}

/// How wide the wording is — ECMA-402 `Intl.RelativeTimeFormat`'s `style`
/// option, selecting CLDR's `<unit>` / `<unit>-short` / `<unit>-narrow` blocks.
///
/// Named for the axis rather than for `style` to match `unit::UnitWidth` and
/// [`crate::list::ListWidth`], which are the same long/short/narrow choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RelativeWidth {
    /// Full wording — "in 3 days".
    #[default]
    Long,
    /// Abbreviated — `en` "in 3 mo." for months.
    Short,
    /// Narrowest — "in 3d".
    Narrow,
}

/// `Intl.RelativeTimeFormat` options. [`Default`] is ECMA-402's default
/// formatter: `numeric: "always"`, `style: "long"`.
///
/// The struct is `#[non_exhaustive]` (so new options can be added without a
/// breaking change): construct it from [`Default`] and set the fields you need,
/// e.g. `let mut o = RelativeTimeFormatOptions::default(); o.width = …;`, or
/// from either axis alone via [`From`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct RelativeTimeFormatOptions {
    /// ECMA-402 `numeric`.
    pub numeric: RelativeNumeric,
    /// ECMA-402 `style`.
    pub width: RelativeWidth,
}

impl From<RelativeNumeric> for RelativeTimeFormatOptions {
    fn from(numeric: RelativeNumeric) -> Self {
        RelativeTimeFormatOptions {
            numeric,
            ..Default::default()
        }
    }
}

impl From<RelativeWidth> for RelativeTimeFormatOptions {
    fn from(width: RelativeWidth) -> Self {
        RelativeTimeFormatOptions {
            width,
            ..Default::default()
        }
    }
}

/// One tagged segment of a formatted relative time (see
/// [`format_relative_to_parts`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelativeTimePart {
    /// What this segment represents. Pattern text is
    /// [`NumberPartType::Literal`]; everything else comes from the number and
    /// carries the number's own kind, as in ECMA-402, whose relative-time parts
    /// are the `Intl.NumberFormat` parts spliced into the pattern.
    pub kind: NumberPartType,
    /// The literal text of this segment.
    pub value: String,
    /// The unit, on the segments that came from the number — ECMA-402 puts a
    /// `unit` field on exactly those. `None` on pattern literals.
    pub unit: Option<RelativeUnit>,
}

/// The relative-time strings for `unit` at `opts.width` in `lang`, walking the
/// tag's subtags off one at a time (`en-GB` → `en`) and ending at `en`, whose
/// fields stand in for CLDR root.
fn field(lang: &str, unit: RelativeUnit, opts: &RelativeTimeFormatOptions) -> RelUnit {
    use crate::cldr::{relative_locale, relative_unit};
    let (u, w) = (unit as usize, opts.width as usize);
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
        if let Some(loc) = relative_locale(&norm[..end]) {
            return relative_unit(loc, u, w);
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => {
                let en = relative_locale("en").expect("root relative fields present");
                return relative_unit(en, u, w);
            }
        }
    }
}

/// The `numeric: "auto"` literal for `value`, if the locale has one.
///
/// ECMA-402 keys the lookup on `ToString(value)`, so only an integer can match
/// — `1.5` never has a literal — and `ToString(-0)` is `"0"`, which is why a
/// negative zero still reads "today" here but "0 days ago" under `Always`.
fn auto_literal(f: &RelUnit, value: f64) -> Option<&'static str> {
    // `f64::fract`/`trunc` are std-only; `% 1.0` is a core operator, and it is
    // NaN (so never `== 0.0`) for the non-finite values that have no literal
    // either.
    if value % 1.0 != 0.0 {
        return None;
    }
    f.literal(value as i64)
}

/// Plural operands for a magnitude, mirroring `unit::operands`: ECMA-402
/// resolves the category on the value the number formatter is about to render.
fn operands(v: f64) -> PluralOperands {
    if v % 1.0 == 0.0 && v > -1e15 && v < 1e15 {
        PluralOperands::from_int(v as i64)
    } else {
        // A plain (non-localized) decimal string for operand extraction.
        PluralOperands::parse(&alloc::format!("{v}")).unwrap_or(PluralOperands::from_int(v as i64))
    }
}

/// The pattern and unsigned magnitude for the numeric path.
///
/// Direction is the *sign bit*, not `value < 0.0`: ECMA-402 sends `-0` down the
/// past branch, which is the only way to ask for "0 days ago" rather than
/// "in 0 days".
fn numeric_pattern(lang: &str, f: &RelUnit, value: f64) -> (&'static str, f64) {
    let past = value.is_sign_negative();
    let table = if past { &f.past } else { &f.future };
    let magnitude = if past { -value } else { value };
    let cat = plural_category(lang, &operands(magnitude));
    let pattern = table[cat as usize]
        .or(table[PluralCategory::Other as usize])
        .unwrap_or("{0}");
    (pattern, magnitude)
}

/// Format `value` of `unit` relative to now: negative is past ("2 days ago"),
/// positive is future ("in 2 days").
///
/// A non-finite `value` has no ECMA-402 meaning (the spec throws); it is
/// rendered through the number formatter's own `NaN` / `∞` spellings rather
/// than panicking.
///
/// ```
/// use intl::relative::{RelativeUnit, RelativeWidth, format_relative};
/// let narrow: intl::relative::RelativeTimeFormatOptions = RelativeWidth::Narrow.into();
/// assert_eq!(format_relative("en", 3.0, RelativeUnit::Quarter, &narrow), "in 3q");
/// assert_eq!(format_relative("en", -0.0, RelativeUnit::Day, &Default::default()), "0 days ago");
/// ```
#[must_use]
pub fn format_relative(
    lang: &str,
    value: f64,
    unit: RelativeUnit,
    opts: &RelativeTimeFormatOptions,
) -> String {
    let f = field(lang, unit, opts);
    if opts.numeric == RelativeNumeric::Auto
        && let Some(s) = auto_literal(&f, value)
    {
        return String::from(s);
    }
    let (pattern, magnitude) = numeric_pattern(lang, &f, value);
    pattern.replace("{0}", &format_decimal(lang, magnitude))
}

/// Format `value` of `unit` relative to now as a list of tagged parts
/// (`Intl.RelativeTimeFormat.prototype.formatToParts`).
///
/// The pattern is split at its `{0}` placeholder and the number's own parts —
/// integer runs, grouping separators, the decimal separator — are spliced in,
/// each carrying `unit`. A literal chosen by `numeric: "auto"` is a single
/// [`NumberPartType::Literal`] part with no unit. Concatenating the values
/// reproduces [`format_relative`].
///
/// ```
/// use intl::number::NumberPartType;
/// use intl::relative::{RelativeUnit, format_relative_to_parts};
/// let parts = format_relative_to_parts("en", 3.0, RelativeUnit::Day, &Default::default());
/// let kinds: Vec<_> = parts.iter().map(|p| (p.kind, p.value.as_str())).collect();
/// assert_eq!(
///     kinds,
///     [
///         (NumberPartType::Literal, "in "),
///         (NumberPartType::Integer, "3"),
///         (NumberPartType::Literal, " days"),
///     ]
/// );
/// assert_eq!(parts[1].unit, Some(RelativeUnit::Day));
/// ```
#[must_use]
pub fn format_relative_to_parts(
    lang: &str,
    value: f64,
    unit: RelativeUnit,
    opts: &RelativeTimeFormatOptions,
) -> Vec<RelativeTimePart> {
    let literal = |s: &str| RelativeTimePart {
        kind: NumberPartType::Literal,
        value: String::from(s),
        unit: None,
    };
    let f = field(lang, unit, opts);
    if opts.numeric == RelativeNumeric::Auto
        && let Some(s) = auto_literal(&f, value)
    {
        return alloc::vec![literal(s)];
    }
    let (pattern, magnitude) = numeric_pattern(lang, &f, value);
    let mut out = Vec::new();
    // A pattern with no `{0}` (CLDR ships none, but the `"{0}"` fallback and a
    // future data revision could) is pure literal: emit it and stop.
    let (head, tail) = match pattern.split_once("{0}") {
        Some((h, t)) => (h, Some(t)),
        None => (pattern, None),
    };
    if !head.is_empty() {
        out.push(literal(head));
    }
    if let Some(tail) = tail {
        // The count is an ordinary decimal in the locale, so its parts are
        // `Intl.NumberFormat`'s — grouping separators and all.
        let number = NumberFormatOptions::default();
        out.extend(
            format_to_parts(lang, magnitude, &number)
                .into_iter()
                .map(|p| RelativeTimePart {
                    kind: p.kind,
                    value: p.value,
                    unit: Some(unit),
                }),
        );
        if !tail.is_empty() {
            out.push(literal(tail));
        }
    }
    out
}
