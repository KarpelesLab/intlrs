//! Readable collation spot-checks. The exhaustive UCA conformance run lives in
//! `collation_conformance.rs`.
#![cfg(feature = "alloc")]

use core::cmp::Ordering;
use intl::unicode::collate::{compare, AlternateHandling, Collator};

#[test]
fn basic_order() {
    assert_eq!(compare("apple", "apply"), Ordering::Less);
    assert_eq!(compare("apple", "apple"), Ordering::Equal);
    assert_eq!(compare("b", "a"), Ordering::Greater);
}

#[test]
fn accents_sort_after_base() {
    // "é" differs from "e" only at the secondary level, so "cafe" < "café",
    // but both sort before "caff" (primary 'e' < 'f').
    assert_eq!(compare("cafe", "café"), Ordering::Less);
    assert_eq!(compare("café", "caff"), Ordering::Less);
}

#[test]
fn case_differs_at_tertiary() {
    // Lowercase sorts before uppercase (a tertiary distinction).
    assert_eq!(compare("a", "A"), Ordering::Less);
    // ...but only after primary/secondary agree.
    assert_eq!(compare("A", "b"), Ordering::Less);
}

#[test]
fn variable_handling() {
    // Default (shifted): leading punctuation is ignored at the primary level, so
    // "b" and "-c" compare as 'b' vs 'c' → "b" < "-c".
    assert_eq!(compare("b", "-c"), Ordering::Less);

    // Non-ignorable: the hyphen is a primary character that sorts before
    // letters, so "-c" < "b".
    let ni = Collator::new(AlternateHandling::NonIgnorable);
    assert_eq!(ni.compare("b", "-c"), Ordering::Greater);
}

#[test]
fn sort_key_matches_compare() {
    let c = Collator::default();
    assert!(c.sort_key("apple") < c.sort_key("apply"));
}
