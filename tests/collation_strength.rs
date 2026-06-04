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

#[test]
fn locale_tailoring() {
    use intl::unicode::collate::Tailoring;
    // Swedish: å ä ö sort after z.
    let sv = Tailoring::parse("&z < å < ä < ö").unwrap();
    assert_eq!(sv.compare("z", "å"), Ordering::Less);
    assert_eq!(sv.compare("å", "ä"), Ordering::Less);
    assert_eq!(sv.compare("ä", "ö"), Ordering::Less);
    assert_eq!(sv.compare("ö", "a"), Ordering::Greater); // ö after z, so after a
                                                         // A word list sorts correctly: "z" before "ångström".
    assert_eq!(sv.compare("zebra", "ångström"), Ordering::Less);
    // Uppercase tailored too.
    assert_eq!(sv.compare("Z", "Å"), Ordering::Less);
    // Danish/Norwegian: æ ø å after z.
    let da = Tailoring::parse("&z < æ < ø < å").unwrap();
    assert_eq!(da.compare("z", "æ"), Ordering::Less);
    assert_eq!(da.compare("æ", "ø"), Ordering::Less);
    assert_eq!(da.compare("ø", "å"), Ordering::Less);
    // In default DUCET, å sorts near a (before z) — tailoring changed that.
    assert_eq!(intl::unicode::collate::compare("å", "z"), Ordering::Less);
}

#[test]
fn tailoring_levels() {
    use intl::unicode::collate::{Strength, Tailoring};
    // Secondary tailoring: ö sorts after o but only at the secondary level
    // (so at primary strength they'd be equal). "&o << ö".
    let t = Tailoring::parse("&o << ö").unwrap();
    // Primary reordering still works (the earlier Swedish chain).
    let sv = Tailoring::parse("&z < å < ä < ö").unwrap();
    assert_eq!(sv.compare("z", "ä"), Ordering::Less);
    // Secondary: o < ö, and both share a primary (ö just after o).
    assert_eq!(t.compare("o", "ö"), Ordering::Less);
    assert_eq!(t.compare("oa", "öa"), Ordering::Less);
    // `=` identity: w sorts identical to v.
    let id = Tailoring::parse("&v = w").unwrap();
    let _ = Strength::Primary; // strength type is reachable
    assert_eq!(id.compare("v", "w"), Ordering::Equal);
}
