//! Code generator for the `unicode` crate.
//!
//! Reads the vendored UCD text files under `data/ucd/<version>/` and emits
//! committed Rust source into `src/unicode/generated/`. The generated code is a
//! two-level "paged" `match` dispatch: an outer `match cp >> 8` selects a
//! 256-codepoint page, and each page resolves the low byte. Pages (and, within
//! page 0, individual arms) are `#[cfg]`-gated by the crate's range-tier
//! features so that excluded ranges simply are not compiled and resolve to the
//! neutral default.
//!
//! Run from the repo root with `cargo run --manifest-path codegen/Cargo.toml`.
//! Output is deterministic.
#![allow(clippy::write_with_newline)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const NUM_CODEPOINTS: usize = 0x11_0000;

/// Canonical `General_Category` order; index == enum discriminant.
const GC_ABBRS: [&str; 30] = [
    "Lu", "Ll", "Lt", "Lm", "Lo", "Mn", "Mc", "Me", "Nd", "Nl", "No", "Pc", "Pd", "Ps", "Pe", "Pi",
    "Pf", "Po", "Sm", "Sc", "Sk", "So", "Zs", "Zl", "Zp", "Cc", "Cf", "Cs", "Co", "Cn",
];
const GC_VARIANTS: [&str; 30] = [
    "UppercaseLetter",
    "LowercaseLetter",
    "TitlecaseLetter",
    "ModifierLetter",
    "OtherLetter",
    "NonspacingMark",
    "SpacingMark",
    "EnclosingMark",
    "DecimalNumber",
    "LetterNumber",
    "OtherNumber",
    "ConnectorPunctuation",
    "DashPunctuation",
    "OpenPunctuation",
    "ClosePunctuation",
    "InitialPunctuation",
    "FinalPunctuation",
    "OtherPunctuation",
    "MathSymbol",
    "CurrencySymbol",
    "ModifierSymbol",
    "OtherSymbol",
    "SpaceSeparator",
    "LineSeparator",
    "ParagraphSeparator",
    "Control",
    "Format",
    "Surrogate",
    "PrivateUse",
    "Unassigned",
];
const GC_UNASSIGNED: u8 = 29;

fn main() {
    // Resolve repo paths relative to this crate's manifest.
    let codegen_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = codegen_dir
        .parent()
        .expect("codegen has a parent dir")
        .to_path_buf();
    let version = "17.0.0";
    let ucd = root.join("data/ucd").join(version);
    let out_dir = root.join("src/unicode/generated");
    fs::create_dir_all(&out_dir).expect("create src/unicode/generated");

    let (vmaj, vmin, vpatch) = parse_version(&ucd.join("ReadMe.txt"));
    eprintln!(
        "codegen: Unicode {vmaj}.{vmin}.{vpatch} from {}",
        ucd.display()
    );

    // Names of the generated modules, collected as we emit them.
    let mut modules: Vec<String> = Vec::new();

    // ---- General_Category ----
    let gc = parse_unicode_data(&ucd.join("UnicodeData.txt"));
    let render_gc: Vec<String> = GC_VARIANTS
        .iter()
        .map(|v| format!("GeneralCategory::{v}"))
        .collect();
    let mut gc_out = String::new();
    write_header(&mut gc_out);
    let _ = write!(
        gc_out,
        "use crate::unicode::category::GeneralCategory;\n\n\
         /// The Unicode version this table was generated from.\n\
         pub const UNICODE_VERSION: (u8, u8, u8) = ({vmaj}, {vmin}, {vpatch});\n\n"
    );
    emit_lookup(
        &mut gc_out,
        "general_category",
        "gc",
        "GeneralCategory",
        &gc,
        u32::from(GC_UNASSIGNED),
        &render_gc,
    );
    write_module(&out_dir, &mut modules, "general_category", &gc_out);

    // ---- Binary properties ----
    let mut bp_out = String::new();
    write_header(&mut bp_out);
    for (fn_name, prefix, file, prop) in [
        ("white_space", "ws", "PropList.txt", "White_Space"),
        (
            "alphabetic",
            "al",
            "DerivedCoreProperties.txt",
            "Alphabetic",
        ),
        ("uppercase", "up", "DerivedCoreProperties.txt", "Uppercase"),
        ("lowercase", "lo", "DerivedCoreProperties.txt", "Lowercase"),
        ("xid_start", "xs", "DerivedCoreProperties.txt", "XID_Start"),
        (
            "xid_continue",
            "xc",
            "DerivedCoreProperties.txt",
            "XID_Continue",
        ),
        ("math", "ma", "DerivedCoreProperties.txt", "Math"),
        (
            "default_ignorable",
            "di",
            "DerivedCoreProperties.txt",
            "Default_Ignorable_Code_Point",
        ),
        ("dash", "da", "PropList.txt", "Dash"),
        ("diacritic", "dc", "PropList.txt", "Diacritic"),
        ("hex_digit", "hx", "PropList.txt", "Hex_Digit"),
        ("quotation_mark", "qm", "PropList.txt", "Quotation_Mark"),
        ("join_control", "jc", "PropList.txt", "Join_Control"),
    ] {
        let codes = parse_binary_prop(&ucd.join(file), prop);
        emit_bool_lookup(&mut bp_out, fn_name, prefix, &codes);
    }
    write_module(&out_dir, &mut modules, "binary_props", &bp_out);

    // ---- East Asian Width ----
    let eaw_map: BTreeMap<&str, u32> =
        [("N", 0), ("A", 1), ("H", 2), ("W", 3), ("F", 4), ("Na", 5)]
            .into_iter()
            .collect();
    let eaw = parse_ranged(&ucd.join("EastAsianWidth.txt"), &eaw_map, 0);
    let eaw_render: Vec<String> = [
        "Neutral",
        "Ambiguous",
        "Halfwidth",
        "Wide",
        "Fullwidth",
        "Narrow",
    ]
    .iter()
    .map(|v| format!("EastAsianWidth::{v}"))
    .collect();
    let mut eaw_out = String::new();
    write_header(&mut eaw_out);
    eaw_out.push_str("use crate::unicode::width::EastAsianWidth;\n\n");
    emit_lookup(
        &mut eaw_out,
        "east_asian_width",
        "eaw",
        "EastAsianWidth",
        &eaw,
        0,
        &eaw_render,
    );
    write_module(&out_dir, &mut modules, "east_asian_width", &eaw_out);

    // ---- Scripts + Script_Extensions ----
    emit_scripts(&out_dir, &mut modules, &ucd);

    // ---- Case mapping ----
    emit_case(&out_dir, &mut modules, &ucd);

    // ---- Numeric values ----
    emit_numeric(&out_dir, &mut modules, &ucd);

    // ---- Normalization ----
    emit_normalization(&out_dir, &mut modules, &ucd);

    // ---- Collation (DUCET) ----
    let uca = root.join("data/uca").join(version);
    emit_collation(&out_dir, &mut modules, &ucd, &uca);

    // ---- Segmentation (UAX #29) ----
    emit_segmentation(&out_dir, &mut modules, &ucd);

    // ---- Confusables (UTS #39) ----
    let security = root.join("data/security").join(version);
    emit_confusables(&out_dir, &mut modules, &security);

    // ---- IDNA mapping (UTS #46) ----
    emit_idna(
        &out_dir,
        &mut modules,
        &root.join("data/idna").join(version),
    );

    // ---- Bidi_Class (UAX #9) ----
    let bc_names = [
        "L", "R", "AL", "EN", "ES", "ET", "AN", "CS", "NSM", "BN", "B", "S", "WS", "ON", "LRE",
        "LRO", "RLE", "RLO", "PDF", "LRI", "RLI", "FSI", "PDI",
    ];
    let bc_map: BTreeMap<&str, u32> = bc_names
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i as u32))
        .collect();
    let bc = parse_ranged(
        &ucd.join("extracted/DerivedBidiClass.txt"),
        &bc_map,
        0, // default Left_To_Right
    );
    let bc_render: Vec<String> = bc_names.iter().map(|n| format!("BidiClass::{n}")).collect();
    let mut bc_out = String::new();
    write_header(&mut bc_out);
    bc_out.push_str("use crate::unicode::bidi::BidiClass;\n\n");
    emit_lookup(
        &mut bc_out,
        "bidi_class",
        "bc",
        "BidiClass",
        &bc,
        0,
        &bc_render,
    );
    // Bidi_Paired_Bracket + type, for rule N0: cp -> (paired, 1=open|2=close).
    let brackets = fs::read_to_string(ucd.join("BidiBrackets.txt")).expect("read BidiBrackets.txt");
    bc_out.push_str(
        "/// `(Bidi_Paired_Bracket, type)` where type is 1 = open, 2 = close, 0 = none.\n\
         pub(crate) const fn bidi_bracket(cp: u32) -> (u32, u8) {\n    match cp {\n",
    );
    for line in brackets.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        let cp = u32::from_str_radix(f[0], 16).unwrap();
        let paired = u32::from_str_radix(f[1], 16).unwrap();
        let ty = if f[2] == "o" { 1 } else { 2 };
        let _ = write!(bc_out, "        {cp:#x} => ({paired:#x}, {ty}),\n");
    }
    bc_out.push_str("        _ => (0, 0),\n    }\n}\n");
    write_module(&out_dir, &mut modules, "bidi", &bc_out);

    // ---- CLDR plural rules (UTS #35) ----
    emit_plurals(
        &out_dir,
        &mut modules,
        &root.join("data/cldr/48/plurals.json"),
        &root.join("data/cldr/48/ordinals.json"),
    );

    // ---- CLDR locale formatter tables -> committed binary blobs (no_std). ----
    let cldr_dir = root.join("src/cldr");
    let cldr = root.join("data/cldr/48");
    emit_numbers(&cldr_dir, &cldr.join("numbers.json"));
    emit_lists(&cldr_dir, &cldr.join("lists.json"));
    emit_relative(&cldr_dir, &cldr.join("relative.json"));
    emit_currency(&cldr_dir, &cldr.join("currency.json"));
    emit_display(&cldr_dir, &cldr.join("display.json"));
    emit_units(&cldr_dir, &cldr.join("units.json"));
    emit_calendar(&cldr_dir, &cldr.join("calendar.json"));
    emit_nested(
        &cldr_dir,
        "skeletons",
        &cldr.join("skeletons.json"),
        "locales",
    );

    // ---- generated/mod.rs ----
    modules.sort();
    let mut mod_out = String::new();
    write_header(&mut mod_out);
    for m in &modules {
        let _ = write!(mod_out, "pub(crate) mod {m};\n");
    }
    fs::write(out_dir.join("mod.rs"), &mod_out).expect("write generated/mod.rs");
    rustfmt(&out_dir.join("mod.rs"));

    eprintln!("codegen: wrote {} modules + mod.rs", modules.len());
}

