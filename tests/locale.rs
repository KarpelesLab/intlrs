//! BCP-47 locale parsing.
#![cfg(feature = "locale")]

use intl::locale::Locale;

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
