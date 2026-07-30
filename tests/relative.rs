//! Locale-aware relative time formatting.
#![cfg(feature = "relative")]

use intl::number::NumberPartType;
use intl::relative::{
    RelativeNumeric, RelativeNumeric::*, RelativeTimeFormatOptions, RelativeUnit, RelativeUnit::*,
    RelativeWidth, RelativeWidth::*, format_relative, format_relative_to_parts,
};

type Case = (
    &'static str,
    RelativeNumeric,
    RelativeWidth,
    RelativeUnit,
    f64,
    &'static str,
);

fn opts(numeric: RelativeNumeric, width: RelativeWidth) -> RelativeTimeFormatOptions {
    let mut o = RelativeTimeFormatOptions::default();
    o.numeric = numeric;
    o.width = width;
    o
}

/// `format_relative` with the two ECMA-402 axes spelled out, for the tables below.
fn fr(
    lang: &str,
    value: f64,
    unit: RelativeUnit,
    numeric: RelativeNumeric,
    width: RelativeWidth,
) -> String {
    format_relative(lang, value, unit, &opts(numeric, width))
}

fn check(cases: &[Case]) {
    for (lang, numeric, width, unit, value, want) in cases {
        assert_eq!(
            &fr(lang, *value, *unit, *numeric, *width),
            want,
            "{lang} {numeric:?} {width:?} {unit:?} {value}"
        );
    }
}

