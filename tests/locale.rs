//! BCP-47 locale parsing.
#![cfg(feature = "alloc")]

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