/// Convert a UCD property-value name (`Old_Italic`, `Latin`) to a PascalCase
/// Rust identifier (`OldItalic`, `Latin`).
fn pascal_case(name: &str) -> String {
    name.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

/// Emit `generated/script.rs`: the `Script` enum (generated from the UCD script
/// names), `script()`, and `script_extensions()`.
fn emit_scripts(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    // ---- Script enum: distinct long names, sorted, plus Unknown (default). ----
    let scripts_txt = fs::read_to_string(ucd.join("Scripts.txt")).expect("read Scripts.txt");
    let mut names: BTreeSet<String> = BTreeSet::new();
    for line in scripts_txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(value) = line.split(';').nth(1) {
            let name = value.split('#').next().unwrap_or("").trim();
            if !name.is_empty() {
                names.insert(name.to_string());
            }
        }
    }
    let long_names: Vec<String> = names.into_iter().collect();
    let unknown_code = long_names.len() as u32;
    // name -> enum code, including Unknown.
    let mut name_to_code: BTreeMap<&str, u32> = BTreeMap::new();
    for (i, n) in long_names.iter().enumerate() {
        name_to_code.insert(n.as_str(), i as u32);
    }
    name_to_code.insert("Unknown", unknown_code);
    let variants: Vec<String> = long_names.iter().map(|n| pascal_case(n)).collect();

    // ---- Per-codepoint Script code. ----
    let script_codes = parse_ranged(&ucd.join("Scripts.txt"), &name_to_code, unknown_code);
    let script_render: Vec<String> = variants
        .iter()
        .map(|v| format!("Script::{v}"))
        .chain(std::iter::once("Script::Unknown".to_string()))
        .collect();

    // ---- Script_Extensions: short script code -> long name (from aliases). ----
    let aliases = fs::read_to_string(ucd.join("PropertyValueAliases.txt")).expect("read aliases");
    let mut short_to_long: BTreeMap<String, String> = BTreeMap::new();
    for line in aliases.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if !line.starts_with("sc ") && !line.starts_with("sc;") {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() >= 3 && f[0] == "sc" {
            short_to_long.insert(f[1].to_string(), f[2].to_string());
        }
    }

    // Distinct extension sets (sorted Script codes) -> table index.
    let scx_txt = fs::read_to_string(ucd.join("ScriptExtensions.txt")).expect("read scx");
    let mut set_index: BTreeMap<Vec<u32>, usize> = BTreeMap::new();
    let mut sets: Vec<Vec<u32>> = Vec::new();
    let mut scx_codes = vec![0u32; NUM_CODEPOINTS]; // 0 == None (use Script(cp))
    for line in scx_txt.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split(';');
        let range = parts.next().unwrap().trim();
        let rest = parts.next().unwrap_or("");
        let shorts = rest.split('#').next().unwrap_or("").split_whitespace();
        let mut codes: Vec<u32> = shorts
            .filter_map(|s| short_to_long.get(s))
            .filter_map(|long| name_to_code.get(long.as_str()).copied())
            .collect();
        codes.sort_unstable();
        codes.dedup();
        if codes.is_empty() {
            continue;
        }
        let idx = *set_index.entry(codes.clone()).or_insert_with(|| {
            sets.push(codes.clone());
            sets.len() - 1
        });
        let (start, end) = parse_range(range);
        for c in start..=end {
            scx_codes[c as usize] = (idx + 1) as u32; // +1: 0 is reserved for None
        }
    }
    // render[0] = None; render[i+1] = Some(&SCX_i)
    let mut scx_render: Vec<String> = vec!["None".to_string()];
    for i in 0..sets.len() {
        scx_render.push(format!("Some(SCX_{i})"));
    }

    // ---- Assemble the file. ----
    let mut out = String::new();
    write_header(&mut out);
    // enum
    out.push_str(
        "/// The Unicode `Script` property (UAX #24).\n\
         ///\n\
         /// Unassigned codepoints, and codepoints outside the compiled range tier,\n\
         /// report [`Script::Unknown`].\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]\n\
         #[repr(u8)]\n\
         pub enum Script {\n",
    );
    for v in &variants {
        let _ = write!(out, "    {v},\n");
    }
    out.push_str("    /// `Zzzz` — Unknown (default).\n    Unknown,\n}\n\n");
    // long_name()
    out.push_str(
        "impl Script {\n    /// The canonical Unicode long name, e.g. `\"Latin\"`.\n    \
         #[must_use]\n    pub const fn long_name(self) -> &'static str {\n        match self {\n",
    );
    for (v, long) in variants.iter().zip(long_names.iter()) {
        let _ = write!(out, "            Script::{v} => \"{long}\",\n");
    }
    out.push_str("            Script::Unknown => \"Unknown\",\n        }\n    }\n}\n\n");
    // extension-set tables
    for (i, set) in sets.iter().enumerate() {
        let elems: Vec<String> = set
            .iter()
            .map(|&c| script_render[c as usize].clone())
            .collect();
        let _ = write!(out, "const SCX_{i}: &[Script] = &[{}];\n", elems.join(", "));
    }
    if !sets.is_empty() {
        out.push('\n');
    }
    // lookups
    emit_lookup(
        &mut out,
        "script",
        "sc",
        "Script",
        &script_codes,
        unknown_code,
        &script_render,
    );
    emit_lookup(
        &mut out,
        "script_extensions",
        "scx",
        "Option<&'static [Script]>",
        &scx_codes,
        0,
        &scx_render,
    );

    write_module(out_dir, modules, "script", &out);
}

/// Parse a hex codepoint into a `char`.
fn hex_char(s: &str) -> char {
    char::from_u32(u32::from_str_radix(s.trim(), 16).unwrap()).expect("valid scalar")
}

/// Parse a space-separated list of hex codepoints into chars.
fn parse_chars(field: &str) -> Vec<char> {
    field.split_whitespace().map(hex_char).collect()
}

/// Render a 1..=3 char case mapping as a `CaseMap` expression.
fn render_casemap(m: &[char]) -> String {
    let lit = |c: char| format!("'\\u{{{:x}}}'", c as u32);
    match m {
        [a] => format!("CaseMap::One({})", lit(*a)),
        [a, b] => format!("CaseMap::Two({}, {})", lit(*a), lit(*b)),
        [a, b, c] => format!("CaseMap::Three({}, {}, {})", lit(*a), lit(*b), lit(*c)),
        _ => panic!("case mapping longer than 3: {m:?}"),
    }
}

/// Emit one case-mapping lookup. A per-codepoint mapping that is empty, or a
/// single char equal to the codepoint itself, is encoded as `CaseMap::Same`
/// (the default) — the public wrapper substitutes the original char.
fn emit_casemap(out: &mut String, fn_name: &str, prefix: &str, maps: &[Vec<char>]) {
    let mut render = vec!["CaseMap::Same".to_string()];
    let mut val_to_code: BTreeMap<Vec<char>, u32> = BTreeMap::new();
    let mut codes = vec![0u32; NUM_CODEPOINTS];
    for (cp, m) in maps.iter().enumerate() {
        if m.is_empty() || (m.len() == 1 && m[0] as usize == cp) {
            continue; // Same
        }
        let code = *val_to_code.entry(m.clone()).or_insert_with(|| {
            render.push(render_casemap(m));
            (render.len() - 1) as u32
        });
        codes[cp] = code;
    }
    emit_lookup(out, fn_name, prefix, "CaseMap", &codes, 0, &render);
}

