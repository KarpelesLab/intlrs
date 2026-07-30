//! Locale-aware list formatting.
#![cfg(feature = "list")]

use intl::list::{ListFormatOptions, ListType, ListType::*, ListWidth, ListWidth::*, format_list};

/// `format_list` with the two ECMA-402 axes spelled out, for the tables below.
fn fl(lang: &str, items: &[&str], list_type: ListType, width: ListWidth) -> String {
    let mut o = ListFormatOptions::default();
    o.list_type = list_type;
    o.width = width;
    format_list(lang, items, &o)
}

/// Every `type` × `style` combination across seven locales, from
/// `new Intl.ListFormat(loc, {type, style}).format([...])` on node 22 / ICU 77.
/// The nine are independent data: `es` unit-long keeps its "y" where `en`
/// unit-long drops the "and", `en` conjunction-short is "&", and `ja` unit-narrow
/// has no separator at all.
#[test]
fn all_nine_type_style_combinations() {
    #[rustfmt::skip]
    const CASES: &[(&str, ListType, ListWidth, &[&str], &str)] = &[
    ("en", Conjunction, Long, &["a", "b"], "a and b"),
    ("en", Conjunction, Long, &["a", "b", "c"], "a, b, and c"),
    ("en", Conjunction, Short, &["a", "b"], "a & b"),
    ("en", Conjunction, Short, &["a", "b", "c"], "a, b, & c"),
    ("en", Conjunction, Narrow, &["a", "b"], "a, b"),
    ("en", Conjunction, Narrow, &["a", "b", "c"], "a, b, c"),
    ("en", Disjunction, Long, &["a", "b"], "a or b"),
    ("en", Disjunction, Long, &["a", "b", "c"], "a, b, or c"),
    ("en", Disjunction, Short, &["a", "b"], "a or b"),
    ("en", Disjunction, Short, &["a", "b", "c"], "a, b, or c"),
    ("en", Disjunction, Narrow, &["a", "b"], "a or b"),
    ("en", Disjunction, Narrow, &["a", "b", "c"], "a, b, or c"),
    ("en", Unit, Long, &["a", "b"], "a, b"),
    ("en", Unit, Long, &["a", "b", "c"], "a, b, c"),
    ("en", Unit, Short, &["a", "b"], "a, b"),
    ("en", Unit, Short, &["a", "b", "c"], "a, b, c"),
    ("en", Unit, Narrow, &["a", "b"], "a b"),
    ("en", Unit, Narrow, &["a", "b", "c"], "a b c"),
    ("es", Conjunction, Long, &["a", "b"], "a y b"),
    ("es", Conjunction, Long, &["a", "b", "c"], "a, b y c"),
    ("es", Conjunction, Short, &["a", "b"], "a y b"),
    ("es", Conjunction, Short, &["a", "b", "c"], "a, b y c"),
    ("es", Conjunction, Narrow, &["a", "b"], "a y b"),
    ("es", Conjunction, Narrow, &["a", "b", "c"], "a, b y c"),
    ("es", Disjunction, Long, &["a", "b"], "a o b"),
    ("es", Disjunction, Long, &["a", "b", "c"], "a, b o c"),
    ("es", Disjunction, Short, &["a", "b"], "a o b"),
    ("es", Disjunction, Short, &["a", "b", "c"], "a, b o c"),
    ("es", Disjunction, Narrow, &["a", "b"], "a o b"),
    ("es", Disjunction, Narrow, &["a", "b", "c"], "a, b o c"),
    ("es", Unit, Long, &["a", "b"], "a y b"),
    ("es", Unit, Long, &["a", "b", "c"], "a, b y c"),
    ("es", Unit, Short, &["a", "b"], "a y b"),
    ("es", Unit, Short, &["a", "b", "c"], "a, b, c"),
    ("es", Unit, Narrow, &["a", "b"], "a b"),
    ("es", Unit, Narrow, &["a", "b", "c"], "a b c"),
    ("de", Conjunction, Long, &["a", "b"], "a und b"),
    ("de", Conjunction, Long, &["a", "b", "c"], "a, b und c"),
    ("de", Conjunction, Short, &["a", "b"], "a und b"),
    ("de", Conjunction, Short, &["a", "b", "c"], "a, b und c"),
    ("de", Conjunction, Narrow, &["a", "b"], "a und b"),
    ("de", Conjunction, Narrow, &["a", "b", "c"], "a, b und c"),
    ("de", Disjunction, Long, &["a", "b"], "a oder b"),
    ("de", Disjunction, Long, &["a", "b", "c"], "a, b oder c"),
    ("de", Disjunction, Short, &["a", "b"], "a oder b"),
    ("de", Disjunction, Short, &["a", "b", "c"], "a, b oder c"),
    ("de", Disjunction, Narrow, &["a", "b"], "a oder b"),
    ("de", Disjunction, Narrow, &["a", "b", "c"], "a, b oder c"),
    ("de", Unit, Long, &["a", "b"], "a, b"),
    ("de", Unit, Long, &["a", "b", "c"], "a, b und c"),
    ("de", Unit, Short, &["a", "b"], "a, b"),
    ("de", Unit, Short, &["a", "b", "c"], "a, b und c"),
    ("de", Unit, Narrow, &["a", "b"], "a, b"),
    ("de", Unit, Narrow, &["a", "b", "c"], "a, b und c"),
    ("fr", Conjunction, Long, &["a", "b"], "a et b"),
    ("fr", Conjunction, Long, &["a", "b", "c"], "a, b et c"),
    ("fr", Conjunction, Short, &["a", "b"], "a et b"),
    ("fr", Conjunction, Short, &["a", "b", "c"], "a, b et c"),
    ("fr", Conjunction, Narrow, &["a", "b"], "a, b"),
    ("fr", Conjunction, Narrow, &["a", "b", "c"], "a, b, c"),
    ("fr", Disjunction, Long, &["a", "b"], "a ou b"),
    ("fr", Disjunction, Long, &["a", "b", "c"], "a, b ou c"),
    ("fr", Disjunction, Short, &["a", "b"], "a ou b"),
    ("fr", Disjunction, Short, &["a", "b", "c"], "a, b ou c"),
    ("fr", Disjunction, Narrow, &["a", "b"], "a ou b"),
    ("fr", Disjunction, Narrow, &["a", "b", "c"], "a, b ou c"),
    ("fr", Unit, Long, &["a", "b"], "a et b"),
    ("fr", Unit, Long, &["a", "b", "c"], "a, b et c"),
    ("fr", Unit, Short, &["a", "b"], "a et b"),
    ("fr", Unit, Short, &["a", "b", "c"], "a, b et c"),
    ("fr", Unit, Narrow, &["a", "b"], "a b"),
    ("fr", Unit, Narrow, &["a", "b", "c"], "a b c"),
    ("pl", Conjunction, Long, &["a", "b"], "a i b"),
    ("pl", Conjunction, Long, &["a", "b", "c"], "a, b i c"),
    ("pl", Conjunction, Short, &["a", "b"], "a i b"),
    ("pl", Conjunction, Short, &["a", "b", "c"], "a, b i c"),
    ("pl", Conjunction, Narrow, &["a", "b"], "a i b"),
    ("pl", Conjunction, Narrow, &["a", "b", "c"], "a, b i c"),
    ("pl", Disjunction, Long, &["a", "b"], "a lub b"),
    ("pl", Disjunction, Long, &["a", "b", "c"], "a, b lub c"),
    ("pl", Disjunction, Short, &["a", "b"], "a lub b"),
    ("pl", Disjunction, Short, &["a", "b", "c"], "a, b lub c"),
    ("pl", Disjunction, Narrow, &["a", "b"], "a lub b"),
    ("pl", Disjunction, Narrow, &["a", "b", "c"], "a, b lub c"),
    ("pl", Unit, Long, &["a", "b"], "a i b"),
    ("pl", Unit, Long, &["a", "b", "c"], "a, b i c"),
    ("pl", Unit, Short, &["a", "b"], "a i b"),
    ("pl", Unit, Short, &["a", "b", "c"], "a, b i c"),
    ("pl", Unit, Narrow, &["a", "b"], "a i b"),
    ("pl", Unit, Narrow, &["a", "b", "c"], "a, b i c"),
    ("ja", Conjunction, Long, &["a", "b"], "a、b"),
    ("ja", Conjunction, Long, &["a", "b", "c"], "a、b、c"),
    ("ja", Conjunction, Short, &["a", "b"], "a、b"),
    ("ja", Conjunction, Short, &["a", "b", "c"], "a、b、c"),
    ("ja", Conjunction, Narrow, &["a", "b"], "a、b"),
    ("ja", Conjunction, Narrow, &["a", "b", "c"], "a、b、c"),
    ("ja", Disjunction, Long, &["a", "b"], "aまたはb"),
    ("ja", Disjunction, Long, &["a", "b", "c"], "a、b、またはc"),
    ("ja", Disjunction, Short, &["a", "b"], "aまたはb"),
    ("ja", Disjunction, Short, &["a", "b", "c"], "a、b、またはc"),
    ("ja", Disjunction, Narrow, &["a", "b"], "aまたはb"),
    ("ja", Disjunction, Narrow, &["a", "b", "c"], "a、b、またはc"),
    ("ja", Unit, Long, &["a", "b"], "a b"),
    ("ja", Unit, Long, &["a", "b", "c"], "a b c"),
    ("ja", Unit, Short, &["a", "b"], "a b"),
    ("ja", Unit, Short, &["a", "b", "c"], "a b c"),
    ("ja", Unit, Narrow, &["a", "b"], "ab"),
    ("ja", Unit, Narrow, &["a", "b", "c"], "abc"),
    ("ar", Conjunction, Long, &["a", "b"], "a وb"),
    ("ar", Conjunction, Long, &["a", "b", "c"], "a وb وc"),
    ("ar", Conjunction, Short, &["a", "b"], "a وb"),
    ("ar", Conjunction, Short, &["a", "b", "c"], "a وb وc"),
    ("ar", Conjunction, Narrow, &["a", "b"], "a وb"),
    ("ar", Conjunction, Narrow, &["a", "b", "c"], "a وb وc"),
    ("ar", Disjunction, Long, &["a", "b"], "a أو b"),
    ("ar", Disjunction, Long, &["a", "b", "c"], "a أو b أو c"),
    ("ar", Disjunction, Short, &["a", "b"], "a أو b"),
    ("ar", Disjunction, Short, &["a", "b", "c"], "a أو b أو c"),
    ("ar", Disjunction, Narrow, &["a", "b"], "a أو b"),
    ("ar", Disjunction, Narrow, &["a", "b", "c"], "a أو b أو c"),
    ("ar", Unit, Long, &["a", "b"], "a وb"),
    ("ar", Unit, Long, &["a", "b", "c"], "a، وb، وc"),
    ("ar", Unit, Short, &["a", "b"], "a وb"),
    ("ar", Unit, Short, &["a", "b", "c"], "a، وb، وc"),
    ("ar", Unit, Narrow, &["a", "b"], "a وb"),
    ("ar", Unit, Narrow, &["a", "b", "c"], "a وb وc"),
    ];
    for (lang, list_type, width, items, want) in CASES {
        assert_eq!(
            &fl(lang, items, *list_type, *width),
            want,
            "{lang} {list_type:?} {width:?} {items:?}"
        );
    }
}

