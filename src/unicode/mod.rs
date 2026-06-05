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
//! | `bmp`    | `U+0000..=U+FFFF`          |
//! | `full`   | `U+0000..=U+10FFFF` (default) |
//!
//! Individual algorithms (normalization, segmentation, bidi, case, collation,
//! idna, confusables, identifiers) are each their own Cargo feature — all on by
//! default, each independently disable-able with `default-features = false`.
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

#[cfg(feature = "bidi")]
pub mod bidi;
#[cfg(feature = "case")]
pub mod case;
pub mod category;
#[cfg(feature = "collation")]
pub mod collate;
pub(crate) mod generated;
#[cfg(feature = "identifiers")]
pub mod ident;
#[cfg(feature = "idna")]
pub mod idna;
#[cfg(feature = "alloc")]
pub mod names;
#[cfg(feature = "normalization")]
pub mod normalize;
pub mod numeric;
mod predicates;
pub mod script;
#[cfg(feature = "segmentation")]
pub mod segment;
#[cfg(feature = "confusables")]
pub mod spoof;
pub mod width;

#[cfg(feature = "bidi")]
pub use bidi::{base_direction, bidi_class, BidiClass, Direction};
#[cfg(feature = "case")]
pub use case::{
    case_fold, fold, lowercase, to_lowercase, to_titlecase, to_uppercase, uppercase, CaseMapIter,
    CaseMapping,
};
#[cfg(all(feature = "case", feature = "alloc"))]
pub use case::{lowercase_str, lowercase_str_lang, titlecase, uppercase_str_lang};
pub use category::{GeneralCategory, Group};
#[cfg(feature = "collation")]
pub use collate::{
    compare, contains, find, index_bucket, index_labels, sort_key, AlternateHandling, Collator,
    Strength, Tailoring,
};
pub use generated::general_category::UNICODE_VERSION;
#[cfg(feature = "identifiers")]
pub use ident::{is_identifier, is_xid_continue, is_xid_start};
#[cfg(feature = "names")]
pub use names::name;
#[cfg(feature = "alloc")]
pub use names::{char_name, hangul_syllable_name};
#[cfg(feature = "normalization")]
pub use normalize::{
    canonical_combining_class, canonical_combining_class_u32, is_nfc, is_nfd, is_nfkc, is_nfkd,
    nfc, nfd, nfkc, nfkd, quick_check_nfc, quick_check_nfd, quick_check_nfkc, quick_check_nfkd,
    Decompositions, IsNormalized, Recompositions,
};
pub use numeric::{numeric_type, numeric_value, numeric_value_u32, NumericType, NumericValue};
pub use predicates::{
    age, bidi_mirror, block, changes_when_casefolded, changes_when_casemapped,
    changes_when_lowercased, changes_when_titlecased, changes_when_uppercased, general_category,
    general_category_u32, indic_positional_category, indic_syllabic_category, is_alphabetic,
    is_assigned, is_bidi_mirrored, is_control, is_dash, is_decimal_digit, is_default_ignorable,
    is_diacritic, is_format, is_hex_digit, is_join_control, is_letter, is_lowercase, is_mark,
    is_math, is_numeric, is_punctuation, is_quotation_mark, is_separator, is_symbol, is_uppercase,
    is_whitespace, joining_group, joining_type, CharExt, IndicPositionalCategory,
    IndicSyllabicCategory, JoiningGroup, JoiningType,
};
pub use script::{
    script, script_extensions, script_extensions_u32, script_u32, Script, ScriptExtensions,
};
#[cfg(feature = "segmentation")]
pub use segment::{
    graphemes, line_breaks, sentences, words, Graphemes, LineBreak, LineBreaks, Sentences, Words,
};
pub use width::{east_asian_width, east_asian_width_u32, EastAsianWidth};