/// Expectations here are `new Intl.RelativeTimeFormat(loc, {numeric,
/// style}).format(v, unit)` on node 22 / ICU 77. ICU 77 carries CLDR 47 and this
/// crate vendors CLDR 48, so a few strings elsewhere in the corpus have been
/// reworded upstream since; none of the cases below is one of them.
///
/// `numeric: "always"` is the ECMA-402 default, and the case this crate could
/// not express at all before: `(1, Day)` is "in 1 day", not "tomorrow".
#[test]
fn numeric_always_and_auto() {
    #[rustfmt::skip]
    const CASES: &[Case] = &[
    ("en", Always, Long, Day, -2.0, "2 days ago"),
    ("en", Always, Long, Day, -1.0, "1 day ago"),
    ("en", Always, Long, Day, 0.0, "in 0 days"),
    ("en", Always, Long, Day, 1.0, "in 1 day"),
    ("en", Always, Long, Day, 5.0, "in 5 days"),
    ("en", Auto, Long, Day, -2.0, "2 days ago"),
    ("en", Auto, Long, Day, -1.0, "yesterday"),
    ("en", Auto, Long, Day, 0.0, "today"),
    ("en", Auto, Long, Day, 1.0, "tomorrow"),
    ("en", Auto, Long, Day, 5.0, "in 5 days"),
    ("es", Always, Long, Day, -2.0, "hace 2 días"),
    ("es", Always, Long, Day, -1.0, "hace 1 día"),
    ("es", Always, Long, Day, 0.0, "dentro de 0 días"),
    ("es", Always, Long, Day, 1.0, "dentro de 1 día"),
    ("es", Always, Long, Day, 5.0, "dentro de 5 días"),
    ("es", Auto, Long, Day, -2.0, "anteayer"),
    ("es", Auto, Long, Day, -1.0, "ayer"),
    ("es", Auto, Long, Day, 0.0, "hoy"),
    ("es", Auto, Long, Day, 1.0, "mañana"),
    ("es", Auto, Long, Day, 5.0, "dentro de 5 días"),
    ("de", Always, Long, Day, -2.0, "vor 2 Tagen"),
    ("de", Always, Long, Day, -1.0, "vor 1 Tag"),
    ("de", Always, Long, Day, 0.0, "in 0 Tagen"),
    ("de", Always, Long, Day, 1.0, "in 1 Tag"),
    ("de", Always, Long, Day, 5.0, "in 5 Tagen"),
    ("de", Auto, Long, Day, -2.0, "vorgestern"),
    ("de", Auto, Long, Day, -1.0, "gestern"),
    ("de", Auto, Long, Day, 0.0, "heute"),
    ("de", Auto, Long, Day, 1.0, "morgen"),
    ("de", Auto, Long, Day, 5.0, "in 5 Tagen"),
    ("fr", Always, Long, Day, -2.0, "il y a 2 jours"),
    ("fr", Always, Long, Day, -1.0, "il y a 1 jour"),
    ("fr", Always, Long, Day, 0.0, "dans 0 jour"),
    ("fr", Always, Long, Day, 1.0, "dans 1 jour"),
    ("fr", Always, Long, Day, 5.0, "dans 5 jours"),
    ("fr", Auto, Long, Day, -2.0, "avant-hier"),
    ("fr", Auto, Long, Day, -1.0, "hier"),
    ("fr", Auto, Long, Day, 0.0, "aujourd’hui"),
    ("fr", Auto, Long, Day, 1.0, "demain"),
    ("fr", Auto, Long, Day, 5.0, "dans 5 jours"),
    ("pl", Always, Long, Day, -2.0, "2 dni temu"),
    ("pl", Always, Long, Day, -1.0, "1 dzień temu"),
    ("pl", Always, Long, Day, 0.0, "za 0 dni"),
    ("pl", Always, Long, Day, 1.0, "za 1 dzień"),
    ("pl", Always, Long, Day, 5.0, "za 5 dni"),
    ("pl", Auto, Long, Day, -2.0, "przedwczoraj"),
    ("pl", Auto, Long, Day, -1.0, "wczoraj"),
    ("pl", Auto, Long, Day, 0.0, "dzisiaj"),
    ("pl", Auto, Long, Day, 1.0, "jutro"),
    ("pl", Auto, Long, Day, 5.0, "za 5 dni"),
    ("ja", Always, Long, Day, -2.0, "2 日前"),
    ("ja", Always, Long, Day, -1.0, "1 日前"),
    ("ja", Always, Long, Day, 0.0, "0 日後"),
    ("ja", Always, Long, Day, 1.0, "1 日後"),
    ("ja", Always, Long, Day, 5.0, "5 日後"),
    ("ja", Auto, Long, Day, -2.0, "一昨日"),
    ("ja", Auto, Long, Day, -1.0, "昨日"),
    ("ja", Auto, Long, Day, 0.0, "今日"),
    ("ja", Auto, Long, Day, 1.0, "明日"),
    ("ja", Auto, Long, Day, 5.0, "5 日後"),
    ("ar", Always, Long, Day, -2.0, "قبل يومين"),
    ("ar", Always, Long, Day, -1.0, "قبل يوم واحد"),
    ("ar", Always, Long, Day, 0.0, "خلال 0 يوم"),
    ("ar", Always, Long, Day, 1.0, "خلال يوم واحد"),
    ("ar", Always, Long, Day, 5.0, "خلال 5 أيام"),
    ("ar", Auto, Long, Day, -2.0, "أول أمس"),
    ("ar", Auto, Long, Day, -1.0, "أمس"),
    ("ar", Auto, Long, Day, 0.0, "اليوم"),
    ("ar", Auto, Long, Day, 1.0, "غدًا"),
    ("ar", Auto, Long, Day, 5.0, "خلال 5 أيام"),
    ];
    check(CASES);
}

