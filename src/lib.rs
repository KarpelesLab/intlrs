//! `intl` — a pure-Rust, `#![no_std]` analog of ICU.
//!
//! The crate is always `#![no_std]`. The Unicode layer and several locale
//! helpers need no allocator; the `alloc` feature enables the allocating APIs
//! (most formatters). UCD and CLDR data are compiled by an offline code
//! generator into committed tables — `const fn` `match` lookups for the Unicode
//! properties and the CLDR tables that have been migrated to generated code,
//! embedded binary blobs for the rest of the CLDR data — so lookups allocate
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

// Embedded CLDR locale tables (no_std), gated by the `_cldr` marker (enabled by
// any CLDR-backed formatter). Each formatter module — and the table(s) it embeds
// — is behind its own feature so a disabled formatter contributes no code or
// data. `calendar` (arithmetic) and `plural` (rules) need no data and are always
// available.
pub mod calendar;
#[cfg(feature = "_cldr")]
pub(crate) mod cldr;
#[cfg(feature = "datetime")]
pub mod datetime;
#[cfg(feature = "displaynames")]
pub mod display;
#[cfg(feature = "list")]
pub mod list;
#[cfg(feature = "locale")]
pub mod locale;
#[cfg(feature = "message")]
pub mod message;
#[cfg(feature = "number")]
pub mod number;
pub mod plural;
#[cfg(feature = "relative")]
pub mod relative;
#[cfg(feature = "spellout")]
pub mod spellout;
// Time-zone support (POSIX TZ + GMT offsets + the IANA bridge) ships with
// `datetime`; the two modules are mutually dependent.
#[cfg(feature = "datetime")]
pub mod timezone;
#[cfg(feature = "transliterate")]
pub mod translit;
pub mod unicode;
#[cfg(feature = "units")]
pub mod unit;
