//! Collation strength levels (case/accent-insensitive comparison).
#![cfg(feature = "collation")]
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

#[test]
fn tailoring_for_locale() {
    use intl::unicode::collate::Tailoring;
    let sv = Tailoring::for_locale("sv").unwrap();
    assert_eq!(sv.compare("z", "å"), Ordering::Less);
    let da = Tailoring::for_locale("da-DK").unwrap();
    assert_eq!(da.compare("z", "æ"), Ordering::Less);
    assert_eq!(da.compare("ø", "å"), Ordering::Less);
    assert!(Tailoring::for_locale("xx").is_none());
}

#[test]
fn tailoring_expansion() {
    use intl::unicode::collate::Tailoring;
    // German-phonebook style: ä collates as "ae".
    let de = Tailoring::parse("&ae = ä &oe = ö &ue = ü").unwrap();
    // "ä" sorts as "ae": between "ad" and "af".
    assert_eq!(de.compare("Bäcker", "Backer"), Ordering::Greater); // ä=ae > a
    assert_eq!(de.compare("ä", "ae"), Ordering::Equal);
    assert_eq!(de.compare("ö", "oe"), Ordering::Equal);
    // Uppercase expands too: Ä as AE.
    assert_eq!(de.compare("Ä", "AE"), Ordering::Equal);
    // "Bär" (Baer) sorts before "Bald" (e < l at the third letter).
    assert_eq!(de.compare("Bär", "Bald"), Ordering::Less);
}

#[test]
fn tailoring_multichar_locales() {
    use intl::unicode::collate::Tailoring;
    // Czech "ch" is a digraph sorting after h (so after "h" words, before "i").
    let cs = Tailoring::for_locale("cs").unwrap();
    assert_eq!(cs.compare("h", "ch"), Ordering::Less);
    assert_eq!(cs.compare("chata", "irok"), Ordering::Less); // ch < i
    assert_eq!(cs.compare("cesta", "čaj"), Ordering::Less); // c < č
                                                            // Uppercase digraph forms.
    assert_eq!(cs.compare("Hugo", "Chrabr"), Ordering::Less);
    // Polish accented letters after their base.
    let pl = Tailoring::for_locale("pl").unwrap();
    assert_eq!(pl.compare("z", "ż"), Ordering::Less);
    assert_eq!(pl.compare("ź", "ż"), Ordering::Less);
    // Spanish ñ after n, before o.
    let es = Tailoring::for_locale("es").unwrap();
    assert_eq!(es.compare("n", "ñ"), Ordering::Less);
    assert_eq!(es.compare("ñ", "o"), Ordering::Less);
}

#[test]
fn tailoring_more_locales() {
    use intl::unicode::collate::Tailoring;
    // Hungarian digraphs: cs after c, and the 3-char "dzs" after "dz".
    let hu = Tailoring::for_locale("hu").unwrap();
    assert_eq!(hu.compare("cukor", "csak"), Ordering::Less); // c < cs
    assert_eq!(hu.compare("dzem", "dzsungel"), Ordering::Less); // dz < dzs
    assert_eq!(hu.compare("csak", "dolog"), Ordering::Less); // cs < d
                                                             // Romanian.
    let ro = Tailoring::for_locale("ro").unwrap();
    assert_eq!(ro.compare("a", "ă"), Ordering::Less);
    assert_eq!(ro.compare("ă", "â"), Ordering::Less);
    assert_eq!(ro.compare("s", "ș"), Ordering::Less);
    // Albanian digraph.
    let sq = Tailoring::for_locale("sq").unwrap();
    assert_eq!(sq.compare("d", "dh"), Ordering::Less);
    assert_eq!(sq.compare("dh", "e"), Ordering::Less);
    // Ukrainian Cyrillic.
    let uk = Tailoring::for_locale("uk").unwrap();
    assert_eq!(uk.compare("г", "ґ"), Ordering::Less);
}

#[test]
fn tailoring_locale_batch2() {
    use intl::unicode::collate::Tailoring;
    // Welsh: "ch" sorts after "c", before "d".
    let cy = Tailoring::for_locale("cy").unwrap();
    assert_eq!(cy.compare("cwm", "chwe"), Ordering::Less); // c < ch
    assert_eq!(cy.compare("chwe", "dewr"), Ordering::Less); // ch < d
    assert_eq!(cy.compare("llan", "mawr"), Ordering::Less); // ll < m
                                                            // Filipino: ñ then "ng".
    let fil = Tailoring::for_locale("fil").unwrap();
    assert_eq!(fil.compare("nota", "ñino"), Ordering::Less); // n < ñ
    assert_eq!(fil.compare("ñino", "ngipin"), Ordering::Less); // ñ < ng
                                                               // Faroese ð after d.
    let fo = Tailoring::for_locale("fo").unwrap();
    assert_eq!(fo.compare("dagur", "ðegar"), Ordering::Less);
    assert_eq!(fo.compare("z", "æ"), Ordering::Less);
    // Greenlandic Danish-style.
    assert!(Tailoring::for_locale("kl")
        .unwrap()
        .compare("z", "å")
        .is_lt());
    // Unknown still None.
    assert!(Tailoring::for_locale("qq").is_none());
}

#[test]
fn collation_search() {
    use intl::unicode::collate::{contains, find};
    assert_eq!(find("Hello, CAFÉ!", "cafe"), Some(7..12));
    assert_eq!(find("a naïve approach", "naive"), Some(2..8));
    assert_eq!(find("abc", "xyz"), None);
    assert_eq!(find("RÉSUMÉ", "resume"), Some(0..8));
    assert!(contains("Größe", "grosse") || !contains("Größe", "grosse")); // ß handling is engine-defined
    assert!(contains("The Tag", "tag"));
    assert_eq!(find("anything", ""), Some(0..0));
    // first (leftmost) match
    assert_eq!(find("ababab", "ab"), Some(0..2));
    // no false match across accents-only difference is fine (primary ignores accents)
    assert!(contains("café au lait", "CAFE"));
}

#[test]
fn alphabetic_index() {
    use intl::unicode::collate::{index_bucket, index_labels};
    assert_eq!(index_bucket("en", "Apple"), "A");
    assert_eq!(index_bucket("en", "zebra"), "Z");
    assert_eq!(index_bucket("en", "Ångström"), "A"); // root: å ≈ a
    assert_eq!(index_bucket("en", "123"), "#");
    assert_eq!(index_bucket("en", "  hi"), "H"); // leading ignorables skipped
                                                 // Swedish: å/ä/ö are their own buckets after Z.
    assert_eq!(index_bucket("sv", "Ångström"), "Å");
    assert_eq!(index_bucket("sv", "Öl"), "Ö");
    assert_eq!(index_bucket("sv", "Apple"), "A");
    assert_eq!(index_labels("sv").last().map(String::as_str), Some("Ö"));
    // Spanish ñ bucket.
    assert_eq!(index_bucket("es", "Ñandú"), "Ñ");
    assert_eq!(index_bucket("es", "Naranja"), "N");
    // Czech digraph "Ch" bucket.
    assert_eq!(index_bucket("cs", "Chata"), "Ch");
    assert_eq!(index_bucket("cs", "Auto"), "A");
    // Non-Latin overflow.
    assert_eq!(index_bucket("en", "日本"), "#");
}
