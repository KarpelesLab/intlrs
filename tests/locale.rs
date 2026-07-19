//! BCP-47 locale parsing.
#![cfg(feature = "locale")]

use intl::locale::Locale;
use intl::locale::{canonicalize, get_canonical_locales};

#[test]
fn parse_and_canonicalize() {
    let l = Locale::parse("zh-hant-hk").unwrap();
    assert_eq!(l.language, "zh");
    assert_eq!(l.script.as_deref(), Some("Hant"));
    assert_eq!(l.region.as_deref(), Some("HK"));
    assert_eq!(l.to_string(), "zh-Hant-HK");

    assert_eq!(Locale::parse("EN_us").unwrap().to_string(), "en-US");
    assert_eq!(Locale::parse("es-419").unwrap().to_string(), "es-419");
    assert_eq!(
        Locale::parse("sl-rozaj-biske").unwrap().to_string(),
        "sl-rozaj-biske"
    );
    assert_eq!(Locale::parse("und").unwrap().to_string(), "und");
    assert_eq!(Locale::parse("fr").unwrap().language, "fr");
}

#[test]
fn rejects_garbage() {
    assert!(Locale::parse("").is_err());
    assert!(Locale::parse("1").is_err());
    assert!(Locale::parse("toolonglang").is_err());
}

#[test]
fn maximize_minimize() {
    use intl::locale::Locale;
    assert_eq!(
        Locale::parse("en").unwrap().maximize().to_string(),
        "en-Latn-US"
    );
    assert_eq!(
        Locale::parse("zh").unwrap().maximize().to_string(),
        "zh-Hans-CN"
    );
    assert_eq!(
        Locale::parse("ja").unwrap().maximize().to_string(),
        "ja-Jpan-JP"
    );
    assert_eq!(
        Locale::parse("de-DE").unwrap().maximize().to_string(),
        "de-Latn-DE"
    );
    // minimize is the inverse.
    assert_eq!(
        Locale::parse("en-Latn-US").unwrap().minimize().to_string(),
        "en"
    );
    assert_eq!(
        Locale::parse("zh-Hans-CN").unwrap().minimize().to_string(),
        "zh"
    );
    assert_eq!(Locale::parse("en-US").unwrap().minimize().to_string(), "en");
    // pt-BR reduces to pt (BR is Portuguese's likely region); pt-PT does not.
    assert_eq!(Locale::parse("pt-BR").unwrap().minimize().to_string(), "pt");
    assert_eq!(
        Locale::parse("pt-PT").unwrap().minimize().to_string(),
        "pt-PT"
    );
}

#[test]
fn negotiation() {
    use intl::locale::{Locale, negotiate};
    let avail = [
        Locale::parse("en").unwrap(),
        Locale::parse("fr").unwrap(),
        Locale::parse("de").unwrap(),
        Locale::parse("zh-Hant").unwrap(),
    ];
    assert_eq!(negotiate(&["fr-CA"], &avail), Some(1)); // fr-CA -> fr
    assert_eq!(negotiate(&["de-AT", "en"], &avail), Some(2)); // de-AT -> de
    assert_eq!(negotiate(&["es", "en-GB"], &avail), Some(0)); // skip es, en-GB -> en
    assert_eq!(negotiate(&["zh-TW"], &avail), Some(3)); // Traditional Chinese
    assert_eq!(negotiate(&["ja"], &avail), None);
    assert_eq!(negotiate(&[], &avail), None);
}

#[test]
fn canonicalize_language_aliases() {
    let c = |t: &str| canonicalize(t).unwrap();
    // Simple deprecated language subtags.
    assert_eq!(c("iw"), "he");
    assert_eq!(c("in"), "id");
    assert_eq!(c("ji"), "yi");
    assert_eq!(c("mo"), "ro");
    assert_eq!(c("tl"), "fil");
    // Macrolanguage alias.
    assert_eq!(c("cmn"), "zh");
    // Multi-subtag replacement fills the (empty) script.
    assert_eq!(c("sh"), "sr-Latn");
    // ...but an explicit script is kept (replacement only fills empty fields).
    assert_eq!(c("sh-Cyrl"), "sr-Cyrl");
    // Language alias applies within a larger tag, keeping the region.
    assert_eq!(c("iw-US"), "he-US");
    assert_eq!(c("sh-fonipa"), "sr-Latn-fonipa");
}

#[test]
fn canonicalize_grandfathered() {
    let c = |t: &str| canonicalize(t).unwrap();
    assert_eq!(c("i-klingon"), "tlh");
    assert_eq!(c("zh-min-nan"), "nan");
    assert_eq!(c("zh-cmn-Hans"), "zh-Hans");
    assert_eq!(c("no-bok"), "nb");
    // Grandfathered irregular tag that does not parse as a normal langtag.
    assert_eq!(c("sgn-BR"), "bzs");
}

