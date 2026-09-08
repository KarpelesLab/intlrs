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
    // German doesn't abbreviate thousands, and compact notation groups `min2`
    // (ECMA-402), so the four digits stay bare: `Intl.NumberFormat('de',
    // {notation: 'compact'}).format(1500)` is "1500".
    assert_eq!(k("de", 1500.0), "1500");
    assert_eq!(k("fr", 1500.0), "1,5\u{a0}k"); // French: NBSP + lowercase k
}

#[test]
fn native_digits() {
    use intl::number::{format_decimal as fdn, to_numbering_system as tns};
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
    // A locale with no block for the requested system inherits *root's* for that
    // system before its own `latn` (UTS #35 resource inheritance): root's `arab`
    // group is U+066C and decimal U+066B, as `Intl.NumberFormat('en-u-nu-arab')`.
    // Root's blocks follow `number-numsys` with the per-locale ones.
    #[cfg(feature = "number-numsys")]
    assert_eq!(f("en-u-nu-arab", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
    #[cfg(not(feature = "number-numsys"))]
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

/// UTS #35 `NumberElements` inheritance for a system the locale does not ship:
/// root's block, then the locale's `latn` — never the locale's `latn` alone.
#[cfg(feature = "number-numsys")]
#[test]
fn numbering_system_root_inheritance() {
    use intl::number::{
        NumberFormatOptions, NumberStyle, format, format_decimal as f, format_percent as p,
    };
    // Root defines real symbols for `arab` and `arabext` only. Every locale
    // without a block of its own gets them, whatever its own `latn` looks like.
    assert_eq!(f("en-US-u-nu-arab", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
    assert_eq!(f("de-u-nu-arab", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
    assert_eq!(f("ja-u-nu-arabext", 1234.5), "۱\u{66c}۲۳۴\u{66b}۵");
    // Root's blocks alias the *decimal* pattern back to the requesting locale's
    // `latn` (`alias source="locale"`), so Indian grouping survives.
    assert_eq!(f("en-IN-u-nu-arab", 1234567.0), "١٢\u{66c}٣٤\u{66c}٥٦٧");
    // Every other system aliases to `latn` outright: no root symbols to inherit.
    assert_eq!(f("de-u-nu-thai", 1234.5), "๑.๒๓๔,๕");
    assert_eq!(f("en-u-nu-thai", 1234.5), "๑,๒๓๔.๕");
    // Root's `arab` percent pattern is its own (suffixed, root's percent sign),
    // so it overrides even a locale that prefixes or spaces its own.
    assert_eq!(p("tr-u-nu-arab", 0.5), "٥٠٪\u{61c}");
    assert_eq!(p("de-u-nu-arab", 0.5), "٥٠٪\u{61c}");
    // `arabext`'s is an alias, so the locale's own pattern shape survives — with
    // root's percent sign substituted into it.
    assert_eq!(p("tr-u-nu-arabext", 0.5), "٪۵۰");
    assert_eq!(p("de-u-nu-arabext", 0.5), "۵۰\u{a0}٪");
    // A locale's own block still wins: `sd`'s `arab` keeps an ASCII decimal.
    assert_eq!(f("sd-u-nu-arab", 1234.5), "١\u{66c}٢٣٤.٥");
    // ICU merges field by field, not block by block: `fa` ships no `arab` block
    // in cldr-json but LDML overrides its percent sign and NaN, so those two
    // come from `fa` and the separators from root.
    assert_eq!(p("fa-u-nu-arab", 0.5), "٥٠٪");
    let mut o = NumberFormatOptions::default();
    o.numbering_system = Some("arab");
    o.style = NumberStyle::Decimal;
    assert_eq!(format("fa", f64::NAN, &o), "ناعدد");
    assert_eq!(format("en", f64::NAN, &o), "NaN"); // root's arab NaN
    // `ur` overrides `arab`'s separators but not its minus sign.
    assert_eq!(f("ur-u-nu-arab", 1234.5), "١,٢٣٤\u{60c}٥");
    assert_eq!(format("ur", -1.0, &o), "\u{61c}-١");
}

/// CLDR gives some region locales a `defaultNumberingSystem` their base language
/// does not have (UTS #35 §3.4); they inherit everything else from it.
#[test]
fn region_default_numbering_system() {
    use intl::number::{default_numbering_system as d, format_decimal as f};
    assert_eq!(d("ar"), "latn");
    assert_eq!(d("ar-EG"), "arab");
    assert_eq!(d("ar-SA"), "arab");
    assert_eq!(d("ar-YE"), "arab");
    assert_eq!(d("ur"), "latn");
    assert_eq!(d("ur-IN"), "arabext");
    // Regions CLDR leaves alone keep the base language's.
    assert_eq!(d("ar-MA"), "latn");
    assert_eq!(d("en-GB"), "latn");

    // `Intl.NumberFormat('ar-EG').format(1.5)` is "١٫٥"; plain `ar` is "1.5".
    #[cfg(feature = "number-numsys")]
    {
        assert_eq!(f("ar-EG", 1.5), "١\u{66b}٥");
        assert_eq!(f("ar-SA", 1.5), "١\u{66b}٥");
        assert_eq!(f("ur-IN", 1.5), "۱\u{66b}۵");
    }
    assert_eq!(f("ar", 1.5), "1.5");
    // The region tag inherits the base language's symbols, so a request for the
    // same system resolves identically for both.
    assert_eq!(f("ar-EG-u-nu-latn", 1234.5), f("ar", 1234.5));
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
    // A lone `$` is one code point, below ICU's AUTO collapse threshold, so it
    // repeats — and the repetition is what makes the separator take spaces.
    assert_eq!(format_range("en", 2.9, 3.1, &cur), "$2.90 \u{2013} $3.10");
    assert_eq!(format_range("en", 2.999, 3.001, &cur), "~$3.00");
    // `code` now lives in the same modifier layer as the symbol, and "USD" plus
    // its spacing space is over the threshold, so it factors out and the
    // separator stays tight — node gives "USD\u{a0}3.00–5.00".
    let mut code = cur;
    code.currency_display = intl::number::CurrencyDisplay::Code;
    assert_eq!(
        format_range("en", 3.0, 5.0, &code),
        "USD\u{a0}3.00\u{2013}5.00"
    );

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
            ("literal", "shared", " \u{2013} "),
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

/// ICU `NumberRangeFormatterImpl::formatRange` at the AUTO collapse level, which
/// is the one `Intl.NumberFormat.prototype.formatRange` uses. Every expectation
/// here is `node --version` v22 / ICU 77.
#[cfg(all(feature = "number-range", feature = "currency"))]
#[test]
fn format_range_collapses_shared_affixes() {
    use intl::number::{
        NumberFormatOptions, NumberStyle, SignDisplay, format_range as r, format_range_to_parts,
    };
    let cur = |code| {
        let mut o = NumberFormatOptions::default();
        o.style = NumberStyle::Currency;
        o.currency = Some(code);
        o
    };

    // The middle modifier (the pattern's affixes plus the sign) is factored out
    // only when both ends render it alike *and* it is longer than one code
    // point. `+$` and `-$` clear that bar; a bare `$`, `%` or `-` does not.
    let mut plus = cur("USD");
    plus.sign_display = SignDisplay::Always;
    assert_eq!(r("en", 2.9, 3.1, &plus), "+$2.90\u{2013}3.10");
    assert_eq!(r("en", -3.0, -1.0, &cur("USD")), "-$3.00\u{2013}1.00");
    assert_eq!(r("en", 3.0, 5.0, &cur("USD")), "$3.00 \u{2013} $5.00");
    let mut always = NumberFormatOptions::default();
    always.sign_display = SignDisplay::Always;
    assert_eq!(r("en", 3.0, 5.0, &always), "+3 \u{2013} +5");
    assert_eq!(r("en", -3.0, -1.0, &Default::default()), "-3 \u{2013} -1");

    // Four locales, four affix positions. de/fr suffix, nl prefix, ja prefix,
    // he both — all two code points or more, so all collapse and the separator
    // stays tight.
    assert_eq!(
        r("de", 3.0, 5.0, &cur("EUR")),
        "3,00\u{2013}5,00\u{a0}\u{20ac}"
    );
    assert_eq!(
        r("fr", 3.0, 5.0, &cur("EUR")),
        "3,00\u{2013}5,00\u{a0}\u{20ac}"
    );
    assert_eq!(r("nl", 3.0, 5.0, &cur("EUR")), "\u{20ac}\u{a0}3,00-5,00");
    assert_eq!(
        r("ja", 3.0, 5.0, &cur("JPY")),
        "\u{ffe5}3 \u{ff5e} \u{ffe5}5"
    ); // ￥ is one code point
    assert_eq!(
        r("he", 3.0, 5.0, &cur("ILS")),
        "\u{200f}3.00\u{2013}5.00\u{a0}\u{200f}\u{20aa}"
    );

    // Prefix *and* suffix count as one modifier, so a percent sign next to a
    // plus sign is over the threshold even though either alone is not.
    let mut pct = NumberFormatOptions::default();
    pct.style = NumberStyle::Percent;
    assert_eq!(r("en", 0.03, 0.05, &pct), "3% \u{2013} 5%");
    assert_eq!(r("tr", 0.03, 0.05, &pct), "%3 \u{2013} %5");
    pct.sign_display = SignDisplay::Always;
    assert_eq!(r("en", 0.03, 0.05, &pct), "+3\u{2013}5%");

    // Modifiers that differ between the ends never collapse, and the spacing
    // heuristic looks at the *first* end only — so a range starting at an
    // unsigned zero stays tight even though the other end gains a plus.
    let mut xz = NumberFormatOptions::default();
    xz.sign_display = SignDisplay::ExceptZero;
    assert_eq!(r("en", 0.0, 5.0, &xz), "0\u{2013}+5");
    assert_eq!(r("en", -5.0, 0.0, &xz), "-5 \u{2013} 0");
    assert_eq!(r("en", -3.0, 5.0, &cur("USD")), "-$3.00 \u{2013} $5.00");

    // The inner modifier — the notation — is never collapsed at this level, and
    // it forces the spacing on its own.
    let mut compact = NumberFormatOptions::default();
    compact.notation = intl::number::Notation::Compact;
    assert_eq!(r("en", 1200.0, 5000.0, &compact), "1.2K \u{2013} 5K");
    let mut sci = NumberFormatOptions::default();
    sci.notation = intl::number::Notation::Scientific;
    assert_eq!(r("en", 1200.0, 5000.0, &sci), "1.2E3 \u{2013} 5E3");

    // maximumFractionDigits reaches the range because it reaches the number:
    // ECMA-402 `SetNumberFormatDigitOptions` pulls the *minimum* down with it.
    let mut whole = cur("USD");
    whole.maximum_fraction_digits = Some(0);
    assert_eq!(r("en", 3.0, 5.0, &whole), "$3 \u{2013} $5");

    // A collapsed affix is `shared`; the parts either side keep their end.
    let parts = format_range_to_parts("de", 3.0, 5.0, &cur("EUR"));
    let tagged: Vec<_> = parts
        .iter()
        .map(|p| (p.kind.as_str(), p.source.as_str(), p.value.as_str()))
        .collect();
    assert_eq!(
        tagged,
        vec![
            ("integer", "startRange", "3"),
            ("decimal", "startRange", ","),
            ("fraction", "startRange", "00"),
            ("literal", "shared", "\u{2013}"),
            ("integer", "endRange", "5"),
            ("decimal", "endRange", ","),
            ("fraction", "endRange", "00"),
            ("literal", "shared", "\u{a0}"),
            ("currency", "shared", "\u{20ac}"),
        ]
    );
}

/// The outer modifier — a unit phrase — always factors out, and is re-worded for
/// the range's own plural category (CLDR `pluralRanges`).
#[cfg(all(feature = "number-range", feature = "units"))]
#[test]
fn format_range_collapses_units() {
    use intl::number::{
        NumberFormatOptions, NumberStyle, SignDisplay, UnitDisplay, format_range as r,
        format_range_to_parts,
    };
    let km = |display| {
        let mut o = NumberFormatOptions::default();
        o.style = NumberStyle::Unit;
        o.unit = Some("kilometer");
        o.unit_display = display;
        o
    };
    assert_eq!(r("en", 3.0, 5.0, &km(UnitDisplay::Short)), "3\u{2013}5 km");
    assert_eq!(r("en", 3.0, 5.0, &km(UnitDisplay::Narrow)), "3\u{2013}5km");
    // `one` + `other` is `other` in en, `one` + `few` is `few` in ru: the shared
    // wording is the range's, not either end's.
    assert_eq!(
        r("en", 1.0, 2.0, &km(UnitDisplay::Long)),
        "1\u{2013}2 kilometers"
    );
    assert_eq!(
        r("ru", 1.0, 2.0, &km(UnitDisplay::Long)),
        "1\u{2013}2 километра"
    );
    assert_eq!(
        r("ru", 2.0, 5.0, &km(UnitDisplay::Long)),
        "2\u{2013}5 километров"
    );
    assert_eq!(
        r("fr", 1.0, 2.0, &km(UnitDisplay::Long)),
        "1\u{2013}2\u{a0}kilomètres"
    );
    // The unit collapses across a sign change (it is parameterized by plural,
    // not by sign), but the sign itself is a repeated one-code-point modifier.
    assert_eq!(
        r("en", -3.0, 5.0, &km(UnitDisplay::Short)),
        "-3 \u{2013} 5 km"
    );
    let mut signed = km(UnitDisplay::Short);
    signed.sign_display = SignDisplay::Always;
    assert_eq!(r("en", 3.0, 5.0, &signed), "+3 \u{2013} +5 km");
    // Equal ends: the approximately sign goes inside the unit, not around it.
    assert_eq!(r("en", 3.0, 3.0, &km(UnitDisplay::Short)), "~3 km");

    let parts = format_range_to_parts("en", 3.0, 5.0, &km(UnitDisplay::Short));
    let tagged: Vec<_> = parts
        .iter()
        .map(|p| (p.kind.as_str(), p.source.as_str(), p.value.as_str()))
        .collect();
    assert_eq!(
        tagged,
        vec![
            ("integer", "startRange", "3"),
            ("literal", "shared", "\u{2013}"),
            ("integer", "endRange", "5"),
            ("literal", "shared", " "),
            ("unit", "shared", "km"),
        ]
    );
}

/// The unit wrapper is ICU's outermost modifier, so it applies to every notation
/// and to the non-finite forms, not only to standard positional notation.
#[cfg(feature = "units")]
#[test]
fn unit_style_outside_standard_notation() {
    use intl::number::{Notation, NumberFormatOptions, NumberStyle, format};
    let mut o = NumberFormatOptions::default();
    o.style = NumberStyle::Unit;
    o.unit = Some("kilometer");
    assert_eq!(format("en", 3.0, &o), "3 km");
    // Compact's own rounding reaches the unit path too: node gives "12K km",
    // not "12.3K km".
    o.notation = Notation::Compact;
    assert_eq!(format("en", 12345.0, &o), "12K km");
    o.notation = Notation::Scientific;
    assert_eq!(format("en", 12345.0, &o), "1.235E4 km");
    o.notation = Notation::Engineering;
    assert!(format("en", 12345.0, &o).ends_with("E3 km"));
    o.notation = Notation::Standard;
    assert_eq!(format("en", f64::NAN, &o), "NaN km");
    assert_eq!(format("en", f64::INFINITY, &o), "∞ km");
    assert_eq!(format("en", f64::NEG_INFINITY, &o), "-∞ km");
    // An unknown unit still degrades to the bare number, in every notation.
    o.unit = Some("zzzz");
    o.notation = Notation::Compact;
    assert_eq!(format("en", 12000.0, &o), "12K");
}

/// ECMA-402 `SetNumberFormatDigitOptions` step 16.a: one of the fraction-digit
/// bounds moves the other rather than being clamped by it.
#[test]
fn fraction_digit_defaults_interact() {
    use intl::number::{NumberFormatOptions, format};
    let mut o = NumberFormatOptions::default();
    o.maximum_fraction_digits = Some(0);
    assert_eq!(format("en", 3.4, &o), "3");
    #[cfg(feature = "currency")]
    {
        use intl::number::{CurrencyDisplay, NumberStyle};
        // The currency's two digits are the *default* minimum, so an explicit
        // maximum of 0 pulls it to 0: `$3`, not `$3.00`.
        let mut c = NumberFormatOptions::default();
        c.style = NumberStyle::Currency;
        c.currency = Some("USD");
        c.maximum_fraction_digits = Some(0);
        assert_eq!(format("en", 3.0, &c), "$3");
        c.currency_display = CurrencyDisplay::Code;
        assert_eq!(format("en", 3.0, &c), "USD\u{a0}3");
        // An explicit minimum above the default raises the maximum instead.
        let mut m = NumberFormatOptions::default();
        m.style = NumberStyle::Currency;
        m.currency = Some("JPY"); // zero-digit currency
        m.minimum_fraction_digits = Some(2);
        assert_eq!(format("en", 3.0, &m), "¥3.00");
    }
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
    // `code` renders through the ¤ pattern, like the symbol — so `en` puts it
    // in front, with the UTS #35 currency-spacing no-break space.
    assert_eq!(
        format("en", 1234.5, &mk("USD", CurrencyDisplay::Code)),
        "USD\u{a0}1,234.50"
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

/// A non-finite value is a placeholder *inside* the style's affixes on the
/// options path too, not a bare "NaN": both entry points build it from the same
/// body/affix pair. node 22 (ICU 77):
/// `new Intl.NumberFormat('en', {style: 'currency', currency: 'USD'})
///  .format(NaN)` is `"$NaN"` and `{style: 'percent'}` on `Infinity` is `"∞%"`.
#[test]
fn non_finite_keeps_style_affixes() {
    use intl::number::{
        Notation, NumberPartType, NumberStyle, SignDisplay, format, format_percent, format_to_parts,
    };
    let percent = nf(|o| o.style = NumberStyle::Percent);
    assert_eq!(format("en", f64::INFINITY, &percent), "∞%");
    assert_eq!(format("en", f64::NAN, &percent), "NaN%");
    assert_eq!(format("de", f64::INFINITY, &percent), "∞\u{a0}%");
    // …and the free-function path, which always kept them, still agrees.
    assert_eq!(format_percent("en", f64::INFINITY), "∞%");
    assert_eq!(
        format_to_parts("en", f64::NAN, &percent)
            .iter()
            .map(|p| p.kind)
            .collect::<Vec<_>>(),
        vec![NumberPartType::Nan, NumberPartType::PercentSign]
    );

    // The notation modifiers drop out (there are no digits to abbreviate), but
    // the style's affixes do not.
    let sci_percent = nf(|o| {
        o.style = NumberStyle::Percent;
        o.notation = Notation::Scientific;
    });
    assert_eq!(format("en", f64::NAN, &sci_percent), "NaN%");
    let compact_percent = nf(|o| {
        o.style = NumberStyle::Percent;
        o.notation = Notation::Compact;
    });
    assert_eq!(format("en", f64::INFINITY, &compact_percent), "∞%");

    // ECMA-402 `PartitionNumberPattern` treats NaN as it treats zero for the
    // sign: `exceptZero` leaves it unsigned, `always` signs it.
    let always = nf(|o| o.sign_display = SignDisplay::Always);
    assert_eq!(format("en", f64::NAN, &always), "+NaN");
    assert_eq!(format("en", f64::INFINITY, &always), "+∞");
    let except_zero = nf(|o| o.sign_display = SignDisplay::ExceptZero);
    assert_eq!(format("en", f64::NAN, &except_zero), "NaN");

    #[cfg(feature = "currency")]
    {
        use intl::number::CurrencyDisplay;
        let usd = nf(|o| {
            o.style = NumberStyle::Currency;
            o.currency = Some("USD");
        });
        assert_eq!(format("en", f64::NAN, &usd), "$NaN");
        assert_eq!(format("en", f64::NEG_INFINITY, &usd), "-$∞");
        assert_eq!(
            intl::number::format_currency("en", f64::NAN, "USD"),
            format("en", f64::NAN, &usd)
        );
        // No currency spacing here: the placeholder does not start with a digit,
        // so UTS #35's `surroundingMatch` fails — node gives "USDNaN" too.
        let code = nf(|o| {
            o.style = NumberStyle::Currency;
            o.currency = Some("USD");
            o.currency_display = CurrencyDisplay::Code;
        });
        assert_eq!(format("en", f64::NAN, &code), "USDNaN");
        assert_eq!(format("en", f64::INFINITY, &code), "USD∞");
    }
    #[cfg(feature = "units")]
    {
        let km = nf(|o| {
            o.style = NumberStyle::Unit;
            o.unit = Some("kilometer");
        });
        assert_eq!(format("en", f64::NAN, &km), "NaN km");
    }
}

/// ECMA-402 `SetNumberFormatDigitOptions` step 15 gives compact notation a
/// rounding default of its own — `roundingPriority: "morePrecision"` over
/// `maximumFractionDigits: 0` and 2 significant digits — so a compact value
/// keeps a tenth only when that is *more* precise than the units place.
/// Expected values from node 22 (ICU 77),
/// `new Intl.NumberFormat(l, o).format(v)`.
#[test]
fn compact_rounding_defaults() {
    use intl::number::{Notation, NumberStyle, format, format_compact as k};
    // 123.456789 million: the units place wins, so no fraction digit.
    assert_eq!(k("en", 123_456_789.0), "123M");
    assert_eq!(k("en", -123_456_789.0), "-123M");
    assert_eq!(k("en", 12345.0), "12K");
    // 1.5 thousand: the tenth is more precise than the units place, so it stays.
    assert_eq!(k("en", 1500.0), "1.5K");
    assert_eq!(k("en", 1234.0), "1.2K");
    // Rounding that carries into the next band re-selects the magnitude, as
    // ICU's second pass does: "1M", not "1000K".
    assert_eq!(k("en", 999_999.0), "1M");
    // Below the smallest band the value is written out, still at compact's
    // precision — `Intl.NumberFormat('en', {notation: 'compact'})` gives "123".
    assert_eq!(k("en", 123.456), "123");
    assert_eq!(k("en", 0.5), "0.5");
    assert_eq!(k("en", 999.0), "999");

    // Explicit precision options override the default, both ways.
    let compact = nf(|o| o.notation = Notation::Compact);
    assert_eq!(format("en", 123_456_789.0, &compact), "123M");
    let mxfd = nf(|o| {
        o.notation = Notation::Compact;
        o.maximum_fraction_digits = Some(1);
    });
    assert_eq!(format("en", 123_456_789.0, &mxfd), "123.5M");
    let mnfd = nf(|o| {
        o.notation = Notation::Compact;
        o.minimum_fraction_digits = Some(2);
    });
    // The style's `mxfdDefault` (3) applies once fraction digits are requested.
    assert_eq!(format("en", 123_456_789.0, &mnfd), "123.457M");
    let mxsd = nf(|o| {
        o.notation = Notation::Compact;
        o.maximum_significant_digits = Some(3);
    });
    assert_eq!(format("en", 123_456_789.0, &mxsd), "123M");

    // The default carries into every style the notation can wrap.
    let percent = nf(|o| {
        o.notation = Notation::Compact;
        o.style = NumberStyle::Percent;
    });
    assert_eq!(format("en", 12345.0, &percent), "1.2M%");
    #[cfg(feature = "currency")]
    {
        let usd = nf(|o| {
            o.notation = Notation::Compact;
            o.style = NumberStyle::Currency;
            o.currency = Some("USD");
        });
        assert_eq!(format("en", 123_456_789.0, &usd), "$123M");
    }

    // Compact also groups `min2` by default (ECMA-402 step 21), which shows in
    // a locale that does not abbreviate thousands: node gives "1500"/"12.345".
    assert_eq!(k("de", 1500.0), "1500");
    assert_eq!(k("de", 12345.0), "12.345");
    // Past the largest band the mantissa runs long and groups by the same rule:
    // node gives "1000T" for 10^15 and "1,000,000T" for 10^18.
    assert_eq!(k("en", 1e15), "1000T");
    assert_eq!(k("en", 1e18), "1,000,000T");
}

/// The scientific/engineering mantissa takes the *style's* fraction-digit
/// defaults, like every other notation — ECMA-402 defaults
/// `maximumFractionDigits` to 3 for the decimal style, to the currency's digits
/// for a currency. node: `new Intl.NumberFormat('en', {notation: 'scientific'})
/// .format(12345.6789)` is `"1.235E4"`.
#[test]
fn scientific_default_fraction_digits() {
    use intl::number::{Notation, NumberStyle, format, format_scientific};
    let sci = nf(|o| o.notation = Notation::Scientific);
    assert_eq!(format("en", 12345.6789, &sci), "1.235E4");
    assert_eq!(format("en", 12345.0, &sci), "1.235E4");
    let eng = nf(|o| o.notation = Notation::Engineering);
    assert_eq!(format("en", 12345.6789, &eng), "12.346E3");

    // An explicit maximum still wins.
    let wide = nf(|o| {
        o.notation = Notation::Scientific;
        o.maximum_fraction_digits = Some(6);
    });
    assert_eq!(format("en", 12345.6789, &wide), "1.234568E4");
    let sig = nf(|o| {
        o.notation = Notation::Scientific;
        o.maximum_significant_digits = Some(2);
    });
    assert_eq!(format("en", 12345.6789, &sig), "1.2E4");

    // The style's own defaults reach the mantissa: percent rounds to 0 fraction
    // digits (and scales by 100), a currency to its two.
    let percent = nf(|o| {
        o.notation = Notation::Scientific;
        o.style = NumberStyle::Percent;
    });
    assert_eq!(format("en", 12345.6789, &percent), "1E6%");
    #[cfg(feature = "currency")]
    {
        let usd = nf(|o| {
            o.notation = Notation::Scientific;
            o.style = NumberStyle::Currency;
            o.currency = Some("USD");
        });
        assert_eq!(format("en", 12345.6789, &usd), "$1.23E4");
    }

    // The free function takes its mantissa width as an argument and is unchanged.
    assert_eq!(format_scientific("en", 12345.6789, 6), "1.234568E4");
    assert_eq!(format_scientific("en", 12345.6789, 3), "1.235E4");
}

/// `currencyDisplay: "code"` substitutes the ISO code into the locale's `¤`
/// pattern, exactly as a symbol — so `en` writes it in front — with the UTS #35
/// `currencySpacing` no-break space between an alphabetic currency and a digit.
/// Only `"name"` goes through the currency *unit* pattern. node:
/// `new Intl.NumberFormat('en', {style: 'currency', currency: 'USD',
/// currencyDisplay: 'code'}).format(3)` is `"USD\u{a0}3.00"`.
#[cfg(feature = "currency")]
#[test]
fn currency_display_code_uses_the_currency_pattern() {
    use intl::number::{
        CurrencyDisplay, NumberPartType, NumberStyle, format, format_currency, format_to_parts,
    };
    let code = |lang, c| {
        format(
            lang,
            3.0,
            &nf(|o| {
                o.style = NumberStyle::Currency;
                o.currency = Some(c);
                o.currency_display = CurrencyDisplay::Code;
            }),
        )
    };
    assert_eq!(code("en", "USD"), "USD\u{a0}3.00");
    assert_eq!(code("ja", "USD"), "USD\u{a0}3.00");
    // A pattern that already ends in a space inserts none: its `¤`-adjacent
    // character is a space, which `currencyMatch` excludes.
    assert_eq!(code("de", "USD"), "3,00\u{a0}USD");
    assert_eq!(code("fr", "USD"), "3,00\u{a0}USD");
    assert_eq!(code("ja", "JPY"), "JPY\u{a0}3"); // and the currency's digits

    let opts = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
        o.currency_display = CurrencyDisplay::Code;
    });
    assert_eq!(format("en", -3.0, &opts), "-USD\u{a0}3.00");
    // ICU tags the inserted space as a literal, between currency and integer.
    let parts = format_to_parts("en", 3.0, &opts);
    assert_eq!(
        parts
            .iter()
            .map(|p| (p.kind, p.value.as_str()))
            .take(3)
            .collect::<Vec<_>>(),
        vec![
            (NumberPartType::Currency, "USD"),
            (NumberPartType::Literal, "\u{a0}"),
            (NumberPartType::Integer, "3"),
        ]
    );

    // The same spacing applies to an alphabetic *symbol*, on both paths — `en`'s
    // symbol for SEK is "SEK" and node gives "SEK\u{a0}3.00".
    let sek = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("SEK");
    });
    assert_eq!(format("en", 3.0, &sek), "SEK\u{a0}3.00");
    assert_eq!(format_currency("en", 3.0, "SEK"), "SEK\u{a0}3.00");
    // …and never to a symbol that is a symbol.
    assert_eq!(format_currency("en", 3.0, "USD"), "$3.00");

    // `name` still renders through the currency unit pattern.
    let name = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
        o.currency_display = CurrencyDisplay::Name;
    });
    assert_eq!(format("en", 3.0, &name), "3.00 US dollars");
}

/// The numbering system defaults to the locale's CLDR `defaultNumberingSystem`,
/// as ECMA-402 does — `Intl.NumberFormat('ar-EG').format(1234.5)` is
/// "١٬٢٣٤٫٥". Thirty of the vendored locales have a non-`latn` default: the
/// eight languages below plus the twenty-one `ar-*` regions and `ur-IN`.
#[cfg(feature = "number-numsys")]
#[test]
fn default_numbering_system_follows_the_locale() {
    use intl::number::{format, format_decimal as f};
    assert_eq!(f("ar-EG", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
    assert_eq!(f("ar-SA", 1234.5), "١\u{66c}٢٣٤\u{66b}٥");
    assert_eq!(f("fa", 1234.5), "۱\u{66c}۲۳۴\u{66b}۵");
    assert_eq!(f("ps", 1234.5), "۱\u{66c}۲۳۴\u{66b}۵");
    assert_eq!(f("ur-IN", 1234.5), "۱\u{66c}۲۳۴\u{66b}۵");
    assert_eq!(f("bn", 1234.5), "১,২৩৪.৫");
    assert_eq!(f("as", 1234.5), "১,২৩৪.৫");
    assert_eq!(f("mr", 1234.5), "१,२३४.५");
    assert_eq!(f("ne", 1234.5), "१,२३४.५");
    assert_eq!(f("my", 1234.5), "၁,၂၃၄.၅");
    // `sd`'s arab block keeps an ASCII decimal separator; node agrees.
    assert_eq!(f("sd", 1234.5), "١\u{66c}٢٣٤.٥");

    // The other 95 keep Latin digits, `ar` and `hi` included (their CLDR
    // default is `latn`, whatever their native system).
    assert_eq!(f("ar", 1234.5), "1,234.5");
    assert_eq!(f("hi", 1234.5), "1,234.5");
    assert_eq!(f("en", 1234.5), "1,234.5");
    assert_eq!(f("ur", 1234.5), "1,234.5");

    // Both overrides still outrank it, in ECMA-402's order.
    assert_eq!(f("ar-EG-u-nu-latn", 1234.5), "1,234.5");
    let latn = nf(|o| o.numbering_system = Some("latn"));
    assert_eq!(format("ar-EG-u-nu-arab", 1234.5, &latn), "1,234.5");

    // It reaches every entry point, not just the decimal one.
    assert_eq!(pct("ar-EG", 0.5), "٥٠٪\u{61c}");
    assert_eq!(
        intl::number::format_compact("ar-EG", 123_456_789.0),
        "١٢٣\u{a0}مليون"
    );
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
    assert_eq!(k("en-IN", 123456789.0), "12Cr");
    assert_eq!(k("en", 123456789.0), "123M");

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

/// CLDR `numbers/minimumGroupingDigits` gates the separator: grouping only
/// appears once that many digits sit before the first group. It is 2 in
/// `pl`/`es`/`et`/`lv`, so they write 1000 unseparated but 10000 grouped, and 1
/// in `en`/`de`/`pt`/`cs`/`fi`, which group from 1000. Values match V8/ICU.
#[test]
fn minimum_grouping_digits() {
    // minimumGroupingDigits = 2.
    for lang in ["pl", "et", "lv"] {
        assert_eq!(dec(lang, 1000.0), "1000", "{lang}");
        assert_eq!(dec(lang, 10000.0), "10\u{a0}000", "{lang}");
        assert_eq!(dec(lang, 100000.0), "100\u{a0}000", "{lang}");
    }
    assert_eq!(dec("es", 1000.0), "1000");
    assert_eq!(dec("es", 10000.0), "10.000");

    // minimumGroupingDigits = 1 — note `pt`/`cs`/`fi` are 1 in CLDR 48, so they
    // group from 1000 despite sharing the "2" reputation.
    assert_eq!(dec("en", 1000.0), "1,000");
    assert_eq!(dec("de", 1000.0), "1.000");
    assert_eq!(dec("pt", 1000.0), "1.000");
    assert_eq!(dec("cs", 1000.0), "1\u{a0}000");
    assert_eq!(dec("fi", 1000.0), "1\u{a0}000");
    // Indian grouping is unaffected: `hi` is 1, and its secondary group still
    // applies past the first.
    assert_eq!(dec("hi", 100000.0), "1,00,000");

    // It gates every style, not just plain decimals.
    assert_eq!(pct("pl", 10.0), "1000%");
    #[cfg(feature = "currency")]
    assert_eq!(
        intl::number::format_currency("pl", 1000.0, "PLN"),
        "1000,00\u{a0}zł"
    );

    // `useGrouping` still overrides the locale in both directions.
    let always = nf(|o| o.use_grouping = intl::number::UseGrouping::Always);
    let never = nf(|o| o.use_grouping = intl::number::UseGrouping::Never);
    assert_eq!(intl::number::format("pl", 1000.0, &always), "1\u{a0}000");
    assert_eq!(intl::number::format("pl", 1000.0, &never), "1000");
}

/// ECMA-402's default `roundingMode` is `halfExpand` — ties away from zero —
/// where this crate used to round half to even. Values from node 26 / ICU 78.3:
/// `Intl.NumberFormat("en", {maximumFractionDigits: 0}).format(2.5)` is `"3"`,
/// `.format(0.5)` is `"1"`, `.format(-2.5)` is `"-3"`, and
/// `Intl.NumberFormat("en", {maximumSignificantDigits: 2}).format(1.25)` is
/// `"1.3"` — so the significant-digit and compact paths take the same tie-break.
#[test]
fn default_rounding_mode_is_half_expand() {
    use intl::number::{Notation, RoundingMode, format};
    let frac0 = nf(|o| o.maximum_fraction_digits = Some(0));
    assert_eq!(format("en", 2.5, &frac0), "3");
    assert_eq!(format("en", 0.5, &frac0), "1");
    assert_eq!(format("en", 1.5, &frac0), "2");
    assert_eq!(format("en", -2.5, &frac0), "-3");
    assert_eq!(format("en", -0.5, &frac0), "-1");

    // Significant digits round the same way: "1.3" / "1.4", not half-even's
    // "1.2" / "1.4".
    let sig2 = nf(|o| o.maximum_significant_digits = Some(2));
    assert_eq!(format("en", 1.25, &sig2), "1.3");
    assert_eq!(format("en", 1.35, &sig2), "1.4");

    // And so does compact's `morePrecision` pass: node's
    // `{notation: "compact"}` on 1250 is `"1.3K"`, on `de` -1234.5 `"-1235"`.
    let compact = nf(|o| o.notation = Notation::Compact);
    assert_eq!(format("en", 1250.0, &compact), "1.3K");
    assert_eq!(format("de", -1234.5, &compact), "-1235");

    // An explicit mode still wins.
    let half_even = nf(|o| {
        o.maximum_fraction_digits = Some(0);
        o.rounding_mode = RoundingMode::HalfEven;
    });
    assert_eq!(format("en", 2.5, &half_even), "2");
    assert_eq!(RoundingMode::default(), RoundingMode::HalfExpand);
}

/// The free functions round at the same boundary as `format`: they used to go
/// through the float formatter, whose ties are half-even. Node:
/// `Intl.NumberFormat("en", {style: "currency", currency: "JPY"}).format(2.5)`
/// is `"¥3"` and `("ja", ...).format(1234.5)` is `"￥1,235"`; `fa`'s IRR (which
/// CLDR gives 0 fraction digits) writes `"\u{200e}ریال\u{a0}۱٬۲۳۵"`.
#[cfg(feature = "currency")]
#[test]
fn currency_ties_round_away_from_zero() {
    use intl::number::{NumberStyle, format, format_currency};
    let mk = |code: &'static str| {
        nf(move |o| {
            o.style = NumberStyle::Currency;
            o.currency = Some(code);
        })
    };
    assert_eq!(format("en", 2.5, &mk("JPY")), "¥3");
    assert_eq!(format("ja", 1234.5, &mk("JPY")), "￥1,235");
    assert_eq!(format_currency("en", 2.5, "JPY"), "¥3");
    // `fa`'s `arabext` group separator needs the per-system blocks; without
    // `number-numsys` the digits are Persian but the separators stay `latn`.
    #[cfg(feature = "number-numsys")]
    assert_eq!(
        format("fa", 1234.5, &mk("IRR")),
        "\u{200e}ریال\u{a0}\u{6f1}\u{66c}\u{6f2}\u{6f3}\u{6f5}"
    );
}

/// The exponent separator is the numbering system's CLDR `symbols/exponential`,
/// not a hard-coded `E`. Node `{notation: "scientific"}` on 12345:
/// `en` `"1.235E4"`, `sv` `"1,235×10^4"`, `el` `"1,235e4"`, `uk` `"1,235Е4"`
/// (Cyrillic Е), `ar` `"1.235E4"` but `ar-EG` `"١٫٢٣٥أس٤"` — the difference
/// being that `ar` defaults to `latn` and `ar-EG` to `arab`.
#[test]
fn scientific_exponent_separator_is_localized() {
    use intl::number::{Notation, format};
    let sci = nf(|o| o.notation = Notation::Scientific);
    assert_eq!(format("en", 12345.0, &sci), "1.235E4");
    assert_eq!(format("sv", 12345.0, &sci), "1,235×10^4");
    assert_eq!(format("el", 12345.0, &sci), "1,235e4");
    assert_eq!(format("uk", 12345.0, &sci), "1,235\u{415}4");
    assert_eq!(format("ar", 12345.0, &sci), "1.235E4");
    // Engineering notation shares the separator.
    let eng = nf(|o| o.notation = Notation::Engineering);
    assert_eq!(format("sv", 12345.0, &eng), "12,345×10^3");
}

/// The per-numbering-system blocks carry their own exponential symbol, so the
/// locale's default system changes it. Node: `ar-EG` (default `arab`) is
/// `"١٫٢٣٥أس٤"` and `fa` (default `arabext`) `"۱٫۲۳۵×۱۰^۴"`.
#[cfg(feature = "number-numsys")]
#[test]
fn exponent_separator_follows_the_numbering_system() {
    use intl::number::{Notation, format};
    let sci = nf(|o| o.notation = Notation::Scientific);
    assert_eq!(
        format("ar-EG", 12345.0, &sci),
        "\u{661}\u{66b}\u{662}\u{663}\u{665}أس\u{664}"
    );
    assert_eq!(
        format("fa", 12345.0, &sci),
        "\u{6f1}\u{66b}\u{6f2}\u{6f3}\u{6f5}×\u{6f1}\u{6f0}^\u{6f4}"
    );
}

/// Compact patterns are keyed by plural category, and by UTS #35 §3.5's
/// explicit `count="1"` where a locale has one. Node
/// `{notation: "compact", compactDisplay: "long"}`: `fr` 1000 is `"mille"`
/// (its `1000-count-1` is that bare word, with no `{0}`), 1500 `"1,5 millier"`
/// (`count-one`) and 2000 `"2 mille"` (`count-other`); `de` 999999 rounds into
/// the million band as `"1 Million"` against 2e6's `"2 Millionen"`; `ru` 12345
/// is `"12 тысяч"` (`many`) and `ro` 12345 `"12 mii"` (`few`).
#[test]
fn compact_patterns_follow_the_plural_category() {
    use intl::number::{CompactDisplay, Notation, format, format_compact};
    let long = nf(|o| {
        o.notation = Notation::Compact;
        o.compact_display = CompactDisplay::Long;
    });
    assert_eq!(format("fr", 1000.0, &long), "mille");
    assert_eq!(format("fr", 1500.0, &long), "1,5 millier");
    assert_eq!(format("fr", 2000.0, &long), "2 mille");
    assert_eq!(format("de", 999_999.0, &long), "1 Million");
    assert_eq!(format("de", 2e6, &long), "2 Millionen");
    assert_eq!(format("ru", 12345.0, &long), "12 тысяч");
    assert_eq!(format("ru", 1e10, &long), "10 миллиардов");
    assert_eq!(format("ro", 12345.0, &long), "12 mii");

    // Short forms too: `bn`'s `10000000000-count-one` carries a space its
    // `count-other` lacks, which node reproduces as "১ শত কো" / "২শত কো".
    assert_eq!(format_compact("bn", 1e10), "\u{9e7}\u{a0}শত\u{a0}কো");
    assert_eq!(format_compact("bn", 2e10), "\u{9e8}শত\u{a0}কো");
}

/// A compact pattern may carry a negative subpattern of its own after `;`, which
/// places the sign rather than leaving it in front. `sw`'s long forms do, and
/// node writes `"bilioni -1"` for -1e9 against `"bilioni 1"` — while its short
/// `"0B;-0B"` puts the sign back where the default would.
#[test]
fn compact_negative_subpattern() {
    use intl::number::{CompactDisplay, Notation, format};
    let long = nf(|o| {
        o.notation = Notation::Compact;
        o.compact_display = CompactDisplay::Long;
    });
    let short = nf(|o| o.notation = Notation::Compact);
    assert_eq!(format("sw", -1e9, &long), "bilioni -1");
    assert_eq!(format("sw", 1e9, &long), "bilioni 1");
    assert_eq!(format("sw", -1e9, &short), "-1B");
}

/// Compact notation replaces the style's affixes with the compact pattern's, so
/// a percent loses the `percentFormat` `%` and ICU words it through the CLDR
/// `concentr-percent` short unit instead — a different string *and* a different
/// spacing. Node `{style: "percent", notation: "compact"}` on 123.45:
/// `da` `"12\u{a0}t pct."`, `de` `"12.345 %"` (plain space where its pattern
/// uses U+00A0), `ro` `"12\u{a0}K%"` (no space where its pattern has one),
/// `tr` `"%12\u{a0}B"` — with the sign *inside* the affix, `"%-50"` for -0.5,
/// because the unit sits outside the sign.
#[test]
fn compact_percent_uses_the_percent_unit_pattern() {
    use intl::number::{Notation, NumberStyle, format};
    let pctc = nf(|o| {
        o.style = NumberStyle::Percent;
        o.notation = Notation::Compact;
    });
    assert_eq!(format("da", 123.45, &pctc), "12\u{a0}t pct.");
    assert_eq!(format("de", 123.45, &pctc), "12.345 %");
    assert_eq!(format("en", 123.45, &pctc), "12K%");
    assert_eq!(format("ro", 123.45, &pctc), "12\u{a0}K%");
    assert_eq!(format("uk", 123.45, &pctc), "12\u{a0}тис. %");
    assert_eq!(format("tr", 123.45, &pctc), "%12\u{a0}B");
    assert_eq!(format("tr", -0.5, &pctc), "%-50");
    assert_eq!(format("ar", -0.5, &pctc), "\u{200e}-50\u{66a}");

    // It applies below the smallest compact band too, where the value is
    // written out in full: node's `de` is "50 %" compact but "50\u{a0}%" plain.
    assert_eq!(format("de", 0.5, &pctc), "50 %");
    assert_eq!(pct("de", 0.5), "50\u{a0}%");
    assert_eq!(format("uk", 0.5, &pctc), "50 %");
    assert_eq!(pct("uk", 0.5), "50%");

    // The unit is plural-selected: `is` is the one locale in CLDR 48 whose
    // short percent differs by category ("1%" for one, "2 %" for other).
    assert_eq!(format("is", 0.01, &pctc), "1%");
    assert_eq!(format("is", 0.02, &pctc), "2 %");
}

/// The symbol a compact percent picks up is tagged `unit`, not `percentSign` —
/// it comes from the unit rather than from the number pattern. Node's
/// `Intl.NumberFormat("da", {style: "percent", notation: "compact"})
/// .formatToParts(-12345)` ends `... compact("mio.") literal(" ")
/// unit("pct.")`.
///
/// Only the last two are asserted: ICU splits the space *before* a compact
/// suffix into its own `literal` part where this crate keeps it inside the
/// `compact` one, which is a separate (pre-existing) `formatToParts` difference
/// and not what the percent unit changes.
#[test]
fn compact_percent_parts_tag_the_symbol_as_a_unit() {
    use intl::number::{Notation, NumberPartType, NumberStyle, format_to_parts};
    let pctc = nf(|o| {
        o.style = NumberStyle::Percent;
        o.notation = Notation::Compact;
    });
    let parts = format_to_parts("da", -12345.0, &pctc);
    let tail: Vec<(NumberPartType, &str)> = parts
        .iter()
        .rev()
        .take(2)
        .rev()
        .map(|p| (p.kind, p.value.as_str()))
        .collect();
    assert_eq!(
        tail,
        [
            (NumberPartType::Literal, " "),
            (NumberPartType::Unit, "pct."),
        ]
    );
}

/// CLDR currency patterns can carry an explicit negative subpattern after `;`,
/// and it places the sign itself. Node `{style: "currency", currency: "USD"}` on
/// -1234.5: `nl` `"US$\u{a0}-1.234,50"` (sign after the symbol, not before it),
/// `de-CH` `"$-1'234.50"` (which also drops the positive pattern's space),
/// `es-CL` `"US$-1.234,50"`, and `he`
/// `"\u{200f}\u{200e}-1,234.50\u{a0}\u{200f}$"` — the pattern's leading RLM
/// ahead of the sign, where synthesising it put the sign first. `en` has no
/// negative subpattern and keeps `"-$1,234.50"`.
#[cfg(feature = "currency")]
#[test]
fn currency_negative_subpattern() {
    use intl::number::{NumberStyle, SignDisplay, format, format_currency};
    let usd = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
    });
    assert_eq!(format("nl", -1234.5, &usd), "US$\u{a0}-1.234,50");
    assert_eq!(format("nl", 1234.5, &usd), "US$\u{a0}1.234,50");
    assert_eq!(format("de-CH", -1234.5, &usd), "$-1'234.50");
    assert_eq!(format("de-CH", 1234.5, &usd), "$\u{a0}1'234.50");
    assert_eq!(format("es-CL", -1234.5, &usd), "US$-1.234,50");
    assert_eq!(
        format("he", -1234.5, &usd),
        "\u{200f}\u{200e}-1,234.50\u{a0}\u{200f}$"
    );
    assert_eq!(
        format("he", 1234.5, &usd),
        "\u{200f}1,234.50\u{a0}\u{200f}$"
    );
    assert_eq!(format("en", -1234.5, &usd), "-$1,234.50");
    // The free function shares the pattern, so it shares the placement.
    assert_eq!(format_currency("nl", -1234.5, "USD"), "US$\u{a0}-1.234,50");

    // ICU reuses the negative subpattern for a shown `+` (its
    // `PatternSignType::POS_SIGN`) and the positive one when no sign is shown:
    // node writes `"US$ +1.234,50"` and `"US$ 1.234,50"`.
    let always = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
        o.sign_display = SignDisplay::Always;
    });
    let never = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
        o.sign_display = SignDisplay::Never;
    });
    assert_eq!(format("nl", 1234.5, &always), "US$\u{a0}+1.234,50");
    assert_eq!(format("nl", -1234.5, &never), "US$\u{a0}1.234,50");
    assert_eq!(format("de-CH", 1234.5, &always), "$+1'234.50");
}

