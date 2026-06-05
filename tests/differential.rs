//! Differential test: cross-check the `unicode` case mappings and predicates
//! against the Rust standard library over the long-stable codepoint ranges
//! (U+0000..=U+2FFF: ASCII, Latin, IPA, Greek, Cyrillic, Armenian, Hebrew,
//! Arabic, …). These ranges have not changed in many Unicode versions, so they
//! match regardless of which Unicode version `std` was built against.
//!
//! Requires the `bmp` tier: the compared range reaches U+2FFF, so under a
//! narrower tier those codepoints would (correctly) read their neutral default
//! and not match `std`.
#![cfg(all(feature = "bmp", feature = "case"))]

use intl::unicode::{
    is_alphabetic, is_control, is_lowercase, is_uppercase, to_lowercase, to_uppercase,
};

#[test]
fn case_mapping_matches_std() {
    for cp in 0u32..=0x2FFF {
        let Some(c) = char::from_u32(cp) else {
            continue;
        };
        assert!(
            to_uppercase(c).eq(c.to_uppercase()),
            "to_uppercase mismatch at U+{cp:04X}"
        );
        assert!(
            to_lowercase(c).eq(c.to_lowercase()),
            "to_lowercase mismatch at U+{cp:04X}"
        );
    }
}

#[test]
fn predicates_match_std() {
    for cp in 0u32..=0x2FFF {
        let Some(c) = char::from_u32(cp) else {
            continue;
        };
        assert_eq!(is_uppercase(c), c.is_uppercase(), "is_uppercase U+{cp:04X}");
        assert_eq!(is_lowercase(c), c.is_lowercase(), "is_lowercase U+{cp:04X}");
        assert_eq!(
            is_alphabetic(c),
            c.is_alphabetic(),
            "is_alphabetic U+{cp:04X}"
        );
        assert_eq!(is_control(c), c.is_control(), "is_control U+{cp:04X}");
    }
}
