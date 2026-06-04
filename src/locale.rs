//! BCP-47 / Unicode (UTS #35) locale identifiers.
//!
//! Parses the core `language[-script][-region][-variant]*` subtags of a language
//! tag into a [`Locale`] and renders it back in canonical form (lowercase
//! language, Titlecase script, UPPERCASE region). Extensions and the `u-`/`t-`
//! Unicode extension subtags are not yet modelled. Requires the `alloc` feature.

use alloc::string::String;
use alloc::vec::Vec;

/// A parsed locale identifier.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Locale {
    /// The primary language subtag, lowercase (e.g. `"en"`). Empty for the
    /// "undetermined" language (`und`).
    pub language: String,
    /// The script subtag in Titlecase (e.g. `"Latn"`), if present.
    pub script: Option<String>,
    /// The region subtag, uppercase (e.g. `"US"`, `"419"`), if present.
    pub region: Option<String>,
    /// Variant subtags, lowercased (e.g. `["fonipa"]`).
    pub variants: Vec<String>,
}

/// An error parsing a locale identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// The tag was empty or had an empty subtag.
    Empty,
    /// A subtag was malformed for its position.
    InvalidSubtag,
}

fn is_alpha(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_alphabetic())
}
fn is_digit(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_digit())
}
fn is_alnum(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_alphanumeric())
}

impl Locale {
    /// Parse a BCP-47 language tag (the `langtag` core: language, script,
    /// region, variants). Subtags are case-normalized.
    pub fn parse(tag: &str) -> Result<Locale, ParseError> {
        if tag.is_empty() {
            return Err(ParseError::Empty);
        }
        let mut parts = tag.split(['-', '_']).peekable();
        let mut loc = Locale::default();

        // Language: 2-3 or 5-8 ALPHA, or "und".
        let lang = parts.next().ok_or(ParseError::Empty)?;
        if lang.is_empty() || !((2..=8).contains(&lang.len()) && is_alpha(lang)) {
            return Err(ParseError::InvalidSubtag);
        }
        loc.language = if lang.eq_ignore_ascii_case("und") {
            String::new()
        } else {
            lang.to_ascii_lowercase()
        };

        // Script: 4 ALPHA.
        if let Some(&s) = parts.peek() {
            if s.len() == 4 && is_alpha(s) {
                loc.script = Some(titlecase_subtag(s));
                parts.next();
            }
        }
        // Region: 2 ALPHA or 3 DIGIT.
        if let Some(&s) = parts.peek() {
            if (s.len() == 2 && is_alpha(s)) || (s.len() == 3 && is_digit(s)) {
                loc.region = Some(s.to_ascii_uppercase());
                parts.next();
            }
        }
        // Variants: 5-8 ALNUM, or 4 chars starting with a digit.
        while let Some(&s) = parts.peek() {
            let is_variant = ((5..=8).contains(&s.len()) && is_alnum(s))
                || (s.len() == 4 && s.as_bytes()[0].is_ascii_digit() && is_alnum(s));
            if !is_variant {
                break;
            }
            loc.variants.push(s.to_ascii_lowercase());
            parts.next();
        }

        // Anything left (extensions, private use) is not modelled; reject so we
        // don't silently drop it.
        if parts.next().is_some() {
            return Err(ParseError::InvalidSubtag);
        }
        Ok(loc)
    }

