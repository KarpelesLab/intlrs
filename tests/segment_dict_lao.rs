#![cfg(feature = "segmentation-dict-lao")]
//! Dictionary-based Lao word segmentation (feature `segmentation-dict-lao`).
//!
//! The expected segmentations were produced by V8's
//! `Intl.Segmenter('lo', {granularity:'word'})`, which uses the same ICU
//! `laodict` + `LaoBreakEngine` this module reuses (the Thai engine with Lao
//! character classes and no suffix handling). Every case here matches V8
//! exactly, tokens and all.
//!
//! Known limitation, shared with the Thai engine and pre-existing: a digit
//! embedded in a Lao run (e.g. "…ມີ3ຄົນ") is segmented by the UAX #29 rules at
//! the digit boundary, whereas ICU's full rule-based iterator folds the digit
//! into the surrounding word. This is a UAX #29 / dictionary integration quirk,
//! not specific to Lao.

use intl::unicode::words;

fn w(s: &str) -> Vec<&str> {
    words(s).collect()
}

#[test]
fn lao_sentences_match_v8() {
    let cases: &[(&str, &[&str])] = &[
        ("ຂ້ອຍຮັກພາສາລາວ", &["ຂ້ອຍ", "ຮັກ", "ພາສາ", "ລາວ"]),
        ("ສະບາຍດີຕອນເຊົ້າ", &["ສະບາຍດີ", "ຕອນເຊົ້າ"]),
        ("ປະເທດລາວມີປະຊາຊົນ", &["ປະເທດ", "ລາວ", "ມີ", "ປະຊາຊົນ"]),
        ("ຂ້ອຍໄປໂຮງຮຽນທຸກມື້", &["ຂ້ອຍ", "ໄປ", "ໂຮງຮຽນ", "ທຸກມື້"]),
        ("ຄົນລາວກິນເຂົ້າໜຽວ", &["ຄົນ", "ລາວ", "ກິນເຂົ້າ", "ໜຽວ"]),
        (
            "ພາສາລາວເປັນພາສາທາງການ",
            &["ພາສາ", "ລາວ", "ເປັນ", "ພາສາ", "ທາງການ"],
        ),
        ("ຂ້ອຍມັກກິນອາຫານລາວ", &["ຂ້ອຍ", "ມັກ", "ກິນ", "ອາຫານ", "ລາວ"]),
        ("ວັນນີ້ອາກາດດີຫຼາຍ", &["ວັນນີ້", "ອາກາດ", "ດີ", "ຫຼາຍ"]),
        ("ພວກເຮົາຮຽນຮູ້ຮ່ວມກັນ", &["ພວກ", "ເຮົາ", "ຮຽນຮູ້", "ຮ່ວມກັນ"]),
        ("ພາສາລາວແລະພາສາໄທ", &["ພາສາ", "ລາວ", "ແລະ", "ພາສາ", "ໄທ"]),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn lao_mixed_with_punctuation() {
    // Surrounding punctuation/space keeps its UAX #29 behavior; the Lao run is
    // subdivided by the dictionary.
    assert_eq!(w("ສະບາຍດີ, ໂລກ!"), ["ສະບາຍດີ", ",", " ", "ໂລກ", "!"]);
}

#[test]
fn thai_and_lao_runs_are_independent() {
    // Adjacent Thai and Lao runs each go to their own parameters (disjoint
    // U+0E00 / U+0E80 blocks).
    assert_eq!(w("ໄທ"), ["ໄທ"]); // a short Lao run stays whole
    assert_eq!(
        w("ไปกินข้าว"),
        ["ไป", "กิน", "ข้าว"] // Thai still segments as before
    );
}
