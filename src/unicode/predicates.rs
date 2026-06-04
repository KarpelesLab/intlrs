//! Boolean character predicates.
//!
//! Each predicate is available both as a free `const fn` taking a [`char`] and
//! as a method on the [`CharExt`] trait. Predicates backed by a derived Unicode
//! property (`is_alphabetic`, `is_uppercase`, `is_lowercase`, `is_whitespace`)
//! consult their own generated tables; the rest are derived from the
//! [`GeneralCategory`].
//!
//! All predicates are total: a codepoint outside the compiled range tier (or one
//! that is genuinely unassigned) reports its neutral value (`false`, or
//! [`GeneralCategory::Unassigned`]).

use super::category::GeneralCategory;
use super::generated::binary_props;
use super::generated::general_category::general_category as gc_raw;
pub use super::generated::properties::{
    IndicPositionalCategory, IndicSyllabicCategory, JoiningGroup, JoiningType,
};

/// The [`GeneralCategory`] of `c`.
#[inline]
#[must_use]
pub const fn general_category(c: char) -> GeneralCategory {
    gc_raw(c as u32)
}

/// The [`GeneralCategory`] of an arbitrary Unicode scalar value, including
/// values that are not valid [`char`]s (e.g. surrogate codepoints).
#[inline]
#[must_use]
pub const fn general_category_u32(cp: u32) -> GeneralCategory {
    gc_raw(cp)
}

/// `Alphabetic` derived core property (broader than the `L*` categories: it also
/// includes `Other_Alphabetic` codepoints such as many combining marks).
#[inline]
#[must_use]
pub const fn is_alphabetic(c: char) -> bool {
    binary_props::alphabetic(c as u32)
}

/// `Uppercase` derived core property (includes `Other_Uppercase` beyond `Lu`).
#[inline]
#[must_use]
pub const fn is_uppercase(c: char) -> bool {
    binary_props::uppercase(c as u32)
}

/// `Lowercase` derived core property (includes `Other_Lowercase` beyond `Ll`).
#[inline]
#[must_use]
pub const fn is_lowercase(c: char) -> bool {
    binary_props::lowercase(c as u32)
}

/// `White_Space` property (e.g. space, tab, newline, NBSP) — not the same as the
/// `Z*` separator categories.
#[inline]
#[must_use]
pub const fn is_whitespace(c: char) -> bool {
    binary_props::white_space(c as u32)
}

/// Any letter (`L*`).
#[inline]
#[must_use]
pub const fn is_letter(c: char) -> bool {
    general_category(c).is_letter()
}

/// Any mark (`M*`).
#[inline]
#[must_use]
pub const fn is_mark(c: char) -> bool {
    general_category(c).is_mark()
}

/// Any number (`N*`: decimal, letter, or other).
#[inline]
#[must_use]
pub const fn is_numeric(c: char) -> bool {
    general_category(c).is_number()
}

/// A decimal digit (`Nd` only).
#[inline]
#[must_use]
pub const fn is_decimal_digit(c: char) -> bool {
    matches!(general_category(c), GeneralCategory::DecimalNumber)
}

/// Any punctuation (`P*`).
#[inline]
#[must_use]
pub const fn is_punctuation(c: char) -> bool {
    general_category(c).is_punctuation()
}

/// Any symbol (`S*`).
#[inline]
#[must_use]
pub const fn is_symbol(c: char) -> bool {
    general_category(c).is_symbol()
}

/// Any separator (`Z*`).
#[inline]
#[must_use]
pub const fn is_separator(c: char) -> bool {
    general_category(c).is_separator()
}

/// A control character (`Cc`).
#[inline]
#[must_use]
pub const fn is_control(c: char) -> bool {
    matches!(general_category(c), GeneralCategory::Control)
}

/// A format character (`Cf`).
#[inline]
#[must_use]
pub const fn is_format(c: char) -> bool {
    matches!(general_category(c), GeneralCategory::Format)
}

/// `true` if `c` is assigned a category other than `Cn` (Unassigned). Note that
/// a codepoint outside the compiled range tier reports `false` here.
#[inline]
#[must_use]
pub const fn is_assigned(c: char) -> bool {
    general_category(c).is_assigned()
}

/// `Math` property (mathematical symbols and operators).
#[inline]
#[must_use]
pub const fn is_math(c: char) -> bool {
    binary_props::math(c as u32)
}

/// `Default_Ignorable_Code_Point` (e.g. ZWJ, variation selectors, soft hyphen) —
/// characters that should be ignored in rendering when unsupported.
#[inline]
#[must_use]
pub const fn is_default_ignorable(c: char) -> bool {
    binary_props::default_ignorable(c as u32)
}

