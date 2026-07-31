//! Measurement-unit formatting.
#![cfg(feature = "units")]
use intl::unit::{Unit, Unit::*, UnitWidth::*, format_unit as fu, format_unit_id as fid};

#[test]
fn units() {
    assert_eq!(fu("en", 5.0, Kilometer, Long), "5 kilometers");
    assert_eq!(fu("en", 1.0, Kilometer, Long), "1 kilometer");
    assert_eq!(fu("en", 3.0, Hour, Short), "3 hr");
    assert_eq!(fu("en", 1.0, Hour, Long), "1 hour");
    assert_eq!(fu("de", 2.0, Hour, Long), "2 Stunden");
    assert_eq!(fu("fr", 5.0, Meter, Long), "5\u{a0}mètres"); // NBSP in French
    assert_eq!(fu("en", 2.5, Gigabyte, Long), "2.5 gigabytes");
    // Locale fallback to English for unknown locale.
    assert_eq!(fu("xx", 5.0, Kilometer, Long), "5 kilometers");
}

/// The count is formatted by `number`, so it follows the locale's CLDR
/// `defaultNumberingSystem` like everything else — `Intl.NumberFormat('mr',
/// {style: 'unit', unit: 'kilometer', unitDisplay: 'long'}).format(5)` is
/// "५ किलोमीटर" in node 22 (ICU 77), not "5 किलोमीटर".
#[test]
fn count_uses_the_locales_numbering_system() {
    assert_eq!(fu("mr", 5.0, Kilometer, Long), "५ किलोमीटर");
    // CLDR 48 respells `bn`'s "hour" with ণ (U+09A3); ICU 77 carries CLDR 47's
    // ন (U+09A8). The vendored data wins — only the digits are at issue here.
    assert_eq!(fu("bn", 3.0, Hour, Short), "৩ ঘণ্টা");
    assert_eq!(fu("ar-EG", 5.0, Kilometer, Long), "٥ كيلومترات");
    // A `-u-nu-` keyword still overrides it.
    assert_eq!(fu("mr-u-nu-latn", 5.0, Kilometer, Long), "5 किलोमीटर");
}

/// The 19 units added to complete ECMA-402's sanctioned set. Their CLDR category
/// prefixes are irregular (`concentr-percent`, `angle-degree`, `area-acre`), so
/// each is pinned against the vendored data.
#[test]
fn sanctioned_units() {
    let cases = [
        (Acre, "5 acres", "5 ac"),
        (Bit, "5 bits", "5 bit"),
        (Degree, "5 degrees", "5 deg"),
        (FluidOunce, "5 fluid ounces", "5 fl oz"),
        (Gallon, "5 gallons", "5 gal"),
        (Gigabit, "5 gigabits", "5 Gb"),
        (Hectare, "5 hectares", "5 ha"),
        (Kilobit, "5 kilobits", "5 kb"),
        (Megabit, "5 megabits", "5 Mb"),
        (Microsecond, "5 microseconds", "5 μs"),
        (MileScandinavian, "5 miles-scandinavian", "5 smi"),
        (Millisecond, "5 milliseconds", "5 ms"),
        (Nanosecond, "5 nanoseconds", "5 ns"),
        (Percent, "5 percent", "5%"),
        (Petabyte, "5 petabytes", "5 PB"),
        (Stone, "5 stones", "5 st"),
        (Terabit, "5 terabits", "5 Tb"),
        (Terabyte, "5 terabytes", "5 TB"),
        (Yard, "5 yards", "5 yd"),
    ];
    for (unit, long, short) in cases {
        assert_eq!(fu("en", 5.0, unit, Long), long, "{}", unit.ecma_id());
        assert_eq!(fu("en", 5.0, unit, Short), short, "{}", unit.ecma_id());
    }
    // Non-English wording for a few of them.
    assert_eq!(fu("fr", 5.0, Hectare, Long), "5\u{a0}hectares");
    assert_eq!(fu("fr", 5.0, Percent, Long), "5 pour cent");
    assert_eq!(fu("de", 5.0, Yard, Long), "5 Yards");
}

#[test]
fn ecma_ids_round_trip() {
    let ids = [
        "second",
        "minute",
        "hour",
        "day",
        "week",
        "month",
        "year",
        "millimeter",
        "centimeter",
        "meter",
        "kilometer",
        "inch",
        "foot",
        "mile",
        "gram",
        "kilogram",
        "ounce",
        "pound",
        "byte",
        "kilobyte",
        "megabyte",
        "gigabyte",
        "celsius",
        "fahrenheit",
        "kilometer-per-hour",
        "mile-per-hour",
        "liter",
        "milliliter",
        "acre",
        "bit",
        "degree",
        "fluid-ounce",
        "gallon",
        "gigabit",
        "hectare",
        "kilobit",
        "megabit",
        "microsecond",
        "mile-scandinavian",
        "millisecond",
        "nanosecond",
        "percent",
        "petabyte",
        "stone",
        "terabit",
        "terabyte",
        "yard",
    ];
    // 45 ECMA-402 sanctioned simple units + the two pre-composed `speed-…` ones.
    assert_eq!(ids.len(), 47);
    for id in ids {
        let u = Unit::from_ecma_id(id).unwrap_or_else(|| panic!("unresolved: {id}"));
        assert_eq!(u.ecma_id(), id);
    }
    assert_eq!(Unit::from_ecma_id("furlong"), None);
    // Compounds are not single units; `format_unit_id` handles those.
    assert_eq!(Unit::from_ecma_id("meter-per-second"), None);
}