/// Build the full unconditional upper/lower/title/fold mappings from
/// UnicodeData (simple), SpecialCasing (full, unconditional only), and
/// CaseFolding (statuses C + F).
fn parse_case_mappings(ucd: &Path) -> [Vec<Vec<char>>; 4] {
    let n = NUM_CODEPOINTS;
    let (mut upper, mut lower, mut title, mut fold) = (
        vec![vec![]; n],
        vec![vec![]; n],
        vec![vec![]; n],
        vec![vec![]; n],
    );

    let udata = fs::read_to_string(ucd.join("UnicodeData.txt")).expect("read UnicodeData.txt");
    for line in udata.lines() {
        let f: Vec<&str> = line.split(';').collect();
        if f.len() < 15 {
            continue;
        }
        let cp = u32::from_str_radix(f[0], 16).unwrap() as usize;
        if !f[12].is_empty() {
            upper[cp] = vec![hex_char(f[12])];
        }
        if !f[13].is_empty() {
            lower[cp] = vec![hex_char(f[13])];
        }
        if !f[14].is_empty() {
            title[cp] = vec![hex_char(f[14])];
        }
    }

    let special =
        fs::read_to_string(ucd.join("SpecialCasing.txt")).expect("read SpecialCasing.txt");
    for line in special.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() < 4 {
            continue;
        }
        // A non-empty 5th field is a condition (language/context) — skip those,
        // keeping only the unconditional full mappings (matches std behaviour).
        if f.get(4).map(|c| !c.is_empty()).unwrap_or(false) {
            continue;
        }
        let cp = hex_char(f[0]) as usize;
        lower[cp] = parse_chars(f[1]);
        title[cp] = parse_chars(f[2]);
        upper[cp] = parse_chars(f[3]);
    }

    let folding = fs::read_to_string(ucd.join("CaseFolding.txt")).expect("read CaseFolding.txt");
    for line in folding.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() < 3 {
            continue;
        }
        // Full case folding = statuses C (common) and F (full multi-char).
        if f[1] == "C" || f[1] == "F" {
            fold[hex_char(f[0]) as usize] = parse_chars(f[2]);
        }
    }

    [upper, lower, title, fold]
}

/// Emit `generated/case.rs`: to_upper / to_lower / to_title / fold lookups.
fn emit_case(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    let [upper, lower, title, fold] = parse_case_mappings(ucd);
    let mut out = String::new();
    write_header(&mut out);
    out.push_str("use crate::unicode::case::CaseMap;\n\n");
    emit_casemap(&mut out, "to_upper", "up", &upper);
    emit_casemap(&mut out, "to_lower", "lo", &lower);
    emit_casemap(&mut out, "to_title", "ti", &title);
    emit_casemap(&mut out, "fold", "fo", &fold);
    write_module(out_dir, modules, "case", &out);
}

/// Parse an exact numeric value (`3`, `-1/2`) into (numerator, denominator).
fn parse_rational(s: &str) -> (i64, u32) {
    match s.split_once('/') {
        Some((a, b)) => (
            a.trim().parse().expect("numerator fits i64"),
            b.trim().parse().expect("denominator fits u32"),
        ),
        None => (s.trim().parse().expect("integer fits i64"), 1),
    }
}

/// Emit `generated/numeric.rs`: numeric_value() and numeric_type().
fn emit_numeric(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    // ---- Numeric_Value (exact rational). ----
    let values = fs::read_to_string(ucd.join("extracted/DerivedNumericValues.txt"))
        .expect("read DerivedNumericValues.txt");
    let mut render = vec!["None".to_string()];
    let mut val_to_code: BTreeMap<(i64, u32), u32> = BTreeMap::new();
    let mut value_codes = vec![0u32; NUM_CODEPOINTS];
    for line in values.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').collect();
        if f.len() < 4 {
            continue;
        }
        let (num, den) = parse_rational(f[3]);
        let code = *val_to_code.entry((num, den)).or_insert_with(|| {
            render.push(format!(
                "Some(NumericValue {{ numerator: {num}, denominator: {den} }})"
            ));
            (render.len() - 1) as u32
        });
        let (start, end) = parse_range(f[0].trim());
        for c in start..=end {
            value_codes[c as usize] = code;
        }
    }

    // ---- Numeric_Type. ----
    let ty_map: BTreeMap<&str, u32> = [("Decimal", 1), ("Digit", 2), ("Numeric", 3)]
        .into_iter()
        .collect();
    let type_codes = parse_ranged(&ucd.join("extracted/DerivedNumericType.txt"), &ty_map, 0);
    let type_render = vec![
        "None".to_string(),
        "Some(NumericType::Decimal)".to_string(),
        "Some(NumericType::Digit)".to_string(),
        "Some(NumericType::Numeric)".to_string(),
    ];

    let mut out = String::new();
    write_header(&mut out);
    out.push_str("use crate::unicode::numeric::{NumericType, NumericValue};\n\n");
    emit_lookup(
        &mut out,
        "numeric_value",
        "nv",
        "Option<NumericValue>",
        &value_codes,
        0,
        &render,
    );
    emit_lookup(
        &mut out,
        "numeric_type",
        "nt",
        "Option<NumericType>",
        &type_codes,
        0,
        &type_render,
    );
    write_module(out_dir, modules, "numeric", &out);
}

/// Recursively expand the decomposition of `cp`. With `canonical_only`, only
/// canonical (untagged) mappings are followed; otherwise compatibility mappings
/// are followed too. Returns the fully-decomposed sequence (just `[cp]` if `cp`
/// does not decompose).
fn expand_decomp(
    cp: u32,
    raw: &[Option<(bool, Vec<u32>)>],
    canonical_only: bool,
    cache: &mut BTreeMap<u32, Vec<u32>>,
) -> Vec<u32> {
    if let Some(v) = cache.get(&cp) {
        return v.clone();
    }
    let result = match &raw[cp as usize] {
        Some((is_canonical, seq)) if *is_canonical || !canonical_only => seq
            .iter()
            .flat_map(|&c| expand_decomp(c, raw, canonical_only, cache))
            .collect(),
        _ => vec![cp],
    };
    cache.insert(cp, result.clone());
    result
}

/// Emit an `Option<&'static [char]>` lookup backed by deduplicated static
/// arrays (`<cprefix>N`), one per distinct non-empty sequence.
fn emit_char_seq_lookup(
    out: &mut String,
    fn_name: &str,
    prefix: &str,
    cprefix: &str,
    seqs: &[Vec<u32>],
) {
    let mut render = vec!["None".to_string()];
    let mut dedup: BTreeMap<Vec<u32>, u32> = BTreeMap::new();
    let mut codes = vec![0u32; NUM_CODEPOINTS];
    let mut consts = String::new();
    for (cp, seq) in seqs.iter().enumerate() {
        if seq.is_empty() {
            continue;
        }
        let code = *dedup.entry(seq.clone()).or_insert_with(|| {
            let i = render.len();
            let elems: Vec<String> = seq.iter().map(|&c| format!("'\\u{{{c:x}}}'")).collect();
            let _ = write!(
                consts,
                "const {cprefix}{i}: &[char] = &[{}];\n",
                elems.join(", ")
            );
            render.push(format!("Some({cprefix}{i})"));
            i as u32
        });
        codes[cp] = code;
    }
    out.push_str(&consts);
    out.push('\n');
    emit_lookup(
        out,
        fn_name,
        prefix,
        "Option<&'static [char]>",
        &codes,
        0,
        &render,
    );
}

/// Emit `generated/normalization.rs`: CCC, canonical/compatibility
/// decomposition, and canonical composition tables.
fn emit_normalization(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    let n = NUM_CODEPOINTS;
    let mut ccc = vec![0u32; n];
    let mut raw: Vec<Option<(bool, Vec<u32>)>> = vec![None; n];

    let udata = fs::read_to_string(ucd.join("UnicodeData.txt")).expect("read UnicodeData.txt");
    for line in udata.lines() {
        let f: Vec<&str> = line.split(';').collect();
        if f.len() < 6 {
            continue;
        }
        let cp = u32::from_str_radix(f[0], 16).unwrap() as usize;
        ccc[cp] = f[3].parse().unwrap_or(0);
        if !f[5].is_empty() {
            let canonical = !f[5].starts_with('<');
            let seq: Vec<u32> = f[5]
                .split_whitespace()
                .filter(|t| !t.starts_with('<'))
                .map(|t| u32::from_str_radix(t, 16).unwrap())
                .collect();
            raw[cp] = Some((canonical, seq));
        }
    }

    // Fully-expanded canonical and compatibility decompositions (empty = none).
    let mut canon_seqs = vec![vec![]; n];
    let mut compat_seqs = vec![vec![]; n];
    let mut cache_c = BTreeMap::new();
    let mut cache_k = BTreeMap::new();
    for cp in 0..n as u32 {
        if raw[cp as usize].is_none() {
            continue;
        }
        let c = expand_decomp(cp, &raw, true, &mut cache_c);
        if c != [cp] {
            canon_seqs[cp as usize] = c;
        }
        let k = expand_decomp(cp, &raw, false, &mut cache_k);
        if k != [cp] {
            compat_seqs[cp as usize] = k;
        }
    }

    // Canonical composition pairs: primary composites are canonical length-2
    // decompositions that are not Full_Composition_Exclusion.
    let excluded = parse_binary_prop(
        &ucd.join("DerivedNormalizationProps.txt"),
        "Full_Composition_Exclusion",
    );
    let mut compose: BTreeMap<u32, Vec<(u32, u32)>> = BTreeMap::new();
    for cp in 0..n as u32 {
        if let Some((true, seq)) = &raw[cp as usize] {
            if seq.len() == 2 && excluded[cp as usize] == 0 {
                compose.entry(seq[0]).or_default().push((seq[1], cp));
            }
        }
    }

    let mut out = String::new();
    write_header(&mut out);

    // CCC.
    let ccc_render: Vec<String> = (0..=254u32).map(|v| v.to_string()).collect();
    emit_lookup(
        &mut out,
        "canonical_combining_class",
        "ccc",
        "u8",
        &ccc,
        0,
        &ccc_render,
    );

    // Decompositions.
    emit_char_seq_lookup(&mut out, "decompose_canonical", "dc", "DC", &canon_seqs);
    emit_char_seq_lookup(&mut out, "decompose_compatible", "dk", "DK", &compat_seqs);

    // Composition: per-starter (second, composed) pairs.
    let mut comp_codes = vec![0u32; n];
    let mut comp_render = vec!["None".to_string()];
    let mut comp_consts = String::new();
    for (a, mut pairs) in compose {
        pairs.sort_unstable();
        let i = comp_render.len();
        let elems: Vec<String> = pairs
            .iter()
            .map(|(b, c)| format!("('\\u{{{b:x}}}', '\\u{{{c:x}}}')"))
            .collect();
        let _ = write!(
            comp_consts,
            "const CO{i}: &[(char, char)] = &[{}];\n",
            elems.join(", ")
        );
        comp_render.push(format!("Some(CO{i})"));
        comp_codes[a as usize] = i as u32;
    }
    out.push_str(&comp_consts);
    out.push('\n');
    emit_lookup(
        &mut out,
        "compose_pairs",
        "co",
        "Option<&'static [(char, char)]>",
        &comp_codes,
        0,
        &comp_render,
    );

    // Quick-check properties (0 = No, 1 = Maybe, 2 = Yes).
    let qc_render: Vec<String> = vec!["0".into(), "1".into(), "2".into()];
    let dnp = ucd.join("DerivedNormalizationProps.txt");
    for (fn_name, prefix, prop) in [
        ("nfc_qc", "qc", "NFC_QC"),
        ("nfd_qc", "qd", "NFD_QC"),
        ("nfkc_qc", "qe", "NFKC_QC"),
        ("nfkd_qc", "qf", "NFKD_QC"),
    ] {
        let codes = parse_qc(&dnp, prop);
        emit_lookup(&mut out, fn_name, prefix, "u8", &codes, 2, &qc_render);
    }

    write_module(out_dir, modules, "normalization", &out);
}

