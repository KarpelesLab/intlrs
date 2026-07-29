//! Locale-aware number formatting.
#![cfg(feature = "number")]

use intl::number::{format_decimal as dec, format_percent as pct};

/// Build options from `Default` (the struct is `#[non_exhaustive]`).
fn nf(
    build: impl FnOnce(&mut intl::number::NumberFormatOptions),
) -> intl::number::NumberFormatOptions {
    let mut o = intl::number::NumberFormatOptions::default();
    build(&mut o);
    o
}

#[test]
fn decimal_grouping_and_separators() {
    assert_eq!(dec("en", 1234567.0), "1,234,567");
    assert_eq!(dec("de", 1234567.0), "1.234.567");
    assert_eq!(dec("fr", 1234567.0), "1\u{202f}234\u{202f}567"); // narrow no-break space
    assert_eq!(dec("hi", 1234567.0), "12,34,567"); // Indian grouping
    assert_eq!(dec("en", 1234.5), "1,234.5");
    assert_eq!(dec("de", 1234.5), "1.234,5");
}

#[test]
fn fraction_and_sign() {
    // Default max 3 fraction digits, trailing zeros trimmed.
    assert_eq!(dec("en", 0.5), "0.5");
    assert_eq!(dec("en", 1.25), "1.25");
    assert_eq!(dec("en", 1.0), "1");
    assert_eq!(dec("en", -1234.5), "-1,234.5");
    // Rounding to 3 fraction digits.
    assert_eq!(dec("en", 1.23456), "1.235");
}

#[test]
fn percent_formatting() {
    assert_eq!(pct("en", 0.5), "50%");
    assert_eq!(pct("de", 0.5), "50\u{a0}%"); // NBSP before %
    assert_eq!(pct("en", 0.1234), "12%"); // 0 fraction digits in the percent pattern
}

#[test]
fn unknown_locale_falls_back() {
    assert_eq!(dec("xx", 1234.5), dec("en", 1234.5));
    assert_eq!(dec("en-US", 1234.5), "1,234.5"); // region falls back to language
}

#[cfg(feature = "currency")]
#[test]
fn currency() {
    use intl::number::format_currency as fc;
    assert_eq!(fc("en", 1234.5, "USD"), "$1,234.50");
    assert_eq!(fc("de", 1234.5, "EUR"), "1.234,50\u{a0}€");
    assert_eq!(fc("ja", 1234.0, "JPY"), "￥1,234"); // 0 fraction digits
    assert_eq!(fc("en", -5.0, "USD"), "-$5.00");
    // Unknown locale falls back; unknown currency uses its code as the symbol.
    assert_eq!(fc("xx", 5.0, "USD"), "$5.00");
    assert!(fc("en", 5.0, "XYZ").contains("XYZ"));
}

#[test]
fn parsing() {
    use intl::number::{format_decimal as f, parse_decimal as p};
    assert_eq!(p("en", "1,234.5"), Some(1234.5));
    assert_eq!(p("de", "1.234,5"), Some(1234.5));
    assert_eq!(p("fr", "1\u{202f}234,5"), Some(1234.5));
    assert_eq!(p("en", "-42"), Some(-42.0));
    assert_eq!(p("hi", "12,34,567"), Some(1234567.0)); // Indian grouping
    assert_eq!(p("en", "abc"), None);
    assert_eq!(p("en", ""), None);
    // Round-trips: format then parse.
    for &(lang, v) in &[("en", 1234567.0_f64), ("de", -98765.43), ("fr", 1000.0)] {
        assert_eq!(p(lang, &f(lang, v)), Some(v));
    }
}

#[test]
fn scientific() {
    use intl::number::format_scientific as sci;
    assert_eq!(sci("en", 12345.0, 6), "1.2345E4");
    assert_eq!(sci("en", 1.0, 6), "1E0");
    assert_eq!(sci("en", 1000.0, 6), "1E3");
    assert_eq!(sci("en", -250.0, 6), "-2.5E2");
    assert_eq!(sci("de", 0.00042, 6), "4,2E-4");
    assert_eq!(sci("en", 0.0, 6), "0");
    assert_eq!(sci("en", 6.022e23, 6), "6.022E23");
}

