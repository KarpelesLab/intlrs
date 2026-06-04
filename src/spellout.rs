//! Spelling numbers out in words (a `rule-based number format`, RBNF). Requires
//! the `alloc` feature. English only for now (`spell_cardinal` / `spell_ordinal`);
//! other locales need the CLDR RBNF rule engine.
//!
//! ```
//! use intl::spellout::{spell_cardinal, spell_ordinal};
//! assert_eq!(spell_cardinal(0), "zero");
//! assert_eq!(spell_cardinal(1234), "one thousand two hundred thirty-four");
//! assert_eq!(spell_cardinal(-42), "minus forty-two");
//! assert_eq!(spell_ordinal(21), "twenty-first");
//! assert_eq!(spell_ordinal(100), "one hundredth");
//! ```

use alloc::string::{String, ToString};

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const SCALES: [&str; 7] = [
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
];

/// Spell a value 1..=999 in English (no leading/trailing spaces).
fn below_1000(n: u64) -> String {
    let mut s = String::new();
    let (h, r) = (n / 100, n % 100);
    if h > 0 {
        s.push_str(ONES[h as usize]);
        s.push_str(" hundred");
        if r > 0 {
            s.push(' ');
        }
    }
    if r > 0 {
        if r < 20 {
            s.push_str(ONES[r as usize]);
        } else {
            s.push_str(TENS[(r / 10) as usize]);
            if r % 10 > 0 {
                s.push('-');
                s.push_str(ONES[(r % 10) as usize]);
            }
        }
    }
    s
}

/// Spell `n` as an English cardinal number, e.g. `spell_cardinal(1234)` →
/// `"one thousand two hundred thirty-four"`.
#[must_use]
pub fn spell_cardinal(n: i64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    let mut out = String::new();
    if n < 0 {
        out.push_str("minus ");
    }
    let mut groups = [0u64; 7];
    let mut m = n.unsigned_abs();
    let mut count = 0;
    while m > 0 && count < groups.len() {
        groups[count] = m % 1000;
        m /= 1000;
        count += 1;
    }
    let mut first = true;
    for i in (0..count).rev() {
        if groups[i] == 0 {
            continue;
        }
        if !first {
            out.push(' ');
        }
        out.push_str(&below_1000(groups[i]));
        out.push_str(SCALES[i]);
        first = false;
    }
    out
}

/// Turn an English cardinal word into its ordinal form (the irregular cases plus
/// the regular `+th`).
fn ordinalize(word: &str) -> String {
    match word {
        "one" => "first".to_string(),
        "two" => "second".to_string(),
        "three" => "third".to_string(),
        "five" => "fifth".to_string(),
        "eight" => "eighth".to_string(),
        "nine" => "ninth".to_string(),
        "twelve" => "twelfth".to_string(),
        w if w.ends_with('y') => alloc::format!("{}ieth", &w[..w.len() - 1]),
        w => alloc::format!("{w}th"),
    }
}

/// Spell `n` as an English ordinal number, e.g. `spell_ordinal(21)` →
/// `"twenty-first"`, `spell_ordinal(100)` → `"one hundredth"`.
#[must_use]
pub fn spell_ordinal(n: i64) -> String {
    let cardinal = spell_cardinal(n);
    // Ordinalize the final word, where words are split by spaces and hyphens.
    let cut = cardinal.rfind([' ', '-']).map_or(0, |i| i + 1);
    let (head, last) = cardinal.split_at(cut);
    alloc::format!("{head}{}", ordinalize(last))
}