/// Pack a collation element into a u64: bit48 = variable, bits32-47 = primary,
/// bits16-31 = secondary, bits0-15 = tertiary.
fn pack_ce(variable: bool, p: u32, s: u32, t: u32) -> u64 {
    ((variable as u64) << 48) | ((p as u64) << 32) | ((s as u64) << 16) | (t as u64)
}

/// Parse the collation-element side of an allkeys line, e.g.
/// `[.1C47.0020.0002][*0201.0020.0002]`, into packed u64s.
fn parse_ces(s: &str) -> Vec<u64> {
    let mut ces = Vec::new();
    for grp in s.split('[').skip(1) {
        let inner = grp.split(']').next().unwrap_or("");
        if inner.is_empty() {
            continue;
        }
        let variable = inner.starts_with('*');
        let parts: Vec<&str> = inner[1..].split('.').collect();
        if parts.len() < 3 {
            continue;
        }
        let p = u32::from_str_radix(parts[0].trim(), 16).unwrap();
        let s = u32::from_str_radix(parts[1].trim(), 16).unwrap();
        let t = u32::from_str_radix(parts[2].trim(), 16).unwrap();
        ces.push(pack_ce(variable, p, s, t));
    }
    ces
}

/// Emit an `Option<&'static [u64]>` lookup with the CE arrays inlined (promoted
/// to statics), deduplicated by sequence.
fn emit_u64_seq_lookup(out: &mut String, fn_name: &str, prefix: &str, seqs: &[Vec<u64>]) {
    let mut render = vec!["None".to_string()];
    let mut dedup: BTreeMap<Vec<u64>, u32> = BTreeMap::new();
    let mut codes = vec![0u32; NUM_CODEPOINTS];
    for (cp, seq) in seqs.iter().enumerate() {
        if seq.is_empty() {
            continue;
        }
        let code = *dedup.entry(seq.clone()).or_insert_with(|| {
            let i = render.len();
            let elems: Vec<String> = seq.iter().map(|c| format!("0x{c:x}u64")).collect();
            render.push(format!("Some(&[{}])", elems.join(", ")));
            i as u32
        });
        codes[cp] = code;
    }
    emit_lookup(
        out,
        fn_name,
        prefix,
        "Option<&'static [u64]>",
        &codes,
        0,
        &render,
    );
}

/// Emit `generated/collation.rs`: DUCET single-codepoint collation elements,
/// contractions, and the Unified_Ideograph table (for implicit weights).
fn emit_collation(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path, uca: &Path) {
    // First code point -> list of (suffix code points, collation elements).
    type Contractions = BTreeMap<u32, Vec<(Vec<u32>, Vec<u64>)>>;

    let allkeys = fs::read_to_string(uca.join("allkeys.txt")).expect("read allkeys.txt");
    let mut singles: Vec<Vec<u64>> = vec![vec![]; NUM_CODEPOINTS];
    let mut contractions: Contractions = BTreeMap::new();
    for line in allkeys.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() || line.starts_with('@') {
            continue;
        }
        let mut it = line.split(';');
        let left = it.next().unwrap().trim();
        let right = it.next().unwrap_or("").trim();
        if right.is_empty() {
            continue;
        }
        let cps: Vec<u32> = left
            .split_whitespace()
            .map(|h| u32::from_str_radix(h, 16).unwrap())
            .collect();
        let ces = parse_ces(right);
        if cps.len() == 1 {
            singles[cps[0] as usize] = ces;
        } else {
            contractions
                .entry(cps[0])
                .or_default()
                .push((cps[1..].to_vec(), ces));
        }
    }

    let unified = parse_binary_prop(&ucd.join("PropList.txt"), "Unified_Ideograph");

    let mut out = String::new();
    write_header(&mut out);

    emit_u64_seq_lookup(&mut out, "ce_singles", "cs", &singles);

    // Contractions: per first-codepoint list of (suffix, CEs), longest suffix
    // first for greedy matching. Arrays are inlined and promoted to statics.
    let mut codes = vec![0u32; NUM_CODEPOINTS];
    let mut render = vec!["None".to_string()];
    for (cp, mut entries) in contractions {
        entries.sort_by_key(|e| core::cmp::Reverse(e.0.len()));
        let rows: Vec<String> = entries
            .iter()
            .map(|(suf, ces)| {
                let chars: Vec<String> = suf.iter().map(|&c| format!("'\\u{{{c:x}}}'")).collect();
                let cestr: Vec<String> = ces.iter().map(|c| format!("0x{c:x}u64")).collect();
                format!("(&[{}], &[{}])", chars.join(", "), cestr.join(", "))
            })
            .collect();
        let i = render.len();
        render.push(format!("Some(&[{}])", rows.join(", ")));
        codes[cp as usize] = i as u32;
    }
    emit_lookup(
        &mut out,
        "contractions",
        "cn",
        "Option<&'static [(&'static [char], &'static [u64])]>",
        &codes,
        0,
        &render,
    );

    emit_bool_lookup(&mut out, "unified_ideograph", "ui", &unified);

    write_module(out_dir, modules, "collation", &out);
}