#[test]
fn compact() {
    use intl::number::format_compact as k;
    assert_eq!(k("en", 999.0), "999");
    assert_eq!(k("en", 1500.0), "1.5K");
    assert_eq!(k("en", 15000.0), "15K");
    assert_eq!(k("en", 150000.0), "150K");
    assert_eq!(k("en", 2_300_000.0), "2.3M");
    assert_eq!(k("en", 1_000_000_000.0), "1B");
    assert_eq!(k("de", 1500.0), "1.500"); // German doesn't abbreviate thousands
    assert_eq!(k("fr", 1500.0), "1,5\u{a0}k"); // French: NBSP + lowercase k
}

#[test]
fn native_digits() {
    use intl::number::{format_decimal_default_numbering as fdn, to_numbering_system as tns};
    assert_eq!(tns("2024", "arab"), "٢٠٢٤");
    assert_eq!(tns("3.14", "deva"), "३.१४");
    assert_eq!(tns("123", "latn"), "123");
    assert_eq!(tns("123", "unknown"), "123");
    // `to_numbering_system` is digits-only: CLDR has no locale-independent
    // symbol table for a numbering system (see its docs), so the separator is
    // left alone even though `arab` conventionally uses U+066B.
    assert_eq!(tns("1.5", "arab"), "١.٥");
    // Persian defaults to Extended Arabic-Indic digits *and* its arabext
    // separators — U+066C group, U+066B decimal, as `Intl.NumberFormat('fa')`.
    #[cfg(feature = "number-numsys")]
    assert_eq!(fdn("fa", 1234.5), "۱\u{66c}۲۳۴\u{66b}۵");
    // Without the symbol blocks compiled in, the digits still switch but the
    // locale's `latn` separators remain.
    #[cfg(not(feature = "number-numsys"))]
    assert_eq!(fdn("fa", 1234.5), "۱,۲۳۴.۵");
    // English stays Latin.
    assert_eq!(fdn("en", 1234.5), "1,234.5");
    // `ar` defaults to `latn` in CLDR 48, matching `Intl.NumberFormat('ar')`.
    assert_eq!(fdn("ar", 1234.5), "1,234.5");
}

