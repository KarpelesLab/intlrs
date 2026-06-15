//! Validate the cardinal plural rules against the `@integer` / `@decimal`
//! sample numbers embedded in CLDR's plurals.json (every sample must select the
//! category it is listed under).

use intl::plural::{PluralCategory, PluralOperands, ordinal_category, plural_category};

fn cat(name: &str) -> PluralCategory {
    match name {
        "zero" => PluralCategory::Zero,
        "one" => PluralCategory::One,
        "two" => PluralCategory::Two,
        "few" => PluralCategory::Few,
        "many" => PluralCategory::Many,
        _ => PluralCategory::Other,
    }
}

/// Expand a sample token ("5", "2~17", "1.0~1.5") into concrete sample strings.
fn expand(token: &str, out: &mut Vec<String>) {
    let token = token.trim();
    if token.is_empty() || token == "…" || token == "..." {
        return;
    }
    // Compact-decimal samples (e.g. "1c6") exercise the `c`/`e` operand, which
    // PluralOperands does not yet parse; skip them.
    if token.contains('c') || token.contains('e') {
        return;
    }
    if let Some((a, b)) = token.split_once('~') {
        let (a, b) = (a.trim(), b.trim());
        // Integer range: expand fully (capped). Decimal range: just endpoints.
        if !a.contains('.') && !b.contains('.') {
            if let (Ok(lo), Ok(hi)) = (a.parse::<u64>(), b.parse::<u64>()) {
                for v in lo..=hi.min(lo + 200) {
                    out.push(v.to_string());
                }
                return;
            }
        }
        out.push(a.to_string());
        out.push(b.to_string());
    } else {
        out.push(token.to_string());
    }
}

/// Check every embedded sample in a CLDR plural file against `select`.
fn check_samples(file: &str, select: fn(&str, &PluralOperands) -> PluralCategory) -> usize {
    let path = format!("{}/data/cldr/48/{file}", env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(&path).expect("read plural json");

    let mut locale = String::new();
    let mut checked = 0usize;
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("\"pluralRule-count-") {
            let (catname, after) = rest.split_once('"').unwrap();
            let value = after.split_once(": \"").map_or("", |x| x.1);
            let value = value.trim_end_matches(',').trim_end_matches('"');
            let expected = cat(catname);

            let mut samples = Vec::new();
            for section in value.split('@').skip(1) {
                let nums = section
                    .trim_start_matches("integer")
                    .trim_start_matches("decimal");
                for token in nums.split(',') {
                    expand(token, &mut samples);
                }
            }
            for s in samples {
                let op = PluralOperands::parse(&s)
                    .unwrap_or_else(|| panic!("parse sample {s:?} ({locale})"));
                let got = select(&locale, &op);
                assert_eq!(
                    got, expected,
                    "{file} {locale}, sample {s:?}: got {got:?}, want {expected:?}"
                );
                checked += 1;
            }
        } else if t.ends_with('{') && t.starts_with('"') {
            if let Some(name) = t.split('"').nth(1) {
                if !name.starts_with("pluralRule") {
                    locale = name.to_string();
                }
            }
        }
    }
    checked
}

#[test]
fn cardinal_samples() {
    let n = check_samples("plurals.json", plural_category);
    eprintln!("cardinal samples checked: {n}");
    assert!(n > 5000, "too few samples checked: {n}");
}

#[test]
fn ordinal_samples() {
    let n = check_samples("ordinals.json", ordinal_category);
    eprintln!("ordinal samples checked: {n}");
    assert!(n > 500, "too few samples checked: {n}");
}
