//! East Asian Width lookups.

use intl::unicode::{east_asian_width as eaw, EastAsianWidth as W};

#[test]
fn ascii_width() {
    assert_eq!(eaw('A'), W::Narrow);
    assert_eq!(eaw('z'), W::Narrow);
    assert_eq!(eaw('\t'), W::Neutral); // control U+0009
    assert!(!eaw('A').is_wide());
    assert_eq!(W::Wide.abbr(), "W");
    assert_eq!(W::Narrow.abbr(), "Na");
}

#[cfg(feature = "latin1")]
#[test]
fn latin1_width() {
    assert_eq!(eaw('§'), W::Ambiguous); // U+00A7
}

#[cfg(feature = "bmp")]
#[test]
fn bmp_width() {
    assert_eq!(eaw('中'), W::Wide); // U+4E2D (CJK)
    assert_eq!(eaw('Ａ'), W::Fullwidth); // U+FF21
    assert_eq!(eaw('｡'), W::Halfwidth); // U+FF61
    assert_eq!(eaw('①'), W::Ambiguous); // U+2460
    assert!(eaw('中').is_wide() && eaw('Ａ').is_wide());
}
