//! Locale-aware measurement-unit formatting (CLDR / UTS #35): "5 kilometers",
//! "3 hr", "2,5 Stunden", "5 meters per second". Requires the `alloc` feature.
//!
//! The unit wording is chosen with the CLDR plural rules and the number is
//! rendered with [`crate::number`]. All 45 ECMA-402 sanctioned unit identifiers
//! are covered, plus arbitrary `<unit>-per-<unit>` compounds.
//!
//! ```
//! use intl::unit::{format_unit, format_compound_unit, Unit, UnitWidth};
//! assert_eq!(format_unit("en", 5.0, Unit::Kilometer, UnitWidth::Long), "5 kilometers");
//! assert_eq!(format_unit("en", 1.0, Unit::Hour, UnitWidth::Long), "1 hour");
//! assert_eq!(format_unit("en", 3.0, Unit::Hour, UnitWidth::Short), "3 hr");
//! assert_eq!(format_unit("de", 2.0, Unit::Hour, UnitWidth::Long), "2 Stunden");
//! assert_eq!(
//!     format_compound_unit("en", 5.0, Unit::Meter, Unit::Second, UnitWidth::Long),
//!     "5 meters per second"
//! );
//! ```

use crate::cldr::generated::units as data;
use crate::number::format_decimal;
use crate::plural::{PluralOperands, plural_category};
use alloc::string::{String, ToString};

/// A measurement unit. The discriminant order matches the generated table.
///
/// The variants are the 45 ECMA-402 sanctioned unit identifiers plus
/// [`Unit::KilometerPerHour`] and [`Unit::MilePerHour`], which CLDR ships
/// pre-composed (`"5 mph"`, not the derived `"5 mi/h"`). Other ratios are formed
/// with [`format_compound_unit`] or an `"<unit>-per-<unit>"` string passed to
/// [`format_unit_id`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Unit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
    Inch,
    Foot,
    Mile,
    Gram,
    Kilogram,
    Ounce,
    Pound,
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Celsius,
    Fahrenheit,
    KilometerPerHour,
    MilePerHour,
    Liter,
    Milliliter,
    Acre,
    Bit,
    Degree,
    FluidOunce,
    Gallon,
    Gigabit,
    Hectare,
    Kilobit,
    Megabit,
    Microsecond,
    MileScandinavian,
    Millisecond,
    Nanosecond,
    Percent,
    Petabyte,
    Stone,
    Terabit,
    Terabyte,
    Yard,
}

// The enum and the generated table are written independently; keep them in step.
const _: () = assert!(data::UNIT_COUNT as usize == Unit::Yard as usize + 1);