    /// Add the likely script and region for this locale (CLDR `likelySubtags` /
    /// UTS #35 "Add Likely Subtags"). For example `en` → `en-Latn-US`,
    /// `zh` → `zh-Hans-CN`. Subtags already present are kept.
    #[must_use]
    pub fn maximize(&self) -> Locale {
        let lang = if self.language.is_empty() {
            "und"
        } else {
            &self.language
        };
        // Candidate keys, most specific first.
        let mut candidates: Vec<String> = Vec::new();
        if let (Some(s), Some(r)) = (&self.script, &self.region) {
            candidates.push(alloc::format!("{lang}-{s}-{r}"));
        }
        if let Some(r) = &self.region {
            candidates.push(alloc::format!("{lang}-{r}"));
        }
        if let Some(s) = &self.script {
            candidates.push(alloc::format!("{lang}-{s}"));
        }
        candidates.push(String::from(lang));

        for key in &candidates {
            if let Some(v) = crate::cldr::likely_subtags(key) {
                if let Ok(m) = Locale::parse(v) {
                    return Locale {
                        language: if self.language.is_empty() {
                            m.language
                        } else {
                            self.language.clone()
                        },
                        script: self.script.clone().or(m.script),
                        region: self.region.clone().or(m.region),
                        variants: self.variants.clone(),
                    };
                }
            }
        }
        self.clone()
    }

    /// Remove the script and region subtags that are implied by
    /// [`maximize`](Self::maximize), producing the shortest equivalent locale.
    /// For example `en-Latn-US` → `en`, `zh-Hans-CN` → `zh`.
    #[must_use]
    pub fn minimize(&self) -> Locale {
        let max = self.maximize();
        let lang_only = Locale {
            language: self.language.clone(),
            ..Locale::default()
        };
        let lang_region = Locale {
            language: self.language.clone(),
            region: self.region.clone(),
            ..Locale::default()
        };
        let lang_script = Locale {
            language: self.language.clone(),
            script: self.script.clone(),
            ..Locale::default()
        };
        for trial in [lang_only, lang_region, lang_script] {
            if trial.maximize() == max {
                return trial;
            }
        }
        max
    }
}

/// Renders the canonical string form, e.g. `"zh-Hant-HK"`; the undetermined
/// language renders as `"und"`. Use `.to_string()` (via `Display`).
impl core::fmt::Display for Locale {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.language.is_empty() {
            f.write_str("und")?;
        } else {
            f.write_str(&self.language)?;
        }
        if let Some(s) = &self.script {
            write!(f, "-{s}")?;
        }
        if let Some(r) = &self.region {
            write!(f, "-{r}")?;
        }
        for v in &self.variants {
            write!(f, "-{v}")?;
        }
        Ok(())
    }
}

/// Choose the best `available` locale for a user's `requested` preference list
/// (BCP-47 tags, most-preferred first), returning its index in `available`.
///
/// For each requested locale in turn, matches are tried from most to least
/// specific (after [`maximize`](Locale::maximize)): same language+script+region,
/// then language+script, then language. Returns `None` if nothing matches.
///
/// ```
/// use intl::locale::{negotiate, Locale};
/// let avail = [
///     Locale::parse("en").unwrap(),
///     Locale::parse("fr").unwrap(),
///     Locale::parse("pt-BR").unwrap(),
/// ];
/// assert_eq!(negotiate(&["fr-CA", "en"], &avail), Some(1)); // fr-CA -> fr
/// assert_eq!(negotiate(&["pt-PT", "pt"], &avail), Some(2)); // pt -> pt-BR (only pt)
/// assert_eq!(negotiate(&["ja"], &avail), None);
/// ```
#[must_use]
pub fn negotiate(requested: &[&str], available: &[Locale]) -> Option<usize> {
    let maxed: Vec<Locale> = available.iter().map(Locale::maximize).collect();
    for req in requested {
        let Ok(r) = Locale::parse(req) else { continue };
        let r = r.maximize();
        // Three passes from most to least specific.
        for level in 0..3 {
            for (i, a) in maxed.iter().enumerate() {
                let hit = match level {
                    0 => a.language == r.language && a.script == r.script && a.region == r.region,
                    1 => a.language == r.language && a.script == r.script,
                    _ => a.language == r.language,
                };
                if hit {
                    return Some(i);
                }
            }
        }
    }
    None
}

fn titlecase_subtag(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for (i, b) in s.bytes().enumerate() {
        out.push(if i == 0 {
            b.to_ascii_uppercase() as char
        } else {
            b.to_ascii_lowercase() as char
        });
    }
    out
}
