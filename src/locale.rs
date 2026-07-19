//! BCP-47 / Unicode (UTS #35) locale identifiers.
//!
//! Parses the core `language[-script][-region][-variant]*` subtags of a language
//! tag into a [`Locale`] and renders it back in canonical form (lowercase
//! language, Titlecase script, UPPERCASE region). Extensions and the `u-`/`t-`
//! Unicode extension subtags are not yet modelled. Requires the `alloc` feature.

use alloc::string::{String, ToString};
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
    /// Extension and private-use sequences, each as `singleton-subtag-…`,
    /// lowercased and sorted by singleton (`x` last). E.g. `["u-ca-buddhist"]`.
    pub extensions: Vec<String>,
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
        if let Some(&s) = parts.peek()
            && s.len() == 4
            && is_alpha(s)
        {
            loc.script = Some(titlecase_subtag(s));
            parts.next();
        }
        // Region: 2 ALPHA or 3 DIGIT.
        if let Some(&s) = parts.peek()
            && ((s.len() == 2 && is_alpha(s)) || (s.len() == 3 && is_digit(s)))
        {
            loc.region = Some(s.to_ascii_uppercase());
            parts.next();
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

        // Extensions and private use: `singleton (-subtag)+`, where a singleton
        // is one alphanumeric character ('x' = private use).
        let mut current: Option<String> = None;
        for p in parts {
            // Under private use ('x'), single characters are subtags, not new
            // singletons; otherwise a single alphanumeric starts a new singleton.
            let in_private = current.as_deref().is_some_and(|c| c.starts_with('x'));
            if p.len() == 1 && p.bytes().next().unwrap().is_ascii_alphanumeric() && !in_private {
                if let Some(ext) = current.take() {
                    if !ext.contains('-') {
                        return Err(ParseError::InvalidSubtag); // singleton with no subtag
                    }
                    loc.extensions.push(ext);
                }
                current = Some(p.to_ascii_lowercase());
            } else if let Some(ext) = current.as_mut() {
                if p.is_empty() || !p.bytes().all(|b| b.is_ascii_alphanumeric()) {
                    return Err(ParseError::InvalidSubtag);
                }
                ext.push('-');
                ext.push_str(&p.to_ascii_lowercase());
            } else {
                return Err(ParseError::InvalidSubtag); // subtag before any singleton
            }
        }
        if let Some(ext) = current {
            if !ext.contains('-') {
                return Err(ParseError::InvalidSubtag);
            }
            loc.extensions.push(ext);
        }
        // Canonical order: by singleton, with private use ('x') last.
        loc.extensions.sort_by(|a, b| {
            let key = |s: &String| (s.starts_with("x-"), s.clone());
            key(a).cmp(&key(b))
        });
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
            if let Some(v) = crate::cldr::likely_subtags(key)
                && let Ok(m) = Locale::parse(v)
            {
                return Locale {
                    language: if self.language.is_empty() {
                        m.language
                    } else {
                        self.language.clone()
                    },
                    script: self.script.clone().or(m.script),
                    region: self.region.clone().or(m.region),
                    variants: self.variants.clone(),
                    extensions: self.extensions.clone(),
                };
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
        for e in &self.extensions {
            write!(f, "-{e}")?;
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

/// Canonicalize a BCP-47 language tag per UTS #35 / ECMA-402, substituting
/// deprecated subtags using the CLDR alias tables: language aliases (`iw`→`he`,
/// `sh`→`sr-Latn`), grandfathered/redundant whole tags (`i-klingon`→`tlh`,
/// `zh-min-nan`→`nan`), script aliases (`Qaai`→`Zinh`), territory aliases
/// (`BU`→`MM`, one→many like `SU`), and variant aliases (`heploc`→`alalc97`).
/// The result is also structurally canonicalized (case, subtag order).
///
/// Unicode (`-u-`) and Transform (`-t-`) extension keywords are also
/// canonicalized (UTS #35 §3.6.5 / ECMA-402 `CanonicalizeUnicodeLocaleId`):
/// attributes and keywords are sorted, deprecated key/type values are replaced
/// with their CLDR canonical forms (e.g. `-u-ca-islamicc` → `-u-ca-islamic-civil`,
/// `-u-ms-imperial` → `-u-ms-uksystem`), a `true`/`yes` keyword type is dropped
/// (`-u-kn-true` → `-u-kn`), and a `-t-` tag's embedded language is canonicalized.
///
/// Returns `None` if the tag (after any grandfathered replacement) fails to
/// parse.
#[must_use]
pub fn canonicalize(tag: &str) -> Option<String> {
    // 1. Whole-tag (grandfathered / redundant) alias: if the entire input tag
    //    matches an `l`-prefixed alias key (lowercased, `-`→`_`), replace it
    //    wholesale and canonicalize the replacement. This must run before parsing
    //    because irregular grandfathered tags (e.g. `i-klingon`) do not parse as
    //    normal `langtag`s.
    let whole_key = alloc::format!("l{}", tag.to_ascii_lowercase().replace('-', "_"));
    let working = match crate::cldr::alias_lookup(&whole_key) {
        Some(repl) => subtags_to_tag(repl),
        None => String::from(tag),
    };

    let mut loc = Locale::parse(&working).ok()?;

    // 2. Language alias. Try the most specific key first: `lang_region` (which,
    //    when it matches, consumes the region), then `lang`. A multi-subtag
    //    replacement (e.g. `sh`→`sr-Latn`) fills the script/region only where the
    //    source left them empty (UTS #35).
    let mut lang_repl: Option<(&'static str, bool)> = None;
    if let Some(region) = &loc.region {
        let key = alloc::format!("l{}_{}", loc.language, region.to_ascii_lowercase());
        if let Some(r) = crate::cldr::alias_lookup(&key) {
            lang_repl = Some((r, true));
        }
    }
    if lang_repl.is_none() && !loc.language.is_empty() {
        let key = alloc::format!("l{}", loc.language);
        if let Some(r) = crate::cldr::alias_lookup(&key) {
            lang_repl = Some((r, false));
        }
    }
    if let Some((repl, consumes_region)) = lang_repl
        && let Ok(rl) = Locale::parse(&subtags_to_tag(repl))
    {
        loc.language = rl.language;
        if consumes_region {
            loc.region = None;
        }
        if loc.script.is_none() {
            loc.script = rl.script;
        }
        if loc.region.is_none() {
            loc.region = rl.region;
        }
        if loc.variants.is_empty() {
            loc.variants = rl.variants;
        }
    }

    // 3. Script alias.
    if let Some(script) = &loc.script {
        let key = alloc::format!("s{script}");
        if let Some(r) = crate::cldr::alias_lookup(&key) {
            loc.script = Some(String::from(r));
        }
    }

    // 4. Variant aliases (each variant substituted independently).
    for v in &mut loc.variants {
        let key = alloc::format!("v{v}");
        if let Some(r) = crate::cldr::alias_lookup(&key) {
            *v = String::from(r);
        }
    }

    // 5. Territory (region) alias, with the UTS #35 one→many disambiguation rule:
    //    when a region maps to several candidate replacements, prefer the one that
    //    matches the likely region of the (already-substituted) language/script;
    //    otherwise fall back to the first candidate in CLDR order.
    if let Some(region) = loc.region.clone() {
        let key = alloc::format!("t{region}");
        if let Some(r) = crate::cldr::alias_lookup(&key) {
            loc.region = Some(pick_territory(&loc, r));
        }
    }

    // 6. Canonicalize each Unicode (`-u-`) / Transform (`-t-`) extension keyword.
    for e in &mut loc.extensions {
        if let Some(body) = e.strip_prefix("u-") {
            *e = canonicalize_unicode_ext(body);
        } else if let Some(body) = e.strip_prefix("t-") {
            *e = canonicalize_transform_ext(body);
        }
    }

    Some(loc.to_string())
}

/// Canonicalize a Unicode (`-u-`) extension body (the subtags after the `u`
/// singleton, already lowercased). Attributes are sorted; keywords are sorted by
/// key with the first occurrence of a duplicate key kept; each keyword's type is
/// replaced by its CLDR canonical value and a resulting `true`/`yes` type is
/// dropped. Returns the full extension string, e.g. `"u-ca-islamic-civil"`.
fn canonicalize_unicode_ext(body: &str) -> String {
    let subs: Vec<&str> = body.split('-').collect();
    let mut i = 0;
    // Attributes: leading non-key (len != 2) subtags.
    let mut attrs: Vec<&str> = Vec::new();
    while i < subs.len() && subs[i].len() != 2 {
        attrs.push(subs[i]);
        i += 1;
    }
    attrs.sort_unstable();
    attrs.dedup();

    // Keywords: `key (type-subtag)*`, key == a 2-char subtag.
    let mut keywords: Vec<(&str, String)> = Vec::new();
    while i < subs.len() {
        let key = subs[i];
        i += 1;
        let start = i;
        while i < subs.len() && subs[i].len() != 2 {
            i += 1;
        }
        // Skip a duplicate key (keep the first occurrence).
        if keywords.iter().any(|(k, _)| *k == key) {
            continue;
        }
        let value = subs[start..i].join("-");
        keywords.push((key, canonical_keyword_type(key, &value)));
    }
    keywords.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::from("u");
    for a in &attrs {
        out.push('-');
        out.push_str(a);
    }
    for (key, value) in &keywords {
        out.push('-');
        out.push_str(key);
        if !value.is_empty() {
            out.push('-');
            out.push_str(value);
        }
    }
    out
}

/// The canonical type value for a Unicode-extension keyword `key` whose raw type
/// is `value` (possibly multi-subtag, `-`-joined). Applies the CLDR bcp47 type
/// alias, then drops a `true`/`yes` value (returned as an empty string).
fn canonical_keyword_type(key: &str, value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    let alias_key = alloc::format!("{key}/{value}");
    let canonical = crate::cldr::bcp47_type_alias(&alias_key).unwrap_or(value);
    if canonical == "true" || canonical == "yes" {
        String::new()
    } else {
        String::from(canonical)
    }
}

/// Canonicalize a Transform (`-t-`) extension body (subtags after the `t`
/// singleton, already lowercased): the leading tlang is canonicalized like a
/// language tag (then lowercased), tfields are sorted by their `<alpha><digit>`
/// key, and deprecated tfield values are replaced with their CLDR canonical form.
fn canonicalize_transform_ext(body: &str) -> String {
    let subs: Vec<&str> = body.split('-').collect();
    let is_tkey = |s: &str| {
        s.len() == 2 && s.as_bytes()[0].is_ascii_alphabetic() && s.as_bytes()[1].is_ascii_digit()
    };
    let mut i = 0;
    // tlang: leading subtags up to the first tfield key.
    while i < subs.len() && !is_tkey(subs[i]) {
        i += 1;
    }
    let tlang = if i > 0 {
        let joined = subs[..i].join("-");
        // Canonicalize as a language tag, then lowercase (transform tlangs are
        // rendered lowercase, e.g. `sh` → `sr-latn`, `iw` → `he`).
        canonicalize(&joined).unwrap_or(joined).to_ascii_lowercase()
    } else {
        String::new()
    };

    // tfields: `tkey (value-subtag)+`.
    let mut fields: Vec<(&str, String)> = Vec::new();
    while i < subs.len() {
        let key = subs[i];
        i += 1;
        let start = i;
        while i < subs.len() && !is_tkey(subs[i]) {
            i += 1;
        }
        if fields.iter().any(|(k, _)| *k == key) {
            continue;
        }
        let value = subs[start..i].join("-");
        let alias_key = alloc::format!("{key}/{value}");
        let canonical = crate::cldr::bcp47_type_alias(&alias_key).unwrap_or(&value);
        fields.push((key, String::from(canonical)));
    }
    fields.sort_by(|a, b| a.0.cmp(b.0));

    let mut out = String::from("t");
    if !tlang.is_empty() {
        out.push('-');
        out.push_str(&tlang);
    }
    for (key, value) in &fields {
        out.push('-');
        out.push_str(key);
        if !value.is_empty() {
            out.push('-');
            out.push_str(value);
        }
    }
    out
}

/// Rewrite a space-separated subtag string (as stored in the alias blob) into a
/// `-`-separated language tag, e.g. `"sr Latn"` → `"sr-Latn"`.
fn subtags_to_tag(subtags: &str) -> String {
    let mut out = String::with_capacity(subtags.len());
    for (i, s) in subtags.split(' ').enumerate() {
        if i > 0 {
            out.push('-');
        }
        out.push_str(s);
    }
    out
}

/// Resolve a territory-alias replacement, applying the one→many disambiguation.
/// `replacement` is a space-separated candidate list (a single element for the
/// common one→one case).
fn pick_territory(loc: &Locale, replacement: &str) -> String {
    let first = replacement.split(' ').next().unwrap_or("");
    if !replacement.contains(' ') {
        return String::from(first);
    }
    // Disambiguate: maximize the language(+script) ignoring the region being
    // replaced, and use its likely region if it is one of the candidates.
    let probe = Locale {
        language: loc.language.clone(),
        script: loc.script.clone(),
        ..Locale::default()
    };
    if let Some(likely) = probe.maximize().region
        && replacement.split(' ').any(|c| c == likely)
    {
        return likely;
    }
    String::from(first)
}

/// Canonicalize each tag (ECMA-402 `CanonicalizeLocaleList`): drop tags that fail
/// to parse, then dedupe preserving first-occurrence order.
#[must_use]
pub fn get_canonical_locales(tags: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for &tag in tags {
        if let Some(c) = canonicalize(tag)
            && !out.iter().any(|x| x == &c)
        {
            out.push(c);
        }
    }
    out
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