/// Emit `generated/segmentation.rs`: Grapheme_Cluster_Break,
/// Extended_Pictographic, and Indic_Conjunct_Break tables (UAX #29).
fn emit_segmentation(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    let mut out = String::new();
    write_header(&mut out);
    out.push_str("use crate::unicode::segment::{Gcb, Incb, Lb, Sb, Wb};\n\n");

    let gcb_map: BTreeMap<&str, u32> = [
        ("CR", 1),
        ("LF", 2),
        ("Control", 3),
        ("Extend", 4),
        ("ZWJ", 5),
        ("Regional_Indicator", 6),
        ("Prepend", 7),
        ("SpacingMark", 8),
        ("L", 9),
        ("V", 10),
        ("T", 11),
        ("LV", 12),
        ("LVT", 13),
    ]
    .into_iter()
    .collect();
    let gcb = parse_ranged(
        &ucd.join("auxiliary/GraphemeBreakProperty.txt"),
        &gcb_map,
        0,
    );
    let gcb_render: Vec<String> = [
        "Other",
        "CR",
        "LF",
        "Control",
        "Extend",
        "ZWJ",
        "RegionalIndicator",
        "Prepend",
        "SpacingMark",
        "L",
        "V",
        "T",
        "LV",
        "LVT",
    ]
    .iter()
    .map(|v| format!("Gcb::{v}"))
    .collect();
    emit_lookup(
        &mut out,
        "grapheme_break",
        "gb",
        "Gcb",
        &gcb,
        0,
        &gcb_render,
    );

    let ep = parse_binary_prop(&ucd.join("emoji/emoji-data.txt"), "Extended_Pictographic");
    emit_bool_lookup(&mut out, "extended_pictographic", "ep", &ep);

    let incb_map: BTreeMap<&str, u32> = [("Consonant", 1), ("Linker", 2), ("Extend", 3)]
        .into_iter()
        .collect();
    let incb = parse_prop_value(&ucd.join("DerivedCoreProperties.txt"), "InCB", &incb_map, 0);
    let incb_render: Vec<String> = ["None", "Consonant", "Linker", "Extend"]
        .iter()
        .map(|v| format!("Incb::{v}"))
        .collect();
    emit_lookup(
        &mut out,
        "indic_conjunct_break",
        "ib",
        "Incb",
        &incb,
        0,
        &incb_render,
    );

    // Word_Break (UAX #29).
    let wb_names = [
        "CR",
        "LF",
        "Newline",
        "Extend",
        "ZWJ",
        "Regional_Indicator",
        "Format",
        "Katakana",
        "Hebrew_Letter",
        "ALetter",
        "Single_Quote",
        "Double_Quote",
        "MidNumLet",
        "MidLetter",
        "MidNum",
        "Numeric",
        "ExtendNumLet",
        "WSegSpace",
    ];
    let wb_map: BTreeMap<&str, u32> = wb_names
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, (i + 1) as u32))
        .collect();
    let wb = parse_ranged(&ucd.join("auxiliary/WordBreakProperty.txt"), &wb_map, 0);
    let mut wb_render = vec!["Wb::Other".to_string()];
    wb_render.extend(wb_names.iter().map(|n| format!("Wb::{}", pascal_case(n))));
    emit_lookup(&mut out, "word_break", "wb", "Wb", &wb, 0, &wb_render);

    // Sentence_Break (UAX #29).
    let sb_names = [
        "CR",
        "LF",
        "Extend",
        "Sep",
        "Format",
        "Sp",
        "Lower",
        "Upper",
        "OLetter",
        "Numeric",
        "ATerm",
        "SContinue",
        "STerm",
        "Close",
    ];
    let sb_map: BTreeMap<&str, u32> = sb_names
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, (i + 1) as u32))
        .collect();
    let sb = parse_ranged(&ucd.join("auxiliary/SentenceBreakProperty.txt"), &sb_map, 0);
    let mut sb_render = vec!["Sb::Other".to_string()];
    sb_render.extend(sb_names.iter().map(|n| format!("Sb::{}", pascal_case(n))));
    emit_lookup(&mut out, "sentence_break", "sb", "Sb", &sb, 0, &sb_render);

    // Line_Break (UAX #14), with LB1 resolution baked in.
    let lb_names = [
        "AI", "AK", "AL", "AP", "AS", "B2", "BA", "BB", "BK", "CB", "CJ", "CL", "CM", "CP", "CR",
        "EB", "EM", "EX", "GL", "H2", "H3", "HH", "HL", "HY", "ID", "IN", "IS", "JL", "JT", "JV",
        "LF", "NL", "NS", "NU", "OP", "PO", "PR", "QU", "RI", "SA", "SG", "SP", "SY", "VF", "VI",
        "WJ", "XX", "ZW", "ZWJ",
    ];
    let code = |n: &str| lb_names.iter().position(|&x| x == n).unwrap() as u32;
    let lb_map: BTreeMap<&str, u32> = lb_names
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i as u32))
        .collect();
    let al = code("AL");
    let raw = parse_ranged(&ucd.join("LineBreak.txt"), &lb_map, code("XX"));
    let gc = parse_unicode_data(&ucd.join("UnicodeData.txt")); // for SA resolution
    let (ai, sg, xx, cj, sa, ns, cm) = (
        code("AI"),
        code("SG"),
        code("XX"),
        code("CJ"),
        code("SA"),
        code("NS"),
        code("CM"),
    );
    let lb: Vec<u32> = raw
        .iter()
        .enumerate()
        .map(|(cp, &c)| {
            if c == ai || c == sg || c == xx {
                al
            } else if c == cj {
                ns
            } else if c == sa {
                // SA: Mn (5) / Mc (6) -> CM, else AL.
                if matches!(gc[cp], 5 | 6) {
                    cm
                } else {
                    al
                }
            } else {
                c
            }
        })
        .collect();
    let lb_render: Vec<String> = lb_names.iter().map(|n| format!("Lb::{n}")).collect();
    emit_lookup(&mut out, "line_break", "lb", "Lb", &lb, al, &lb_render);

    write_module(out_dir, modules, "segmentation", &out);
}

/// Emit `generated/confusables.rs`: the UTS #39 confusable prototype mapping
/// (source codepoint -> prototype character sequence).
fn emit_confusables(out_dir: &Path, modules: &mut Vec<String>, security: &Path) {
    let text = fs::read_to_string(security.join("confusables.txt")).expect("read confusables.txt");
    let mut protos: Vec<Vec<u32>> = vec![vec![]; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut f = line.split(';');
        let src = f.next().unwrap().trim();
        let tgt = f.next().unwrap_or("").trim();
        let Ok(cp) = u32::from_str_radix(src, 16) else {
            continue;
        };
        protos[cp as usize] = tgt
            .split_whitespace()
            .map(|h| u32::from_str_radix(h, 16).unwrap())
            .collect();
    }
    let mut out = String::new();
    write_header(&mut out);
    emit_char_seq_lookup(&mut out, "confusable_prototype", "cf", "CF", &protos);
    write_module(out_dir, modules, "confusables", &out);
}

/// Emit `generated/idna.rs`: the UTS #46 mapping table, collapsed to the
/// nontransitional, non-STD3 profile (status 0 valid, 1 mapped, 2 ignored,
/// 3 disallowed) plus the per-codepoint mapping for `mapped` status.
fn emit_idna(out_dir: &Path, modules: &mut Vec<String>, idna: &Path) {
    let text =
        fs::read_to_string(idna.join("IdnaMappingTable.txt")).expect("read IdnaMappingTable.txt");
    let mut status = vec![3u32; NUM_CODEPOINTS]; // unassigned -> disallowed
    let mut mapped: Vec<Vec<u32>> = vec![vec![]; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        let (start, end) = parse_range(f[0]);
        let st = f.get(1).copied().unwrap_or("disallowed");
        let code = match st {
            "valid" | "disallowed_STD3_valid" | "deviation" => 0,
            "mapped" | "disallowed_STD3_mapped" => 1,
            "ignored" => 2,
            _ => 3, // disallowed
        };
        let seq: Vec<u32> = if code == 1 {
            f.get(2)
                .map(|m| {
                    m.split_whitespace()
                        .map(|h| u32::from_str_radix(h, 16).unwrap())
                        .collect()
                })
                .unwrap_or_default()
        } else {
            vec![]
        };
        for cp in start..=end {
            status[cp as usize] = code;
            if code == 1 {
                mapped[cp as usize] = seq.clone();
            }
        }
    }
    let mut out = String::new();
    write_header(&mut out);
    let status_render: Vec<String> = (0..=3u32).map(|v| v.to_string()).collect();
    emit_lookup(
        &mut out,
        "idna_status",
        "is",
        "u8",
        &status,
        3,
        &status_render,
    );
    emit_char_seq_lookup(&mut out, "idna_mapped", "im", "IM", &mapped);
    write_module(out_dir, modules, "idna", &out);
}

// ---- Minimal JSON parser (CLDR data), std-only to keep codegen dependency-free. ----

enum Json {
    Obj(Vec<(String, Json)>),
    Arr(Vec<Json>),
    Str(String),
    Other,
}

impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(entries) => entries.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }
    fn entries(&self) -> &[(String, Json)] {
        match self {
            Json::Obj(e) => e,
            _ => &[],
        }
    }
    fn array(&self) -> &[Json] {
        match self {
            Json::Arr(a) => a,
            _ => &[],
        }
    }
    fn as_str(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }
}

fn json_parse(s: &str) -> Json {
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    json_value(&chars, &mut i)
}
fn json_ws(c: &[char], i: &mut usize) {
    while *i < c.len() && c[*i].is_whitespace() {
        *i += 1;
    }
}
fn json_value(c: &[char], i: &mut usize) -> Json {
    json_ws(c, i);
    match c[*i] {
        '{' => json_obj(c, i),
        '[' => json_arr(c, i),
        '"' => Json::Str(json_str(c, i)),
        _ => {
            while *i < c.len() && !matches!(c[*i], ',' | '}' | ']') {
                *i += 1;
            }
            Json::Other
        }
    }
}
fn json_obj(c: &[char], i: &mut usize) -> Json {
    *i += 1; // '{'
    let mut entries = Vec::new();
    loop {
        json_ws(c, i);
        if c[*i] == '}' {
            *i += 1;
            break;
        }
        let key = json_str(c, i);
        json_ws(c, i);
        *i += 1; // ':'
        let val = json_value(c, i);
        entries.push((key, val));
        json_ws(c, i);
        if c[*i] == ',' {
            *i += 1;
        }
    }
    Json::Obj(entries)
}
fn json_arr(c: &[char], i: &mut usize) -> Json {
    *i += 1; // '['
    let mut items = Vec::new();
    loop {
        json_ws(c, i);
        if c[*i] == ']' {
            *i += 1;
            break;
        }
        items.push(json_value(c, i));
        json_ws(c, i);
        if c[*i] == ',' {
            *i += 1;
        }
    }
    Json::Arr(items)
}
fn json_str(c: &[char], i: &mut usize) -> String {
    *i += 1; // opening quote
    let mut s = String::new();
    while c[*i] != '"' {
        if c[*i] == '\\' {
            *i += 1;
            match c[*i] {
                'u' => {
                    let hex: String = c[*i + 1..*i + 5].iter().collect();
                    if let Some(ch) = u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                        s.push(ch);
                    }
                    *i += 4;
                }
                'n' => s.push('\n'),
                't' => s.push('\t'),
                other => s.push(other),
            }
        } else {
            s.push(c[*i]);
        }
        *i += 1;
    }
    *i += 1; // closing quote
    s
}

