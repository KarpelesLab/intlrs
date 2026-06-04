//! CLDR cardinal plural selection.
use intl::plural::{plural_category as pc, PluralCategory::*, PluralOperands as Op};

#[test]
fn cardinal_categories() {
    // English: one vs other; "1.0" has a visible fraction so it's other.
    assert_eq!(pc("en", &Op::from_int(1)), One);
    assert_eq!(pc("en", &Op::from_int(2)), Other);
    assert_eq!(pc("en", &Op::parse("1.0").unwrap()), Other);

    // Welsh has the full set including two/few/many.
    assert_eq!(pc("cy", &Op::from_int(0)), Zero);
    assert_eq!(pc("cy", &Op::from_int(2)), Two);
    assert_eq!(pc("cy", &Op::from_int(3)), Few);
    assert_eq!(pc("cy", &Op::from_int(6)), Many);

    // Polish few/many.
    assert_eq!(pc("pl", &Op::from_int(2)), Few);
    assert_eq!(pc("pl", &Op::from_int(5)), Many);

    // Japanese is always "other".
    assert_eq!(pc("ja", &Op::from_int(1)), Other);

    // Locale fallback: an unknown region uses the language rules.
    assert_eq!(pc("en-US", &Op::from_int(1)), One);
    assert_eq!(pc("fr-CA", &Op::from_int(1)), One); // French: 0 and 1 are "one"
}
