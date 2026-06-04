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

pub mod bidi;
pub mod case;
pub mod category;
#[cfg(feature = "alloc")]
pub mod collate;
pub(crate) mod generated;
pub mod ident;
#[cfg(feature = "alloc")]
pub mod idna;
#[cfg(feature = "alloc")]
pub mod names;
pub mod normalize;
pub mod numeric;
mod predicates;
pub mod script;
pub mod segment;
#[cfg(feature = "alloc")]
pub mod spoof;
pub mod width;

pub use bidi::{base_direction, bidi_class, BidiClass, Direction};
pub use case::{
    case_fold, fold, lowercase, to_lowercase, to_titlecase, to_uppercase, uppercase, CaseMapIter,
    CaseMapping,
};
#[cfg(feature = "alloc")]
pub use case::{lowercase_str, lowercase_str_lang, titlecase, uppercase_str_lang};
pub use category::{GeneralCategory, Group};
#[cfg(feature = "alloc")]
pub use collate::{compare, sort_key, AlternateHandling, Collator};
pub use generated::general_category::UNICODE_VERSION;
pub use ident::{is_identifier, is_xid_continue, is_xid_start};
#[cfg(feature = "alloc")]
pub use names::{char_name, hangul_syllable_name};
pub use normalize::{
    canonical_combining_class, canonical_combining_class_u32, is_nfc, is_nfd, is_nfkc, is_nfkd,
    nfc, nfd, nfkc, nfkd, quick_check_nfc, quick_check_nfd, quick_check_nfkc, quick_check_nfkd,
    Decompositions, IsNormalized, Recompositions,
};
pub use numeric::{numeric_type, numeric_value, numeric_value_u32, NumericType, NumericValue};
pub use predicates::{
    age, block, general_category, general_category_u32, indic_positional_category,
    indic_syllabic_category, is_alphabetic, is_assigned, is_control, is_dash, is_decimal_digit,
    is_default_ignorable, is_diacritic, is_format, is_hex_digit, is_join_control, is_letter,
    is_lowercase, is_mark, is_math, is_numeric, is_punctuation, is_quotation_mark, is_separator,
    is_symbol, is_uppercase, is_whitespace, joining_type, CharExt, IndicPositionalCategory,
    IndicSyllabicCategory, JoiningType,
};
pub use script::{
    script, script_extensions, script_extensions_u32, script_u32, Script, ScriptExtensions,
};
pub use segment::{
    graphemes, line_breaks, sentences, words, Graphemes, LineBreak, LineBreaks, Sentences, Words,
};
pub use width::{east_asian_width, east_asian_width_u32, EastAsianWidth};
