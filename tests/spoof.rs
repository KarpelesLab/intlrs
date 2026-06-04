//! UTS #39 confusable / spoof detection.
#![cfg(feature = "alloc")]

use intl::unicode::spoof::{confusable, is_single_script, skeleton};

#[test]
fn confusable_skeletons() {
    // Cyrillic 'а' (U+0430) vs Latin 'a' — classic homograph.
    assert!(confusable("pаypal", "paypal"));
    assert_eq!(skeleton("pаypal"), skeleton("paypal"));

    // Identical strings are not "confusable" with themselves.
    assert!(!confusable("paypal", "paypal"));
    // Genuinely different words are not confusable.
    assert!(!confusable("apple", "orange"));
}

#[test]
fn single_script() {
    assert!(is_single_script("hello"));
    assert!(is_single_script("Москва"));
    assert!(is_single_script("abc123!")); // digits/punct are script-neutral
    assert!(is_single_script(""));
    // Latin 'a' mixed with Cyrillic letters is mixed-script.
    assert!(!is_single_script("pаypal")); // the 'а' here is Cyrillic
}