/// `Changes_When_Lowercased`: `c` is altered by full lowercasing.
#[inline]
#[must_use]
pub const fn changes_when_lowercased(c: char) -> bool {
    binary_props::changes_when_lowercased(c as u32)
}

/// `Changes_When_Uppercased`: `c` is altered by full uppercasing.
#[inline]
#[must_use]
pub const fn changes_when_uppercased(c: char) -> bool {
    binary_props::changes_when_uppercased(c as u32)
}

/// `Changes_When_Titlecased`: `c` is altered by full titlecasing.
#[inline]
#[must_use]
pub const fn changes_when_titlecased(c: char) -> bool {
    binary_props::changes_when_titlecased(c as u32)
}

/// `Changes_When_Casefolded`: `c` is altered by case folding.
#[inline]
#[must_use]
pub const fn changes_when_casefolded(c: char) -> bool {
    binary_props::changes_when_casefolded(c as u32)
}

/// `Changes_When_Casemapped`: `c` is altered by any of lower/upper/title-casing.
#[inline]
#[must_use]
pub const fn changes_when_casemapped(c: char) -> bool {
    binary_props::changes_when_casemapped(c as u32)
}

/// The Unicode `Age` of `c`: the `(major, minor)` version in which the codepoint
/// was assigned, or `None` if it is unassigned (in the compiled range).
///
/// ```
/// use intl::unicode::age;
/// assert_eq!(age('A'), Some((1, 1)));
/// assert_eq!(age('\u{20BF}'), Some((10, 0))); // ₿ BITCOIN SIGN, Unicode 10.0
/// assert_eq!(age('\u{E0000}'), None); // unassigned
/// ```
#[inline]
#[must_use]
pub const fn age(c: char) -> Option<(u8, u8)> {
    crate::unicode::generated::properties::age(c as u32)
}

/// The Unicode `Block` name of `c` (e.g. `"Basic Latin"`, `"CJK Unified
/// Ideographs"`), or `"No_Block"` if the codepoint is in no assigned block.
///
/// ```
/// use intl::unicode::block;
/// assert_eq!(block('A'), "Basic Latin");
/// assert_eq!(block('日'), "CJK Unified Ideographs");
/// assert_eq!(block('\u{0590}'), "Hebrew");
/// ```
#[inline]
#[must_use]
pub const fn block(c: char) -> &'static str {
    crate::unicode::generated::properties::block(c as u32)
}

/// The `Joining_Type` of `c` (Arabic/Syriac cursive joining behavior, UAX #9).
///
/// ```
/// use intl::unicode::{joining_type, JoiningType};
/// assert_eq!(joining_type('\u{0628}'), JoiningType::DualJoining);  // ARABIC BEH
/// assert_eq!(joining_type('\u{0627}'), JoiningType::RightJoining); // ARABIC ALEF
/// assert_eq!(joining_type('\u{0640}'), JoiningType::JoinCausing);  // TATWEEL
/// assert_eq!(joining_type('A'), JoiningType::NonJoining);
/// ```
#[inline]
#[must_use]
pub const fn joining_type(c: char) -> JoiningType {
    crate::unicode::generated::properties::joining_type(c as u32)
}

/// The `Joining_Group` of `c` (the Arabic/Syriac letter shaping class, UAX #9) —
/// `NoJoiningGroup` for non-joining characters.
///
/// ```
/// use intl::unicode::{joining_group, JoiningGroup};
/// assert_eq!(joining_group('\u{0628}'), JoiningGroup::Beh);  // ARABIC BEH
/// assert_eq!(joining_group('\u{0627}'), JoiningGroup::Alef); // ARABIC ALEF
/// assert_eq!(joining_group('A'), JoiningGroup::NoJoiningGroup);
/// ```
#[inline]
#[must_use]
pub const fn joining_group(c: char) -> JoiningGroup {
    crate::unicode::generated::properties::joining_group(c as u32)
}

/// The `Indic_Syllabic_Category` of `c` (UAX #44) — the structural role a
/// character plays in Indic-style scripts; `Other` for everything else.
///
/// ```
/// use intl::unicode::{indic_syllabic_category, IndicSyllabicCategory};
/// assert_eq!(indic_syllabic_category('\u{0915}'), IndicSyllabicCategory::Consonant); // DEVANAGARI KA
/// assert_eq!(indic_syllabic_category('\u{094D}'), IndicSyllabicCategory::Virama);    // DEVANAGARI VIRAMA
/// assert_eq!(indic_syllabic_category('A'), IndicSyllabicCategory::Other);
/// ```
#[inline]
#[must_use]
pub const fn indic_syllabic_category(c: char) -> IndicSyllabicCategory {
    crate::unicode::generated::properties::indic_syllabic_category(c as u32)
}