#[test]
fn numbering_system_resolution() {
    use intl::number::{
        default_numbering_system as d, format_decimal as f, native_numbering_system as n,
    };
    // The default and the native system are different data points: `ar` and `hi`
    // both default to `latn` in CLDR 48 but are natively arab/deva.
    assert_eq!((d("ar"), n("ar")), ("latn", "arab"));
    assert_eq!((d("hi"), n("hi")), ("latn", "deva"));
    assert_eq!((d("fa"), n("fa")), ("arabext", "arabext"));
    assert_eq!((d("zh"), n("zh")), ("latn", "hanidec"));
    assert_eq!((d("en-GB"), n("en-GB")), ("latn", "latn")); // through the chain
    assert_eq!((d("qqq"), n("qqq")), ("latn", "latn")); // unknown -> root

    // A `-u-nu-` keyword switches the digits regardless of `number-numsys`.
    assert_eq!(f("hi-u-nu-native", 1234567.0), "१२,३४,५६७");
    assert_eq!(f("zh-u-nu-native", 1234.5), "一,二三四.五");
    // A locale with no block for the requested system keeps its own `latn`
    // symbols — `Intl.NumberFormat('en-u-nu-arab').format(1234.5)` is the same.
    assert_eq!(f("en-u-nu-arab", 1234.5), "١,٢٣٤.٥");
    // An unknown or non-positional system falls back to Latin digits.
    assert_eq!(f("en-u-nu-zzzz", 1234.5), "1,234.5");
    assert_eq!(f("ja-u-nu-jpan", 1234.5), "1,234.5");
    // Other `-u-` keywords are skipped, and the plain tag is unaffected.
    assert_eq!(f("ar", 1234.5), "1,234.5");

    // With the per-system blocks compiled in, the request also picks up that
    // locale's symbols and patterns, as ICU's `NumberElements/<ns>` lookup does.
    #[cfg(feature = "number-numsys")]
    {
        assert_eq!(f("ar-u-nu-arab", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
        assert_eq!(f("ar-u-ca-islamic-nu-arab", 1.5), "١\u{66b}٥");
        // Same numbering system, different locale, different symbols: `sd`'s
        // arab block keeps an ASCII decimal separator while `ar`'s uses U+066B.
        assert_eq!(f("sd-u-nu-arab", 1234.5), "١\u{66c}٢٣٤.٥");
        // Patterns are per-system too: `te` groups Indian-style in `latn` only.
        assert_eq!(f("te", 1234567.0), "12,34,567");
        assert_eq!(f("te-u-nu-telu", 1234567.0), "౧,౨౩౪,౫౬౭");
    }
}

#[test]
fn numbering_system_option() {
    use intl::number::{NumberFormatOptions, format};
    let mut o = NumberFormatOptions::default();
    // ECMA-402 `ResolveLocale`: an explicit option outranks the `-u-nu-` tag.
    o.numbering_system = Some("latn");
    assert_eq!(format("ar-u-nu-arab", 1234.5, &o), "1,234.5");
    // `"native"` is the UTS #35 alias for `otherNumberingSystems.native`.
    o.numbering_system = Some("native");
    assert_eq!(format("hi", 1234.0, &o), "१,२३४");
    o.numbering_system = Some("arab");
    #[cfg(feature = "number-numsys")]
    assert_eq!(format("ar", 1234.5, &o), "١\u{66c}٢٣٤\u{66b}٥");
    #[cfg(not(feature = "number-numsys"))]
    assert_eq!(format("ar", 1234.5, &o), "١,٢٣٤.٥");
}

#[test]
fn locale_nan_and_infinity() {
    use intl::number::{
        NumberFormatOptions, NumberPartType, NumberStyle, format, format_decimal, format_percent,
        format_scientific, format_to_parts,
    };
    let o = NumberFormatOptions::default();
    // CLDR `symbols/nan`; `Intl.NumberFormat('ar').format(NaN)` is the same.
    assert_eq!(format("ar", f64::NAN, &o), "ليس\u{a0}رقمًا");
    assert_eq!(format("fa", f64::NAN, &o), "ناعدد");
    assert_eq!(format("en", f64::NAN, &o), "NaN");
    // The non-options paths agree (they share the locale spec now).
    assert_eq!(format_decimal("ar", f64::NAN), "ليس\u{a0}رقمًا");
    assert_eq!(format_scientific("ar", f64::NAN, 6), "ليس\u{a0}رقمًا");
    // Every vendored locale spells infinity U+221E, including in a percent
    // pattern (where the symbol still wraps the placeholder).
    assert_eq!(format("de", f64::INFINITY, &o), "∞");
    assert_eq!(format_percent("de", f64::INFINITY), "∞\u{a0}%");
    assert_eq!(format("en", f64::NEG_INFINITY, &o), "-∞");

    let parts = format_to_parts("ar", f64::NAN, &o);
    assert_eq!(parts.len(), 1);
    assert_eq!(parts[0].kind, NumberPartType::Nan);
    assert_eq!(parts[0].value, "ليس\u{a0}رقمًا");

    // The per-numbering-system block wins: `ar`'s arab exponential differs but
    // its nan/infinity match latn, while `sd` differs the other way (its arab
    // nan is plain "NaN").
    let mut ns = NumberFormatOptions::default();
    ns.numbering_system = Some("arab");
    ns.style = NumberStyle::Decimal;
    assert_eq!(format("sd", f64::NAN, &ns), "NaN");
}

#[cfg(feature = "number-range")]
#[test]
fn format_range_basics() {
    use intl::number::{NumberFormatOptions, format_range as r};
    let o = NumberFormatOptions::default();
    // CLDR `miscPatterns/range`; `Intl.NumberFormat('en').formatRange(3, 5)`
    // gives the same en dash with no spacing.
    assert_eq!(r("en", 3.0, 5.0, &o), "3\u{2013}5");
    assert_eq!(r("zh", 3.0, 5.0, &o), "3-5"); // zh uses an ASCII hyphen
    assert_eq!(r("ko", 3.0, 5.0, &o), "3~5");
    assert_eq!(r("ja", 3.0, 5.0, &o), "3\u{ff5e}5");
    assert_eq!(r("de", 1234.5, 2345.5, &o), "1.234,5\u{2013}2.345,5");

    // ECMA-402 `PartitionNumberRangePattern` step 5: ends that format alike
    // collapse to the `approximately` pattern, not to a degenerate range.
    assert_eq!(r("en", 3.0, 3.0, &o), "~3");
    assert_eq!(r("de", 3.0, 3.0, &o), "\u{2248}3");
    // `approximately` (miscPatterns) is independent of `approximatelySign`
    // (symbols): fr's are U+2248 and U+2243 respectively.
    assert_eq!(r("fr", 3.0, 3.0, &o), "\u{2248}3");
    assert_eq!(r("ja", 3.0, 3.0, &o), "約 3");

    // Distinct values that round to the same string still collapse.
    let mut two = NumberFormatOptions::default();
    two.maximum_fraction_digits = Some(0);
    assert_eq!(r("en", 2.9999, 3.0001, &two), "~3");

    // An unknown locale falls back to the root (English) forms.
    assert_eq!(r("qqq", 3.0, 5.0, &o), "3\u{2013}5");
    assert_eq!(r("qqq", 3.0, 3.0, &o), "~3");
}

#[cfg(all(feature = "number-range", feature = "currency"))]
#[test]
fn format_range_currency_and_parts() {
    use intl::number::{
        NumberFormatOptions, NumberPartType, NumberRangeSource, NumberStyle, format_range,
        format_range_to_parts,
    };
    let mut cur = NumberFormatOptions::default();
    cur.style = NumberStyle::Currency;
    cur.currency = Some("USD");
    // The CLDR `range` pattern is applied verbatim, so the affixes repeat and no
    // spacing is inserted around the separator. (ICU's NumberRangeFormatter has
    // extra collapse/padding heuristics for affixed forms that we do not model.)
    assert_eq!(format_range("en", 2.9, 3.1, &cur), "$2.90\u{2013}$3.10");
    assert_eq!(format_range("en", 2.999, 3.001, &cur), "~$3.00");

    let parts = format_range_to_parts("en", 2.9, 3.1, &cur);
    let tagged: Vec<_> = parts
        .iter()
        .map(|p| (p.kind.as_str(), p.source.as_str(), p.value.as_str()))
        .collect();
    assert_eq!(
        tagged,
        vec![
            ("currency", "startRange", "$"),
            ("integer", "startRange", "2"),
            ("decimal", "startRange", "."),
            ("fraction", "startRange", "90"),
            ("literal", "shared", "\u{2013}"),
            ("currency", "endRange", "$"),
            ("integer", "endRange", "3"),
            ("decimal", "endRange", "."),
            ("fraction", "endRange", "10"),
        ]
    );

    // Collapsed: everything is `shared` and the sign is its own part, per
    // ECMA-402 `FormatApproximately`.
    let approx = format_range_to_parts("en", 3.0, 3.0, &cur);
    assert!(approx.iter().all(|p| p.source == NumberRangeSource::Shared));
    assert_eq!(approx[0].kind, NumberPartType::ApproximatelySign);
    assert_eq!(approx[0].value, "~");
    // ja puts the sign and a space before the number; only the sign is tagged.
    let ja = format_range_to_parts("ja", 3.0, 3.0, &Default::default());
    assert_eq!(
        ja.iter()
            .map(|p| (p.kind.as_str(), p.value.as_str()))
            .collect::<Vec<_>>(),
        vec![
            ("approximatelySign", "約"),
            ("literal", " "),
            ("integer", "3"),
        ]
    );
}

#[test]
fn ordinals() {
    use intl::number::format_ordinal as o;
    assert_eq!(o("en", 1), "1st");
    assert_eq!(o("en", 2), "2nd");
    assert_eq!(o("en", 3), "3rd");
    assert_eq!(o("en", 4), "4th");
    assert_eq!(o("en", 11), "11th");
    assert_eq!(o("en", 21), "21st");
    assert_eq!(o("en", 102), "102nd");
    assert_eq!(o("fr", 1), "1er");
    assert_eq!(o("fr", 2), "2e");
    assert_eq!(o("de", 2), "2."); // period convention
    assert_eq!(o("sv", 1), "1:a");
}

#[test]
fn compact_non_finite() {
    use intl::number::format_compact as k;
    // Non-finite values must not panic (NaN < 1000.0 is false).
    let _ = k("en", f64::NAN);
    let _ = k("en", f64::INFINITY);
    let _ = k("en", f64::NEG_INFINITY);
}

#[cfg(feature = "units")]
#[test]
fn unit_style() {
    use intl::number::{NumberPartType, NumberStyle, UnitDisplay, format, format_to_parts};
    let mk = |unit, disp| {
        nf(move |o| {
            o.style = NumberStyle::Unit;
            o.unit = Some(unit);
            o.unit_display = disp;
        })
    };
    assert_eq!(
        format("en", 5.0, &mk("kilometer", UnitDisplay::Long)),
        "5 kilometers"
    );
    assert_eq!(
        format("en", 1.0, &mk("kilometer", UnitDisplay::Long)),
        "1 kilometer"
    );
    assert_eq!(format("en", 3.0, &mk("hour", UnitDisplay::Short)), "3 hr");
    assert_eq!(
        format("en", 5.0, &mk("kilometer-per-hour", UnitDisplay::Short)),
        "5 km/h"
    );
    assert_eq!(
        format("de", 2.0, &mk("hour", UnitDisplay::Long)),
        "2 Stunden"
    );
    // Parts: number core, then a literal space and the unit.
    let parts = format_to_parts("en", 1.5, &mk("meter", UnitDisplay::Short));
    assert_eq!(parts.last().unwrap().kind, NumberPartType::Unit);
    assert_eq!(parts.last().unwrap().value, "m");
    // Unknown unit degrades to the bare number.
    assert_eq!(format("en", 5.0, &mk("furlong", UnitDisplay::Long)), "5");
    // ... including an unsanctioned half of a compound.
    assert_eq!(
        format("en", 5.0, &mk("meter-per-furlong", UnitDisplay::Long)),
        "5"
    );
}

/// ECMA-402 `unit` identifiers that are not a single CLDR unit: any
/// `<unit>-per-<unit>` pair. Values match V8/ICU.
#[cfg(feature = "units")]
#[test]
fn unit_style_compound() {
    use intl::number::{NumberPartType, NumberStyle, UnitDisplay, format, format_to_parts};
    let mk = |unit, disp| {
        nf(move |o| {
            o.style = NumberStyle::Unit;
            o.unit = Some(unit);
            o.unit_display = disp;
        })
    };
    // perUnitPattern path (`second` has one).
    assert_eq!(
        format("en", 5.0, &mk("meter-per-second", UnitDisplay::Long)),
        "5 meters per second"
    );
    assert_eq!(
        format("en", 5.0, &mk("meter-per-second", UnitDisplay::Short)),
        "5 m/s"
    );
    // compoundUnitPattern path (`mile` has none).
    assert_eq!(
        format("en", 5.0, &mk("gallon-per-mile", UnitDisplay::Long)),
        "5 gallons per mile"
    );
    assert_eq!(
        format("de", 5.0, &mk("gallon-per-mile", UnitDisplay::Long)),
        "5\u{a0}Gallonen pro Meile"
    );
    // ICU tags the whole unit phrase, interior spaces included, as one part.
    let parts = format_to_parts("en", 5.0, &mk("meter-per-second", UnitDisplay::Long));
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].kind, NumberPartType::Integer);
    assert_eq!(parts[1].kind, NumberPartType::Literal);
    assert_eq!(parts[1].value, " ");
    assert_eq!(parts[2].kind, NumberPartType::Unit);
    assert_eq!(parts[2].value, "meters per second");
}

