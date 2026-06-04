//! The Unicode `General_Category` property and its major groupings.

/// A Unicode `General_Category` value (UAX #44).
///
/// Every codepoint has exactly one general category. Codepoints that are
/// unassigned — or that fall outside the codepoint range compiled in via the
/// crate's feature tiers — report [`GeneralCategory::Unassigned`] (`Cn`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum GeneralCategory {
    // Letter
    /// `Lu` — Uppercase Letter
    UppercaseLetter = 0,
    /// `Ll` — Lowercase Letter
    LowercaseLetter,
    /// `Lt` — Titlecase Letter
    TitlecaseLetter,
    /// `Lm` — Modifier Letter
    ModifierLetter,
    /// `Lo` — Other Letter
    OtherLetter,
    // Mark
    /// `Mn` — Nonspacing Mark
    NonspacingMark,
    /// `Mc` — Spacing Mark
    SpacingMark,
    /// `Me` — Enclosing Mark
    EnclosingMark,
    // Number
    /// `Nd` — Decimal Number
    DecimalNumber,
    /// `Nl` — Letter Number
    LetterNumber,
    /// `No` — Other Number
    OtherNumber,
    // Punctuation
    /// `Pc` — Connector Punctuation
    ConnectorPunctuation,
    /// `Pd` — Dash Punctuation
    DashPunctuation,
    /// `Ps` — Open Punctuation
    OpenPunctuation,
    /// `Pe` — Close Punctuation
    ClosePunctuation,
    /// `Pi` — Initial Punctuation
    InitialPunctuation,
    /// `Pf` — Final Punctuation
    FinalPunctuation,
    /// `Po` — Other Punctuation
    OtherPunctuation,
    // Symbol
    /// `Sm` — Math Symbol
    MathSymbol,
    /// `Sc` — Currency Symbol
    CurrencySymbol,
    /// `Sk` — Modifier Symbol
    ModifierSymbol,
    /// `So` — Other Symbol
    OtherSymbol,
    // Separator
    /// `Zs` — Space Separator
    SpaceSeparator,
    /// `Zl` — Line Separator
    LineSeparator,
    /// `Zp` — Paragraph Separator
    ParagraphSeparator,
    // Other
    /// `Cc` — Control
    Control,
    /// `Cf` — Format
    Format,
    /// `Cs` — Surrogate
    Surrogate,
    /// `Co` — Private Use
    PrivateUse,
    /// `Cn` — Unassigned (also reported for non-compiled ranges)
    Unassigned,
}

/// The seven major-class groupings of [`GeneralCategory`] (the first letter of
/// the two-letter abbreviation): `L`, `M`, `N`, `P`, `S`, `Z`, `C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Group {
    /// `L` — Letter
    Letter,
    /// `M` — Mark
    Mark,
    /// `N` — Number
    Number,
    /// `P` — Punctuation
    Punctuation,
    /// `S` — Symbol
    Symbol,
    /// `Z` — Separator
    Separator,
    /// `C` — Other
    Other,
}

impl GeneralCategory {
    /// The major-class [`Group`] this category belongs to.
    #[inline]
    #[must_use]
    pub const fn group(self) -> Group {
        use GeneralCategory::*;
        match self {
            UppercaseLetter | LowercaseLetter | TitlecaseLetter | ModifierLetter | OtherLetter => {
                Group::Letter
            }
            NonspacingMark | SpacingMark | EnclosingMark => Group::Mark,
            DecimalNumber | LetterNumber | OtherNumber => Group::Number,
            ConnectorPunctuation | DashPunctuation | OpenPunctuation | ClosePunctuation
            | InitialPunctuation | FinalPunctuation | OtherPunctuation => Group::Punctuation,
            MathSymbol | CurrencySymbol | ModifierSymbol | OtherSymbol => Group::Symbol,
            SpaceSeparator | LineSeparator | ParagraphSeparator => Group::Separator,
            Control | Format | Surrogate | PrivateUse | Unassigned => Group::Other,
        }
    }

    /// The canonical two-letter abbreviation, e.g. `"Lu"` for
    /// [`GeneralCategory::UppercaseLetter`].
    #[inline]
    #[must_use]
    pub const fn abbr(self) -> &'static str {
        use GeneralCategory::*;
        match self {
            UppercaseLetter => "Lu",
            LowercaseLetter => "Ll",
            TitlecaseLetter => "Lt",
            ModifierLetter => "Lm",
            OtherLetter => "Lo",
            NonspacingMark => "Mn",
            SpacingMark => "Mc",
            EnclosingMark => "Me",
            DecimalNumber => "Nd",
            LetterNumber => "Nl",
            OtherNumber => "No",
            ConnectorPunctuation => "Pc",
            DashPunctuation => "Pd",
            OpenPunctuation => "Ps",
            ClosePunctuation => "Pe",
            InitialPunctuation => "Pi",
            FinalPunctuation => "Pf",
            OtherPunctuation => "Po",
            MathSymbol => "Sm",
            CurrencySymbol => "Sc",
            ModifierSymbol => "Sk",
            OtherSymbol => "So",
            SpaceSeparator => "Zs",
            LineSeparator => "Zl",
            ParagraphSeparator => "Zp",
            Control => "Cc",
            Format => "Cf",
            Surrogate => "Cs",
            PrivateUse => "Co",
            Unassigned => "Cn",
        }
    }

    /// `true` if this is any letter (`L*`).
    #[inline]
    #[must_use]
    pub const fn is_letter(self) -> bool {
        matches!(self.group(), Group::Letter)
    }

    /// `true` if this is a cased letter (`Lu`, `Ll`, or `Lt`).
    #[inline]
    #[must_use]
    pub const fn is_cased_letter(self) -> bool {
        use GeneralCategory::*;
        matches!(self, UppercaseLetter | LowercaseLetter | TitlecaseLetter)
    }

    /// `true` if this is any mark (`M*`).
    #[inline]
    #[must_use]
    pub const fn is_mark(self) -> bool {
        matches!(self.group(), Group::Mark)
    }

    /// `true` if this is any number (`N*`).
    #[inline]
    #[must_use]
    pub const fn is_number(self) -> bool {
        matches!(self.group(), Group::Number)
    }

    /// `true` if this is any punctuation (`P*`).
    #[inline]
    #[must_use]
    pub const fn is_punctuation(self) -> bool {
        matches!(self.group(), Group::Punctuation)
    }

    /// `true` if this is any symbol (`S*`).
    #[inline]
    #[must_use]
    pub const fn is_symbol(self) -> bool {
        matches!(self.group(), Group::Symbol)
    }

    /// `true` if this is any separator (`Z*`).
    #[inline]
    #[must_use]
    pub const fn is_separator(self) -> bool {
        matches!(self.group(), Group::Separator)
    }

    /// `true` if this is in the "Other" group (`C*`): control, format,
    /// surrogate, private-use, or unassigned.
    #[inline]
    #[must_use]
    pub const fn is_other(self) -> bool {
        matches!(self.group(), Group::Other)
    }

    /// `true` if a codepoint with this category is assigned, i.e. not
    /// [`GeneralCategory::Unassigned`].
    ///
    /// Note: private-use (`Co`) and surrogate (`Cs`) codepoints count as
    /// assigned; only `Cn` is unassigned.
    #[inline]
    #[must_use]
    pub const fn is_assigned(self) -> bool {
        !matches!(self, GeneralCategory::Unassigned)
    }
}
