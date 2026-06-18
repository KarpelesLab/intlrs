//! RBNF cardinal spell-out (locale-driven).
#![cfg(feature = "spellout")]
use intl::spellout::spell_cardinal as c;

fn strip(s: Option<String>) -> String {
    s.unwrap().replace('\u{ad}', "").replace('\u{a0}', " ") // drop soft hyphens / nbsp
}

#[test]
fn english() {
    assert_eq!(c("en", 0).as_deref(), Some("zero"));
    assert_eq!(c("en", 7).as_deref(), Some("seven"));
    assert_eq!(c("en", 42).as_deref(), Some("forty-two"));
    assert_eq!(c("en", 100).as_deref(), Some("one hundred"));
    assert_eq!(
        c("en", 1234).as_deref(),
        Some("one thousand two hundred thirty-four")
    );
    assert_eq!(c("en", 2000000).as_deref(), Some("two million"));
    assert_eq!(c("en", -42).as_deref(), Some("minus forty-two"));
}

#[test]
fn other_locales() {
    // French: 80 = "quatre-vingts", 91 = "quatre-vingt-onze".
    assert_eq!(strip(c("fr", 80)), "quatre-vingts");
    assert_eq!(strip(c("fr", 91)), "quatre-vingt-onze");
    assert_eq!(strip(c("fr", 100)), "cent");
    // German: 21 = "einundzwanzig", 100 = "einhundert".
    assert_eq!(strip(c("de", 21)), "einundzwanzig");
    assert_eq!(strip(c("de", 100)), "einhundert");
    // Spanish.
    assert_eq!(strip(c("es", 21)), "veintiuno");
    // Unsupported locale.
    assert_eq!(c("ja", 1), None);
}