/// Units added to complete the ECMA-402 sanctioned set, reached through the
/// string identifier `NumberFormat` takes.
#[cfg(feature = "units")]
#[test]
fn unit_style_sanctioned_set() {
    use intl::number::{NumberStyle, UnitDisplay, format};
    let mk = |unit, disp| {
        nf(move |o| {
            o.style = NumberStyle::Unit;
            o.unit = Some(unit);
            o.unit_display = disp;
        })
    };
    assert_eq!(format("en", 5.0, &mk("acre", UnitDisplay::Long)), "5 acres");
    assert_eq!(
        format("en", 5.0, &mk("degree", UnitDisplay::Short)),
        "5 deg"
    );
    assert_eq!(
        format("en", 5.0, &mk("fluid-ounce", UnitDisplay::Long)),
        "5 fluid ounces"
    );
    assert_eq!(
        format("en", 5.0, &mk("mile-scandinavian", UnitDisplay::Short)),
        "5 smi"
    );
    assert_eq!(format("en", 5.0, &mk("percent", UnitDisplay::Short)), "5%");
    assert_eq!(format("en", 5.0, &mk("stone", UnitDisplay::Short)), "5 st");
    assert_eq!(format("en", 5.0, &mk("yard", UnitDisplay::Long)), "5 yards");
}

