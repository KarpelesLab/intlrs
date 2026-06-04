//! Locale-aware list formatting.
#![cfg(feature = "alloc")]

use intl::list::{format_list as fl, ListStyle::*};

#[test]
fn english_lists() {
    assert_eq!(fl("en", &[], And), "");
    assert_eq!(fl("en", &["a"], And), "a");
    assert_eq!(fl("en", &["a", "b"], And), "a and b");
    assert_eq!(fl("en", &["a", "b", "c"], And), "a, b, and c");
    assert_eq!(fl("en", &["a", "b", "c", "d"], And), "a, b, c, and d");
    assert_eq!(fl("en", &["a", "b"], Or), "a or b");
    assert_eq!(fl("en", &["a", "b", "c"], Or), "a, b, or c");
}

#[test]
fn other_locales_and_fallback() {
    assert_eq!(fl("de", &["a", "b", "c"], And), "a, b und c");
    assert_eq!(fl("fr", &["a", "b", "c"], And), "a, b et c");
    assert_eq!(fl("en-GB", &["a", "b"], And), "a and b"); // region fallback
}
