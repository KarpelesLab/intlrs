//! Collation strength levels (case/accent-insensitive comparison).
#![cfg(feature = "alloc")]
use core::cmp::Ordering;
use intl::unicode::collate::{AlternateHandling, Collator, Strength};

#[test]
fn strength_levels() {
    let prim = Collator::new(AlternateHandling::NonIgnorable).with_strength(Strength::Primary);
    let sec = Collator::new(AlternateHandling::NonIgnorable).with_strength(Strength::Secondary);
    let ter = Collator::new(AlternateHandling::NonIgnorable).with_strength(Strength::Tertiary);

    // Primary: base letters only — accents and case are equal.
    assert_eq!(prim.compare("café", "CAFE"), Ordering::Equal);
    assert_eq!(prim.compare("résumé", "resume"), Ordering::Equal);

    // Secondary: case-insensitive, but accents distinguish.
    assert_eq!(sec.compare("cafe", "CAFE"), Ordering::Equal);
    assert_ne!(sec.compare("café", "cafe"), Ordering::Equal);

    // Tertiary: everything matters.
    assert_ne!(ter.compare("cafe", "CAFE"), Ordering::Equal);
    assert_ne!(ter.compare("café", "cafe"), Ordering::Equal);

    // Ordering by base letter is preserved at every strength.
    assert_eq!(prim.compare("apple", "banana"), Ordering::Less);
}

#[test]
fn numeric_ordering() {
    let n = Collator::default().with_numeric(true);
    let plain = Collator::default();
    // Natural numeric order.
    assert_eq!(n.compare("file2", "file10"), Ordering::Less);
    assert_eq!(n.compare("file10", "file9"), Ordering::Greater);
    assert_eq!(n.compare("item100", "item99"), Ordering::Greater);
    // Plain (codepoint) order sorts "file10" before "file2".
    assert_eq!(plain.compare("file10", "file2"), Ordering::Less);
    // Leading zeros: equal numeric value.
    assert_eq!(n.compare("v007", "v7"), Ordering::Equal);
    // Mixed text after the number.
    assert_eq!(n.compare("x2y", "x10y"), Ordering::Less);
    // Equal strings stay equal.
    assert_eq!(n.compare("abc123", "abc123"), Ordering::Equal);
    // Pure text unaffected.
    assert_eq!(n.compare("apple", "banana"), Ordering::Less);
}
