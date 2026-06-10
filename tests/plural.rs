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

#[test]
fn multibyte_overlong_tag_falls_back_to_prefix() {
    // A tag whose first subtag is a known language ("pl") followed by a very
    // long subtag containing multibyte chars that overflow the 40-byte stack
    // normalization buffer. The buffer truncation point splits a 2-byte char;
    // truncating on a char boundary keeps a valid prefix so the `-` strip
    // still finds "pl" (Polish rules: 2 -> few), instead of the buffer
    // failing UTF-8 validation and silently resolving to Other.
    let tag = format!("pl-{}", "à".repeat(40));
    assert_eq!(pc(&tag, &Op::from_int(2)), Few);
    assert_eq!(pc(&tag, &Op::from_int(5)), Many);
    // Sanity: the all-multibyte garbage subtag itself (no known prefix) is Other.
    let garbage = "à".repeat(40);
    assert_eq!(pc(&garbage, &Op::from_int(2)), Other);
}

#[test]
fn ordinal_categories() {
    use intl::plural::ordinal_category as oc;
    // English ordinals: 1st (one), 2nd (two), 3rd (few), 4th (other).
    assert_eq!(oc("en", &Op::from_int(1)), One);
    assert_eq!(oc("en", &Op::from_int(2)), Two);
    assert_eq!(oc("en", &Op::from_int(3)), Few);
    assert_eq!(oc("en", &Op::from_int(4)), Other);
    assert_eq!(oc("en", &Op::from_int(11)), Other); // 11th, not 11st
                                                    // A language without ordinal rules: always other.
    assert_eq!(oc("ja", &Op::from_int(1)), Other);
}

#[test]
fn compact_operands() {
    use intl::plural::PluralOperands as P;
    // "1.2c6" = 1,200,000 (integer): i=1200000, v=0, c=6.
    let a = P::parse("1.2c6").unwrap();
    assert_eq!((a.i, a.v, a.f, a.c), (1_200_000, 0, 0, 6));
    // "1.2e6" alias.
    assert_eq!(P::parse("1.2e6").unwrap().i, 1_200_000);
    // Non-integer expansion keeps fraction operands: "1.23c1" = 12.3.
    let b = P::parse("1.23c1").unwrap();
    assert_eq!((b.i, b.v, b.f, b.c), (12, 1, 3, 1));
    // c0 = no shift.
    let c = P::parse("1.2c0").unwrap();
    assert_eq!((c.i, c.v, c.f, c.c), (1, 1, 2, 0));
    // Plain numbers still parse with c=0.
    assert_eq!(P::parse("42").unwrap().c, 0);
    assert!(P::parse("1.2cx").is_none());
}