/// Emit `generated/plurals.rs`: per-language cardinal and ordinal plural
/// selection, compiled from the CLDR plural rules into `match` functions.
fn emit_plurals(out_dir: &Path, modules: &mut Vec<String>, cardinal: &Path, ordinal: &Path) {
    let mut out = String::new();
    write_header(&mut out);
    out.push_str(
        "use crate::plural::in_set;\nuse crate::plural::PluralCategory::{self, *};\nuse crate::plural::PluralOperands as Op;\n\n",
    );
    emit_plural_fn(
        &mut out,
        cardinal,
        "plurals-type-cardinal",
        "plural_category",
        "cardinal",
    );
    emit_plural_fn(
        &mut out,
        ordinal,
        "plurals-type-ordinal",
        "ordinal_category",
        "ordinal",
    );
    write_module(out_dir, modules, "plurals", &out);
}

/// Emit one plural-selection function (cardinal or ordinal) from a CLDR file.
fn emit_plural_fn(out: &mut String, path: &Path, section: &str, fn_name: &str, kind: &str) {
    let text = fs::read_to_string(path).expect("read plural json");
    let json = json_parse(&text);
    let table = json
        .get("supplemental")
        .and_then(|s| s.get(section))
        .expect("plural section");

    let cats = [
        ("zero", "Zero"),
        ("one", "One"),
        ("two", "Two"),
        ("few", "Few"),
        ("many", "Many"),
    ];
    // Group languages by identical compiled rule body (many share one).
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (lang, rules) in table.entries() {
        let mut body = String::new();
        for (cat, variant) in cats {
            if let Some(rule) = rules
                .get(&format!("pluralRule-count-{cat}"))
                .and_then(Json::as_str)
            {
                let cond = rule.split('@').next().unwrap_or("").trim();
                if cond.is_empty() {
                    continue;
                }
                let _ = write!(
                    body,
                    "            if {} {{ return Some({variant}); }}\n",
                    compile_condition(cond)
                );
            }
        }
        groups.entry(body).or_default().push(lang.clone());
    }

    let _ = write!(
        out,
        "/// CLDR {kind} plural category for an exact locale key (already\n\
         /// case-normalized), or `None` if the key is unknown (caller falls back).\n\
         pub(crate) fn {fn_name}(lang: &str, op: &Op) -> Option<PluralCategory> {{\n    match lang {{\n",
    );
    for (body, langs) in &groups {
        if body.is_empty() {
            continue; // languages with only `other`
        }
        let pats: Vec<String> = langs
            .iter()
            .map(|l| format!("{:?}", l.to_ascii_lowercase()))
            .collect();
        let _ = write!(
            out,
            "        {} => {{\n{}            Some(Other)\n        }}\n",
            pats.join(" | "),
            body
        );
    }
    out.push_str("        _ => None,\n    }\n}\n\n");
}

/// Write `cldr/numbers.bin`: per-locale symbols + decimal/percent patterns.
fn emit_numbers(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read numbers.json");
    let json = json_parse(&text);
    let mut records = Vec::new();
    for (lang, n) in json.get("locales").expect("locales").entries() {
        let s = |k: &str| n.get(k).and_then(Json::as_str).unwrap_or("");
        let percent = s("percent");
        let mut p = Vec::new();
        for sym in [s("decimal"), s("group"), s("minus"), s("plus"), percent] {
            enc_str(&mut p, sym);
        }
        enc_pattern(&mut p, &parse_number_pattern(s("decimalPattern"), percent));
        enc_pattern(&mut p, &parse_number_pattern(s("percentPattern"), percent));
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "numbers", &records);
}

/// Write `cldr/lists.bin`: per-locale list connector patterns (and / or).
fn emit_lists(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read lists.json");
    let json = json_parse(&text);
    let mut records = Vec::new();
    for (lang, spec) in json.get("locales").expect("locales").entries() {
        let mut p = Vec::new();
        for style in ["and", "or"] {
            let st = spec.get(style).expect("style");
            for k in ["start", "middle", "end", "two"] {
                enc_str(&mut p, st.get(k).and_then(Json::as_str).unwrap_or(""));
            }
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "lists", &records);
}

/// Write `cldr/relative.bin`: per-locale relative-time strings (7 units).
fn emit_relative(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read relative.json");
    let json = json_parse(&text);
    let units = ["year", "month", "week", "day", "hour", "minute", "second"];
    let cat_index = |name: &str| match name {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "few" => 3,
        "many" => 4,
        _ => 5,
    };
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let mut p = Vec::new();
        for u in units {
            let f = loc.get(u).expect("unit");
            enc_opt(&mut p, f.get("prev").and_then(Json::as_str));
            enc_opt(&mut p, f.get("cur").and_then(Json::as_str));
            enc_opt(&mut p, f.get("next").and_then(Json::as_str));
            for tense in ["past", "future"] {
                let mut arr: [Option<&str>; 6] = [None; 6];
                if let Some(obj) = f.get(tense) {
                    for (count, pat) in obj.entries() {
                        if let Some(s) = pat.as_str() {
                            arr[cat_index(count)] = Some(s);
                        }
                    }
                }
                for slot in arr {
                    enc_opt(&mut p, slot);
                }
            }
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "relative", &records);
}

/// Write `cldr/currency.bin` (per-locale pattern + symbols) and
/// `cldr/currency_digits.bin` (per-currency fraction digits).
fn emit_currency(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read currency.json");
    let json = json_parse(&text);
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let mut p = Vec::new();
        let pat = loc.get("pattern").and_then(Json::as_str).unwrap_or("");
        enc_pattern(&mut p, &parse_number_pattern(pat, ""));
        let syms = loc.get("symbols").expect("symbols");
        p.push(syms.entries().len() as u8);
        for (code, sym) in syms.entries() {
            enc_str(&mut p, code);
            enc_str(&mut p, sym.as_str().unwrap_or(code));
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "currency", &records);

    // Per-currency fraction digits (bare JSON numbers parse as `Other`, so read
    // them directly from the source). Key = code, payload = one byte.
    let mut digit_records = Vec::new();
    for (code, _) in json.get("digits").expect("digits").entries() {
        digit_records.push((code.clone(), vec![digit_value(&text, code)]));
    }
    write_blob(cldr_dir, "currency_digits", &digit_records);
}

/// Write `cldr/calendar.bin`: per-locale Gregorian month/day names, am/pm, and
/// date/time/combining patterns. Payload (all required strings): months_wide(12),
/// months_abbr(12), days_wide(7), days_abbr(7), am, pm, date(4), time(4),
/// datetime(4) — styles in full/long/medium/short order.
fn emit_calendar(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read calendar.json");
    let json = json_parse(&text);
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let mut p = Vec::new();
        let push_arr = |p: &mut Vec<u8>, key: &str| {
            for v in loc.get(key).map(Json::array).unwrap_or(&[]) {
                enc_str(p, v.as_str().unwrap_or(""));
            }
        };
        push_arr(&mut p, "months_wide");
        push_arr(&mut p, "months_abbr");
        push_arr(&mut p, "days_wide");
        push_arr(&mut p, "days_abbr");
        enc_str(&mut p, loc.get("am").and_then(Json::as_str).unwrap_or("AM"));
        enc_str(&mut p, loc.get("pm").and_then(Json::as_str).unwrap_or("PM"));
        push_arr(&mut p, "date");
        push_arr(&mut p, "time");
        push_arr(&mut p, "datetime");
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "calendar", &records);
}

/// The curated measurement units, in the order the runtime `Unit` enum expects.
const UNITS: [&str; 28] = [
    "duration-second",
    "duration-minute",
    "duration-hour",
    "duration-day",
    "duration-week",
    "duration-month",
    "duration-year",
    "length-millimeter",
    "length-centimeter",
    "length-meter",
    "length-kilometer",
    "length-inch",
    "length-foot",
    "length-mile",
    "mass-gram",
    "mass-kilogram",
    "mass-ounce",
    "mass-pound",
    "digital-byte",
    "digital-kilobyte",
    "digital-megabyte",
    "digital-gigabyte",
    "temperature-celsius",
    "temperature-fahrenheit",
    "speed-kilometer-per-hour",
    "speed-mile-per-hour",
    "volume-liter",
    "volume-milliliter",
];

/// Write `cldr/units.bin`: per-locale unit patterns. Payload is
/// `width(long, short) × unit(28) × plural-count(6)` optional strings, in that
/// nested order.
fn emit_units(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read units.json");
    let json = json_parse(&text);
    let counts = ["zero", "one", "two", "few", "many", "other"];
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let mut p = Vec::new();
        for width in ["long", "short"] {
            let w = loc.get(width).expect("width");
            for unit in UNITS {
                let f = w.get(unit);
                for count in counts {
                    let pat = f.and_then(|x| x.get(count)).and_then(Json::as_str);
                    enc_opt(&mut p, pat);
                }
            }
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "units", &records);
}

