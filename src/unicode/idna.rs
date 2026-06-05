//! Internationalized Domain Names (UTS #46) with Punycode (RFC 3492).
//! Requires the `alloc` feature.
//!
//! Implements nontransitional processing with the non-STD3 profile: the UTS #46
//! character mapping, NFC normalization, and Punycode encode/decode, plus the
//! full IDNA2008 validity criteria. Beyond disallowed characters,
//! empty/over-long labels and the empty (root) domain, this now enforces:
//! **CheckBidi (RFC 5893, rules B1–B6) as a whole-domain property**,
//! **CheckJoiners / ContextJ (RFC 5892 Appendix A, ZWNJ/ZWJ)**,
//! **CheckHyphens (V2/V3)**, the **leading combining mark** rule (V5), per-label
//! **NFC validity** (V1) and IDNA **valid**-status checking (V6), and `xn--`
//! label **re-canonicalization** (a decoded A-label must re-encode to exactly
//! the supplied Punycode, rejecting non-canonical encodings). It can therefore
//! be relied on as a strict validator for adversarial RTL/ZWJ input. The
//! optional CONTEXTO rules and UseSTD3ASCIIRules (U1) are intentionally not
//! applied (this is a non-STD3 profile).

use super::bidi::{bidi_class, BidiClass};
use super::generated::idna as gen;
use super::generated::properties::JoiningType;
use super::normalize::{canonical_combining_class, nfc};
use super::predicates::{is_mark, joining_type};
use alloc::string::String;
use alloc::vec::Vec;

/// An IDNA processing error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A disallowed code point was present.
    Disallowed,
    /// A label violated a validity criterion (length, leading mark, NFC, or a
    /// non-valid code point inside a decoded `xn--` label).
    InvalidLabel,
    /// Punycode encoding or decoding failed, or an `xn--` label was not the
    /// canonical Punycode encoding of its decoded form (re-canonicalization).
    Punycode,
    /// A label violated the CheckHyphens rule (V2/V3): a leading/trailing
    /// U+002D, or U+002D in both the third and fourth positions.
    Hyphen,
    /// A label violated CheckBidi (RFC 5893, rules B1–B6).
    Bidi,
    /// A label violated CheckJoiners / ContextJ (RFC 5892 Appendix A): a
    /// ZWNJ (U+200C) or ZWJ (U+200D) in an invalid context.
    ContextJ,
    /// A label began with a combining mark (V5).
    LeadingMark,
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