/// UTS #35 compound units via the denominator's `perUnitPattern` — the path taken
/// by the 18 of 45 sanctioned units that carry one.
#[test]
fn compound_per_unit_pattern() {
    // `second` long has perUnitPattern "{0} per second"; values match V8/ICU.
    assert_eq!(
        fid("en", 5.0, "meter-per-second", Long).unwrap(),
        "5 meters per second"
    );
    assert_eq!(
        fid("en", 1.0, "meter-per-second", Long).unwrap(),
        "1 meter per second"
    );
    assert_eq!(fid("en", 5.0, "meter-per-second", Short).unwrap(), "5 m/s");
    // `kilometer` long has perUnitPattern "{0} per kilometer".
    assert_eq!(
        fid("en", 5.0, "liter-per-kilometer", Long).unwrap(),
        "5 liters per kilometer"
    );
    assert_eq!(
        fid("en", 5.0, "liter-per-kilometer", Short).unwrap(),
        "5 L/km"
    );
    // The plural category is the numerator's, under the locale's own rules.
    assert_eq!(
        fid("ru", 1.0, "meter-per-second", Long).unwrap(),
        "1 метр в секунду"
    );
    assert_eq!(
        fid("ru", 5.0, "meter-per-second", Long).unwrap(),
        "5 метров в секунду"
    );
    assert_eq!(
        fid("ja", 5.0, "meter-per-second", Long).unwrap(),
        "5 メートル/秒"
    );
}

/// UTS #35 compound units via the locale's `per` `compoundUnitPattern` — the
/// fallback for the 27 sanctioned units with no `perUnitPattern`. The
/// denominator's singular wording, placeholder stripped, fills `{1}`.
#[test]
fn compound_unit_pattern_fallback() {
    // `mile` has no perUnitPattern at any width; values match V8/ICU.
    assert_eq!(
        fid("en", 5.0, "gallon-per-mile", Long).unwrap(),
        "5 gallons per mile"
    );
    assert_eq!(
        fid("en", 1.0, "gallon-per-mile", Long).unwrap(),
        "1 gallon per mile"
    );
    assert_eq!(
        fid("en", 5.0, "gallon-per-mile", Short).unwrap(),
        "5 gal/mi"
    );
    // The connector is localized: de "{0} pro {1}", fr "{0} par {1}", ja "{0}毎{1}".
    assert_eq!(
        fid("de", 5.0, "gallon-per-mile", Long).unwrap(),
        "5\u{a0}Gallonen pro Meile"
    );
    assert_eq!(
        fid("fr", 5.0, "gallon-per-mile", Long).unwrap(),
        "5 gallons par mile"
    );
    assert_eq!(
        fid("ja", 5.0, "gallon-per-mile", Long).unwrap(),
        "5 ガロン毎マイル"
    );
    // `millisecond` likewise has none.
    assert_eq!(
        fid("en", 5.0, "byte-per-millisecond", Long).unwrap(),
        "5 bytes per millisecond"
    );
    assert_eq!(
        fid("en", 5.0, "byte-per-millisecond", Short).unwrap(),
        "5 byte/ms"
    );
}

#[test]
fn compound_unit_edge_cases() {
    // CLDR ships the two `speed-…` units pre-composed and the canned wording wins:
    // deriving would give "5 mi/h" here.
    assert_eq!(fid("en", 5.0, "mile-per-hour", Short).unwrap(), "5 mph");
    assert_eq!(
        fid("en", 5.0, "kilometer-per-hour", Short).unwrap(),
        "5 km/h"
    );
    assert_eq!(
        fid("en", 5.0, "kilometer-per-hour", Long).unwrap(),
        "5 kilometers per hour"
    );
    // A denominator that puts the placeholder inside its phrase (ja
    // "摂氏 {0} 度") must not make the number show up twice.
    assert_eq!(
        fid("ja", 5.0, "meter-per-celsius", Long).unwrap(),
        "5 メートル毎摂氏  度"
    );
    // ECMA-402 allows exactly one "-per-", a sanctioned unit on each side.
    assert_eq!(fid("en", 5.0, "furlong", Long), None);
    assert_eq!(fid("en", 5.0, "meter-per-furlong", Long), None);
    assert_eq!(fid("en", 5.0, "meter-per-second-per-second", Long), None);
    // The typed API agrees with the string one.
    assert_eq!(
        intl::unit::format_compound_unit("en", 5.0, Gallon, Mile, Long),
        fid("en", 5.0, "gallon-per-mile", Long).unwrap()
    );
}