/// `style` selects CLDR's `<unit>` / `<unit>-short` / `<unit>-narrow` blocks,
/// which are independent data — `en` day-short is word-for-word day-long while
/// day-narrow is "in 5d", `de` abbreviates only `quarter`, and `fr` day-short
/// glues its unit on with a no-break space.
#[test]
fn styles_are_independent_data() {
    #[rustfmt::skip]
    const CASES: &[Case] = &[
    ("en", Always, Long, Day, 5.0, "in 5 days"),
    ("en", Always, Short, Day, 5.0, "in 5 days"),
    ("en", Always, Narrow, Day, 5.0, "in 5d"),
    ("en", Always, Long, Month, 5.0, "in 5 months"),
    ("en", Always, Short, Month, 5.0, "in 5 mo."),
    ("en", Always, Narrow, Month, 5.0, "in 5mo"),
    ("en", Always, Long, Quarter, 5.0, "in 5 quarters"),
    ("en", Always, Short, Quarter, 5.0, "in 5 qtrs."),
    ("en", Always, Narrow, Quarter, 5.0, "in 5q"),
    ("de", Always, Long, Day, 5.0, "in 5 Tagen"),
    ("de", Always, Short, Day, 5.0, "in 5 Tagen"),
    ("de", Always, Narrow, Day, 5.0, "in 5 Tagen"),
    ("de", Always, Long, Month, 5.0, "in 5 Monaten"),
    ("de", Always, Short, Month, 5.0, "in 5 Monaten"),
    ("de", Always, Narrow, Month, 5.0, "in 5 Monaten"),
    ("de", Always, Long, Quarter, 5.0, "in 5 Quartalen"),
    ("de", Always, Short, Quarter, 5.0, "in 5 Quart."),
    ("de", Always, Narrow, Quarter, 5.0, "in 5 Q"),
    ("fr", Always, Long, Day, 5.0, "dans 5 jours"),
    ("fr", Always, Short, Day, 5.0, "dans 5\u{a0}j"),
    ("fr", Always, Narrow, Day, 5.0, "+5 j"),
    ("fr", Always, Long, Month, 5.0, "dans 5 mois"),
    ("fr", Always, Short, Month, 5.0, "dans 5 m."),
    ("fr", Always, Narrow, Month, 5.0, "+5 m."),
    ("fr", Always, Long, Quarter, 5.0, "dans 5 trimestres"),
    ("fr", Always, Short, Quarter, 5.0, "dans 5 trim."),
    ("fr", Always, Narrow, Quarter, 5.0, "+5 trim."),
    ("ja", Always, Long, Day, 5.0, "5 日後"),
    ("ja", Always, Short, Day, 5.0, "5 日後"),
    ("ja", Always, Narrow, Day, 5.0, "5日後"),
    ("ja", Always, Long, Month, 5.0, "5 か月後"),
    ("ja", Always, Short, Month, 5.0, "5 か月後"),
    ("ja", Always, Narrow, Month, 5.0, "5か月後"),
    ("ja", Always, Long, Quarter, 5.0, "5 四半期後"),
    ("ja", Always, Short, Quarter, 5.0, "5 四半期後"),
    ("ja", Always, Narrow, Quarter, 5.0, "5四半期後"),
    ("pl", Always, Long, Day, 5.0, "za 5 dni"),
    ("pl", Always, Short, Day, 5.0, "za 5 dni"),
    ("pl", Always, Narrow, Day, 5.0, "za 5 dni"),
    ("pl", Always, Long, Month, 5.0, "za 5 miesięcy"),
    ("pl", Always, Short, Month, 5.0, "za 5 mies."),
    ("pl", Always, Narrow, Month, 5.0, "za 5 mies."),
    ("pl", Always, Long, Quarter, 5.0, "za 5 kwartałów"),
    ("pl", Always, Short, Quarter, 5.0, "za 5 kw."),
    ("pl", Always, Narrow, Quarter, 5.0, "za 5 kw."),
    ];
    check(CASES);
}

/// Polish has four plural categories, and a fractional count falls into `other`
/// rather than into any of the integer ones — a naive one/other split gets most
/// of these wrong.
#[test]
fn polish_plural_categories() {
    #[rustfmt::skip]
    const CASES: &[Case] = &[
    ("pl", Always, Long, Day, 0.0, "za 0 dni"),
    ("pl", Always, Long, Day, 1.0, "za 1 dzień"),
    ("pl", Always, Long, Day, 2.0, "za 2 dni"),
    ("pl", Always, Long, Day, 5.0, "za 5 dni"),
    ("pl", Always, Long, Day, 22.0, "za 22 dni"),
    ("pl", Always, Long, Day, 1.5, "za 1,5 dnia"),
    ("pl", Always, Long, Day, -1.5, "1,5 dnia temu"),
    ("pl", Always, Long, Month, 0.0, "za 0 miesięcy"),
    ("pl", Always, Long, Month, 1.0, "za 1 miesiąc"),
    ("pl", Always, Long, Month, 2.0, "za 2 miesiące"),
    ("pl", Always, Long, Month, 5.0, "za 5 miesięcy"),
    ("pl", Always, Long, Month, 22.0, "za 22 miesiące"),
    ("pl", Always, Long, Month, 1.5, "za 1,5 miesiąca"),
    ("pl", Always, Long, Month, -1.5, "1,5 miesiąca temu"),
    ];
    check(CASES);
}

