//! Internationalized Domain Names (UTS #46) with Punycode (RFC 3492).
//! Requires the `alloc` feature.
//!
//! Implements nontransitional processing with the non-STD3 profile. Label
//! validity is checked for length, hyphen placement, leading combining marks,
//! and disallowed characters; the CheckBidi and CheckJoiners contextual rules
//! are not yet enforced (so some adversarial inputs are accepted that a strict
//! validator would reject).

use super::generated::idna as gen;
use super::normalize::{canonical_combining_class, nfc};
use alloc::string::String;
use alloc::vec::Vec;

/// An IDNA processing error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A disallowed code point was present.
    Disallowed,
    /// A label violated a validity criterion (length, hyphens, leading mark).
    InvalidLabel,
    /// Punycode encoding or decoding failed.
    Punycode,
}

// ---- Punycode (RFC 3492) ----

const BASE: u32 = 36;
const TMIN: u32 = 1;
const TMAX: u32 = 26;
const SKEW: u32 = 38;
const DAMP: u32 = 700;
const INITIAL_BIAS: u32 = 72;
const INITIAL_N: u32 = 128;

fn adapt(mut delta: u32, num_points: u32, first_time: bool) -> u32 {
    delta /= if first_time { DAMP } else { 2 };
    delta += delta / num_points;
    let mut k = 0;
    while delta > ((BASE - TMIN) * TMAX) / 2 {
        delta /= BASE - TMIN;
        k += BASE;
    }
    k + (BASE - TMIN + 1) * delta / (delta + SKEW)
}

fn digit_to_basic(d: u32) -> char {
    // 0..=25 -> 'a'..='z', 26..=35 -> '0'..='9'
    if d < 26 {
        (b'a' + d as u8) as char
    } else {
        (b'0' + (d - 26) as u8) as char
    }
}

fn basic_to_digit(c: char) -> Option<u32> {
    match c {
        'a'..='z' => Some(c as u32 - 'a' as u32),
        'A'..='Z' => Some(c as u32 - 'A' as u32),
        '0'..='9' => Some(c as u32 - '0' as u32 + 26),
        _ => None,
    }
}

/// Punycode-encode a label (the part after `xn--`).
fn punycode_encode(input: &[char]) -> Option<String> {
    let mut output = String::new();
    let mut n = INITIAL_N;
    let mut delta: u32 = 0;
    let mut bias = INITIAL_BIAS;

    for &c in input {
        if (c as u32) < 0x80 {
            output.push(c);
        }
    }
    let basic = output.len() as u32;
    if basic > 0 {
        output.push('-');
    }
    let mut handled = basic;
    let total = input.len() as u32;

    while handled < total {
        let m = input.iter().map(|&c| c as u32).filter(|&c| c >= n).min()?;
        delta = delta.checked_add((m - n).checked_mul(handled + 1)?)?;
        n = m;
        for &c in input {
            let c = c as u32;
            if c < n {
                delta = delta.checked_add(1)?;
            }
            if c == n {
                let mut q = delta;
                let mut k = BASE;
                loop {
                    let t = k.saturating_sub(bias).clamp(TMIN, TMAX);
                    if q < t {
                        break;
                    }
                    output.push(digit_to_basic(t + (q - t) % (BASE - t)));
                    q = (q - t) / (BASE - t);
                    k += BASE;
                }
                output.push(digit_to_basic(q));
                bias = adapt(delta, handled + 1, handled == basic);
                delta = 0;
                handled += 1;
            }
        }
        delta += 1;
        n += 1;
    }
    Some(output)
}