/// ECMA-402 `unitDisplay: "narrow"`. Values match V8/ICU.
#[cfg(feature = "units-narrow")]
#[test]
fn narrow_width() {
    assert_eq!(fu("en", 5.0, Kilometer, Narrow), "5km");
    assert_eq!(fu("en", 3.0, Hour, Narrow), "3h");
    assert_eq!(fu("en", 5.0, Percent, Narrow), "5%");
    assert_eq!(fu("en", 5.0, Degree, Narrow), "5°");
    // Narrow is not just "short minus the space" — several locales keep one.
    assert_eq!(fu("de", 5.0, Kilometer, Narrow), "5 km");
    assert_eq!(fu("de", 5.0, Percent, Narrow), "5 %");
    assert_eq!(fu("fr", 1.0, Meter, Narrow), "1m");
    assert_eq!(fu("ja", 5.0, Kilometer, Narrow), "5km");
    assert_eq!(fu("ru", 5.0, Kilometer, Narrow), "5 км");
    assert_eq!(fu("cs", 3.0, Hour, Narrow), "3 h");
    // Compounds assemble at the narrow width too (both paths).
    assert_eq!(fid("en", 5.0, "meter-per-second", Narrow).unwrap(), "5m/s");
    assert_eq!(
        fid("en", 5.0, "gallon-per-mile", Narrow).unwrap(),
        "5gal/mi"
    );
}

/// Without `units-narrow` the narrow patterns are not compiled in, and UTS #35's
/// width fallback (narrow → short) applies.
#[cfg(not(feature = "units-narrow"))]
#[test]
fn narrow_falls_back_to_short() {
    assert_eq!(fu("en", 5.0, Kilometer, Narrow), "5 km");
    assert_eq!(
        fu("en", 5.0, Kilometer, Narrow),
        fu("en", 5.0, Kilometer, Short)
    );
    assert_eq!(fid("en", 5.0, "meter-per-second", Narrow).unwrap(), "5 m/s");
}

#[test]
fn durations() {
    use intl::unit::{UnitWidth::*, format_duration as fd};
    assert_eq!(fd("en", 3661, Long), "1 hour 1 minute 1 second");
    assert_eq!(fd("en", 90, Long), "1 minute 30 seconds");
    assert_eq!(fd("en", 90, Short), "1 min 30 sec");
    assert_eq!(fd("en", 0, Long), "0 seconds");
    assert_eq!(fd("en", 86400 + 3600, Long), "1 day 1 hour");
    assert_eq!(fd("en", -120, Long), "-2 minutes");
    // Localized: German wording + number.
    assert!(fd("de", 3661, Long).contains("Stunde"));
}

/// Traditional Chinese has its own unit bundle, and Hong Kong/Macau differ from
/// Taiwan again. Every `zh*` tag used to serve the Simplified data, because the
/// runtime trims `-` subtags and does no script inference. Values match V8/ICU.
#[test]
fn traditional_chinese_units() {
    use intl::unit::{Unit::KilometerPerHour, UnitWidth::*, format_unit};

    // Simplified stays Simplified — `zh` short really is the Latin abbreviation.
    assert_eq!(
        format_unit("zh", -987.0, KilometerPerHour, Long),
        "每小时-987公里"
    );
    assert_eq!(
        format_unit("zh", -987.0, KilometerPerHour, Short),
        "-987 km/h"
    );
    assert_eq!(
        format_unit("zh-CN", -987.0, KilometerPerHour, Short),
        "-987 km/h"
    );

    // Traditional: its own wording, and spaces around the number.
    assert_eq!(
        format_unit("zh-Hant", -987.0, KilometerPerHour, Long),
        "每小時 -987 公里"
    );
    assert_eq!(
        format_unit("zh-Hant", -987.0, KilometerPerHour, Short),
        "-987 公里/小時"
    );
    // A region tag CLDR maximizes onto Hant reaches it without script inference.
    assert_eq!(
        format_unit("zh-TW", -987.0, KilometerPerHour, Short),
        "-987 公里/小時"
    );
    // ...and Hong Kong/Macau have their *own* bundle, not Taiwan's, so the alias
    // has to prefer the most specific vendored record.
    assert_eq!(
        format_unit("zh-HK", -987.0, KilometerPerHour, Short),
        "-987 公里每小時"
    );
    assert_eq!(
        format_unit("zh-MO", -987.0, KilometerPerHour, Short),
        "-987 公里每小時"
    );
}

#[cfg(feature = "units-narrow")]
#[test]
fn traditional_chinese_units_narrow() {
    use intl::unit::{Unit::KilometerPerHour, UnitWidth::Narrow, format_unit};
    assert_eq!(
        format_unit("zh", -987.0, KilometerPerHour, Narrow),
        "-987km/h"
    );
    assert_eq!(
        format_unit("zh-Hant", -987.0, KilometerPerHour, Narrow),
        "-987公里/小時"
    );
    // Hong Kong narrows all the way to the Latin "kph".
    assert_eq!(
        format_unit("zh-HK", -987.0, KilometerPerHour, Narrow),
        "-987kph"
    );
}
