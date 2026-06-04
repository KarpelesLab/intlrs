//! `intl` — pure-Rust, `no_std` internationalization primitives.
//!
//! The crate is `#![no_std]` and does not use `alloc`. Functionality is grouped
//! into modules; today it provides:
//!
//! - [`unicode`] — Unicode rune analysis (General_Category and character
//!   predicates), with property tables compiled directly into `const fn` `match`
//!   lookups and feature-selectable codepoint ranges.
//!
//! ```
//! use intl::unicode::{general_category, GeneralCategory, CharExt};
//!
//! assert_eq!(general_category('A'), GeneralCategory::UppercaseLetter);
//! assert!('A'.is_uppercase());
//! ```
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod locale;
pub mod plural;
pub mod unicode;