#[test]
fn english_lists() {
    assert_eq!(fl("en", &[], Conjunction, Long), "");
    assert_eq!(fl("en", &["a"], Conjunction, Long), "a");
    assert_eq!(
        fl("en", &["a", "b", "c", "d"], Conjunction, Long),
        "a, b, c, and d"
    );
    assert_eq!(
        fl("en", &["a", "b", "c", "d"], Disjunction, Long),
        "a, b, c, or d"
    );
    assert_eq!(fl("en", &["a", "b", "c", "d"], Unit, Narrow), "a b c d");
}

#[test]
fn defaults_match_ecma402() {
    // ECMA-402 defaults `type` to "conjunction" and `style` to "long".
    let o = ListFormatOptions::default();
    assert_eq!(o.list_type, ListType::Conjunction);
    assert_eq!(o.width, ListWidth::Long);
    assert_eq!(format_list("en", &["a", "b", "c"], &o), "a, b, and c");
    // Either axis alone builds the options, leaving the other at its default.
    let or: ListFormatOptions = ListType::Disjunction.into();
    assert_eq!(format_list("en", &["a", "b"], &or), "a or b");
    let short: ListFormatOptions = ListWidth::Short.into();
    assert_eq!(format_list("en", &["a", "b"], &short), "a & b");
}