/// ECMA-402 sends `-0` down the past branch and `+0` down the future one, which
/// is the only way to ask for "0 days ago" rather than "in 0 days". Under
/// `numeric: "auto"` both still resolve to the offset-0 literal, because the
/// spec keys that lookup on `ToString(value)` and `ToString(-0)` is `"0"`.
#[test]
fn signed_zero_picks_the_direction() {
    assert_eq!(fr("en", -0.0, Day, Always, Long), "0 days ago");
    assert_eq!(fr("en", 0.0, Day, Always, Long), "in 0 days");
    assert_eq!(fr("pl", -0.0, Day, Always, Long), "0 dni temu");
    assert_eq!(fr("pl", 0.0, Day, Always, Long), "za 0 dni");
    assert_eq!(fr("de", -0.0, Hour, Always, Long), "vor 0 Stunden");
    assert_eq!(fr("de", 0.0, Hour, Always, Long), "in 0 Stunden");
    assert_eq!(fr("en", -0.0, Day, Auto, Long), "today");
    assert_eq!(fr("en", 0.0, Day, Auto, Long), "today");
}

/// `numeric: "auto"` is not limited to −1/0/+1: CLDR carries `day` literals out
/// to −2/+2 in most locales, and ECMA-402 reaches them by the same rule.
#[test]
fn auto_reaches_the_two_day_literals() {
    assert_eq!(fr("de", -2.0, Day, Auto, Long), "vorgestern");
    assert_eq!(fr("de", 2.0, Day, Auto, Long), "\u{fc}bermorgen");
    assert_eq!(fr("fr", -2.0, Day, Auto, Long), "avant-hier");
    assert_eq!(fr("es", 2.0, Day, Auto, Long), "pasado ma\u{f1}ana");
    assert_eq!(fr("ja", -2.0, Day, Auto, Long), "\u{4e00}\u{6628}\u{65e5}");
    // `en` defines no −2 literal, so `auto` falls through to the numeric form.
    assert_eq!(fr("en", -2.0, Day, Auto, Long), "2 days ago");
    // Beyond the literals CLDR ships, every locale falls through.
    assert_eq!(fr("de", -3.0, Day, Auto, Long), "vor 3 Tagen");
    // A fractional value can never match: ECMA-402 keys the lookup on
    // `ToString(value)`, and no field is named "1.5".
    assert_eq!(fr("en", 1.5, Day, Auto, Long), "in 1.5 days");
}

/// The eighth ECMA-402 unit, absent before.
#[test]
fn quarter_unit() {
    assert_eq!(fr("en", 3.0, Quarter, Always, Long), "in 3 quarters");
    assert_eq!(fr("en", 3.0, Quarter, Always, Narrow), "in 3q");
    assert_eq!(fr("en", -1.0, Quarter, Auto, Long), "last quarter");
    assert_eq!(fr("de", 3.0, Quarter, Always, Long), "in 3 Quartalen");
    assert_eq!(RelativeUnit::Quarter.as_str(), "quarter");
    assert_eq!(RelativeUnit::from_ecma_id("quarters"), Some(Quarter));
    assert_eq!(RelativeUnit::from_ecma_id("quarter"), Some(Quarter));
    assert_eq!(RelativeUnit::from_ecma_id("fortnight"), None);
}

/// Defaults are ECMA-402's: `numeric: "always"`, `style: "long"`.
#[test]
fn defaults_match_ecma402() {
    let o = RelativeTimeFormatOptions::default();
    assert_eq!(o.numeric, RelativeNumeric::Always);
    assert_eq!(o.width, RelativeWidth::Long);
    assert_eq!(format_relative("en", 1.0, Day, &o), "in 1 day");
    let auto: RelativeTimeFormatOptions = RelativeNumeric::Auto.into();
    assert_eq!(format_relative("en", 1.0, Day, &auto), "tomorrow");
    let narrow: RelativeTimeFormatOptions = RelativeWidth::Narrow.into();
    assert_eq!(format_relative("en", 1.0, Day, &narrow), "in 1d");
}

/// The number goes through the locale's decimal format, so it groups.
#[test]
fn the_count_is_a_localized_number() {
    assert_eq!(fr("en", 1234567.0, Day, Always, Long), "in 1,234,567 days");
    assert_eq!(fr("de", 1234567.0, Day, Always, Long), "in 1.234.567 Tagen");
    assert_eq!(fr("en", 1.5, Day, Always, Long), "in 1.5 days");
    assert_eq!(fr("de", 1.5, Day, Always, Long), "in 1,5 Tagen");
}

