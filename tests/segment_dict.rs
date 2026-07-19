#![cfg(feature = "segmentation-dict")]
//! Dictionary-based Thai word segmentation (feature `segmentation-dict`).
//!
//! The expected segmentations below were produced by V8's
//! `Intl.Segmenter('th', {granularity:'word'})` (which uses the same ICU
//! dictionary + ThaiBreakEngine this port reproduces). Filtering out spaces,
//! every case here matches V8 exactly.

use intl::unicode::words;

fn w(s: &str) -> Vec<&str> {
    words(s).collect()
}

#[test]
fn thai_classic_sentence() {
    // The canonical ICU/Intl.Segmenter Thai example.
    assert_eq!(
        w("ไปกินข้าวที่ร้านอาหารกับเพื่อน"),
        ["ไป", "กิน", "ข้าว", "ที่", "ร้าน", "อาหาร", "กับ", "เพื่อน"]
    );
}

#[test]
fn thai_real_sentences_match_v8() {
    let cases: &[(&str, &[&str])] = &[
        ("ฉันรักภาษาไทย", &["ฉัน", "รัก", "ภาษา", "ไทย"]),
        ("รถไฟฟ้ามหานคร", &["รถไฟฟ้า", "มหานคร"]),
        (
            "การเดินทางไปต่างประเทศ",
            &["การ", "เดิน", "ทาง", "ไป", "ต่าง", "ประเทศ"],
        ),
        (
            "ประเทศไทยมีประชากรหกสิบล้านคน",
            &["ประเทศไทย", "มี", "ประชากร", "หก", "สิบ", "ล้าน", "คน"],
        ),
        (
            "โปรแกรมเมอร์เขียนโค้ดทุกวัน",
            &["โปรแกรมเมอร์", "เขียน", "โค้ด", "ทุก", "วัน"],
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(&w(input), expected, "input: {input}");
    }
}

#[test]
fn short_thai_run_stays_whole() {
    // Runs of at most THAI_MIN_WORD_SPAN (4) code points are left undivided,
    // exactly as ICU's early-out does.
    assert_eq!(w("ก"), ["ก"]);
    assert_eq!(w("ไป"), ["ไป"]);
    assert_eq!(w("กก"), ["กก"]);
    assert_eq!(w("ข้าว"), ["ข้าว"]); // 4 code points
    assert_eq!(w("น้ำ"), ["น้ำ"]);
}

#[test]
fn mixed_script_runs_break_at_boundaries() {
    // Spaces / Latin / digits around a Thai run: the Thai run is subdivided by
    // the dictionary; the surrounding UAX #29 tokens are untouched.
    assert_eq!(w("สวัสดีครับ ยินดีต้อนรับ"), ["สวัสดี", "ครับ", " ", "ยินดี", "ต้อนรับ"]);
    let v: Vec<&str> = w("hello ไปกินข้าว world");
    assert_eq!(v, ["hello", " ", "ไป", "กิน", "ข้าว", " ", "world"]);
}

#[test]
fn non_dictionary_scripts_unaffected() {
    // Pure ASCII / CJK / Latin behave exactly as plain UAX #29.
    assert_eq!(w("Hello, world!"), ["Hello", ",", " ", "world", "!"]);
    assert_eq!(w("one two three"), ["one", " ", "two", " ", "three"]);
}

#[test]
fn thai_digits_are_not_dictionary_chars() {
    // Thai DIGIT characters (U+0E50..) are Numeric, not dictionary (SA) chars,
    // so a Thai numeral run is one Numeric token, never fed to the dictionary.
    assert_eq!(w("๑๒๓"), ["๑๒๓"]);
}