impl Unit {
    /// The ECMA-402 unit identifier for this unit, e.g. `"fluid-ounce"`.
    #[must_use]
    pub const fn ecma_id(self) -> &'static str {
        match self {
            Unit::Second => "second",
            Unit::Minute => "minute",
            Unit::Hour => "hour",
            Unit::Day => "day",
            Unit::Week => "week",
            Unit::Month => "month",
            Unit::Year => "year",
            Unit::Millimeter => "millimeter",
            Unit::Centimeter => "centimeter",
            Unit::Meter => "meter",
            Unit::Kilometer => "kilometer",
            Unit::Inch => "inch",
            Unit::Foot => "foot",
            Unit::Mile => "mile",
            Unit::Gram => "gram",
            Unit::Kilogram => "kilogram",
            Unit::Ounce => "ounce",
            Unit::Pound => "pound",
            Unit::Byte => "byte",
            Unit::Kilobyte => "kilobyte",
            Unit::Megabyte => "megabyte",
            Unit::Gigabyte => "gigabyte",
            Unit::Celsius => "celsius",
            Unit::Fahrenheit => "fahrenheit",
            Unit::KilometerPerHour => "kilometer-per-hour",
            Unit::MilePerHour => "mile-per-hour",
            Unit::Liter => "liter",
            Unit::Milliliter => "milliliter",
            Unit::Acre => "acre",
            Unit::Bit => "bit",
            Unit::Degree => "degree",
            Unit::FluidOunce => "fluid-ounce",
            Unit::Gallon => "gallon",
            Unit::Gigabit => "gigabit",
            Unit::Hectare => "hectare",
            Unit::Kilobit => "kilobit",
            Unit::Megabit => "megabit",
            Unit::Microsecond => "microsecond",
            Unit::MileScandinavian => "mile-scandinavian",
            Unit::Millisecond => "millisecond",
            Unit::Nanosecond => "nanosecond",
            Unit::Percent => "percent",
            Unit::Petabyte => "petabyte",
            Unit::Stone => "stone",
            Unit::Terabit => "terabit",
            Unit::Terabyte => "terabyte",
            Unit::Yard => "yard",
        }
    }

    /// Resolve an ECMA-402 unit identifier that names a single table entry — the
    /// 45 sanctioned simple units and the two pre-composed `speed-…` compounds.
    /// General `"<unit>-per-<unit>"` identifiers are handled by
    /// [`format_unit_id`]; this returns `None` for them.
    #[must_use]
    pub fn from_ecma_id(id: &str) -> Option<Self> {
        Some(match id {
            "second" => Unit::Second,
            "minute" => Unit::Minute,
            "hour" => Unit::Hour,
            "day" => Unit::Day,
            "week" => Unit::Week,
            "month" => Unit::Month,
            "year" => Unit::Year,
            "millimeter" => Unit::Millimeter,
            "centimeter" => Unit::Centimeter,
            "meter" => Unit::Meter,
            "kilometer" => Unit::Kilometer,
            "inch" => Unit::Inch,
            "foot" => Unit::Foot,
            "mile" => Unit::Mile,
            "gram" => Unit::Gram,
            "kilogram" => Unit::Kilogram,
            "ounce" => Unit::Ounce,
            "pound" => Unit::Pound,
            "byte" => Unit::Byte,
            "kilobyte" => Unit::Kilobyte,
            "megabyte" => Unit::Megabyte,
            "gigabyte" => Unit::Gigabyte,
            "celsius" => Unit::Celsius,
            "fahrenheit" => Unit::Fahrenheit,
            "kilometer-per-hour" => Unit::KilometerPerHour,
            "mile-per-hour" => Unit::MilePerHour,
            "liter" => Unit::Liter,
            "milliliter" => Unit::Milliliter,
            "acre" => Unit::Acre,
            "bit" => Unit::Bit,
            "degree" => Unit::Degree,
            "fluid-ounce" => Unit::FluidOunce,
            "gallon" => Unit::Gallon,
            "gigabit" => Unit::Gigabit,
            "hectare" => Unit::Hectare,
            "kilobit" => Unit::Kilobit,
            "megabit" => Unit::Megabit,
            "microsecond" => Unit::Microsecond,
            "mile-scandinavian" => Unit::MileScandinavian,
            "millisecond" => Unit::Millisecond,
            "nanosecond" => Unit::Nanosecond,
            "percent" => Unit::Percent,
            "petabyte" => Unit::Petabyte,
            "stone" => Unit::Stone,
            "terabit" => Unit::Terabit,
            "terabyte" => Unit::Terabyte,
            "yard" => Unit::Yard,
            _ => return None,
        })
    }
}

/// The display width of a unit ("kilometers" vs "km" vs "km" with no space).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitWidth {
    /// Full words ("5 kilometers").
    Long,
    /// Abbreviated ("5 km").
    Short,
    /// Tightest abbreviation, typically without the separating space ("5km").
    ///
    /// The narrow patterns are a third of the unit table, so they are gated on
    /// the `units-narrow` cargo feature (on by default). Without it this width
    /// falls back to [`UnitWidth::Short`], which is also UTS #35's own narrow →
    /// short width fallback.
    ///
    /// ```
    /// # #[cfg(feature = "units-narrow")] {
    /// use intl::unit::{format_unit, Unit, UnitWidth};
    /// assert_eq!(format_unit("en", 5.0, Unit::Kilometer, UnitWidth::Narrow), "5km");
    /// # }
    /// ```
    Narrow,
}

/// A unit as CLDR resolves it: a single table entry, or a `<numerator>-per-
/// <denominator>` ratio assembled at runtime (UTS #35 "compound units").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Measure {
    Simple(Unit),
    Per(Unit, Unit),
}

impl Measure {
    /// Parse an ECMA-402 unit identifier. ECMA-402 allows exactly one `-per-`,
    /// with a sanctioned simple unit on each side; anything else is `None`.
    pub(crate) fn parse(id: &str) -> Option<Self> {
        if let Some(u) = Unit::from_ecma_id(id) {
            return Some(Measure::Simple(u));
        }
        let (num, den) = id.split_once("-per-")?;
        Some(Measure::Per(
            Unit::from_ecma_id(num)?,
            Unit::from_ecma_id(den)?,
        ))
    }
}

