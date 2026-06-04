//! Latin-ASCII transliteration.
#![cfg(feature = "alloc")]
use intl::translit::latin_ascii as t;

#[test]
fn latin_ascii() {
    assert_eq!(t("café"), "cafe");
    assert_eq!(t("naïve"), "naive");
    assert_eq!(t("Straße"), "Strasse");
    assert_eq!(t("Łódź"), "Lodz");
    assert_eq!(t("Æsir Øystein"), "AEsir Oystein");
    assert_eq!(t("Þór"), "Thor");
    assert_eq!(t("Œuvre œuf"), "OEuvre oeuf");
    assert_eq!(t("Ĳsselmeer"), "IJsselmeer");
    // Typographic punctuation.
    assert_eq!(t("“quote” ‘x’ — …"), "\"quote\" 'x' - ...");
    // ASCII unchanged; non-Latin preserved.
    assert_eq!(t("hello"), "hello");
    assert_eq!(t("日本"), "日本");
    // Composed and decomposed inputs fold identically.
    assert_eq!(t("e\u{0301}"), t("é"));
}
