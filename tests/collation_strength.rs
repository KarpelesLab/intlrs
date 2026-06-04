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
