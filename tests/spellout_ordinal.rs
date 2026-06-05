#![cfg(feature = "alloc")]
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