/// The negative subpattern belongs to the ¤ affixes, so it applies wherever they
/// do and nowhere else. Node, for `nl` USD: `"US$ -∞"` keeps it and so does
/// scientific notation, `currencyDisplay: "name"` drops it with the affixes
/// (`"-1.234,50 Amerikaanse dollar"`), and compact notation drops it because the
/// compact pattern replaces them and has none (`"-US$ 1,2 mln."`).
#[cfg(feature = "currency")]
#[test]
fn currency_negative_subpattern_scope() {
    use intl::number::{CurrencyDisplay, Notation, NumberFormatOptions, NumberStyle, format};
    let base = |build: fn(&mut NumberFormatOptions)| {
        nf(|o| {
            o.style = NumberStyle::Currency;
            o.currency = Some("USD");
            build(o);
        })
    };
    let usd = base(|_| {});
    assert_eq!(format("nl", f64::NEG_INFINITY, &usd), "US$\u{a0}-∞");
    assert_eq!(format("nl", f64::NAN, &usd), "US$\u{a0}NaN");
    // (The mantissa keeps the currency's two fraction digits, where ICU uses the
    // decimal pattern's — an unrelated divergence; the sign placement is what
    // this asserts.)
    let sci = base(|o| o.notation = Notation::Scientific);
    assert_eq!(format("nl", -1000.0, &sci), "US$\u{a0}-1,00E3");
    let compact = base(|o| o.notation = Notation::Compact);
    assert_eq!(
        format("nl", -1_234_500.0, &compact),
        "-US$\u{a0}1,2\u{a0}mln."
    );
    let name = base(|o| o.currency_display = CurrencyDisplay::Name);
    assert_eq!(format("nl", -1234.5, &name), "-1.234,50 Amerikaanse dollar");
}

