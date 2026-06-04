//! Unicode rune analysis driven by the official Unicode Character Database
//! (UCD).
//!
//! Character properties are compiled directly into Rust `match` dispatch by an
//! offline code generator (see `codegen/`), so lookups are `const fn`, allocate
//! nothing, and require no runtime initialization.
//!
//! # Range tiers
//!
//! Cargo features select how much of the codepoint space is compiled in, trading
//! coverage for binary size. The tiers are nested:
//!
//! | feature  | codepoints compiled        |
//! |----------|----------------------------|
//! | `ascii`  | `U+0000..=U+007F`          |
//! | `latin1` | `U+0000..=U+00FF`          |
//! | `bmp`    | `U+0000..=U+FFFF` (default) |
//! | `full`   | `U+0000..=U+10FFFF`        |
//!
//! Any codepoint outside the compiled range — like a genuinely unassigned one —
//! resolves to the neutral default: [`GeneralCategory::Unassigned`] / `false`.
//!
//! # Examples
//!
//! ```
//! use intl::unicode::{general_category, GeneralCategory, CharExt};
//!
//! assert_eq!(general_category('A'), GeneralCategory::UppercaseLetter);
//! assert!('A'.is_uppercase());
//! assert!('٣'.is_numeric()); // Arabic-Indic digit three
//! assert!(!'\u{0378}'.is_assigned()); // a reserved codepoint
//! ```

pub mod case;
pub mod category;
mod generated;
pub mod normalize;
pub mod numeric;
mod predicates;
pub mod script;
pub mod width;

pub use case::{case_fold, to_lowercase, to_titlecase, to_uppercase, CaseMapIter};
pub use category::{GeneralCategory, Group};
pub use generated::general_category::UNICODE_VERSION;
pub use normalize::{
    canonical_combining_class, canonical_combining_class_u32, nfc, nfd, nfkc, nfkd, Decompositions,
    Recompositions,
};
pub use numeric::{numeric_type, numeric_value, numeric_value_u32, NumericType, NumericValue};
pub use predicates::{
    general_category, general_category_u32, is_alphabetic, is_assigned, is_control,
    is_decimal_digit, is_format, is_letter, is_lowercase, is_mark, is_numeric, is_punctuation,
    is_separator, is_symbol, is_uppercase, is_whitespace, CharExt,
};
pub use script::{
    script, script_extensions, script_extensions_u32, script_u32, Script, ScriptExtensions,
};
pub use width::{east_asian_width, east_asian_width_u32, EastAsianWidth};