/// Write a nested blob `<blob>.bin`: outer key (lowercased) -> a `[u16 count]`
/// inner table of `key -> value` (inner keys are kept verbatim, case-sensitive).
fn emit_nested(cldr_dir: &Path, blob: &str, path: &Path, section: &str) {
    let text = fs::read_to_string(path).expect("read nested json");
    let json = json_parse(&text);
    let table = json.get(section).expect("section");
    let mut records = Vec::new();
    for (outer, inner) in table.entries() {
        let entries = inner.entries();
        let mut payload = Vec::new();
        payload.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        for (k, v) in entries {
            enc_str(&mut payload, k);
            enc_str(&mut payload, v.as_str().unwrap_or(""));
        }
        records.push((outer.to_ascii_lowercase(), payload));
    }
    write_blob(cldr_dir, blob, &records);
}

/// Write `cldr/display_languages.bin` and `cldr/display_territories.bin`: for
/// each display locale, a nested `[u16 count]` table of `code -> name`.
fn emit_display(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read display.json");
    let json = json_parse(&text);
    for (section, blob) in [
        ("languages", "display_languages"),
        ("territories", "display_territories"),
    ] {
        let table = json.get(section).expect("section");
        let mut records = Vec::new();
        for (disp, codes) in table.entries() {
            let entries = codes.entries();
            let mut payload = Vec::new();
            payload.extend_from_slice(&(entries.len() as u16).to_le_bytes());
            for (code, name) in entries {
                enc_str(&mut payload, code);
                enc_str(&mut payload, name.as_str().unwrap_or(""));
            }
            records.push((disp.to_ascii_lowercase(), payload));
        }
        write_blob(cldr_dir, blob, &records);
    }
}

/// Pull the integer value of `"CODE":N` out of the digits object of the raw
/// currency JSON (the minimal JSON parser collapses bare numbers to `Other`).
fn digit_value(raw: &str, code: &str) -> u8 {
    let digits_start = raw.rfind("\"digits\"").map_or(0, |i| i);
    let key = format!("\"{code}\":");
    if let Some(p) = raw[digits_start..].find(&key) {
        let after = raw[digits_start + p + key.len()..].trim_start();
        let num: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
        return num.parse().unwrap_or(2);
    }
    2
}

/// Parse a CLDR number pattern (e.g. `#,##0.###`, `#,##0 %`) into a Rust
/// `Pattern { ... }` literal. `%` in the affixes is replaced by `percent_sym`.
/// The parsed fields of a CLDR number pattern.
struct PatFields {
    prefix: String,
    suffix: String,
    min_int: u8,
    min_frac: u8,
    max_frac: u8,
    primary: u8,
    secondary: u8,
}

fn parse_number_pattern(pat: &str, percent_sym: &str) -> PatFields {
    let pat = pat.split(';').next().unwrap_or(pat); // positive subpattern only
    let is_core = |c: char| matches!(c, '#' | '0' | '.' | ',');
    let first = pat.find(is_core).unwrap_or(0);
    let last = pat
        .rfind(is_core)
        .map_or(0, |i| i + pat[i..].chars().next().unwrap().len_utf8());
    let prefix = pat[..first].replace('%', percent_sym);
    let suffix = pat[last..].replace('%', percent_sym);
    let core = &pat[first..last];

    let (int_part, frac_part) = match core.split_once('.') {
        Some((a, b)) => (a, b),
        None => (core, ""),
    };
    let min_int = int_part.chars().filter(|&c| c == '0').count().max(1) as u8;
    let groups: Vec<&str> = int_part.split(',').collect();
    let (primary, secondary) = if groups.len() < 2 {
        (0u8, 0u8)
    } else {
        let primary = groups[groups.len() - 1].chars().count() as u8;
        let secondary = if groups.len() >= 3 {
            groups[groups.len() - 2].chars().count() as u8
        } else {
            primary
        };
        (primary, secondary)
    };
    let min_frac = frac_part.chars().filter(|&c| c == '0').count() as u8;
    let max_frac = frac_part
        .chars()
        .filter(|&c| matches!(c, '0' | '#'))
        .count() as u8;
    PatFields {
        prefix,
        suffix,
        min_int,
        min_frac,
        max_frac,
        primary,
        secondary,
    }
}

// ---- Binary blob encoding for the locale formatter tables (psl2 style). ----
//
// Each table is a flat `.bin` committed under `src/cldr/` and `include_bytes!`d
// by the `no_std` `crate::cldr` module, so the data has no dependency on the
// (alloc-only) formatter runtime types. Layout:
//   [u16 LE: record count]
//   record × count: [u8 key_len][key bytes][u16 LE payload_len][payload bytes]
// Strings inside a payload are `[u8 len][bytes]`; optional strings use a leading
// 0xFF byte for `None` (every string in this data is < 255 bytes).

fn enc_str(buf: &mut Vec<u8>, s: &str) {
    assert!(s.len() < 255, "formatter string too long: {s:?}");
    buf.push(s.len() as u8);
    buf.extend_from_slice(s.as_bytes());
}
fn enc_opt(buf: &mut Vec<u8>, s: Option<&str>) {
    match s {
        None => buf.push(0xFF),
        Some(x) => enc_str(buf, x),
    }
}
fn enc_pattern(buf: &mut Vec<u8>, p: &PatFields) {
    enc_str(buf, &p.prefix);
    enc_str(buf, &p.suffix);
    buf.extend_from_slice(&[p.min_int, p.min_frac, p.max_frac, p.primary, p.secondary]);
}

/// Write a keyed-record blob to `<cldr_dir>/<name>.bin`.
fn write_blob(cldr_dir: &Path, name: &str, records: &[(String, Vec<u8>)]) {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(records.len() as u16).to_le_bytes());
    for (key, payload) in records {
        buf.push(key.len() as u8);
        buf.extend_from_slice(key.as_bytes());
        buf.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        buf.extend_from_slice(payload);
    }
    fs::create_dir_all(cldr_dir).expect("create src/cldr");
    fs::write(cldr_dir.join(format!("{name}.bin")), buf).expect("write blob");
}

/// Compile a CLDR plural-rule condition (e.g. `i = 1 and v = 0`) into a Rust
/// boolean expression over `op` / `in_set`.
fn compile_condition(cond: &str) -> String {
    cond.split(" or ")
        .map(|and_cond| {
            let ands: Vec<String> = and_cond.split(" and ").map(compile_relation).collect();
            format!("({})", ands.join(" && "))
        })
        .collect::<Vec<_>>()
        .join(" || ")
}

fn compile_relation(rel: &str) -> String {
    let rel = rel.trim();
    let (neg, lhs, rhs) = if let Some(idx) = rel.find("!=") {
        (true, rel[..idx].trim(), rel[idx + 2..].trim())
    } else if let Some(idx) = rel.find('=') {
        (false, rel[..idx].trim(), rel[idx + 1..].trim())
    } else {
        panic!("bad plural relation: {rel}");
    };
    let expr = if let Some(p) = lhs.find('%') {
        let m: f64 = lhs[p + 1..].trim().parse().unwrap();
        format!("({} % {m:?})", operand_expr(lhs[..p].trim()))
    } else {
        operand_expr(lhs)
    };
    let ranges: Vec<String> = rhs
        .split(',')
        .map(|tok| {
            let tok = tok.trim();
            if let Some((a, b)) = tok.split_once("..") {
                let (a, b): (f64, f64) = (a.trim().parse().unwrap(), b.trim().parse().unwrap());
                format!("({a:?}, {b:?})")
            } else {
                let v: f64 = tok.parse().unwrap();
                format!("({v:?}, {v:?})")
            }
        })
        .collect();
    let call = format!("in_set({expr}, &[{}])", ranges.join(", "));
    if neg {
        format!("!{call}")
    } else {
        call
    }
}

fn operand_expr(o: &str) -> String {
    match o {
        "n" => "op.n".to_string(),
        "i" => "(op.i as f64)".to_string(),
        "v" => "(op.v as f64)".to_string(),
        "w" => "(op.w as f64)".to_string(),
        "f" => "(op.f as f64)".to_string(),
        "t" => "(op.t as f64)".to_string(),
        "c" | "e" => "(op.c as f64)".to_string(),
        other => panic!("unknown plural operand: {other}"),
    }
}

/// Write `content` to `<out_dir>/<name>.rs`, rustfmt it, and record the module.
fn write_module(out_dir: &Path, modules: &mut Vec<String>, name: &str, content: &str) {
    let path = out_dir.join(format!("{name}.rs"));
    fs::write(&path, content).unwrap_or_else(|_| panic!("write {}", path.display()));
    rustfmt(&path);
    modules.push(name.to_string());
}

/// Run `rustfmt` over a generated file (best effort; warns if rustfmt is absent).
fn rustfmt(path: &Path) {
    match Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .status()
    {
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("codegen: rustfmt exited with {s} on {}", path.display()),
        Err(e) => eprintln!("codegen: could not run rustfmt ({e}); output left unformatted"),
    }
}

fn write_header(out: &mut String) {
    out.push_str(
        "// @generated by codegen — DO NOT EDIT.\n\
         // Regenerate with `cargo run -p codegen` after updating data/ucd/.\n\
         #![allow(clippy::all)]\n\
         #![allow(unreachable_patterns)]\n\
         #![allow(unused_parens)]\n\
         #![allow(dead_code)]\n\n",
    );
}