/// Region/script subtags are stripped one at a time; an unknown tag ends at
/// `en`, which stands in for CLDR root.
#[test]
fn locale_fallback() {
    assert_eq!(fr("en-GB", 3.0, Day, Always, Long), "in 3 days");
    assert_eq!(fr("de_AT", -1.0, Day, Auto, Long), "gestern");
    assert_eq!(fr("zz", 3.0, Day, Always, Long), "in 3 days");
    assert_eq!(fr("zh-Hant-TW", -1.0, Day, Auto, Long), "\u{6628}\u{5929}");
}

/// The parts carry the number's own kinds, and only the number's parts carry a
/// unit — the shape `Intl.RelativeTimeFormat.prototype.formatToParts` returns.
#[test]
fn parts_shape() {
    let p = format_relative_to_parts("en", 1234567.0, Day, &Default::default());
    let got: Vec<_> = p
        .iter()
        .map(|x| (x.kind, x.value.as_str(), x.unit))
        .collect();
    assert_eq!(
        got,
        [
            (NumberPartType::Literal, "in ", None),
            (NumberPartType::Integer, "1", Some(Day)),
            (NumberPartType::Group, ",", Some(Day)),
            (NumberPartType::Integer, "234", Some(Day)),
            (NumberPartType::Group, ",", Some(Day)),
            (NumberPartType::Integer, "567", Some(Day)),
            (NumberPartType::Literal, " days", None),
        ]
    );
    // A fraction splits into integer / decimal / fraction, like NumberFormat.
    let p = format_relative_to_parts("en", 1.5, Day, &Default::default());
    let kinds: Vec<_> = p.iter().map(|x| x.kind).collect();
    assert_eq!(
        kinds,
        [
            NumberPartType::Literal,
            NumberPartType::Integer,
            NumberPartType::Decimal,
            NumberPartType::Fraction,
            NumberPartType::Literal,
        ]
    );
    // A past pattern that starts with `{0}` has no leading literal.
    let p = format_relative_to_parts("en", -3.0, Day, &Default::default());
    assert_eq!(p[0].kind, NumberPartType::Integer);
    assert_eq!(p[0].unit, Some(Day));
    // An `auto` literal is one unit-less literal part.
    let p = format_relative_to_parts("en", -1.0, Day, &RelativeNumeric::Auto.into());
    assert_eq!(p.len(), 1);
    assert_eq!(p[0].kind, NumberPartType::Literal);
    assert_eq!(p[0].value, "yesterday");
    assert_eq!(p[0].unit, None);
}

/// Concatenating the parts must reproduce `format_relative` exactly across the
/// numeric × width × sign × unit matrix. The two share the pattern lookup but
/// not the number rendering, so this is what keeps them from drifting.
#[test]
fn parts_join_to_format() {
    for lang in ["en", "es", "de", "fr", "pl", "ja", "ar"] {
        for numeric in [Always, Auto] {
            for width in [Long, Short, Narrow] {
                for unit in [Year, Quarter, Month, Week, Day, Hour, Minute, Second] {
                    for value in [-1234.5, -22.0, -1.0, -0.0, 0.0, 1.0, 2.0, 22.0, 1234.5] {
                        let o = opts(numeric, width);
                        let joined: String = format_relative_to_parts(lang, value, unit, &o)
                            .iter()
                            .map(|p| p.value.as_str())
                            .collect();
                        assert_eq!(
                            joined,
                            format_relative(lang, value, unit, &o),
                            "{lang} {numeric:?} {width:?} {unit:?} {value}"
                        );
                    }
                }
            }
        }
    }
}

/// Non-finite input has no ECMA-402 meaning (the spec throws). The crate has no
/// error channel here, so it renders through the number formatter's own
/// spellings rather than panicking.
#[test]
fn non_finite_does_not_panic() {
    assert_eq!(
        fr("en", f64::INFINITY, Day, Always, Long),
        "in \u{221e} days"
    );
    assert!(fr("en", f64::NAN, Day, Always, Long).contains("NaN"));
    assert!(!format_relative_to_parts("en", f64::NAN, Day, &Default::default()).is_empty());
}
