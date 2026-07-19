//! Readable collation spot-checks. The exhaustive UCA conformance run lives in
//! `collation_conformance.rs`.
#![cfg(feature = "collation")]

use core::cmp::Ordering;
use intl::unicode::collate::{AlternateHandling, Collator, compare};

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

/// Signature orderings for locales newly bundled from the official CLDR-48
/// collation rules (see `data/cldr/48/collation.json`). Each asserts the
/// characteristic tailored order that root DUCET does *not* produce.
#[test]
fn newly_added_cldr_tailorings() {
    use intl::unicode::collate::Tailoring;
    let lt = |t: &Tailoring, a: &str, b: &str| {
        assert_eq!(t.compare(a, b), Ordering::Less, "expected {a} < {b}");
    };

    // Polish: ogonek/acute letters each sort right after their base letter.
    let pl = Tailoring::for_locale("pl").unwrap();
    lt(&pl, "a", "ą");
    lt(&pl, "ą", "b");
    lt(&pl, "c", "ć");
    lt(&pl, "z", "ź");
    lt(&pl, "ź", "ż");

    // Galician: ñ after n (imported from Spanish).
    let gl = Tailoring::for_locale("gl").unwrap();
    lt(&gl, "n", "ñ");

    // Northern Sotho / Tswana: circumflex vowels and š after their bases.
    for loc in ["nso", "tn"] {
        let t = Tailoring::for_locale(loc).unwrap();
        lt(&t, "e", "ê");
        lt(&t, "o", "ô");
        lt(&t, "s", "š");
    }

    // Wolof: à, é/ë, ñ/ŋ, ó after their base letters.
    let wo = Tailoring::for_locale("wo").unwrap();
    lt(&wo, "a", "à");
    lt(&wo, "e", "é");
    lt(&wo, "é", "ë");
    lt(&wo, "n", "ñ");
    lt(&wo, "ñ", "ŋ");

    // Yoruba: dotted-below vowels and the "gb" digraph.
    let yo = Tailoring::for_locale("yo").unwrap();
    lt(&yo, "e", "ẹ");
    lt(&yo, "g", "gb");
    lt(&yo, "s", "ṣ");

    // Igbo: digraphs (gb/gh/gw, kp/kw, nw/ny) and dotted-below vowels.
    let ig = Tailoring::for_locale("ig").unwrap();
    lt(&ig, "g", "gb");
    lt(&ig, "gb", "gh");
    lt(&ig, "n", "nw");
    lt(&ig, "i", "ị");

    // Ewe: dz digraph and the open vowels ɛ/ɔ after e/o.
    let ee = Tailoring::for_locale("ee").unwrap();
    lt(&ee, "d", "dz");
    lt(&ee, "e", "ɛ");
    lt(&ee, "o", "ɔ");
    lt(&ee, "n", "ŋ");

    // Belarusian / Kyrgyz (Cyrillic): ё after е, ў after у.
    let be = Tailoring::for_locale("be").unwrap();
    lt(&be, "Е", "ё");
    lt(&be, "у", "ў");
    let ky = Tailoring::for_locale("ky").unwrap();
    lt(&ky, "е", "ё");

    // Macedonian (Cyrillic): ѓ and ќ as their own letters.
    let mk = Tailoring::for_locale("mk").unwrap();
    lt(&mk, "ԃ", "ѓ");
    lt(&mk, "ћ", "ќ");
}
