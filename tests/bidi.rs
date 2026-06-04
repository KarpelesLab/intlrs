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
