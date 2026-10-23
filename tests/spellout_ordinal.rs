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
    for l in [
        "en", "de", "fr", "nl", "es", "it", "pt", "sv", "ru", "uk", "pl", "ja",
    ] {
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

#[test]
fn ordinal_new_locales() {
    assert_eq!(spell_ordinal("ru", 1).as_deref(), Some("первый"));
    assert_eq!(spell_ordinal("ru", 21).as_deref(), Some("двадцать первый"));
    assert_eq!(spell_ordinal("ru", 1000).as_deref(), Some("тысячный"));
    assert_eq!(
        spell_ordinal("ru", 1001).as_deref(),
        Some("одна тысяча первый")
    );
    assert_eq!(spell_ordinal("ru", 2000).as_deref(), Some("двухтысячный"));
    assert_eq!(
        spell_ordinal("ru", 2001).as_deref(),
        Some("две тысячи первый")
    );
    assert_eq!(spell_ordinal("pl", 11).as_deref(), Some("jedenasty"));
    assert_eq!(spell_ordinal("pl", 22).as_deref(), Some("dwudziesty drugi"));
    // `[then|else]`: "sto pierwszy" but "setny".
    assert_eq!(spell_ordinal("pl", 100).as_deref(), Some("setny"));
    assert_eq!(spell_ordinal("pl", 101).as_deref(), Some("sto pierwszy"));
    assert_eq!(spell_ordinal("pl", 2000).as_deref(), Some("dwutysięczny"));
    assert_eq!(spell_ordinal("ja", 1234).as_deref(), Some("第千二百三十四"));
    // Ukrainian has no ordinal spell-out rules in CLDR.
    assert_eq!(spell_ordinal("uk", 1), None);
    // French uses its public masculine ordinal, not the private helper.
    assert_eq!(spell_ordinal("fr", 0).as_deref(), Some("zéroième"));
    // ICU's roll-back rule: 10000 uses the `2000:` rule, not `2001:`.
    assert_eq!(
        spell_ordinal("it", 10_000)
            .map(|s| s.replace('\u{ad}', ""))
            .as_deref(),
        Some("diecimillesimo")
    );
    // A leading apostrophe is not output.
    assert_eq!(
        spell_ordinal("en", 101).as_deref(),
        Some("one hundred first")
    );
}