#[test]
#[allow(deprecated)]
fn deprecated_list_style_maps_onto_list_type() {
    use intl::list::ListStyle;
    assert_eq!(ListType::from(ListStyle::And), ListType::Conjunction);
    assert_eq!(ListType::from(ListStyle::Or), ListType::Disjunction);
}

#[test]
fn other_locales_and_fallback() {
    assert_eq!(fl("de", &["a", "b", "c"], Conjunction, Long), "a, b und c");
    assert_eq!(fl("fr", &["a", "b", "c"], Conjunction, Long), "a, b et c");
    // Region and script subtags are stripped one at a time; an unknown tag ends
    // at `en`, which stands in for CLDR root.
    assert_eq!(fl("en-GB", &["a", "b"], Conjunction, Long), "a and b");
    assert_eq!(fl("de_AT", &["a", "b"], Conjunction, Long), "a und b");
    assert_eq!(fl("zz", &["a", "b"], Conjunction, Long), "a and b");
    // The width fallback is per-locale data, not a chain: `fr` narrow really is
    // comma-only while `fr` short keeps "et".
    assert_eq!(fl("fr", &["a", "b"], Conjunction, Narrow), "a, b");
    assert_eq!(fl("fr", &["a", "b"], Conjunction, Short), "a et b");
}