/// `unitDisplay: "narrow"` is a distinct width, not an alias for `"short"`.
#[cfg(feature = "units-narrow")]
#[test]
fn unit_style_narrow() {
    use intl::number::{NumberStyle, UnitDisplay, format};
    let mk = |unit, disp| {
        nf(move |o| {
            o.style = NumberStyle::Unit;
            o.unit = Some(unit);
            o.unit_display = disp;
        })
    };
    assert_eq!(
        format("en", 5.0, &mk("kilometer", UnitDisplay::Narrow)),
        "5km"
    );
    assert_eq!(format("en", 3.0, &mk("hour", UnitDisplay::Narrow)), "3h");
    assert_eq!(
        format("de", 5.0, &mk("kilometer", UnitDisplay::Narrow)),
        "5 km"
    );
    assert_eq!(
        format("en", 5.0, &mk("meter-per-second", UnitDisplay::Narrow)),
        "5m/s"
    );
    assert_ne!(
        format("en", 5.0, &mk("kilometer", UnitDisplay::Narrow)),
        format("en", 5.0, &mk("kilometer", UnitDisplay::Short))
    );
}

#[test]
fn compact_long() {
    use intl::number::{CompactDisplay, Notation, format};
    let lo = nf(|o| {
        o.notation = Notation::Compact;
        o.compact_display = CompactDisplay::Long;
    });
    assert_eq!(format("en", 1500.0, &lo), "1.5 thousand");
    assert_eq!(format("en", 2_300_000.0, &lo), "2.3 million");
    // Short remains the default.
    let sh = nf(|o| o.notation = Notation::Compact);
    assert_eq!(format("en", 1500.0, &sh), "1.5K");
}