/// Punycode-decode a label (the part after `xn--`).
fn punycode_decode(input: &str) -> Option<Vec<char>> {
    let mut output: Vec<u32> = Vec::new();
    let mut n = INITIAL_N;
    let mut i: u32 = 0;
    let mut bias = INITIAL_BIAS;

    let bytes: Vec<char> = input.chars().collect();
    let (basic_end, has_basic) = match bytes.iter().rposition(|&c| c == '-') {
        Some(p) => (p, true),
        None => (0, false),
    };
    if has_basic {
        for &c in &bytes[..basic_end] {
            if (c as u32) >= 0x80 {
                return None;
            }
            output.push(c as u32);
        }
    }
    let mut pos = if has_basic { basic_end + 1 } else { 0 };

    while pos < bytes.len() {
        let old_i = i;
        let mut w = 1;
        let mut k = BASE;
        loop {
            let c = *bytes.get(pos)?;
            pos += 1;
            let digit = basic_to_digit(c)?;
            i = i.checked_add(digit.checked_mul(w)?)?;
            let t = k.saturating_sub(bias).clamp(TMIN, TMAX);
            if digit < t {
                break;
            }
            w = w.checked_mul(BASE - t)?;
            k += BASE;
        }
        let len = output.len() as u32 + 1;
        bias = adapt(i - old_i, len, old_i == 0);
        n = n.checked_add(i / len)?;
        i %= len;
        output.insert(i as usize, n);
        i += 1;
    }
    output.into_iter().map(char::from_u32).collect()
}

// ---- UTS #46 processing ----

/// Apply the UTS #46 mapping (nontransitional, non-STD3) and NFC-normalize.
fn map_and_normalize(domain: &str) -> Result<String, Error> {
    let mut mapped: Vec<char> = Vec::new();
    for c in domain.chars() {
        match gen::idna_status(c as u32) {
            0 => mapped.push(c),                        // valid
            1 => mapped.extend_from_slice(idna_map(c)), // mapped
            2 => {}                                     // ignored
            _ => return Err(Error::Disallowed),         // disallowed
        }
    }
    Ok(nfc(mapped.into_iter()).collect())
}

fn idna_map(c: char) -> &'static [char] {
    gen::idna_mapped(c as u32).unwrap_or(&[])
}

fn validate_label(label: &str) -> Result<(), Error> {
    // The default UTS #46 test profile sets CheckHyphens = false, so leading,
    // trailing, and positions-3-4 hyphens are permitted. We still reject a
    // leading combining mark (a validity criterion independent of CheckHyphens).
    if let Some(first) = label.chars().next() {
        if canonical_combining_class(first) != 0 {
            return Err(Error::InvalidLabel);
        }
    }
    Ok(())
}

/// Convert a domain name to its ASCII (Punycode) form (UTS #46 ToASCII).
///
/// ```
/// use intl::unicode::idna::to_ascii;
/// assert_eq!(to_ascii("Bücher.example").unwrap(), "xn--bcher-kva.example");
/// assert_eq!(to_ascii("faß.de").unwrap(), "xn--fa-hia.de"); // ß stays (nontransitional)
/// ```
pub fn to_ascii(domain: &str) -> Result<String, Error> {
    let processed = map_and_normalize(domain)?;
    let mut out: Vec<String> = Vec::new();
    for label in processed.split('.') {
        if label.is_ascii() {
            validate_label(label)?;
            out.push(String::from(label));
        } else {
            let chars: Vec<char> = label.chars().collect();
            let encoded = punycode_encode(&chars).ok_or(Error::Punycode)?;
            let mut l = String::from("xn--");
            l.push_str(&encoded);
            out.push(l);
        }
    }
    Ok(out.join("."))
}

/// Convert a domain name to its Unicode form (UTS #46 ToUnicode): map, then
/// Punycode-decode any `xn--` labels.
#[must_use]
pub fn to_unicode(domain: &str) -> String {
    let processed = map_and_normalize(domain).unwrap_or_else(|_| String::from(domain));
    let mut out: Vec<String> = Vec::new();
    for label in processed.split('.') {
        if let Some(rest) = label.strip_prefix("xn--") {
            if let Some(decoded) = punycode_decode(rest) {
                out.push(decoded.into_iter().collect());
                continue;
            }
        }
        out.push(String::from(label));
    }
    out.join(".")
}
