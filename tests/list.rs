//! Locale-aware list formatting.
#![cfg(feature = "alloc")]

use intl::list::{ListStyle::*, format_list as fl};

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

#[test]
fn five_item_list_exact_output() {
    // Linear connector folding must be byte-identical to the prior O(N^2) fold.
    assert_eq!(
        fl("en", &["a", "b", "c", "d", "e"], And),
        "a, b, c, d, and e"
    );
    assert_eq!(fl("en", &["a", "b", "c", "d", "e"], Or), "a, b, c, d, or e");
    assert_eq!(
        fl("de", &["a", "b", "c", "d", "e"], And),
        "a, b, c, d und e"
    );
    assert_eq!(fl("fr", &["a", "b", "c", "d", "e"], And), "a, b, c, d et e");
}

#[test]
fn large_list_is_linear_time() {
    // 50k items: with the old O(N^2) fold this would copy ~1.25 billion bytes;
    // the linear single-pass build completes near-instantly. We assert the
    // structure (prefix, count of separators, suffix) rather than the full
    // multi-megabyte string.
    let items: Vec<String> = (0..50_000).map(|i| i.to_string()).collect();
    let refs: Vec<&str> = items.iter().map(String::as_str).collect();
    let out = fl("en", &refs, And);
    assert!(out.starts_with("0, 1, 2, "));
    assert!(out.ends_with(", and 49999"));
    // 49,999 ", " separators between the 50,000 items, but the last one is
    // ", and " — so 49,998 plain ", " plus the final ", and ".
    assert_eq!(out.matches(", and ").count(), 1);
    assert_eq!(out.matches(", ").count(), 49_999);
}