/// Slot holding a unit's `perUnitPattern` (slots 0..=5 are the plural counts).
const SLOT_PER_UNIT: u16 = 6;
/// Pseudo-unit holding the locale's `per` `compoundUnitPattern`.
const COMPOUND_UNIT: u16 = data::UNIT_COUNT;

const fn width_index(width: UnitWidth) -> usize {
    match width {
        UnitWidth::Long => 0,
        UnitWidth::Short => 1,
        #[cfg(feature = "units-narrow")]
        UnitWidth::Narrow => 2,
        // Narrow data not compiled in: UTS #35's width fallback is narrow → short.
        #[cfg(not(feature = "units-narrow"))]
        UnitWidth::Narrow => 1,
    }
}

/// Resolve `lang` against the table once, walking the CLDR fallback chain (full
/// tag, successively shorter prefixes, then `en`). A compound unit needs several
/// lookups — numerator, denominator, `per` pattern — and they must all come from
/// the same locale, so the locale is pinned here rather than per lookup.
fn resolve_locale(lang: &str) -> u16 {
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
        if let Some(i) = data::locale_index(&norm[..end]) {
            return i;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return data::EN,
        }
    }
}

#[inline]
fn slot(loc: u16, width: usize, unit: u16, which: u16) -> Option<&'static str> {
    data::pattern(loc, width, unit * 8 + which)
}

/// The pattern for `unit` in plural category `cat`, falling back to `other` —
/// the one count CLDR guarantees for every unit.
fn count_pattern(loc: u16, width: usize, unit: Unit, cat: usize) -> &'static str {
    let u = unit as u16;
    slot(loc, width, u, cat as u16)
        .or_else(|| slot(loc, width, u, 5))
        .unwrap_or("{0}")
}

/// Strip a unit pattern down to the bare unit name, for use as the `{1}`
/// argument of a `compoundUnitPattern`: `"{0} km"` and `"{0}km"` both give
/// `"km"`. Every placeholder is dropped, not just a leading/trailing one — a few
/// locales put it inside the phrase (`ja` `"摂氏 {0} 度"`), and leaving it would
/// make the number appear twice in the assembled compound. This is ICU's
/// `SimpleFormatter::getTextWithNoArguments().trim()`.
fn core_pattern(pat: &str) -> String {
    String::from(pat.replace("{0}", "").trim())
}

/// Build the full CLDR pattern for `m` — `{0}` still standing in for the number.
///
/// For a ratio, UTS #35 prefers the *denominator's* `perUnitPattern` (`"{0} per
/// hour"`, `"{0}/h"`); only 18 of the 45 sanctioned units have one, so the rest
/// go through the locale's `per` `compoundUnitPattern` with the denominator
/// named in the singular.
fn measure_pattern(loc: u16, width: usize, m: Measure, cat: usize) -> String {
    match m {
        Measure::Simple(u) => count_pattern(loc, width, u, cat).to_string(),
        Measure::Per(num, den) => {
            let numerator = count_pattern(loc, width, num, cat);
            if let Some(per) = slot(loc, width, den as u16, SLOT_PER_UNIT) {
                return per.replace("{0}", numerator);
            }
            let compound = slot(loc, width, COMPOUND_UNIT, 0).unwrap_or("{0}/{1}");
            // UTS #35 names the denominator in the singular (CLDR count `one`).
            let denominator = core_pattern(count_pattern(loc, width, den, 1));
            compound
                .replace("{0}", numerator)
                .replace("{1}", &denominator)
        }
    }
}

/// The pattern (with `{0}` for the number) for the ECMA-402 unit identifier
/// `id`, or `None` if `id` is not sanctioned. Shared with [`crate::number`]'s
/// `style: "unit"`, which needs the pattern rather than the finished string so it
/// can tag the parts.
pub(crate) fn pattern_for_id(lang: &str, id: &str, width: UnitWidth, cat: usize) -> Option<String> {
    let m = Measure::parse(id)?;
    Some(measure_pattern(
        resolve_locale(lang),
        width_index(width),
        m,
        cat,
    ))
}