#[test]
fn canonicalize_script_and_variant() {
    let c = |t: &str| canonicalize(t).unwrap();
    // Script alias (the only one in CLDR): Qaai -> Zinh.
    assert_eq!(c("und-Qaai"), "und-Zinh");
    assert_eq!(c("ru-Qaai"), "ru-Zinh");
    // Variant alias.
    assert_eq!(c("und-polytoni"), "und-polyton");
}

#[test]
fn canonicalize_territory_aliases() {
    let c = |t: &str| canonicalize(t).unwrap();
    // One→one territory aliases.
    assert_eq!(c("en-BU"), "en-MM");
    assert_eq!(c("de-DD"), "de-DE");
    // One→many: `SU` maps to a list whose first (CLDR-order) element is RU. With
    // no informative language, the first candidate wins.
    assert_eq!(c("und-SU"), "und-RU");
    // One→many disambiguated by the language's likely region: Russian's likely
    // region (RU) is among the SU candidates, so it is chosen.
    assert_eq!(c("ru-SU"), "ru-RU");
}

#[test]
fn canonicalize_structural() {
    // Structural canonicalization (case/order) still applies with no aliasing.
    assert_eq!(canonicalize("EN-us").unwrap(), "en-US");
    assert_eq!(canonicalize("zh-hant-hk").unwrap(), "zh-Hant-HK");
    assert_eq!(canonicalize("es-419").unwrap(), "es-419");
    // Extensions are preserved (and reordered by the existing canonical form).
    assert_eq!(
        canonicalize("DE-Latn-u-co-phonebk-t-de").unwrap(),
        "de-Latn-t-de-u-co-phonebk"
    );
    // Garbage fails to canonicalize.
    assert!(canonicalize("").is_none());
    assert!(canonicalize("toolonglang").is_none());
}

#[test]
fn canonicalize_unicode_extension_keywords() {
    let c = |t: &str| canonicalize(t).unwrap();
    // Deprecated type-value aliases (all cross-checked against V8
    // `Intl.getCanonicalLocales`).
    assert_eq!(c("en-u-ca-islamicc"), "en-u-ca-islamic-civil");
    assert_eq!(c("en-u-ca-ethiopic-amete-alem"), "en-u-ca-ethioaa");
    assert_eq!(c("en-u-ms-imperial"), "en-u-ms-uksystem");
    assert_eq!(c("en-u-ks-primary"), "en-u-ks-level1");
    assert_eq!(c("en-u-ks-tertiary"), "en-u-ks-level3");
    assert_eq!(c("en-u-tz-uct"), "en-u-tz-utc");
    assert_eq!(c("en-u-tz-gmt0"), "en-u-tz-gmt");
    // A `true`/`yes` type is dropped for any key; `false`/`no` are kept verbatim.
    assert_eq!(c("en-u-kn-true"), "en-u-kn");
    assert_eq!(c("en-u-kn-yes"), "en-u-kn");
    assert_eq!(c("en-u-ca-true"), "en-u-ca");
    assert_eq!(c("en-u-kn-false"), "en-u-kn-false");
    assert_eq!(c("en-u-kn-no"), "en-u-kn-no");
    assert_eq!(c("en-u-kf-no"), "en-u-kf-no");
    // Attribute + keyword sorting; attributes precede keywords.
    assert_eq!(
        c("en-u-foo-bar-ca-gregory-nu-latn"),
        "en-u-bar-foo-ca-gregory-nu-latn"
    );
    assert_eq!(c("en-u-nu-latn-ca-gregory"), "en-u-ca-gregory-nu-latn");
    // Case normalization.
    assert_eq!(c("EN-U-CA-ISLAMICC"), "en-u-ca-islamic-civil");
    // A `true` attribute (not a keyword type) is kept.
    assert_eq!(c("en-u-true-ca-gregory"), "en-u-true-ca-gregory");
    // Duplicate key: first occurrence wins.
    assert_eq!(c("en-u-ca-buddhist-ca-gregory"), "en-u-ca-buddhist");
}

#[test]
fn canonicalize_transform_extension() {
    let c = |t: &str| canonicalize(t).unwrap();
    // tlang is canonicalized like a language tag, then lowercased.
    assert_eq!(c("en-t-iw"), "en-t-he");
    assert_eq!(c("en-t-sh"), "en-t-sr-latn");
    assert_eq!(c("en-t-en-us"), "en-t-en-us");
    // tfield value alias (transform mechanism `m0`).
    assert_eq!(c("en-t-en-m0-names"), "en-t-en-m0-prprname");
    // A `-t-` and a `-u-` extension coexist and are ordered t before u.
    assert_eq!(c("de-t-de-u-ca-gregory"), "de-t-de-u-ca-gregory");
    // No tlang, just a tfield.
    assert_eq!(c("en-t-k0-qwerty"), "en-t-k0-qwerty");
}

#[test]
fn canonical_locale_list_dedupes() {
    assert_eq!(
        get_canonical_locales(&["en-US", "EN-us", "iw"]),
        ["en-US", "he"]
    );
    // Invalid tags are dropped.
    assert_eq!(get_canonical_locales(&["1", "fr", "fr"]), ["fr"]);
}