#[cfg(feature = "currency")]
#[test]
fn currency_display() {
    use intl::number::{CurrencyDisplay, NumberStyle, format};
    let mk = |code, disp| {
        nf(move |o| {
            o.style = NumberStyle::Currency;
            o.currency = Some(code);
            o.currency_display = disp;
        })
    };
    assert_eq!(
        format("en", 1234.5, &mk("USD", CurrencyDisplay::Symbol)),
        "$1,234.50"
    );
    assert_eq!(
        format("en", 1234.5, &mk("USD", CurrencyDisplay::Code)),
        "1,234.50 USD"
    );
    assert_eq!(
        format("en", 1234.5, &mk("USD", CurrencyDisplay::Name)),
        "1,234.50 US dollars"
    );
    assert_eq!(
        format("de", 1234.5, &mk("EUR", CurrencyDisplay::Symbol)),
        "1.234,50\u{a0}€"
    );
    assert_eq!(
        format("ja", 1234.0, &mk("JPY", CurrencyDisplay::Symbol)),
        "￥1,234"
    );
}

/// Non-finite input goes through the same ECMA-402 `∞` / `NaN` spelling on every
/// path, not Rust's `inf`. Regression for the `format_scientific` mantissa loop,
/// which never terminated for an infinity (`inf / 10.0 == inf`).
#[test]
fn non_finite() {
    use intl::number::{format_compact, format_scientific};

    // Plain decimal, both signs. NaN is unsigned per ECMA-402.
    assert_eq!(dec("en", f64::INFINITY), "∞");
    assert_eq!(dec("en", f64::NEG_INFINITY), "-∞");
    assert_eq!(dec("en", f64::NAN), "NaN");
    assert_eq!(dec("en", -f64::NAN), "NaN");

    // The locale minus sign is used, not an ASCII hyphen.
    assert_eq!(dec("sv", f64::NEG_INFINITY), "\u{2212}∞");

    // The pattern affixes survive: percent keeps its suffix, currency its symbol.
    assert_eq!(pct("en", f64::INFINITY), "∞%");
    #[cfg(feature = "currency")]
    {
        assert_eq!(
            intl::number::format_currency("en", f64::INFINITY, "USD"),
            "$∞"
        );
        assert_eq!(
            intl::number::format_currency("en", f64::NEG_INFINITY, "USD"),
            "-$∞"
        );
    }

    // Compact delegates to the decimal path.
    assert_eq!(format_compact("en", f64::INFINITY), "∞");
    assert_eq!(format_compact("en", f64::NAN), "NaN");

    // Scientific: the guard that keeps the normalization loop from spinning.
    assert_eq!(format_scientific("en", f64::INFINITY, 6), "∞");
    assert_eq!(format_scientific("en", f64::NEG_INFINITY, 6), "-∞");
    assert_eq!(format_scientific("sv", f64::NEG_INFINITY, 6), "\u{2212}∞");
    assert_eq!(format_scientific("en", f64::NAN, 6), "NaN");

    // The `format`/`format_to_parts` path already agreed; keep the two in step.
    let o = intl::number::NumberFormatOptions::default();
    assert_eq!(intl::number::format("en", f64::INFINITY, &o), "∞");
    assert_eq!(intl::number::format("en", f64::NAN, &o), "NaN");
}