#[test]
fn five_item_list_exact_output() {
    // Linear connector folding must be byte-identical to the prior O(N^2) fold.
    assert_eq!(
        fl("en", &["a", "b", "c", "d", "e"], Conjunction, Long),
        "a, b, c, d, and e"
    );
    assert_eq!(
        fl("en", &["a", "b", "c", "d", "e"], Disjunction, Long),
        "a, b, c, d, or e"
    );
    assert_eq!(
        fl("de", &["a", "b", "c", "d", "e"], Conjunction, Long),
        "a, b, c, d und e"
    );
    assert_eq!(
        fl("fr", &["a", "b", "c", "d", "e"], Conjunction, Long),
        "a, b, c, d et e"
    );
    // `ar` unit uses a different `2` pattern from its start/middle/end, so the
    // 5-item assembly must not reuse the two-item connector.
    assert_eq!(
        fl("ar", &["a", "b", "c", "d", "e"], Unit, Long),
        "a\u{60c} \u{648}b\u{60c} \u{648}c\u{60c} \u{648}d\u{60c} \u{648}e"
    );
}

#[test]
fn large_list_is_linear_time() {
    // 50k items: with the old O(N^2) fold this would copy ~1.25 billion bytes;
    // the linear single-pass build completes near-instantly. We assert the
    // structure (prefix, count of separators, suffix) rather than the full
    // multi-megabyte string.
    let items: Vec<String> = (0..50_000).map(|i| i.to_string()).collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let out = fl("en", &refs, Conjunction, Long);
    assert!(out.starts_with("0, 1, 2, "));
    assert!(out.ends_with(", and 49999"));
    // 49,999 ", " separators between the 50,000 items, but the last one is
    // ", and " — so 49,998 plain ", " plus the final ", and ".
    assert_eq!(out.matches(", and ").count(), 1);
    assert_eq!(out.matches(", ").count(), 49_999);
}