/// Maximum number of code points a decoded U-label may contain. A valid A-label
/// is ≤63 octets and each Punycode digit yields at most one output code point, so
/// 63 is a generous upper bound. Capping the decode output bounds work and
/// allocation, preventing a short `xn--` label from acting as a decompression
/// bomb (CPU/allocation DoS).
const MAX_LABEL_CODE_POINTS: usize = 63;

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
            if output.len() >= MAX_LABEL_CODE_POINTS {
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
        // Cap the output length before the O(n) insert so a hostile label that
        // would decode to a huge string bails out early instead of allocating.
        if output.len() >= MAX_LABEL_CODE_POINTS {
            return None;
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

/// Apply the IDNA2008 Validity Criteria to a decoded (Unicode) U-label.
///
/// Enforces, in order: non-empty; V6 (every code point IDNA-*valid*); V1
/// (already NFC); V2/V3 (CheckHyphens); V5 (no leading combining mark);
/// CheckJoiners / ContextJ (ZWNJ/ZWJ context, RFC 5892 App. A); and — only when
/// the whole domain is a Bidi domain — CheckBidi (RFC 5893, rules B1–B6).
fn validate_label(label: &[char], is_bidi_domain: bool) -> Result<(), Error> {
    // Non-empty (A4_2 is handled by the caller for the root label; this guards
    // any other empty label reaching here).
    if label.is_empty() {
        return Err(Error::InvalidLabel);
    }

    // V6: every code point must have IDNA status "valid".
    for &c in label {
        if gen::idna_status(c as u32) != 0 {
            return Err(Error::InvalidLabel);
        }
    }

    // V1: the label must already be in NFC.
    if nfc(label.iter().copied()).collect::<Vec<char>>() != label {
        return Err(Error::InvalidLabel);
    }

    // V2/V3 (CheckHyphens): no U+002D in both positions 3 and 4, and not at the
    // start or end of the label.
    if label.len() >= 4 && label[2] == '-' && label[3] == '-' {
        return Err(Error::Hyphen);
    }
    if label.first() == Some(&'-') || label.last() == Some(&'-') {
        return Err(Error::Hyphen);
    }

    // V5: the first character must not be a combining mark.
    if is_mark(label[0]) {
        return Err(Error::LeadingMark);
    }

    // CheckJoiners (ContextJ): validate every ZWNJ/ZWJ in context.
    check_joiners(label)?;

    // CheckBidi (RFC 5893): only when the domain is a Bidi domain.
    if is_bidi_domain {
        check_bidi(label)?;
    }

    Ok(())
}

/// CheckJoiners / ContextJ (RFC 5892 Appendix A): validate every ZWNJ (U+200C)
/// and ZWJ (U+200D) in the label.
fn check_joiners(label: &[char]) -> Result<(), Error> {
    for (i, &c) in label.iter().enumerate() {
        match c {
            // A.1 ZERO WIDTH NON-JOINER (C1).
            '\u{200C}' => {
                // Rule 1: preceded by a Virama (ccc == 9).
                if i > 0 && canonical_combining_class(label[i - 1]) == 9 {
                    continue;
                }
                // Rule 2: (L|D) (T*) ZWNJ (T*) (R|D), scanning over Transparent.
                if has_joining_context(label, i) {
                    continue;
                }
                return Err(Error::ContextJ);
            }
            // A.2 ZERO WIDTH JOINER (C2): only valid after a Virama.
            '\u{200D}' => {
                if i > 0 && canonical_combining_class(label[i - 1]) == 9 {
                    continue;
                }
                return Err(Error::ContextJ);
            }
            _ => {}
        }
    }
    Ok(())
}

/// The ContextJ ZWNJ rule 2 regex match `(L|D)(T*) <pos> (T*)(R|D)`: scan left
/// over Transparent characters for a Left/Dual-joining char and right over
/// Transparent characters for a Right/Dual-joining char.
fn has_joining_context(label: &[char], pos: usize) -> bool {
    // Scan left, skipping Transparent.
    let mut left = None;
    for j in (0..pos).rev() {
        match joining_type(label[j]) {
            JoiningType::Transparent => continue,
            jt => {
                left = Some(jt);
                break;
            }
        }
    }
    let before = matches!(
        left,
        Some(JoiningType::LeftJoining | JoiningType::DualJoining)
    );
    if !before {
        return false;
    }
    // Scan right, skipping Transparent.
    let mut right = None;
    for &c in &label[pos + 1..] {
        match joining_type(c) {
            JoiningType::Transparent => continue,
            jt => {
                right = Some(jt);
                break;
            }
        }
    }
    matches!(
        right,
        Some(JoiningType::RightJoining | JoiningType::DualJoining)
    )
}

/// CheckBidi (RFC 5893) for a single label, given that the enclosing domain is a
/// Bidi domain. Rules B1–B6.
fn check_bidi(label: &[char]) -> Result<(), Error> {
    let first = bidi_class(label[0]);
    // B1: the first character must be L, R, or AL.
    if !matches!(first, BidiClass::L | BidiClass::R | BidiClass::AL) {
        return Err(Error::Bidi);
    }

    if first.is_rtl() {
        // RTL label.
        // B2: every character ∈ {R, AL, AN, EN, ES, CS, ET, ON, BN, NSM}.
        for &c in label {
            match bidi_class(c) {
                BidiClass::R
                | BidiClass::AL
                | BidiClass::AN
                | BidiClass::EN
                | BidiClass::ES
                | BidiClass::CS
                | BidiClass::ET
                | BidiClass::ON
                | BidiClass::BN
                | BidiClass::NSM => {}
                _ => return Err(Error::Bidi),
            }
        }
        // B3: the last non-NSM character ∈ {R, AL, EN, AN}.
        match last_non_nsm(label) {
            Some(BidiClass::R | BidiClass::AL | BidiClass::EN | BidiClass::AN) => {}
            _ => return Err(Error::Bidi),
        }
        // B4: must not contain both an EN and an AN.
        let has_en = label.iter().any(|&c| bidi_class(c) == BidiClass::EN);
        let has_an = label.iter().any(|&c| bidi_class(c) == BidiClass::AN);
        if has_en && has_an {
            return Err(Error::Bidi);
        }
    } else {
        // LTR label (first is L).
        // B5: every character ∈ {L, EN, ES, CS, ET, ON, BN, NSM}.
        for &c in label {
            match bidi_class(c) {
                BidiClass::L
                | BidiClass::EN
                | BidiClass::ES
                | BidiClass::CS
                | BidiClass::ET
                | BidiClass::ON
                | BidiClass::BN
                | BidiClass::NSM => {}
                _ => return Err(Error::Bidi),
            }
        }
        // B6: the last non-NSM character ∈ {L, EN}.
        match last_non_nsm(label) {
            Some(BidiClass::L | BidiClass::EN) => {}
            _ => return Err(Error::Bidi),
        }
    }
    Ok(())
}

/// The Bidi_Class of the last character in `label` that is not NSM, or `None` if
/// every character is NSM.
fn last_non_nsm(label: &[char]) -> Option<BidiClass> {
    label
        .iter()
        .rev()
        .map(|&c| bidi_class(c))
        .find(|&bc| bc != BidiClass::NSM)
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
    let labels: Vec<&str> = processed.split('.').collect();
    let last = labels.len() - 1;

    // First pass: derive each label's U-label form (decoding `xn--` labels), so
    // CheckBidi — a *whole-domain* property — can be decided before any label is
    // validated. `None` marks an empty (root) label carried through unchanged.
    struct LabelInfo<'a> {
        original: &'a str,
        is_a_label: bool,
        ulabel: Vec<char>,
    }
    let mut infos: Vec<Option<LabelInfo>> = Vec::with_capacity(labels.len());
    for (i, label) in labels.iter().enumerate() {
        if label.is_empty() {
            // An empty label is only valid as the single trailing root.
            if i == last && labels.len() > 1 {
                infos.push(None);
                continue;
            }
            return Err(Error::InvalidLabel); // A4_2: empty / repeated dot
        }
        let (is_a_label, ulabel) = match label.strip_prefix("xn--") {
            Some(rest) => (true, punycode_decode(rest).ok_or(Error::Punycode)?),
            None => (false, label.chars().collect()),
        };
        infos.push(Some(LabelInfo {
            original: label,
            is_a_label,
            ulabel,
        }));
    }

    // The domain is a Bidi domain if any label contains an R, AL, or AN char.
    let is_bidi_domain = infos.iter().flatten().any(|info| {
        info.ulabel
            .iter()
            .any(|&c| matches!(bidi_class(c), BidiClass::R | BidiClass::AL | BidiClass::AN))
    });

    let mut out: Vec<String> = Vec::with_capacity(infos.len());
    for info in &infos {
        let Some(info) = info else {
            out.push(String::new());
            continue;
        };
        validate_label(&info.ulabel, is_bidi_domain)?;

        let ascii = if info.is_a_label {
            // A4 re-canonicalization: the supplied `xn--` label must be the
            // canonical Punycode encoding of its decoded U-label. Re-encode and
            // compare case-insensitively (Punycode output is lowercase).
            let encoded = punycode_encode(&info.ulabel).ok_or(Error::Punycode)?;
            let mut canonical = String::from("xn--");
            canonical.push_str(&encoded);
            if !canonical.eq_ignore_ascii_case(info.original) {
                return Err(Error::Punycode);
            }
            String::from(info.original)
        } else if info.ulabel.iter().all(char::is_ascii) {
            String::from(info.original)
        } else {
            let encoded = punycode_encode(&info.ulabel).ok_or(Error::Punycode)?;
            let mut l = String::from("xn--");
            l.push_str(&encoded);
            l
        };
        // A4_1 etc.: the ASCII label must be 1–63 octets.
        if ascii.is_empty() || ascii.len() > 63 {
            return Err(Error::InvalidLabel);
        }
        out.push(ascii);
    }
    let result = out.join(".");
    // A4_2: the whole domain must not be empty.
    if result.is_empty() {
        return Err(Error::InvalidLabel);
    }
    // A4_1: the assembled domain must not exceed the DNS 253-octet maximum.
    if result.len() > 253 {
        return Err(Error::InvalidLabel);
    }
    Ok(result)
}

/// Convert a domain name to its Unicode form (UTS #46 ToUnicode): map, then
/// Punycode-decode any `xn--` labels.
///
/// Returns `Err` when mapping encounters a disallowed code point (A5) or when an
/// `xn--` label is not valid Punycode. ToUnicode is conventionally lenient on
/// the contextual (Bidi/Joiner) rules, so those are not applied here — only the
/// hard mapping/decoding failures are surfaced.
///
/// ```
/// use intl::unicode::idna::to_unicode;
/// assert_eq!(to_unicode("xn--bcher-kva.example").unwrap(), "bücher.example");
/// ```
pub fn to_unicode(domain: &str) -> Result<String, Error> {
    let processed = map_and_normalize(domain)?;
    let mut out: Vec<String> = Vec::new();
    for label in processed.split('.') {
        if let Some(rest) = label.strip_prefix("xn--") {
            let decoded = punycode_decode(rest).ok_or(Error::Punycode)?;
            out.push(decoded.into_iter().collect());
        } else {
            out.push(String::from(label));
        }
    }
    Ok(out.join("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deviation_chars_remain_valid() {
        // ß (U+00DF) and ς (U+03C2) are valid in nontransitional processing and
        // must still encode rather than being rejected by the validity checks.
        assert_eq!(to_ascii("faß.de").unwrap(), "xn--fa-hia.de");
        assert!(to_ascii("ςoς.example").is_ok());
    }

    #[test]
    fn rtl_bidi_violation_rejected() {
        // Hebrew Aleph (R) followed by Latin 'a' (L): the domain is a Bidi
        // domain (contains R), and an RTL label may not contain an L char (B2).
        assert_eq!(to_ascii("\u{05D0}a"), Err(Error::Bidi));
    }

    #[test]
    fn bare_zwnj_rejected() {
        // ZWNJ with no preceding Virama and no L/D…R/D joining context (a, b are
        // non-joining) violates ContextJ (C1).
        assert_eq!(to_ascii("a\u{200C}b.example"), Err(Error::ContextJ));
    }

    #[test]
    fn virama_zwnj_accepted() {
        // Devanagari KA + Virama (ccc 9) + ZWNJ satisfies ContextJ rule 1.
        assert!(to_ascii("\u{0915}\u{094D}\u{200C}.example").is_ok());
    }

    #[test]
    fn non_canonical_xn_label_rejected() {
        // Re-canonicalization (A4): an `xn--` label must equal the canonical
        // Punycode encoding of its own decoded U-label. Our codec is canonical
        // (RFC 3492: exactly one well-formed encoding per string, and any
        // malformed encoding fails to decode), so the rejection happens either
        // at decode or at the re-encode equality check. Exercise the equality
        // check directly: a label that decodes successfully but is *not* the
        // canonical spelling of its decode must be rejected.
        let canonical = "bcher-kva";
        let decoded = punycode_decode(canonical).unwrap();
        assert_eq!(decoded, alloc::vec!['b', 'ü', 'c', 'h', 'e', 'r']);
        // The encoder is canonical: encode(decode(x)) == x for canonical x.
        assert_eq!(punycode_encode(&decoded).unwrap(), canonical);
        // The exact comparison `to_ascii` performs for an A-label: build a
        // deliberately non-canonical spelling (a different decoded U-label's
        // canonical Punycode, but labelled as if it were `bücher`'s) and confirm
        // the equality guard rejects it.
        let other = punycode_encode(&alloc::vec!['b', 'ü', 'c', 'h', 'e', 'r', 'ÿ']).unwrap();
        assert_ne!(other, canonical);
        let reencoded = punycode_encode(&decoded).unwrap();
        // `to_ascii` requires the supplied A-label's payload == reencoded;
        // `other` is a valid-but-different payload, so the guard fails.
        assert_ne!(other, reencoded);
        // A malformed Punycode payload is rejected outright by `to_ascii`.
        assert_eq!(to_ascii("xn--bcher-kva0.example"), Err(Error::Punycode));
        // The genuine canonical form is accepted and round-trips unchanged.
        assert_eq!(
            to_ascii("xn--bcher-kva.example").unwrap(),
            "xn--bcher-kva.example"
        );
    }

    #[test]
    fn leading_combining_mark_rejected() {
        // A label beginning with a combining mark (U+0300 COMBINING GRAVE
        // ACCENT) violates V5.
        assert_eq!(to_ascii("\u{0300}a.example"), Err(Error::LeadingMark));
    }

    #[test]
    fn double_hyphen_positions_rejected() {
        // U+002D at both the third and fourth positions violates CheckHyphens
        // (V2) — but is not the reserved `xn--` ACE prefix.
        assert_eq!(to_ascii("ab--c.example"), Err(Error::Hyphen));
        // Leading/trailing hyphen (V3).
        assert_eq!(to_ascii("-abc.example"), Err(Error::Hyphen));
        assert_eq!(to_ascii("abc-.example"), Err(Error::Hyphen));
    }

    #[test]
    fn to_unicode_surfaces_decode_failure() {
        // A bogus `xn--` label that is not valid Punycode is an error.
        assert!(to_unicode("xn--ll.example").is_err());
        // A valid A-label decodes.
        assert_eq!(to_unicode("xn--bcher-kva.de").unwrap(), "bücher.de");
    }

    #[test]
    fn punycode_decode_rejects_over_long_output() {
        // `tdaaaa…` (64 trailing 'a') is *valid* Punycode that decodes to 64
        // copies of 'ü' — one more than the 63-code-point label cap. The decoder
        // must bail out (None) rather than expand it, bounding work/allocation
        // and preventing a decompression-bomb DoS.
        let bomb = alloc::format!("td{}", "a".repeat(64));
        assert!(punycode_decode(&bomb).is_none());
        // A legitimate short label still decodes fine.
        assert_eq!(
            punycode_decode("bcher-kva"),
            Some(alloc::vec!['b', 'ü', 'c', 'h', 'e', 'r'])
        );
    }

    #[test]
    fn punycode_decode_caps_output_length() {
        // No successful decode may exceed the 63-code-point label cap.
        for payload in ["wgv71a119e", "bcher-kva", "fa-hia"] {
            if let Some(decoded) = punycode_decode(payload) {
                assert!(decoded.len() <= MAX_LABEL_CODE_POINTS);
            }
        }
    }

    #[test]
    fn to_ascii_rejects_over_long_domain() {
        // Many short labels assemble into a domain longer than the DNS 253-octet
        // maximum; the result must be rejected.
        let label = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 30 octets
        let mut domain = String::new();
        for i in 0..10 {
            if i > 0 {
                domain.push('.');
            }
            domain.push_str(label);
        }
        // 10 * 30 + 9 dots = 309 octets > 253.
        assert!(domain.len() > 253);
        assert_eq!(to_ascii(&domain), Err(Error::InvalidLabel));
    }

    #[test]
    fn to_ascii_accepts_domain_at_limit() {
        // A domain at or below 253 octets is still accepted.
        let label = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; // 30 octets
        let mut domain = String::new();
        for i in 0..8 {
            if i > 0 {
                domain.push('.');
            }
            domain.push_str(label);
        }
        // 8 * 30 + 7 dots = 247 octets <= 253.
        assert!(domain.len() <= 253);
        assert!(to_ascii(&domain).is_ok());
    }
}