/// The `Indic_Positional_Category` of `c` (UAX #44) — where a dependent
/// character sits relative to its base; `NotApplicable` for everything else.
///
/// ```
/// use intl::unicode::{indic_positional_category, IndicPositionalCategory};
/// assert_eq!(indic_positional_category('\u{093F}'), IndicPositionalCategory::Left); // DEVANAGARI VOWEL SIGN I
/// assert_eq!(indic_positional_category('A'), IndicPositionalCategory::NotApplicable);
/// ```
#[inline]
#[must_use]
pub const fn indic_positional_category(c: char) -> IndicPositionalCategory {
    crate::unicode::generated::properties::indic_positional_category(c as u32)
}

/// `Dash` property (dash punctuation and dash symbols).
#[inline]
#[must_use]
pub const fn is_dash(c: char) -> bool {
    binary_props::dash(c as u32)
}

/// `Diacritic` property (accents and other modifying marks).
#[inline]
#[must_use]
pub const fn is_diacritic(c: char) -> bool {
    binary_props::diacritic(c as u32)
}

/// `Hex_Digit` property (characters usable as hexadecimal digits, ASCII and
/// fullwidth).
#[inline]
#[must_use]
pub const fn is_hex_digit(c: char) -> bool {
    binary_props::hex_digit(c as u32)
}

/// `Quotation_Mark` property.
#[inline]
#[must_use]
pub const fn is_quotation_mark(c: char) -> bool {
    binary_props::quotation_mark(c as u32)
}

/// `Join_Control` property (ZWNJ and ZWJ).
#[inline]
#[must_use]
pub const fn is_join_control(c: char) -> bool {
    binary_props::join_control(c as u32)
}

/// Character-classification methods mirroring the free functions in this module.
///
/// Implemented for [`char`]; bring it into scope to call e.g. `'x'.is_letter()`.
pub trait CharExt {
    /// See [`general_category`].
    fn general_category(&self) -> GeneralCategory;
    /// See [`is_alphabetic`].
    fn is_alphabetic(&self) -> bool;
    /// See [`is_uppercase`].
    fn is_uppercase(&self) -> bool;
    /// See [`is_lowercase`].
    fn is_lowercase(&self) -> bool;
    /// See [`is_whitespace`].
    fn is_whitespace(&self) -> bool;
    /// See [`is_letter`].
    fn is_letter(&self) -> bool;
    /// See [`is_mark`].
    fn is_mark(&self) -> bool;
    /// See [`is_numeric`].
    fn is_numeric(&self) -> bool;
    /// See [`is_decimal_digit`].
    fn is_decimal_digit(&self) -> bool;
    /// See [`is_punctuation`].
    fn is_punctuation(&self) -> bool;
    /// See [`is_symbol`].
    fn is_symbol(&self) -> bool;
    /// See [`is_separator`].
    fn is_separator(&self) -> bool;
    /// See [`is_control`].
    fn is_control(&self) -> bool;
    /// See [`is_format`].
    fn is_format(&self) -> bool;
    /// See [`is_assigned`].
    fn is_assigned(&self) -> bool;
}

impl CharExt for char {
    #[inline]
    fn general_category(&self) -> GeneralCategory {
        general_category(*self)
    }
    #[inline]
    fn is_alphabetic(&self) -> bool {
        is_alphabetic(*self)
    }
    #[inline]
    fn is_uppercase(&self) -> bool {
        is_uppercase(*self)
    }
    #[inline]
    fn is_lowercase(&self) -> bool {
        is_lowercase(*self)
    }
    #[inline]
    fn is_whitespace(&self) -> bool {
        is_whitespace(*self)
    }
    #[inline]
    fn is_letter(&self) -> bool {
        is_letter(*self)
    }
    #[inline]
    fn is_mark(&self) -> bool {
        is_mark(*self)
    }
    #[inline]
    fn is_numeric(&self) -> bool {
        is_numeric(*self)
    }
    #[inline]
    fn is_decimal_digit(&self) -> bool {
        is_decimal_digit(*self)
    }
    #[inline]
    fn is_punctuation(&self) -> bool {
        is_punctuation(*self)
    }
    #[inline]
    fn is_symbol(&self) -> bool {
        is_symbol(*self)
    }
    #[inline]
    fn is_separator(&self) -> bool {
        is_separator(*self)
    }
    #[inline]
    fn is_control(&self) -> bool {
        is_control(*self)
    }
    #[inline]
    fn is_format(&self) -> bool {
        is_format(*self)
    }
    #[inline]
    fn is_assigned(&self) -> bool {
        is_assigned(*self)
    }
}
