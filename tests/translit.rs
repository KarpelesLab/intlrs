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

#[test]
fn diacritics() {
    use intl::translit::remove_diacritics as d;
    assert_eq!(d("café Müller"), "cafe Muller");
    assert_eq!(d("naïve"), "naive");
    assert_eq!(d("Crème brûlée"), "Creme brulee");
    // Non-Latin base letters are preserved (only the accents go).
    assert_eq!(d("ψυχή"), "ψυχη");
    // ß/ø are not accented letters -> unchanged (vs latin_ascii which maps them).
    assert_eq!(d("Straße"), "Straße");
    // Composed input.
    assert_eq!(d("é"), "e");
}

#[test]
fn cyrillic() {
    use intl::translit::{cyrillic_to_latin as c, latin_ascii};
    assert_eq!(c("Москва"), "Moskva");
    assert_eq!(c("Привет, мир"), "Privet, mir");
    assert_eq!(c("Достоевский"), "Dostoevskij");
    assert_eq!(c("ЖУК"), "ŽUK");
    // Chained to ASCII.
    assert_eq!(latin_ascii(&c("Чехов")), "Cehov");
    assert_eq!(latin_ascii(&c("Шостакович")), "Sostakovic");
    // Non-Cyrillic passes through.
    assert_eq!(c("hello"), "hello");
}

#[test]
fn greek() {
    use intl::translit::greek_to_latin as g;
    assert_eq!(g("Αθήνα"), "Athina");
    assert_eq!(g("ψυχή"), "psychi");
    assert_eq!(g("Ελλάδα"), "Ellada");
    assert_eq!(g("φιλοσοφία"), "filosofia");
    assert_eq!(g("ΘΕΟΣ"), "THEOS");
    assert_eq!(g("hello"), "hello"); // non-Greek passthrough
}

#[test]
fn any_ascii_mixed() {
    use intl::translit::any_ascii as a;
    assert_eq!(a("Москва café Αθήνα"), "Moskva cafe Athina");
    assert_eq!(a("Straße"), "Strasse");
    assert_eq!(a("Чехов & Δίας"), "Cehov & Dias");
    // CJK (no romanization here) passes through.
    assert_eq!(a("東京 Tokyo"), "東京 Tokyo");
}