/// `currencyFormats` is per numbering system, and the blocks differ in ways a
/// `latn`-only table cannot express: `ar`'s `arab` pattern has *no* negative
/// subpattern where its `latn` one does, so node writes `ar` (default `latn`)
/// as `"\u{200f}\u{200e}-1,234.50\u{a0}US$"` but `ar-EG` (default `arab`) as
/// `"\u{61c}-\u{200f}١٬٢٣٤٫٥٠\u{a0}US$"`, with the sign in front. `fa`'s
/// `arabext` pattern drops the space its `latn` one puts after the symbol:
/// `"\u{200e}$۱٬۲۳۴٫۵۰"`.
#[cfg(all(feature = "currency", feature = "number-numsys"))]
#[test]
fn currency_pattern_follows_the_numbering_system() {
    use intl::number::{NumberStyle, format};
    let usd = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
    });
    assert_eq!(
        format("ar", -1234.5, &usd),
        "\u{200f}\u{200e}-1,234.50\u{a0}US$"
    );
    assert_eq!(
        format("ar-EG", -1234.5, &usd),
        "\u{61c}-\u{200f}\u{661}\u{66c}\u{662}\u{663}\u{664}\u{66b}\u{665}\u{660}\u{a0}US$"
    );
    assert_eq!(
        format("fa", 1234.5, &usd),
        "\u{200e}$\u{6f1}\u{66c}\u{6f2}\u{6f3}\u{6f4}\u{66b}\u{6f5}\u{6f0}"
    );
}