/// Region and script tags that carry their own CLDR number data. The tables are
/// keyed by the vendored locale, and the lookup truncates a tag at each `-` with
/// no script inference — so `zh-Hant` needs its own record, and `zh-TW` needs an
/// alias onto it or it would fall through to Simplified `zh`.
#[test]
fn script_and_region_locales() {
    use intl::number::format_compact as k;

    // Indian grouping: 2-then-3 digits, from `en-IN`'s `#,##,##0.###` pattern.
    assert_eq!(dec("en-IN", 12345678.0), "1,23,45,678");
    assert_eq!(dec("hi", 12345678.0), "1,23,45,678");
    // Plain `en` and other `en-*` regions keep uniform 3-digit grouping.
    assert_eq!(dec("en", 12345678.0), "12,345,678");
    assert_eq!(dec("en-GB", 12345678.0), "12,345,678");
    // `en-IN` also abbreviates in crores rather than millions.
    assert_eq!(k("en-IN", 123456789.0), "12.3Cr");
    assert_eq!(k("en", 123456789.0), "123.5M");

    // Traditional Chinese uses 億; Simplified uses 亿.
    assert_eq!(k("zh-Hant", 123456789.0), "1.2億");
    assert_eq!(k("zh", 123456789.0), "1.2亿");
    // Region tags CLDR maximizes onto Hant reach the Traditional data...
    assert_eq!(k("zh-TW", 123456789.0), "1.2億");
    assert_eq!(k("zh-HK", 123456789.0), "1.2億");
    assert_eq!(k("zh-MO", 123456789.0), "1.2億");
    // ...and the ones it maximizes onto Hans keep falling back to `zh`.
    assert_eq!(k("zh-CN", 123456789.0), "1.2亿");
    assert_eq!(k("zh-SG", 123456789.0), "1.2亿");
}
