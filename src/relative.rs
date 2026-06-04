//! Locale-aware relative time formatting (CLDR / UTS #35): "in 3 days",
//! "2 hours ago", "yesterday". Requires the `alloc` feature.
//!
//! The count-specific wording is chosen with the CLDR plural rules
//! ([`crate::plural`]) and the number is rendered with [`crate::number`].
//!
//! ```
//! use intl::relative::{format_relative, RelativeUnit};
//! assert_eq!(format_relative("en", -1, RelativeUnit::Day), "yesterday");
//! assert_eq!(format_relative("en", 3, RelativeUnit::Day), "in 3 days");
//! assert_eq!(format_relative("en", -2, RelativeUnit::Hour), "2 hours ago");
//! ```

use crate::number::format_decimal;
use crate::plural::{plural_category, PluralCategory, PluralOperands};
use alloc::string::String;

/// A relative time unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelativeUnit {
    /// Years.
    Year,
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

/// CLDR relative-time strings for one unit. `past`/`future` are indexed by the
/// [`PluralCategory`] discriminant.
#[derive(Debug, Clone, Copy)]
pub struct RelUnit {
    /// Literal for offset −1 (e.g. "yesterday"), if any.
    pub prev: Option<&'static str>,
    /// Literal for offset 0 (e.g. "today"), if any.
    pub cur: Option<&'static str>,
    /// Literal for offset +1 (e.g. "tomorrow"), if any.
    pub next: Option<&'static str>,
    /// Past patterns ("{0} days ago") by plural category.
    pub past: [Option<&'static str>; 6],
    /// Future patterns ("in {0} days") by plural category.
    pub future: [Option<&'static str>; 6],
}

/// CLDR relative-time strings for all units of one locale.
#[derive(Debug, Clone, Copy)]
pub struct RelativeSpec {
    /// One [`RelUnit`] per [`RelativeUnit`], in declaration order.
    pub units: [RelUnit; 7],
}

fn spec(lang: &str) -> RelativeSpec {
    use crate::unicode::generated::relative::relative_spec;
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
        if let Some(s) = relative_spec(&norm[..end]) {
            return s;
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return relative_spec("en").expect("root relative spec present"),
        }
    }
}

/// Format `value` of `unit` relative to now: negative is past ("2 days ago"),
/// positive is future ("in 2 days"), with the literal forms used for −1/0/+1
/// when the locale defines them ("yesterday"/"today"/"tomorrow").
#[must_use]
pub fn format_relative(lang: &str, value: i64, unit: RelativeUnit) -> String {
    let ru = spec(lang).units[unit as usize];

    if value == -1 {
        if let Some(s) = ru.prev {
            return String::from(s);
        }
    } else if value == 0 {
        if let Some(s) = ru.cur {
            return String::from(s);
        }
    } else if value == 1 {
        if let Some(s) = ru.next {
            return String::from(s);
        }
    }

    let table = if value < 0 { &ru.past } else { &ru.future };
    let magnitude = value.unsigned_abs();
    let cat = plural_category(lang, &PluralOperands::from_int(magnitude as i64));
    let pattern = table[cat as usize]
        .or(table[PluralCategory::Other as usize])
        .unwrap_or("{0}");
    let number = format_decimal(lang, magnitude as f64);
    pattern.replace("{0}", &number)
}