/// The per-numbering-system currency pattern is the *standard* pattern, and
/// compact notation does not use it. ICU feeds compact currency from CLDR's
/// `currencyFormats/short/standard` patterns, a separate table that is not
/// split per numbering system, so the affixes can disagree with the system's
/// standard pattern — and where they do, the `latn` pattern is what they match.
///
/// `fa`'s `arabext` standard pattern drops the space after the symbol, but both
/// its short blocks keep it, so node compacts to `"\u{200e}$\u{a0}۱٫۲ هزار"`
/// with the space back. `sd` is starker: its `arab` standard pattern puts the
/// symbol *last*, and CLDR ships no `arab` short block at all, so ICU reads the
/// `latn` one and node writes the symbol first, `"US$\u{a0}١ هزار"`.
///
/// Below the smallest compact band nothing abbreviates and the ordinary pattern
/// applies again — `ar-EG` at -1 keeps its `arab` sign placement.
#[cfg(all(feature = "currency", feature = "number-numsys"))]
#[test]
fn compact_currency_does_not_use_the_numbering_system_pattern() {
    use intl::number::{Notation, NumberStyle, format};
    let usd = nf(|o| {
        o.style = NumberStyle::Currency;
        o.currency = Some("USD");
        o.notation = Notation::Compact;
    });
    // fa: the space the `arabext` standard pattern drops is back.
    assert_eq!(
        format("fa", 1234.5, &usd),
        "\u{200e}$\u{a0}\u{6f1}\u{66b}\u{6f2}\u{a0}هزار"
    );
    // sd: symbol first, as the `latn` short block has it — not last, as the
    // `arab` standard pattern would put it.
    assert_eq!(format("sd", 1000.0, &usd), "US$\u{a0}\u{661}\u{a0}هزار");
    // Under the band, the numbering system's own pattern still decides.
    assert_eq!(
        format("ar-EG", -1.0, &usd),
        "\u{61c}-\u{200f}\u{661}\u{a0}US$"
    );
}

