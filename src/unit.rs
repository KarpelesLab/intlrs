//! Locale-aware measurement-unit formatting (CLDR / UTS #35): "5 kilometers",
//! "3 hr", "2,5 Stunden". Requires the `alloc` feature.
//!
//! The unit wording is chosen with the CLDR plural rules and the number is
//! rendered with [`crate::number`].
//!
//! ```
//! use intl::unit::{format_unit, Unit, UnitWidth};
//! assert_eq!(format_unit("en", 5.0, Unit::Kilometer, UnitWidth::Long), "5 kilometers");
//! assert_eq!(format_unit("en", 1.0, Unit::Hour, UnitWidth::Long), "1 hour");
//! assert_eq!(format_unit("en", 3.0, Unit::Hour, UnitWidth::Short), "3 hr");
//! assert_eq!(format_unit("de", 2.0, Unit::Hour, UnitWidth::Long), "2 Stunden");
//! ```

use crate::number::format_decimal;
use crate::plural::{plural_category, PluralOperands};
use alloc::string::String;

/// A measurement unit. The discriminant order matches the embedded table.
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
}

/// The display width of a unit ("kilometers" vs "km").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitWidth {
    /// Full words ("5 kilometers").
    Long,
    /// Abbreviated ("5 km").
    Short,
}

fn operands(v: f64) -> PluralOperands {
    // `f64::fract` is std-only; `% 1.0` is a core operator.
    if v % 1.0 == 0.0 && v.abs() < 1e15 {
        PluralOperands::from_int(v as i64)
    } else {
        // A plain (non-localized) decimal string for operand extraction.
        PluralOperands::parse(&alloc::format!("{v}")).unwrap_or(PluralOperands::from_int(v as i64))
    }
}

/// Format `value` with `unit` in `lang`, e.g. `"5 kilometers"`. The unit wording
/// agrees with the plural category of `value`, and the number is localized.
#[must_use]
pub fn format_unit(lang: &str, value: f64, unit: Unit, width: UnitWidth) -> String {
    let w = width as usize;
    let u = unit as usize;
    let cat = plural_category(lang, &operands(value)) as usize;

    // Locale fallback chain (full tag, shorter prefixes, then English).
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
        if let Some(p) = crate::cldr::unit_pattern(&norm[..end], w, u, cat) {
            pattern = p;
            break;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => {
                if let Some(p) = crate::cldr::unit_pattern("en", w, u, cat) {
                    pattern = p;
                }
                break;
            }
        }
    }

    let number = format_decimal(lang, value);
    pattern.replace("{0}", &number)
}
