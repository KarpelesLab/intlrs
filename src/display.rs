//! Locale display names (CLDR / UTS #35): the name of a language or region as
//! written in a given display locale. `no_std`, no `alloc` (results borrow from
//! the embedded tables).
//!
//! Data is a curated set of display locales (`en`, `de`, `fr`, `es`, `ja`,
//! `zh`, `ru`, `ar`, `pt`, `it`); unknown display locales fall back through the
//! locale chain to English.
//!
//! ```
//! use intl::display::{language_name, region_name};
//! assert_eq!(language_name("en", "fr"), Some("French"));
//! assert_eq!(language_name("fr", "de"), Some("allemand"));
//! assert_eq!(region_name("en", "JP"), Some("Japan"));
//! assert_eq!(region_name("de", "US"), Some("Vereinigte Staaten"));
//! ```

/// Lowercase + `_`→`-` normalize a tag into a stack buffer (no alloc).
fn norm_into<'a>(buf: &'a mut [u8; 40], tag: &str) -> &'a str {
    let bytes = tag.as_bytes();
    let len = bytes.len().min(buf.len());
    for k in 0..len {
        let b = bytes[k].to_ascii_lowercase();
        buf[k] = if b == b'_' { b'-' } else { b };
    }
    core::str::from_utf8(&buf[..len]).unwrap_or("")
}

/// Resolve through the display-locale fallback chain (full tag, then shorter
/// prefixes, then English).
fn lookup(
    display_locale: &str,
    code: &str,
    f: fn(&str, &str) -> Option<&'static str>,
) -> Option<&'static str> {
    let mut buf = [0u8; 40];
    let norm = norm_into(&mut buf, display_locale);
    let mut end = norm.len();
    loop {
        if let Some(name) = f(&norm[..end], code) {
            return Some(name);
        }
        match norm[..end].rfind('-') {
            Some(i) => end = i,
            None => return f("en", code),
        }
    }
}

/// The display name of language `code` (a BCP-47 primary language subtag, e.g.
/// `"fr"`) as written in `display_locale`. Returns `None` if unknown.
#[must_use]
pub fn language_name(display_locale: &str, code: &str) -> Option<&'static str> {
    let mut buf = [0u8; 40];
    let code = norm_into(&mut buf, code); // language subtags are lowercase
    lookup(display_locale, code, crate::cldr::language_name)
}

/// The display name of region `code` (an ISO 3166-1 alpha-2 code, e.g. `"JP"`,
/// or a UN M.49 numeric code) as written in `display_locale`. Returns `None` if
/// unknown.
#[must_use]
pub fn region_name(display_locale: &str, code: &str) -> Option<&'static str> {
    let mut buf = [0u8; 8];
    let bytes = code.as_bytes();
    let len = bytes.len().min(buf.len());
    for k in 0..len {
        buf[k] = bytes[k].to_ascii_uppercase(); // region codes are uppercase
    }
    let code = core::str::from_utf8(&buf[..len]).unwrap_or("");
    lookup(display_locale, code, crate::cldr::region_name)
}