/// UTS #35 §3.5's explicit `count="1"` matches the literal *signed* value 1, so
/// a negative mantissa falls through to the plural category. `fr` has both
/// `1000-count-1` ("mille") and `1000-count-one` ("0 millier"), and node
/// compacts 1000 to `"mille"` but -1000 to `"-1 millier"` — including under
/// `signDisplay: "never"`, which shows it is the value and not the printed sign
/// that decides. `it` has no `count-1` at all; its `1000-count-one` is the bare
/// word, so -1000 really is `"-mille"` there.
#[test]
fn explicit_count_one_is_signed() {
    use intl::number::{CompactDisplay, Notation, SignDisplay, format};
    let long = nf(|o| {
        o.notation = Notation::Compact;
        o.compact_display = CompactDisplay::Long;
    });
    assert_eq!(format("fr", 1000.0, &long), "mille");
    assert_eq!(format("fr", -1000.0, &long), "-1 millier");
    assert_eq!(format("fr", 1500.0, &long), "1,5 millier");
    assert_eq!(format("fr", 2000.0, &long), "2 mille");
    // Not the printed sign: hiding it does not bring `count-1` back.
    let never = nf(|o| {
        o.notation = Notation::Compact;
        o.compact_display = CompactDisplay::Long;
        o.sign_display = SignDisplay::Never;
    });
    assert_eq!(format("fr", -1000.0, &never), "1 millier");
    // `it` reaches the bare word through `count-one`, so its negative keeps it.
    assert_eq!(format("it", -1000.0, &long), "-mille");
}

/// `fr-CA`'s short `concentr-percent` pattern is `"{0}\u{a0}%"` where `fr`'s is
/// `"{0} %"`, and compact percent formats through that unit pattern — so the
/// bundle is vendored on top of the base language, as `zh-Hant`'s is. Node:
/// `fr-CA` `{style: "percent", notation: "compact"}` on 12345 is
/// `"1,2\u{a0}M\u{a0}%"`, against `fr`'s `"1,2\u{a0}M %"`.
#[test]
fn compact_percent_fr_ca_keeps_its_no_break_space() {
    use intl::number::{Notation, NumberStyle, format};
    let o = nf(|o| {
        o.style = NumberStyle::Percent;
        o.notation = Notation::Compact;
    });
    assert_eq!(format("fr-CA", 12345.0, &o), "1,2\u{a0}M\u{a0}%");
    assert_eq!(format("fr-CA", 0.5, &o), "50\u{a0}%");
    assert_eq!(format("fr", 12345.0, &o), "1,2\u{a0}M %");
}
