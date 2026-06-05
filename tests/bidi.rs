#![cfg(feature = "bidi")]
//! Bidi_Class + paragraph base direction (UAX #9 P2-P3).
use intl::unicode::{base_direction, bidi_class, BidiClass, Direction};

#[test]
fn classes() {
    assert_eq!(bidi_class('A'), BidiClass::L);
    assert_eq!(bidi_class('5'), BidiClass::EN);
    assert_eq!(bidi_class(' '), BidiClass::WS);
}

#[cfg(feature = "bmp")]
#[test]
fn rtl_classes_and_direction() {
    assert_eq!(bidi_class('א'), BidiClass::R); // Hebrew alef
    assert_eq!(bidi_class('ا'), BidiClass::AL); // Arabic alef
    assert!(bidi_class('א').is_rtl());

    assert_eq!(base_direction("hello"), Direction::LeftToRight);
    assert_eq!(base_direction("שלום"), Direction::RightToLeft);
    assert_eq!(base_direction("123 שלום"), Direction::RightToLeft); // numbers aren't strong
    assert_eq!(base_direction(""), Direction::LeftToRight);
}

#[cfg(feature = "alloc")]
#[test]
fn reorder_simple() {
    use intl::unicode::bidi::process;
    // Pure LTR: identity order, all level 0.
    let info = process("abc", None);
    assert_eq!(info.paragraph_level, 0);
    assert_eq!(info.visual_order, [0, 1, 2]);

    // "abc" + Hebrew "אבג" in an LTR paragraph: the RTL run is reversed.
    let info = process("abc\u{5D0}\u{5D1}\u{5D2}", None);
    assert_eq!(info.paragraph_level, 0);
    assert_eq!(info.visual_order, [0, 1, 2, 5, 4, 3]);
    assert_eq!(info.levels[3], Some(1)); // Hebrew is odd level

    // A Hebrew-first paragraph auto-detects RTL.
    let info = process("\u{5D0}\u{5D1} a", None);
    assert_eq!(info.paragraph_level, 1);
}