fn parse_version(readme: &Path) -> (u8, u8, u8) {
    let text = fs::read_to_string(readme).unwrap_or_default();
    // Look for "Version X.Y.Z".
    if let Some(idx) = text.find("Version ") {
        let rest = &text[idx + "Version ".len()..];
        let token: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() == 3 {
            if let (Ok(a), Ok(b), Ok(c)) = (parts[0].parse(), parts[1].parse(), parts[2].parse()) {
                return (a, b, c);
            }
        }
    }
    (0, 0, 0)
}

/// Parse `UnicodeData.txt` into a per-codepoint category-code table.
fn parse_unicode_data(path: &Path) -> Vec<u32> {
    let abbr_to_code: BTreeMap<&str, u32> = GC_ABBRS
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i as u32))
        .collect();

    let text = fs::read_to_string(path).expect("read UnicodeData.txt");
    let mut codes = vec![u32::from(GC_UNASSIGNED); NUM_CODEPOINTS];

    let mut range_start: Option<u32> = None;
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let mut fields = line.split(';');
        let cp = u32::from_str_radix(fields.next().unwrap(), 16).expect("hex codepoint");
        let name = fields.next().unwrap_or("");
        let cat_abbr = fields.next().unwrap_or("Cn");
        let cat = *abbr_to_code
            .get(cat_abbr)
            .unwrap_or(&u32::from(GC_UNASSIGNED));

        if name.ends_with(", First>") {
            range_start = Some(cp);
            continue;
        }
        if name.ends_with(", Last>") {
            let start = range_start.take().expect("Last without First");
            for c in start..=cp {
                codes[c as usize] = cat;
            }
            continue;
        }
        codes[cp as usize] = cat;
    }
    codes
}

/// Parse a single named boolean property from a PropList-style file (ranges of
/// the form `XXXX` or `XXXX..YYYY ; PropName # ...`).
fn parse_binary_prop(path: &Path, prop: &str) -> Vec<u32> {
    let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let mut codes = vec![0u32; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(';');
        let range = parts.next().unwrap().trim();
        let name = parts.next().map(str::trim).unwrap_or("");
        if name != prop {
            continue;
        }
        let (start, end) = parse_range(range);
        for c in start..=end {
            codes[c as usize] = 1;
        }
    }
    codes
}

/// Parse a `range ; VALUE # comment` file (e.g. Scripts.txt, EastAsianWidth.txt)
/// into a per-codepoint code table, mapping each VALUE token through `val_code`.
/// Lines whose value is not in `val_code` are ignored. `@missing` / comment lines
/// are skipped.
fn parse_ranged(path: &Path, val_code: &BTreeMap<&str, u32>, default: u32) -> Vec<u32> {
    let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let mut codes = vec![default; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split(';');
        let range = parts.next().unwrap().trim();
        // Value is the first whitespace-delimited token after the ';', before '#'.
        let rest = parts.next().unwrap_or("");
        let value = rest.split('#').next().unwrap_or("").trim();
        let value = value.split_whitespace().next().unwrap_or("");
        let Some(&code) = val_code.get(value) else {
            continue;
        };
        let (start, end) = parse_range(range);
        for c in start..=end {
            codes[c as usize] = code;
        }
    }
    codes
}

/// Parse a `*_QC` quick-check property from DerivedNormalizationProps.txt into
/// per-codepoint codes: 0 = No, 1 = Maybe, 2 = Yes (the default).
fn parse_qc(path: &Path, prop: &str) -> Vec<u32> {
    let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let mut codes = vec![2u32; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() < 3 || f[1] != prop {
            continue;
        }
        let code = match f[2] {
            "N" => 0,
            "M" => 1,
            _ => 2,
        };
        let (start, end) = parse_range(f[0]);
        for c in start..=end {
            codes[c as usize] = code;
        }
    }
    codes
}

/// Parse a `range ; PROP ; VALUE # ...` file (e.g. InCB in
/// DerivedCoreProperties.txt) into per-codepoint codes from `val_code`, keeping
/// only lines whose middle field is `prop`.
fn parse_prop_value(
    path: &Path,
    prop: &str,
    val_code: &BTreeMap<&str, u32>,
    default: u32,
) -> Vec<u32> {
    let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let mut codes = vec![default; NUM_CODEPOINTS];
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() < 3 || f[1] != prop {
            continue;
        }
        let Some(&code) = val_code.get(f[2]) else {
            continue;
        };
        let (start, end) = parse_range(f[0]);
        for c in start..=end {
            codes[c as usize] = code;
        }
    }
    codes
}

/// Parse a `XXXX` or `XXXX..YYYY` hex range.
fn parse_range(range: &str) -> (u32, u32) {
    match range.split_once("..") {
        Some((a, b)) => (
            u32::from_str_radix(a.trim(), 16).unwrap(),
            u32::from_str_radix(b.trim(), 16).unwrap(),
        ),
        None => {
            let v = u32::from_str_radix(range.trim(), 16).unwrap();
            (v, v)
        }
    }
}

/// Tier feature gating a whole dispatcher page (page 0 is handled separately).
fn page_cfg(page: usize) -> &'static str {
    if page == 0 {
        "#[cfg(feature = \"ascii\")] "
    } else if page <= 0xFF {
        "#[cfg(feature = \"bmp\")] "
    } else {
        "#[cfg(feature = \"full\")] "
    }
}

/// Emit a paged lookup over `codes` (one value-code per codepoint) returning
/// `ret_ty`, where `render[code]` is the Rust expression for each value-code and
/// `default_code` is the fall-through value.
fn emit_lookup(
    out: &mut String,
    fn_name: &str,
    prefix: &str,
    ret_ty: &str,
    codes: &[u32],
    default_code: u32,
    render: &[String],
) {
    let default_expr = &render[default_code as usize];
    let mut dispatch = String::new();
    let mut funcs = String::new();
    let num_pages = NUM_CODEPOINTS / 256;

    for page in 0..num_pages {
        let slice = &codes[page * 256..page * 256 + 256];
        let cfg = page_cfg(page);

        if page == 0 {
            // Page 0 straddles the ascii (0x00..=0x7F) / latin1 (0x80..=0xFF)
            // boundary, so split its arms and cfg-gate the latin1 half.
            let fname = format!("{prefix}_p0");
            let _ = write!(
                funcs,
                "{cfg}const fn {fname}(b: u8) -> {ret_ty} {{\n    match b {{\n"
            );
            emit_arms(
                &mut funcs,
                &slice[0x00..0x80],
                0x00,
                default_code,
                render,
                "",
            );
            emit_arms(
                &mut funcs,
                &slice[0x80..0x100],
                0x80,
                default_code,
                render,
                "#[cfg(feature = \"latin1\")] ",
            );
            let _ = write!(funcs, "        _ => {default_expr},\n    }}\n}}\n\n");
            let _ = write!(dispatch, "        {cfg}0x000 => {fname}(cp as u8),\n");
            continue;
        }

        // Skip pages that are entirely the default value.
        if slice.iter().all(|&c| c == default_code) {
            continue;
        }
        // Collapse uniform non-default pages straight into the dispatcher arm.
        let first = slice[0];
        if slice.iter().all(|&c| c == first) {
            let _ = write!(
                dispatch,
                "        {cfg}0x{page:03x} => {},\n",
                render[first as usize]
            );
            continue;
        }
        // Mixed page: emit a dedicated function.
        let fname = format!("{prefix}_p{page:x}");
        let _ = write!(
            funcs,
            "{cfg}const fn {fname}(b: u8) -> {ret_ty} {{\n    match b {{\n"
        );
        emit_arms(&mut funcs, slice, 0x00, default_code, render, "");
        let _ = write!(funcs, "        _ => {default_expr},\n    }}\n}}\n\n");
        let _ = write!(
            dispatch,
            "        {cfg}0x{page:03x} => {fname}(cp as u8),\n"
        );
    }

    let _ = write!(
        out,
        "#[inline]\n\
         pub(crate) const fn {fn_name}(cp: u32) -> {ret_ty} {{\n    \
         match cp >> 8 {{\n{dispatch}        _ => {default_expr},\n    }}\n}}\n\n{funcs}"
    );
}

/// Convenience wrapper for boolean properties.
fn emit_bool_lookup(out: &mut String, fn_name: &str, prefix: &str, codes: &[u32]) {
    let render = [String::from("false"), String::from("true")];
    emit_lookup(out, fn_name, prefix, "bool", codes, 0, &render);
}

/// Emit coalesced `match` arms for one (sub)slice of low-byte values, skipping
/// runs equal to the default. `base` is the low byte of `slice[0]`. Each arm is
/// prefixed with `arm_cfg` (e.g. a latin1 cfg, or empty).
fn emit_arms(
    out: &mut String,
    slice: &[u32],
    base: usize,
    default_code: u32,
    render: &[String],
    arm_cfg: &str,
) {
    let mut i = 0;
    while i < slice.len() {
        let code = slice[i];
        let mut j = i + 1;
        while j < slice.len() && slice[j] == code {
            j += 1;
        }
        if code != default_code {
            let lo = base + i;
            let hi = base + j - 1;
            if lo == hi {
                let _ = write!(
                    out,
                    "        {arm_cfg}0x{lo:02x} => {},\n",
                    render[code as usize]
                );
            } else {
                let _ = write!(
                    out,
                    "        {arm_cfg}0x{lo:02x}..=0x{hi:02x} => {},\n",
                    render[code as usize]
                );
            }
        }
        i = j;
    }
}