#[test]
fn extensions() {
    use intl::locale::Locale;
    let l = Locale::parse("en-US-u-ca-buddhist-nu-thai").unwrap();
    assert_eq!(l.language, "en");
    assert_eq!(l.region.as_deref(), Some("US"));
    assert_eq!(l.extensions, ["u-ca-buddhist-nu-thai"]);
    assert_eq!(l.to_string(), "en-US-u-ca-buddhist-nu-thai");
    // Extensions are reordered into canonical (singleton-alphabetical) form,
    // case-normalized, with private use ('x') last.
    assert_eq!(
        Locale::parse("DE-Latn-u-co-phonebk-t-de")
            .unwrap()
            .to_string(),
        "de-Latn-t-de-u-co-phonebk"
    );
    assert_eq!(
        Locale::parse("en-u-nu-thai-x-Foo").unwrap().to_string(),
        "en-u-nu-thai-x-foo"
    );
    // A singleton with no subtag is invalid; single-char subtags need 'x'.
    assert!(Locale::parse("en-u").is_err());
    assert!(Locale::parse("en-a-b").is_err()); // 'b' too short for a non-x ext
    assert_eq!(Locale::parse("en-x-a-b").unwrap().to_string(), "en-x-a-b");
}

/// Structural validation of extension/singleton subtag lengths (UTS #35 ABNF /
/// ECMA-402 `IsStructurallyValidLanguageTag`). Every accept/reject below was
/// cross-checked against V8 `Intl.getCanonicalLocales`: a tag V8 throws on must
/// be dropped by `get_canonical_locales` and yield `None` from `canonicalize`.
#[test]
fn rejects_structurally_invalid_extensions() {
    // (tag, valid?) — `valid == false` means V8 throws RangeError.
    let cases: &[(&str, bool)] = &[
        // -u-: a type longer than 8 chars, or a 1-char / 2-char-ending-in-digit
        // subtag, is invalid; legal short and long forms are kept.
        ("en-u-ca-gregorian", false),       // 9-char type
        ("en-u-attr-toolongvalue9", false), // 13-char type
        ("en-u-c", false),                  // 1-char subtag
        ("en-u-a1", false),                 // 2-char subtag ending in a digit
        ("en-u-11", false),                 // 2-char subtag, no alpha
        ("en-u-ca-x1", false),              // 2-char type ending in a digit
        ("en-u-ca-gregory", true),
        ("en-u-ca-islamic-civil", true),
        ("en-u-kn", true),
        ("en-u-ca", true),
        ("en-u-nu-latn", true),
        ("en-u-attr", true),   // 4-char attribute
        ("en-u-ab", true),     // 2-char attribute/key (2nd char alpha)
        ("en-u-1a", true),     // key: 2 chars, 2nd char alpha
        ("en-u-ca-1x", true),  // 2-char type, 2nd char alpha
        ("en-u-ca-123", true), // 3-char numeric type
        // Generic singletons (`a`-`s`, `v`-`w`, `y`-`z`): each subtag 2-8 alnum.
        ("en-a-b", false),         // 1-char subtag
        ("en-0", false),           // singleton with no subtag
        ("en-a-abcdefghi", false), // 9-char subtag
        ("en-a-bc", true),
        ("en-a-a1", true), // no alpha-ending restriction for generic singletons
        ("en-a-11", true),
        // -t- transform: valid tlang, tkey = <alpha><digit>, tvalue = 3-8 alnum.
        ("en-t-de-a", false),     // tvalue 1 char
        ("en-t-k0-a", false),     // tvalue 1 char (no tlang)
        ("de-t-de-0", false),     // stray 1-char subtag
        ("de-t-0", false),        // invalid tlang
        ("de-t-de-k0-ab", false), // tvalue 2 chars (too short)
        ("de-t-de-k0-a1", false), // "a1" parsed as a new tkey -> k0 has no value
        ("de-t-de-k0", false),    // tkey with no tvalue
        ("de-t-de-u-ca-gregory", true),
        ("en-t-en-us", true),
        ("en-t-k0-qwerty", true),
        ("de-t-en-latn-us-k0-qwerty", true),
        // -x- private use: each subtag 1-8 alnum.
        ("en-x-toolongprivateuse9", false), // 15-char subtag
        ("en-x-anylongprivateuse", false),  // 16-char subtag
        ("en-x-abcdefghi", false),          // 9-char subtag
        ("en-x-a", true),
        ("en-x-a-b", true),
        ("en-x-abcdefgh", true), // 8-char subtag
    ];
    for &(tag, valid) in cases {
        assert_eq!(
            canonicalize(tag).is_some(),
            valid,
            "canonicalize({tag:?}) validity mismatch (V8 valid = {valid})"
        );
        // `get_canonical_locales` drops the invalid ones and keeps the valid.
        let got = get_canonical_locales(&[tag]);
        assert_eq!(
            got.is_empty(),
            !valid,
            "get_canonical_locales([{tag:?}]) = {got:?} (V8 valid = {valid})"
        );
    }
}