/// The CLDR plural category of `value` in `lang`.
pub(crate) fn category(lang: &str, value: f64) -> usize {
    plural_category(lang, &operands(value)) as usize
}

fn operands(v: f64) -> PluralOperands {
    // `f64::fract` is std-only; `% 1.0` is a core operator.
    if v % 1.0 == 0.0 && v > -1e15 && v < 1e15 {
        PluralOperands::from_int(v as i64)
    } else {
        // A plain (non-localized) decimal string for operand extraction.
        PluralOperands::parse(&alloc::format!("{v}")).unwrap_or(PluralOperands::from_int(v as i64))
    }
}

fn format_measure(lang: &str, value: f64, m: Measure, width: UnitWidth) -> String {
    let pattern = measure_pattern(
        resolve_locale(lang),
        width_index(width),
        m,
        category(lang, value),
    );
    pattern.replace("{0}", &format_decimal(lang, value))
}

/// Format `value` with `unit` in `lang`, e.g. `"5 kilometers"`. The unit wording
/// agrees with the plural category of `value`, and the number is localized.
#[must_use]
pub fn format_unit(lang: &str, value: f64, unit: Unit, width: UnitWidth) -> String {
    format_measure(lang, value, Measure::Simple(unit), width)
}

/// Format `value` as a `numerator`-per-`denominator` ratio, e.g.
/// `format_compound_unit("en", 5.0, Unit::Gallon, Unit::Mile, UnitWidth::Long)`
/// → `"5 gallons per mile"`.
///
/// ```
/// use intl::unit::{format_compound_unit, Unit, UnitWidth};
/// // `hour` carries a perUnitPattern ("{0}/h"); `mile` does not, so the
/// // locale's compoundUnitPattern ("{0}/{1}") assembles the short form.
/// assert_eq!(
///     format_compound_unit("en", 5.0, Unit::Liter, Unit::Hour, UnitWidth::Short),
///     "5 L/h"
/// );
/// assert_eq!(
///     format_compound_unit("en", 5.0, Unit::Gallon, Unit::Mile, UnitWidth::Short),
///     "5 gal/mi"
/// );
/// ```
#[must_use]
pub fn format_compound_unit(
    lang: &str,
    value: f64,
    numerator: Unit,
    denominator: Unit,
    width: UnitWidth,
) -> String {
    format_measure(lang, value, Measure::Per(numerator, denominator), width)
}

/// Format `value` with the ECMA-402 unit identifier `id` — a sanctioned simple
/// unit or a `"<unit>-per-<unit>"` compound. `None` if `id` is not sanctioned.
///
/// ```
/// use intl::unit::{format_unit_id, UnitWidth};
/// assert_eq!(
///     format_unit_id("en", 5.0, "meter-per-second", UnitWidth::Long).as_deref(),
///     Some("5 meters per second")
/// );
/// assert_eq!(format_unit_id("en", 5.0, "furlong", UnitWidth::Long), None);
/// ```
#[must_use]
pub fn format_unit_id(lang: &str, value: f64, id: &str, width: UnitWidth) -> Option<String> {
    Measure::parse(id).map(|m| format_measure(lang, value, m, width))
}

/// Format a duration given as a whole number of seconds, e.g.
/// `format_duration("en", 3661, UnitWidth::Long)` → `"1 hour 1 minute 1 second"`.
/// The largest non-zero units (days, hours, minutes, seconds) are each rendered
/// with [`format_unit`] (plural-correct, localized) and joined with a space —
/// CLDR's narrow unit-list convention. A zero duration renders as `0` seconds.
#[must_use]
pub fn format_duration(lang: &str, total_seconds: i64, width: UnitWidth) -> String {
    let neg = total_seconds < 0;
    let mut rem = total_seconds.unsigned_abs();
    let parts = [
        (86_400u64, Unit::Day),
        (3_600, Unit::Hour),
        (60, Unit::Minute),
        (1, Unit::Second),
    ];
    let mut out = String::new();
    for (size, unit) in parts {
        let v = rem / size;
        rem %= size;
        // Skip leading zero components, but always keep seconds if nothing else.
        if v == 0 && !(unit == Unit::Second && out.is_empty()) {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&format_unit(lang, v as f64, unit, width));
    }
    if neg {
        let mut signed = String::from("-");
        signed.push_str(&out);
        signed
    } else {
        out
    }
}
