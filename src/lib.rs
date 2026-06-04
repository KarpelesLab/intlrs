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

// Embedded CLDR locale tables (no_std). Compiled in every configuration so the
// data layer never depends on `alloc`; the formatters that consume it are
// `alloc`-gated.
pub(crate) mod cldr;
pub mod display;
#[cfg(feature = "alloc")]
pub mod list;
#[cfg(feature = "alloc")]
pub mod locale;
#[cfg(feature = "alloc")]
pub mod message;
#[cfg(feature = "alloc")]
pub mod number;
pub mod plural;
#[cfg(feature = "alloc")]
pub mod relative;
pub mod unicode;
#[cfg(feature = "alloc")]
pub mod unit;
