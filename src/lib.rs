//! `intl` — a pure-Rust, `#![no_std]` analog of ICU.
//!
//! The crate is always `#![no_std]`. The Unicode layer and several locale
//! helpers need no allocator; the `alloc` feature enables the allocating APIs
//! (most formatters). UCD and CLDR data are compiled by an offline code
//! generator into committed tables — `const fn` `match` lookups for the Unicode
//! properties and embedded binary blobs for the CLDR data — so lookups allocate
//! nothing and need no runtime initialization.
//!
//! Modules:
//!
//! - [`unicode`] — General_Category and predicates, scripts, East Asian Width,
//!   numeric values, case mapping/folding, normalization (UAX #15), collation
//!   (UTS #10), segmentation (UAX #29), line breaking (UAX #14), bidi (UAX #9),
//!   identifiers (UAX #31), confusables (UTS #39), IDNA (UTS #46). Property
//!   tables are gated by feature-selectable codepoint ranges.
//! - [`locale`] — BCP-47 parsing/canonicalization, likely-subtags
//!   maximize/minimize, negotiation. *(alloc)*
//! - [`plural`] — CLDR cardinal/ordinal plural categories.
//! - [`number`] — decimal / percent / currency formatting. *(alloc)*
//! - [`datetime`] — Gregorian date/time formatting (styles, skeletons), ISO-8601
//!   I/O, date arithmetic, localized GMT offsets, Islamic/Persian rendering.
//!   *(alloc)*
//! - [`calendar`] — Gregorian / Islamic / Persian / Hebrew / Japanese / ISO-week
//!   date conversions.
//! - [`relative`], [`list`], `unit`, [`message`] — relative-time, list,
//!   measurement-unit, and MessageFormat formatting. *(alloc)*
//! - [`display`] — locale display names (language / region).
//!
//! ```
//! use intl::unicode::{general_category, GeneralCategory};
//! assert_eq!(general_category('A'), GeneralCategory::UppercaseLetter);
//! ```
#![no_std]
#![forbid(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Embedded CLDR locale tables (no_std). Compiled in every configuration so the
// data layer never depends on `alloc`; the formatters that consume it are
// `alloc`-gated.
pub mod calendar;
pub(crate) mod cldr;
#[cfg(feature = "alloc")]
pub mod datetime;
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
#[cfg(feature = "alloc")]
pub mod spellout;
#[cfg(feature = "alloc")]
pub mod timezone;
#[cfg(feature = "alloc")]
pub mod translit;
pub mod unicode;
#[cfg(feature = "alloc")]
pub mod unit;
