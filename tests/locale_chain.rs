//! CLDR locale inheritance across the formatters (issues #36, #37): explicit
//! `parentLocales` (`en-GB` → `en-001`, `pt-AO` → `pt-PT`), likely-script
//! insertion (`zh-TW` → `zh-Hant-TW`) and the `nonlikelyScript` rule
//! (`zh-Hant` and `sr-Latn` do not inherit their language's default script).
//! Expected values are node 26 / ICU 78 (CLDR 48).
#![cfg(all(
    feature = "list",
    feature = "datetime",
    feature = "currency",
    feature = "displaynames",
    feature = "relative"
))]

use intl::datetime::{DateStyle, DateTime, format_date};
use intl::list::{ListFormatOptions, format_list};
use intl::number::{CurrencyDisplay, NumberFormatOptions, NumberStyle, format};

const DT: DateTime = DateTime {
    year: 2023,
    month: 8,
    day: 9,
    hour: 15,
    minute: 30,
    second: 0,
    millisecond: 0,
};

fn list(l: &str) -> String {
    format_list(l, &["1", "2", "3"], &ListFormatOptions::default())
}

fn cur(l: &str, v: f64, d: CurrencyDisplay) -> String {
    let mut o = NumberFormatOptions::default();
    o.style = NumberStyle::Currency;
    o.currency = Some("TWD");
    o.currency_display = d;
    format(l, v, &o)
}

fn rel(l: &str) -> String {
    intl::relative::format_relative(
        l,
        -1.0,
        intl::relative::RelativeUnit::Day,
        &intl::relative::RelativeTimeFormatOptions::default(),
    )
}

/// #37: `en-GB` inherits `en-001`'s list patterns (no serial comma), as do the
/// other `en-001` children; `en` keeps its own.
#[test]
fn en_001_list_patterns() {
    assert_eq!(list("en-GB"), "1, 2 and 3");
    assert_eq!(list("en-AU"), "1, 2 and 3");
    assert_eq!(list("en-IN"), "1, 2 and 3");
    assert_eq!(list("en_GB"), "1, 2 and 3");
    assert_eq!(list("en"), "1, 2, and 3");
    assert_eq!(list("en-US"), "1, 2, and 3");
}

/// #36: `pt-PT` has its own dates, day periods, time-zone and relative-time
/// names instead of Brazilian `pt`'s, and `pt-AO` inherits them.
#[test]
fn pt_pt() {
    assert_eq!(format_date("pt-PT", &DT, DateStyle::Short), "09/08/23");
    assert_eq!(format_date("pt-AO", &DT, DateStyle::Short), "09/08/23");
    assert_eq!(format_date("pt", &DT, DateStyle::Short), "09/08/2023");
    assert_eq!(
        intl::datetime::format_skeleton("pt-PT", &DT, "hm"),
        "3:30\u{202f}p.m."
    );
    assert_eq!(
        intl::datetime::format_skeleton("pt", &DT, "hm"),
        "3:30\u{202f}PM"
    );
    assert_eq!(
        cur("pt-PT", 2.0, CurrencyDisplay::Name),
        "2,00 novos dólares taiwaneses"
    );
    assert_eq!(
        cur("pt-AO", 1234.5, CurrencyDisplay::Symbol),
        "1\u{a0}234,50\u{a0}NT$"
    );

    use intl::datetime::{DateTimeFormatOptions, TimeZoneNameStyle, format_options};
    let mut o = DateTimeFormatOptions::default();
    o.time_zone = Some("Europe/Paris");
    o.time_zone_name = Some(TimeZoneNameStyle::Long);
    assert_eq!(
        format_options("pt-PT", &DT, &o).unwrap(),
        "09/08/2023, Hora de verão da Europa Central"
    );
    assert_eq!(
        format_options("pt", &DT, &o).unwrap(),
        "09/08/2023, Horário de Verão da Europa Central"
    );
}

/// #36: `zh-TW` is walked as `zh-Hant-TW` and reaches Traditional data; it
/// must never fall back to Simplified `zh`.
#[test]
fn zh_hant() {
    for l in ["zh-TW", "zh_TW", "zh-Hant", "zh-Hant-TW"] {
        assert_eq!(cur(l, 1234.5, CurrencyDisplay::Symbol), "$1,234.50", "{l}");
        assert_eq!(cur(l, 2.0, CurrencyDisplay::Name), "2.00 新台幣", "{l}");
        assert_eq!(intl::display::region_name(l, "TW"), Some("台灣"), "{l}");
        assert_eq!(list(l), "1、2和3", "{l}");
        assert_eq!(rel(l), "1 天前", "{l}");
        assert_eq!(format_date(l, &DT, DateStyle::Short), "2023/8/9", "{l}");
    }
    // `zh-HK` maximizes to `zh-Hant-HK`, which has its own bundle.
    assert_eq!(list("zh-HK"), "1、2及3");
    assert_eq!(rel("zh-HK"), "1 日前");
    assert_eq!(format_date("zh-HK", &DT, DateStyle::Short), "9/8/2023");
    assert_eq!(cur("zh-HK", 1234.5, CurrencyDisplay::Symbol), "NT$1,234.50");
    // `zh-MO`'s parent is `zh-Hant-HK`.
    assert_eq!(list("zh-MO"), "1、2及3");
    // Simplified stays Simplified.
    assert_eq!(cur("zh", 2.0, CurrencyDisplay::Name), "2.00新台币");
    assert_eq!(intl::display::region_name("zh-CN", "TW"), Some("台湾"));
}

/// `sr-Latn` (and `sr-ME`, which maximizes to it) is Latin, not `sr`'s Cyrillic.
#[test]
fn sr_latn() {
    for l in ["sr-Latn", "sr-ME"] {
        assert_eq!(list(l), "1, 2 i 3", "{l}");
        assert_eq!(rel(l), "pre 1 dana", "{l}");
        assert_eq!(intl::display::region_name(l, "TW"), Some("Tajvan"), "{l}");
    }
    assert_eq!(rel("sr"), "пре 1 дана");
}

/// Regional currency symbols come from each region's own (delta) record.
#[test]
fn regional_currency_symbols() {
    let sym = |l: &str, c: &'static str| {
        let mut o = NumberFormatOptions::default();
        o.style = NumberStyle::Currency;
        o.currency = Some(c);
        format(l, 1234.5, &o)
    };
    assert_eq!(sym("en-AU", "AUD"), "$1,234.50");
    assert_eq!(sym("en-CA", "CAD"), "$1,234.50");
    assert_eq!(sym("en", "AUD"), "A$1,234.50");
    assert_eq!(sym("es-AR", "USD"), "US$\u{a0}1.234,50");
}
