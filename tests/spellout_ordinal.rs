#![cfg(feature = "spellout")]
use intl::spellout::spell_ordinal;

#[test]
fn ordinals() {
    assert_eq!(spell_ordinal("en", 1).as_deref(), Some("first"));
    assert_eq!(spell_ordinal("en", 2).as_deref(), Some("second"));
    assert_eq!(spell_ordinal("en", 3).as_deref(), Some("third"));
    assert_eq!(spell_ordinal("en", 4).as_deref(), Some("fourth"));
    assert_eq!(spell_ordinal("en", 12).as_deref(), Some("twelfth"));
    assert_eq!(spell_ordinal("en", 21).as_deref(), Some("twenty-first"));
    assert_eq!(spell_ordinal("en", 100).as_deref(), Some("one hundredth"));
    // Gendered-only locales fall back to masculine.
    assert_eq!(spell_ordinal("es", 1).as_deref(), Some("primero"));
    assert_eq!(spell_ordinal("it", 3).as_deref(), Some("terzo"));
    assert_eq!(spell_ordinal("pt", 1).as_deref(), Some("primeiro"));
    // Locale fallback (region -> language) and unknown.
    assert_eq!(spell_ordinal("en-US", 2).as_deref(), Some("second"));
    assert_eq!(spell_ordinal("xx", 1), None);
}

#[test]
fn ordinal_pathological_values_terminate() {
    // Adversarial inputs must never hang or overflow the stack (the RBNF engine
    // bounds recursion depth + total work). i64::MIN is the classic abs-overflow.
    for l in ["en", "de", "fr", "nl", "es", "it", "pt", "sv"] {
        for v in [
            i64::MIN,
            i64::MAX,
            i64::MIN + 1,
            -1,
            1_000_000_000_000_000_000,
        ] {
            let _ = spell_ordinal(l, v);
            let _ = intl::spellout::spell_cardinal(l, v);
        }
    }
    // And a correctness spot-check still holds after the guards.
    assert_eq!(spell_ordinal("en", 100).as_deref(), Some("one hundredth"));
}
