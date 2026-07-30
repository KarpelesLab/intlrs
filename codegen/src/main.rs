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
        ("cased", "cs", "DerivedCoreProperties.txt", "Cased"),
        (
            "case_ignorable",
            "ci",
            "DerivedCoreProperties.txt",
            "Case_Ignorable",
        ),
        (
            "changes_when_lowercased",
            "cwl",
            "DerivedCoreProperties.txt",
            "Changes_When_Lowercased",
        ),
        (
            "changes_when_uppercased",
            "cwu",
            "DerivedCoreProperties.txt",
            "Changes_When_Uppercased",
        ),
        (
            "changes_when_titlecased",
            "cwt",
            "DerivedCoreProperties.txt",
            "Changes_When_Titlecased",
        ),
        (
            "changes_when_casefolded",
            "cwcf",
            "DerivedCoreProperties.txt",
            "Changes_When_Casefolded",
        ),
        (
            "changes_when_casemapped",
            "cwcm",
            "DerivedCoreProperties.txt",
            "Changes_When_Casemapped",
        ),
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
    emit_properties(&out_dir, &mut modules, &ucd);
    emit_names_blob(&root, &ucd);
    emit_segment_dict(&root);
    emit_cjk_dict(&root);

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
    emit_numbers(
        &cldr_dir,
        &cldr.join("numbers-raw"),
        &cldr.join("likely.json"),
    );
    emit_lists(&cldr_dir, &cldr.join("lists-raw"));
    emit_relative(&cldr_dir, &cldr.join("datefields-raw"));
    emit_currency(
        &cldr_dir,
        &cldr.join("currencies-raw"),
        &cldr.join("numbers-raw"),
        &cldr.join("currencyData.json"),
    );
    emit_display(&cldr_dir, &cldr.join("localenames-raw"));
    emit_units(&cldr_dir, &cldr.join("units-raw"));
    emit_dates(
        &cldr_dir,
        &cldr.join("dates"),
        &cldr.join("dayPeriods.json"),
    );
    emit_intervals(&cldr_dir, &cldr.join("dates"));
    emit_likely(&cldr_dir, &cldr.join("likely.json"));
    emit_aliases(&cldr_dir, &cldr.join("aliases.json"));
    emit_bcp47(&cldr_dir, &cldr.join("bcp47"));
    emit_tz_names(
        &cldr_dir,
        &cldr.join("timezonenames-raw"),
        &cldr.join("metaZones.json"),
        &cldr.join("bcp47/timezone.xml"),
        &cldr.join("primaryZones.json"),
    );
    emit_cldr_generated_mod(&cldr_dir);
    emit_rbnf(&cldr_dir, &cldr.join("rbnf.json"));
    emit_numsys(&cldr_dir, &cldr.join("numberingSystems.json"));
    emit_ordsuffix(&cldr_dir, &cldr.join("ordsuffix.json"));
    emit_collation_rules(
        &cldr_dir,
        &cldr.join("collation.json"),
        &cldr.join("collation"),
    );
    emit_collation_zh(&root, &cldr.join("collation/zh.xml"));
    emit_collation_zh_rs(&root, &ucd.join("Unihan_kRSUnicode.txt"));
    emit_collation_zh_variant(&root, &cldr.join("collation/zh.xml"), "stroke");
    emit_collation_zh_variant(&root, &cldr.join("collation/zh.xml"), "zhuyin");
    emit_alt_calendar(&cldr_dir, "islamic", &cldr.join("islamic-raw"));
    emit_alt_calendar(&cldr_dir, "persian", &cldr.join("persian-raw"));
    emit_chinese(&cldr_dir, &cldr.join("chinese-raw"));
    emit_japanese(&cldr_dir, &cldr.join("japanese-raw"));
    emit_japanese_hist(&cldr_dir, &cldr.join("japanese-raw"));

    // ---- generated/mod.rs ----
    // Gate the large per-component tables behind their Cargo feature so that
    // disabling a component (e.g. `collation`, ~1.9 MB) drops its table from the
    // build entirely. Foundational tables (general_category, binary_props,
    // properties, numeric, script, east_asian_width, plurals) are always built.
    let module_feature = |m: &str| -> Option<&'static str> {
        match m {
            "bidi" => Some("bidi"),
            "case" => Some("case"),
            "collation" => Some("collation"),
            "confusables" => Some("confusables"),
            "idna" => Some("idna"),
            "normalization" => Some("normalization"),
            "segmentation" => Some("segmentation"),
            _ => None,
        }
    };
    modules.sort();
    let mut mod_out = String::new();
    write_header(&mut mod_out);
    for m in &modules {
        if let Some(feat) = module_feature(m) {
            let _ = write!(mod_out, "#[cfg(feature = \"{feat}\")]\n");
        }
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
/// Emit `age` (Unicode version a codepoint was assigned in) and `block`
/// (Unicode block name) lookups from `DerivedAge.txt` and `Blocks.txt`.
/// Write `src/unicode/names.bin`: the tabulated character `Name` database for
/// the explicitly-named codepoints (algorithmic names — Hangul/CJK/… — are
/// excluded, handled in code). Layout: `[u32 count]`, then `count` sorted
/// `[u32 cp]`, then `count+1` `[u32 offset]`, then the UTF-8 name bytes. Used
/// only when the `names` feature is enabled, but always committed/shipped.
fn emit_names_blob(root: &Path, ucd: &Path) {
    let text = fs::read_to_string(ucd.join("UnicodeData.txt")).expect("read UnicodeData.txt");
    let mut entries: Vec<(u32, &str)> = Vec::new();
    for line in text.lines() {
        let mut f = line.split(';');
        let (Some(cp), Some(name)) = (f.next(), f.next()) else {
            continue;
        };
        // Skip the algorithmic/range/control rows (their name starts with '<').
        if name.starts_with('<') {
            continue;
        }
        if let Ok(cp) = u32::from_str_radix(cp, 16) {
            entries.push((cp, name));
        }
    }
    entries.sort_by_key(|&(cp, _)| cp);
    let count = entries.len() as u32;
    let mut blob = Vec::new();
    blob.extend_from_slice(&count.to_le_bytes());
    for &(cp, _) in &entries {
        blob.extend_from_slice(&cp.to_le_bytes());
    }
    let mut off = 0u32;
    for &(_, name) in &entries {
        blob.extend_from_slice(&off.to_le_bytes());
        off += name.len() as u32;
    }
    blob.extend_from_slice(&off.to_le_bytes()); // sentinel end offset
    for &(_, name) in &entries {
        blob.extend_from_slice(name.as_bytes());
    }
    let path = root.join("src/unicode/names.bin");
    fs::write(&path, &blob).expect("write names.bin");
    println!(
        "codegen: wrote names.bin ({} names, {} KB)",
        count,
        blob.len() / 1024
    );
}

/// Write `src/unicode/segment_dict.bin`: a minimized DAWG (deterministic acyclic
/// word graph) of ICU's Thai break dictionary (`data/brkitr/thaidict.txt`), used
/// by the `segmentation-dict` feature to drive ICU-style Thai word segmentation.
///
/// Layout (all little-endian):
/// ```text
///   u32 node_count N
///   u32 edge_count E
///   u32 root_id
///   final-flag bitmap: ceil(N/8) bytes (bit i%8 of byte i/8 == node i is a word end)
///   edge-offset table: (N+1) idx (cumulative edge index; node i owns edges[off[i]..off[i+1]])
///   edges: E records of [u8 sym][idx target], sorted ascending by sym within
///          each node (binary search at runtime)
/// ```
/// `sym` is `codepoint - base`, where `base` is the script's block base (U+0E00
/// for Thai/Lao, U+1780 for Khmer, U+1000 for the main Myanmar block). `idx` is a
/// `u16` when both `N <= 0xFFFF` and `E <= 0xFFFF` (Thai/Lao — 3-byte edge
/// records), otherwise a `u32` (Khmer — 5-byte edge records); the runtime reader
/// picks the width from the `N`/`E` header, so small blobs stay byte-identical.
/// The DAWG shares common suffixes, so it is far smaller than the raw trie while
/// preserving exactly the set of dictionary words (word ends are per-node flags).
fn emit_segment_dict(root: &Path) {
    // Thai and Lao share this word-list DAWG format and the U+0E00-relative edge
    // symbol (Lao lives at U+0E80..=U+0EFF, i.e. byte offsets 128..=255).
    emit_thai_family_dict(root, "thaidict.txt", "segment_dict.bin", 0x0E00);
    emit_thai_family_dict(root, "laodict.txt", "segment_dict_lao.bin", 0x0E00);
    // Khmer: edge symbols are U+1780-relative (whole Khmer block fits a u8).
    emit_thai_family_dict(root, "khmerdict.txt", "segment_dict_km.bin", 0x1780);
    // Burmese: edge symbols are U+1000-relative, restricted to the main Myanmar
    // block (Extended-A/B code points fall outside a u8 of U+1000; ICU's
    // burmesedict.txt contains none of them).
    emit_thai_family_dict(root, "burmesedict.txt", "segment_dict_my.bin", 0x1000);
}

/// Build one Thai-family (word-list, no frequencies) DAWG from `data/brkitr/
/// <dict_file>` and write it to `src/unicode/<out_file>`, using `base` as the
/// edge-symbol block base. Words containing any code point outside
/// `base..=base + 0xFF` are dropped (e.g. a handful of Khmer/Burmese entries that
/// embed ZWNJ/ZWJ), matching the runtime `sym`'s `u8`-from-base range. See the
/// doc comment on [`emit_segment_dict`] for the byte layout.
fn emit_thai_family_dict(root: &Path, dict_file: &str, out_file: &str, base: u32) {
    let text = fs::read_to_string(root.join("data/brkitr").join(dict_file))
        .unwrap_or_else(|e| panic!("read {dict_file}: {e}"));
    let mut words: Vec<Vec<char>> = Vec::new();
    let mut dropped = 0usize;
    for line in text.lines() {
        // Strip a leading UTF-8 BOM and surrounding whitespace; skip comments.
        let line = line.trim_start_matches('\u{feff}').trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let w: Vec<char> = line.chars().collect();
        // Drop words with any code point outside the u8-from-base symbol range.
        if w.iter().any(|&c| (c as u32).wrapping_sub(base) > 0xFF) {
            dropped += 1;
            continue;
        }
        words.push(w);
    }
    words.sort();
    words.dedup();

    // Build a trie (node 0 == root).
    struct TNode {
        is_final: bool,
        edges: BTreeMap<char, usize>,
    }
    let mut trie: Vec<TNode> = vec![TNode {
        is_final: false,
        edges: BTreeMap::new(),
    }];
    for w in &words {
        let mut n = 0usize;
        for &c in w {
            let next = match trie[n].edges.get(&c) {
                Some(&x) => x,
                None => {
                    let id = trie.len();
                    trie.push(TNode {
                        is_final: false,
                        edges: BTreeMap::new(),
                    });
                    trie[n].edges.insert(c, id);
                    id
                }
            };
            n = next;
        }
        trie[n].is_final = true;
    }
    let trie_nodes = trie.len();

    // Minimize into a DAWG: canonicalize nodes bottom-up. Two subtrees are merged
    // when they have identical finality and identical (already-canonical) edge
    // sets. Post-order assignment guarantees every child id is < its parent id.
    type CanonNode = (bool, Vec<(u32, u32)>); // (is_final, sorted [(sym, child_id)])
    fn minimize(
        t: usize,
        trie: &[TNode],
        map: &mut BTreeMap<CanonNode, u32>,
        nodes: &mut Vec<CanonNode>,
    ) -> u32 {
        let mut edges: Vec<(u32, u32)> = Vec::new();
        for (&c, &child) in &trie[t].edges {
            let cid = minimize(child, trie, map, nodes);
            edges.push((c as u32, cid));
        }
        edges.sort_unstable();
        let key: CanonNode = (trie[t].is_final, edges);
        if let Some(&id) = map.get(&key) {
            return id;
        }
        let id = nodes.len() as u32;
        nodes.push(key.clone());
        map.insert(key, id);
        id
    }
    let mut map: BTreeMap<CanonNode, u32> = BTreeMap::new();
    let mut nodes: Vec<CanonNode> = Vec::new();
    let root_id = minimize(0, &trie, &mut map, &mut nodes);

    let n = nodes.len();
    let e: usize = nodes.iter().map(|(_, es)| es.len()).sum();
    assert!(n <= u32::MAX as usize, "DAWG node count {n} exceeds u32");
    assert!(e <= u32::MAX as usize, "DAWG edge count {e} exceeds u32");
    // When a dictionary is small enough (Thai/Lao), node ids and edge indices fit
    // a u16, and the compact u16 layout is used (unchanged from the original). A
    // large dictionary (Khmer) auto-widens the offset table and edge targets to
    // u32; the runtime reader detects the width from the u32 `n`/`e` header, so no
    // format flag is needed and small blobs stay byte-identical.
    let wide = n > u16::MAX as usize || e > u16::MAX as usize;

    let mut blob = Vec::new();
    blob.extend_from_slice(&(n as u32).to_le_bytes());
    blob.extend_from_slice(&(e as u32).to_le_bytes());
    blob.extend_from_slice(&root_id.to_le_bytes());
    // Final-flag bitmap.
    let mut bitmap = vec![0u8; n.div_ceil(8)];
    for (i, (is_final, _)) in nodes.iter().enumerate() {
        if *is_final {
            bitmap[i / 8] |= 1 << (i % 8);
        }
    }
    blob.extend_from_slice(&bitmap);
    // Edge-offset table (N+1 cumulative indices, u16 or u32), then edge records.
    let mut off = 0u32;
    for (_, es) in &nodes {
        if wide {
            blob.extend_from_slice(&off.to_le_bytes());
        } else {
            blob.extend_from_slice(&(off as u16).to_le_bytes());
        }
        off += es.len() as u32;
    }
    if wide {
        blob.extend_from_slice(&off.to_le_bytes());
    } else {
        blob.extend_from_slice(&(off as u16).to_le_bytes());
    }
    // Edge records: [u8 sym][target], target u16 or u32.
    for (_, es) in &nodes {
        for &(sym, target) in es {
            let delta = sym
                .checked_sub(base)
                .filter(|d| *d <= 0xFF)
                .unwrap_or_else(|| {
                    panic!("dict codepoint U+{sym:04X} out of U+{base:04X} byte range")
                });
            blob.push(delta as u8);
            if wide {
                blob.extend_from_slice(&target.to_le_bytes());
            } else {
                blob.extend_from_slice(&(target as u16).to_le_bytes());
            }
        }
    }

    let path = root.join("src/unicode").join(out_file);
    fs::write(&path, &blob).unwrap_or_else(|e| panic!("write {out_file}: {e}"));
    println!(
        "codegen: wrote {out_file} ({} words{}, trie {} nodes -> DAWG {} nodes / {} edges, {} KB)",
        words.len(),
        if dropped > 0 {
            format!(", {dropped} dropped (out of U+{base:04X} byte range)")
        } else {
            String::new()
        },
        trie_nodes,
        n,
        e,
        blob.len() / 1024
    );
}

/// Write `src/unicode/segment_dict_cjk.bin`: a minimized DAWG of ICU's
/// Chinese/Japanese break dictionary (`data/brkitr/cjdict.txt`), *with* a
/// per-word cost, used by the `segmentation-dict-cjk` feature to drive the
/// ICU-style `CjkBreakEngine` Viterbi minimum-cost word segmentation.
///
/// Each `word<whitespace>value` line contributes a word whose `value` is the
/// self-negative-log-probability cost that ICU's `gendict` stores verbatim
/// (range ~27..251, always < 255 = `maxSnlp`, so it fits a `u8`; the value `0`
/// is used as the "not a word end" sentinel, since no real cost is 0).
///
/// Layout (all little-endian):
/// ```text
///   u32 node_count N
///   u32 edge_count E
///   u32 root_id
///   values: N bytes — u8 cost per node (0 == not a word end / non-final)
///   edge-offset table: (N+1) u32 (cumulative edge index; node i owns
///                       edges[off[i]..off[i+1]])
///   edges: E records of 5 bytes each — [u16 sym][u24 target], sym = codepoint
///          (cjdict is entirely within the BMP), sorted ascending by sym within
///          each node (binary search at runtime).
/// ```
/// Unlike the Thai DAWG this stores full codepoints (not a byte offset) so it
/// can represent Han; word-end costs are the per-node `values`. The DAWG shares
/// common suffixes, so two word-ends merge only when they share both cost and
/// continuation, still yielding a large but bounded blob.
fn emit_cjk_dict(root: &Path) {
    let text = fs::read_to_string(root.join("data/brkitr/cjdict.txt")).expect("read cjdict.txt");
    // (word, cost) pairs. gendict parses "word [spaces] value"; the word is the
    // first whitespace-delimited token, the cost the second.
    let mut words: Vec<(Vec<char>, u8)> = Vec::new();
    for line in text.lines() {
        let line = line.trim_start_matches('\u{feff}');
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut it = trimmed.split_whitespace();
        let (Some(word), Some(val)) = (it.next(), it.next()) else {
            continue;
        };
        let Ok(cost) = val.parse::<u32>() else {
            continue;
        };
        // Costs are self-neg-log-probabilities; ICU's Viterbi caps the
        // single-char fallback at maxSnlp = 255, and every real cost is < 255.
        assert!(
            (1..=254).contains(&cost),
            "cjdict cost {cost} out of the expected 1..=254 range"
        );
        words.push((word.chars().collect(), cost as u8));
    }
    words.sort();
    words.dedup_by(|a, b| a.0 == b.0);

    // Build a trie (node 0 == root). Each node carries an optional word cost.
    struct TNode {
        cost: u8, // 0 == not a word end
        edges: BTreeMap<char, usize>,
    }
    let mut trie: Vec<TNode> = vec![TNode {
        cost: 0,
        edges: BTreeMap::new(),
    }];
    for (w, cost) in &words {
        let mut n = 0usize;
        for &c in w {
            let next = match trie[n].edges.get(&c) {
                Some(&x) => x,
                None => {
                    let id = trie.len();
                    trie.push(TNode {
                        cost: 0,
                        edges: BTreeMap::new(),
                    });
                    trie[n].edges.insert(c, id);
                    id
                }
            };
            n = next;
        }
        trie[n].cost = *cost;
    }
    let trie_nodes = trie.len();

    // Minimize into a DAWG: two subtrees merge when they have identical cost and
    // identical (already-canonical) edge sets. Post-order assignment guarantees
    // every child id is < its parent id.
    type CanonNode = (u8, Vec<(u32, u32)>); // (cost, sorted [(sym, child_id)])
    fn minimize(
        t: usize,
        trie: &[TNode],
        map: &mut BTreeMap<CanonNode, u32>,
        nodes: &mut Vec<CanonNode>,
    ) -> u32 {
        let mut edges: Vec<(u32, u32)> = Vec::new();
        for (&c, &child) in &trie[t].edges {
            let cid = minimize(child, trie, map, nodes);
            edges.push((c as u32, cid));
        }
        edges.sort_unstable();
        let key: CanonNode = (trie[t].cost, edges);
        if let Some(&id) = map.get(&key) {
            return id;
        }
        let id = nodes.len() as u32;
        nodes.push(key.clone());
        map.insert(key, id);
        id
    }
    let mut map: BTreeMap<CanonNode, u32> = BTreeMap::new();
    let mut nodes: Vec<CanonNode> = Vec::new();
    let root_id = minimize(0, &trie, &mut map, &mut nodes);

    let n = nodes.len();
    let e: usize = nodes.iter().map(|(_, es)| es.len()).sum();
    // Targets are stored as u24; node ids must fit.
    assert!(n <= 0x00FF_FFFF, "CJK DAWG node count {n} exceeds u24");

    let mut blob = Vec::new();
    blob.extend_from_slice(&(n as u32).to_le_bytes());
    blob.extend_from_slice(&(e as u32).to_le_bytes());
    blob.extend_from_slice(&root_id.to_le_bytes());
    // Per-node cost byte (0 == non-final).
    for (cost, _) in &nodes {
        blob.push(*cost);
    }
    // Edge-offset table (N+1 cumulative u32), then the edge records.
    let mut off = 0u32;
    for (_, es) in &nodes {
        blob.extend_from_slice(&off.to_le_bytes());
        off += es.len() as u32;
    }
    blob.extend_from_slice(&off.to_le_bytes());
    for (_, es) in &nodes {
        for &(sym, target) in es {
            let sym = u16::try_from(sym).unwrap_or_else(|_| {
                panic!("cjdict codepoint U+{sym:04X} outside the BMP (u16 sym range)")
            });
            blob.extend_from_slice(&sym.to_le_bytes());
            let t = target.to_le_bytes();
            blob.extend_from_slice(&[t[0], t[1], t[2]]); // u24
        }
    }

    let path = root.join("src/unicode/segment_dict_cjk.bin");
    fs::write(&path, &blob).expect("write segment_dict_cjk.bin");
    println!(
        "codegen: wrote segment_dict_cjk.bin ({} words, trie {} nodes -> DAWG {} nodes / {} edges, {} KB)",
        words.len(),
        trie_nodes,
        n,
        e,
        blob.len() / 1024
    );
}

fn emit_properties(out_dir: &Path, modules: &mut Vec<String>, ucd: &Path) {
    let mut out = String::new();

    // ---- Age: codepoint -> Option<(major, minor)> (None == unassigned). ----
    let age_txt = fs::read_to_string(ucd.join("DerivedAge.txt")).expect("read DerivedAge.txt");
    let mut age_render = vec!["None".to_string()];
    let mut age_code: BTreeMap<(u8, u8), u32> = BTreeMap::new();
    let mut age_codes = vec![0u32; NUM_CODEPOINTS];
    for line in age_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(';');
        let range = parts.next().unwrap().trim();
        let ver = parts.next().unwrap_or("").trim();
        let mut vp = ver.split('.');
        let (Some(maj), Some(min)) = (
            vp.next().and_then(|s| s.parse::<u8>().ok()),
            vp.next().and_then(|s| s.parse::<u8>().ok()),
        ) else {
            continue;
        };
        let code = *age_code.entry((maj, min)).or_insert_with(|| {
            age_render.push(format!("Some(({maj}, {min}))"));
            (age_render.len() - 1) as u32
        });
        let (start, end) = parse_range(range);
        for c in start..=end {
            age_codes[c as usize] = code;
        }
    }
    emit_lookup(
        &mut out,
        "age",
        "age",
        "Option<(u8, u8)>",
        &age_codes,
        0,
        &age_render,
    );

    // ---- Block: codepoint -> &'static str ("No_Block" == default). ----
    let blocks_txt = fs::read_to_string(ucd.join("Blocks.txt")).expect("read Blocks.txt");
    let mut blk_render = vec!["\"No_Block\"".to_string()];
    let mut blk_code: BTreeMap<String, u32> = BTreeMap::new();
    let mut blk_codes = vec![0u32; NUM_CODEPOINTS];
    for line in blocks_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((range, name)) = line.split_once(';') else {
            continue;
        };
        let name = name.trim();
        let code = *blk_code.entry(name.to_string()).or_insert_with(|| {
            blk_render.push(format!("{name:?}"));
            (blk_render.len() - 1) as u32
        });
        let (start, end) = parse_range(range.trim());
        for c in start..=end {
            blk_codes[c as usize] = code;
        }
    }
    emit_lookup(
        &mut out,
        "block",
        "block",
        "&'static str",
        &blk_codes,
        0,
        &blk_render,
    );

    // ---- Joining_Type (Arabic shaping, UAX #9 / ArabicShaping.txt). ----
    out.push_str(
        "/// The `Joining_Type` property (Arabic/Syriac cursive joining, UAX #9).\n\
         #[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum JoiningType {\n    \
         /// `U` — does not join (the default).\n    NonJoining,\n    \
         /// `C` — join-causing (e.g. ARABIC TATWEEL).\n    JoinCausing,\n    \
         /// `D` — dual-joining.\n    DualJoining,\n    \
         /// `L` — left-joining.\n    LeftJoining,\n    \
         /// `R` — right-joining.\n    RightJoining,\n    \
         /// `T` — transparent (combining marks, format chars).\n    Transparent,\n}\n\n",
    );
    let jt_render: Vec<String> = [
        "NonJoining",
        "JoinCausing",
        "DualJoining",
        "LeftJoining",
        "RightJoining",
        "Transparent",
    ]
    .iter()
    .map(|v| format!("JoiningType::{v}"))
    .collect();
    let mut jt_letter: BTreeMap<&str, u32> = BTreeMap::new();
    jt_letter.insert("C", 1);
    jt_letter.insert("D", 2);
    jt_letter.insert("L", 3);
    jt_letter.insert("R", 4);
    jt_letter.insert("T", 5);
    let jt_codes = parse_ranged(&ucd.join("extracted/DerivedJoiningType.txt"), &jt_letter, 0);
    emit_lookup(
        &mut out,
        "joining_type",
        "jt",
        "JoiningType",
        &jt_codes,
        0,
        &jt_render,
    );

    // ---- Indic_Syllabic_Category (UAX #44 / IndicSyllabicCategory.txt). ----
    let isc_txt = fs::read_to_string(ucd.join("IndicSyllabicCategory.txt"))
        .expect("read IndicSyllabicCategory.txt");
    let mut isc_names: BTreeSet<String> = BTreeSet::new();
    for line in isc_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if let Some(v) = line.split(';').nth(1) {
            let v = v.trim();
            if !v.is_empty() {
                isc_names.insert(v.to_string());
            }
        }
    }
    // Index 0 is the default `Other` (the @missing value); the rest are sorted.
    let isc_list: Vec<String> = isc_names.into_iter().filter(|n| n != "Other").collect();
    let mut isc_code: BTreeMap<&str, u32> = BTreeMap::new();
    isc_code.insert("Other", 0);
    let mut isc_render = vec!["IndicSyllabicCategory::Other".to_string()];
    let mut isc_variants = String::from("    Other,\n");
    for (i, n) in isc_list.iter().enumerate() {
        let v = pascal_case(n);
        isc_code.insert(n.as_str(), (i + 1) as u32);
        isc_render.push(format!("IndicSyllabicCategory::{v}"));
        let _ = write!(isc_variants, "    {v},\n");
    }
    let _ = write!(
        out,
        "/// The `Indic_Syllabic_Category` property (UAX #44) for complex-script \
         shaping.\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum IndicSyllabicCategory {{\n{isc_variants}}}\n\n"
    );
    let isc_codes = parse_ranged(&ucd.join("IndicSyllabicCategory.txt"), &isc_code, 0);
    emit_lookup(
        &mut out,
        "indic_syllabic_category",
        "isc",
        "IndicSyllabicCategory",
        &isc_codes,
        0,
        &isc_render,
    );

    // ---- Indic_Positional_Category (UAX #44 / IndicPositionalCategory.txt). ----
    emit_value_enum(
        &mut out,
        ucd,
        "IndicPositionalCategory.txt",
        "IndicPositionalCategory",
        "indic_positional_category",
        "ipc",
        "Not_Applicable",
        "The `Indic_Positional_Category` property (UAX #44): where a dependent \
         character is positioned relative to its base.",
    );

    // ---- Bidi mirroring: codepoint -> mirrored glyph (BidiMirroring.txt). ----
    let bm_txt = fs::read_to_string(ucd.join("BidiMirroring.txt")).expect("read BidiMirroring.txt");
    let mut bm_render = vec!["None".to_string()];
    let mut bm_to_code: BTreeMap<u32, u32> = BTreeMap::new();
    let mut bm_codes = vec![0u32; NUM_CODEPOINTS];
    for line in bm_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(';');
        let (Some(a), Some(b)) = (parts.next(), parts.next()) else {
            continue;
        };
        let (Ok(cp), Ok(mir)) = (
            u32::from_str_radix(a.trim(), 16),
            u32::from_str_radix(b.trim(), 16),
        ) else {
            continue;
        };
        let code = *bm_to_code.entry(mir).or_insert_with(|| {
            bm_render.push(format!("Some('\\u{{{mir:x}}}')"));
            (bm_render.len() - 1) as u32
        });
        bm_codes[cp as usize] = code;
    }
    emit_lookup(
        &mut out,
        "bidi_mirror",
        "bm",
        "Option<char>",
        &bm_codes,
        0,
        &bm_render,
    );

    // ---- Bidi_Mirrored property (UnicodeData.txt field 9 == "Y"). ----
    let ud_bmir = fs::read_to_string(ucd.join("UnicodeData.txt")).expect("read UnicodeData.txt");
    let mut bmir_codes = vec![0u32; NUM_CODEPOINTS];
    for line in ud_bmir.lines() {
        let f: Vec<&str> = line.split(';').collect();
        if f.len() > 9 && f[9] == "Y" {
            if let Ok(cp) = u32::from_str_radix(f[0], 16) {
                bmir_codes[cp as usize] = 1;
            }
        }
    }
    emit_lookup(
        &mut out,
        "bidi_mirrored",
        "bmir",
        "bool",
        &bmir_codes,
        0,
        &[String::from("false"), String::from("true")],
    );

    // ---- Joining_Group (ArabicShaping.txt field 3). ----
    let as_txt = fs::read_to_string(ucd.join("ArabicShaping.txt")).expect("read ArabicShaping.txt");
    let mut jg_names: BTreeSet<String> = BTreeSet::new();
    for line in as_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() >= 4 && f[3] != "No_Joining_Group" {
            jg_names.insert(f[3].to_string());
        }
    }
    let jg_list: Vec<String> = jg_names.into_iter().collect();
    let mut jg_code: BTreeMap<&str, u32> = BTreeMap::new();
    jg_code.insert("No_Joining_Group", 0);
    let mut jg_render = vec!["JoiningGroup::NoJoiningGroup".to_string()];
    let mut jg_variants = String::from("    NoJoiningGroup,\n");
    for (i, n) in jg_list.iter().enumerate() {
        // Group names use spaces and/or underscores (e.g. "AFRICAN FEH",
        // "No_Joining_Group"); normalize both to word separators.
        let v = pascal_case(&n.to_lowercase().replace(' ', "_"));
        jg_code.insert(n.as_str(), (i + 1) as u32);
        jg_render.push(format!("JoiningGroup::{v}"));
        let _ = write!(jg_variants, "    {v},\n");
    }
    let mut jg_codes = vec![0u32; NUM_CODEPOINTS];
    for line in as_txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        let f: Vec<&str> = line.split(';').map(str::trim).collect();
        if f.len() >= 4 {
            if let Some(&code) = jg_code.get(f[3]) {
                let (start, end) = parse_range(f[0]);
                for c in start..=end {
                    jg_codes[c as usize] = code;
                }
            }
        }
    }
    let _ = write!(
        out,
        "/// The `Joining_Group` property (Arabic/Syriac letter shaping class, \
         UAX #9).\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum JoiningGroup {{\n{jg_variants}}}\n\n"
    );
    emit_lookup(
        &mut out,
        "joining_group",
        "jg",
        "JoiningGroup",
        &jg_codes,
        0,
        &jg_render,
    );

    // ---- Algorithmic ideograph name ranges (UnicodeData.txt First/Last rows). ----
    // Codepoints in these ranges have a derived `Name` of the form
    // `<prefix><CP hex>` (e.g. "CJK UNIFIED IDEOGRAPH-4E00"). Hangul syllables are
    // handled separately (their suffix is computed from the jamo).
    let ud = fs::read_to_string(ucd.join("UnicodeData.txt")).expect("read UnicodeData.txt");
    let mut first: Option<(u32, String)> = None;
    let mut ranges: Vec<(u32, u32, &str)> = Vec::new();
    for line in ud.lines() {
        let f: Vec<&str> = line.split(';').collect();
        if f.len() < 2 {
            continue;
        }
        let cp = u32::from_str_radix(f[0], 16).unwrap_or(0);
        let name = f[1];
        if let Some(label) = name
            .strip_suffix(", First>")
            .and_then(|s| s.strip_prefix('<'))
        {
            first = Some((cp, label.to_string()));
        } else if let Some(label) = name
            .strip_suffix(", Last>")
            .and_then(|s| s.strip_prefix('<'))
        {
            if let Some((start, ref flabel)) = first {
                if flabel == label {
                    let prefix = if label.contains("CJK Ideograph") {
                        Some("CJK UNIFIED IDEOGRAPH-")
                    } else if label.contains("Tangut Ideograph") {
                        Some("TANGUT IDEOGRAPH-")
                    } else if label.contains("Khitan Small Script") {
                        Some("KHITAN SMALL SCRIPT CHARACTER-")
                    } else if label.contains("Nushu Character") {
                        Some("NUSHU CHARACTER-")
                    } else {
                        None // Hangul (computed), surrogates, private use: no derived prefix
                    };
                    if let Some(p) = prefix {
                        ranges.push((start, cp, p));
                    }
                }
            }
            first = None;
        }
    }
    ranges.sort_unstable();
    out.push_str(
        "/// The derived-`Name` prefix for an algorithmically-named ideograph \
         codepoint\n/// (the full name is this prefix followed by the uppercase \
         hex codepoint), or\n/// `None` if the codepoint is not in such a range.\n\
         pub(crate) const fn ideograph_name_prefix(cp: u32) -> Option<&'static str> {\n    \
         match cp {\n",
    );
    for (start, end, prefix) in &ranges {
        let _ = write!(out, "        {start:#x}..={end:#x} => Some({prefix:?}),\n");
    }
    out.push_str("        _ => None,\n    }\n}\n\n");

    write_module(out_dir, modules, "properties", &out);
}

/// Emit a `value enum` + paged lookup for a simple single-token ranged UCD
/// property file whose `@missing` default is `default_name`. Index 0 is the
/// default; the remaining values are the sorted distinct names (PascalCased).
#[allow(clippy::too_many_arguments)]
fn emit_value_enum(
    out: &mut String,
    ucd: &Path,
    file: &str,
    enum_name: &str,
    fn_name: &str,
    prefix: &str,
    default_name: &str,
    doc: &str,
) {
    let txt = fs::read_to_string(ucd.join(file)).unwrap_or_else(|_| panic!("read {file}"));
    let mut names: BTreeSet<String> = BTreeSet::new();
    for line in txt.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if let Some(v) = line.split(';').nth(1) {
            let v = v.trim();
            if !v.is_empty() {
                names.insert(v.to_string());
            }
        }
    }
    let list: Vec<String> = names.into_iter().filter(|n| n != default_name).collect();
    let mut code: BTreeMap<&str, u32> = BTreeMap::new();
    code.insert(default_name, 0);
    let mut render = vec![format!("{enum_name}::{}", pascal_case(default_name))];
    let mut variants = format!("    {},\n", pascal_case(default_name));
    for (i, n) in list.iter().enumerate() {
        let v = pascal_case(n);
        code.insert(n.as_str(), (i + 1) as u32);
        render.push(format!("{enum_name}::{v}"));
        let _ = write!(variants, "    {v},\n");
    }
    let _ = write!(
        out,
        "/// {doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n\
         pub enum {enum_name} {{\n{variants}}}\n\n"
    );
    let codes = parse_ranged(&ucd.join(file), &code, 0);
    emit_lookup(out, fn_name, prefix, enum_name, &codes, 0, &render);
}

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
/// Compact-notation magnitudes 10³…10¹⁴ (the `decimalFormat` keys are
/// `<magnitude>-count-other`).
const COMPACT_MAGNITUDES: [&str; 12] = [
    "1000",
    "10000",
    "100000",
    "1000000",
    "10000000",
    "100000000",
    "1000000000",
    "10000000000",
    "100000000000",
    "1000000000000",
    "10000000000000",
    "100000000000000",
];

/// Write `cldr/numbers.bin` and `cldr/compact.bin` from the raw, verbatim
/// Unicode CLDR `cldr-numbers-full` `numbers.json` files vendored under
/// `data/cldr/48/numbers-raw/<locale>.json`.
///
/// `numbers.bin` payload: decimal/group/minus/plus/percent symbols, then the
/// parsed decimal and percent `Pattern`s (latn numbering system).
/// `compact.bin` payload: 12 short then 12 long compact patterns (the
/// `count-other` form for each magnitude 10³…10¹⁴).
/// Duplicate every `lang-Script` record under the `lang-REGION` tags that CLDR's
/// likelySubtags maximizes onto that same script. The runtime locale lookup
/// truncates a tag at each `-` and does no script inference, so `zh-TW` would
/// otherwise fall past the Traditional data vendored as `zh-Hant` all the way to
/// `zh`, which is Simplified. The aliases are exact payload copies — a few dozen
/// bytes each — and never shadow a locale that has data of its own.
fn script_region_aliases(records: &[(String, Vec<u8>)], likely: &Json) -> Vec<(String, Vec<u8>)> {
    let keys: Vec<String> = records.iter().map(|(k, _)| k.clone()).collect();
    script_region_alias_keys(&keys, likely)
        .into_iter()
        .map(|(alias, src)| {
            let payload = records
                .iter()
                .find(|(k, _)| *k == src)
                .map(|(_, p)| p.clone())
                .expect("alias source record");
            (alias, payload)
        })
        .collect()
}

/// The `(alias, source)` key pairs behind [`script_region_aliases`], for tables
/// that store one record per locale *index* rather than per payload copy.
fn script_region_alias_keys(keys: &[String], likely: &Json) -> Vec<(String, String)> {
    let is_region = |s: &str| {
        (s.len() == 2 && s.chars().all(|c| c.is_ascii_alphabetic()))
            || (s.len() == 3 && s.chars().all(|c| c.is_ascii_digit()))
    };
    let map = likely.get("map").expect("likely map");
    let mut out: Vec<(String, String)> = Vec::new();
    for key in keys {
        let Some((lang, script)) = key.split_once('-') else {
            continue;
        };
        if script.len() != 4 || !script.chars().all(|c| c.is_ascii_alphabetic()) {
            continue;
        }
        for (from, to) in map.entries() {
            // `zh-TW` -> `zh-Hant-TW`: same language, maximizing onto this script.
            let mut max = to.as_str().unwrap_or("").split('-');
            let (Some(l), Some(s)) = (max.next(), max.next()) else {
                continue;
            };
            if !l.eq_ignore_ascii_case(lang) || !s.eq_ignore_ascii_case(script) {
                continue;
            }
            let alias = from.to_ascii_lowercase();
            let Some((_, region)) = alias.split_once('-') else {
                continue;
            };
            if !is_region(region) || keys.contains(&alias) {
                continue;
            }
            out.push((alias, key.clone()));
        }
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

/// One locale's number spec for a single numbering system: the CLDR
/// `symbols-numberSystem-<ns>` block plus the decimal/percent patterns of
/// `decimalFormats-/percentFormats-numberSystem-<ns>` (which genuinely differ
/// per system — `te` groups Indian-style in `latn` but not in `telu`).
struct NsSpec {
    ns: String,
    decimal: String,
    group: String,
    minus: String,
    plus: String,
    percent: String,
    nan: String,
    infinity: String,
    dec: PatFields,
    pct: PatFields,
}

/// Everything `src/cldr/generated/numbers.rs` holds for one locale.
struct NumbersRecord {
    /// `latn` first, then the other systems CLDR ships symbols for, sorted.
    specs: Vec<NsSpec>,
    /// `defaultNumberingSystem` and `otherNumberingSystems.native`.
    default_ns: String,
    native_ns: String,
    /// The `miscPatterns` `approximately` and `range` forms.
    approximately: String,
    range: String,
}

fn emit_numbers(cldr_dir: &Path, numbers_dir: &Path, likely_path: &Path) {
    let mut files = locale_files(numbers_dir);
    files.sort();

    let mut locales: Vec<String> = Vec::new();
    let mut records: Vec<NumbersRecord> = Vec::new();
    let mut compact_records = Vec::new();
    for locale in files {
        let path = numbers_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let main = json.get("main").expect("main");
        let (_, loc_obj) = main.entries().first().expect("locale entry");
        let n = loc_obj.get("numbers").expect("numbers");
        let g = |o: &Json, k: &str| o.get(k).and_then(Json::as_str).unwrap_or("").to_string();

        // Every numbering system the locale ships symbols for, `latn` first: the
        // runtime resolves a `-u-nu-` request against this set and falls back to
        // `latn`, matching ICU's `NumberElements/<ns>/symbols` lookup.
        let mut systems: Vec<String> = n
            .entries()
            .iter()
            .filter_map(|(k, _)| k.strip_prefix("symbols-numberSystem-"))
            .map(String::from)
            .collect();
        systems.sort();
        systems.sort_by_key(|s| s != "latn");

        let mut specs: Vec<NsSpec> = Vec::new();
        for ns in systems {
            let sym = n
                .get(&alloc_concat("symbols-numberSystem-", &ns))
                .expect("symbols block");
            let percent = g(sym, "percentSign");
            // A system can carry symbols without its own patterns; CLDR's
            // resource inheritance then supplies the `latn` ones.
            let pat = |kind: &str| {
                n.get(&alloc_concat(kind, &alloc_concat("-numberSystem-", &ns)))
                    .or_else(|| n.get(&alloc_concat(kind, "-numberSystem-latn")))
                    .and_then(|x| x.get("standard"))
                    .and_then(Json::as_str)
                    .unwrap_or("")
                    .to_string()
            };
            specs.push(NsSpec {
                dec: parse_number_pattern(&pat("decimalFormats"), &percent),
                pct: parse_number_pattern(&pat("percentFormats"), &percent),
                decimal: g(sym, "decimal"),
                group: g(sym, "group"),
                minus: g(sym, "minusSign"),
                plus: g(sym, "plusSign"),
                nan: g(sym, "nan"),
                infinity: g(sym, "infinity"),
                percent,
                ns,
            });
        }
        assert_eq!(specs[0].ns, "latn", "{locale}: no latn symbols");

        // `miscPatterns` are per numbering system in CLDR but identical across
        // systems in all 103 vendored locales, so one per locale is enough.
        let misc = n
            .get("miscPatterns-numberSystem-latn")
            .expect("miscPatterns-numberSystem-latn");
        let other = n.get("otherNumberingSystems");
        let default_ns = g(n, "defaultNumberingSystem");
        records.push(NumbersRecord {
            specs,
            native_ns: other
                .and_then(|o| o.get("native"))
                .and_then(Json::as_str)
                .unwrap_or(&default_ns)
                .to_string(),
            default_ns,
            approximately: g(misc, "approximately"),
            range: g(misc, "range"),
        });

        // Compact short then long, `count-other` per magnitude.
        let dec_fmt = n
            .get("decimalFormats-numberSystem-latn")
            .expect("decimalFormats");
        let mut c = Vec::new();
        for width in ["short", "long"] {
            let df = dec_fmt.get(width).and_then(|w| w.get("decimalFormat"));
            for mag in COMPACT_MAGNITUDES {
                let key = alloc_concat(mag, "-count-other");
                let pat = df
                    .and_then(|d| d.get(&key))
                    .and_then(Json::as_str)
                    .unwrap_or("0");
                enc_str(&mut c, pat);
            }
        }
        compact_records.push((locale.to_ascii_lowercase(), c));
        locales.push(locale.to_ascii_lowercase());
    }

    let likely_text = fs::read_to_string(likely_path).expect("read likely.json");
    let likely = json_parse(&likely_text);
    compact_records.extend(script_region_aliases(&compact_records, &likely));
    write_blob(cldr_dir, "compact", &compact_records);

    // `lang-REGION` tags that maximize onto a vendored `lang-Script` record share
    // that record's table index rather than duplicating its arms.
    let aliases: Vec<(String, usize)> = script_region_alias_keys(&locales, &likely)
        .into_iter()
        .map(|(alias, src)| {
            let i = locales
                .iter()
                .position(|l| *l == src)
                .expect("alias source");
            (alias, i)
        })
        .collect();

    write_numbers_rs(cldr_dir, &locales, &records, &aliases);
}

/// Render a parsed CLDR number pattern as a `crate::cldr::Pattern` literal.
fn rust_pattern(p: &PatFields) -> String {
    format!(
        "Pattern {{ prefix: {}, suffix: {}, min_int: {}, min_frac: {}, max_frac: {}, primary_group: {}, secondary_group: {} }}",
        rust_str(&p.prefix),
        rust_str(&p.suffix),
        p.min_int,
        p.min_frac,
        p.max_frac,
        p.primary,
        p.secondary,
    )
}

/// Render one numbering system's block as a `crate::cldr::NumberSpec` literal.
fn rust_spec(s: &NsSpec) -> String {
    format!(
        "NumberSpec {{ decimal: {}, group: {}, minus: {}, plus: {}, percent: {}, nan: {}, infinity: {}, dec: {}, pct: {} }}",
        rust_str(&s.decimal),
        rust_str(&s.group),
        rust_str(&s.minus),
        rust_str(&s.plus),
        rust_str(&s.percent),
        rust_str(&s.nan),
        rust_str(&s.infinity),
        rust_pattern(&s.dec),
        rust_pattern(&s.pct),
    )
}

/// Emit `cldr/generated/numbers.rs`: per-locale number symbols and patterns as
/// `match` lookups. Unlike a blob this can be gated arm by arm, which is why the
/// non-`latn` numbering-system blocks and the `miscPatterns` live here.
fn write_numbers_rs(
    cldr_dir: &Path,
    locales: &[String],
    records: &[NumbersRecord],
    aliases: &[(String, usize)],
) {
    let mut out = String::new();
    write_header(&mut out);
    let _ = write!(
        out,
        "//! CLDR number symbols and patterns (UTS #35 `numbers.json`).\n\
         //!\n\
         //! Emitted as Rust rather than a blob because the per-numbering-system\n\
         //! blocks and the `miscPatterns` are wanted only by some builds, and\n\
         //! `#[cfg]` can drop individual match arms.\n\
         //!\n\
         //! Every accessor is keyed by an exact (lowercased) CLDR locale id and\n\
         //! returns `None` for anything else; walking the fallback chain (and the\n\
         //! final drop to `en`) is the caller's job, as for the `.bin` tables.\n\n\
         use crate::cldr::{{NumberSpec, Pattern}};\n\n\
         /// Table index for an exact (lowercased) CLDR locale id. `lang-REGION`\n\
         /// tags that CLDR maximizes onto a vendored `lang-Script` record share\n\
         /// its index (`zh-tw` -> `zh-hant`).\n\
         fn locale_index(lang: &str) -> Option<u16> {{\n    \
         Some(match lang {{\n"
    );
    let mut keys: Vec<(&str, usize)> = locales
        .iter()
        .enumerate()
        .map(|(i, l)| (l.as_str(), i))
        .chain(aliases.iter().map(|(a, i)| (a.as_str(), *i)))
        .collect();
    keys.sort();
    for (key, i) in keys {
        let _ = write!(out, "        \"{key}\" => {i},\n");
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    let _ = write!(
        out,
        "/// The number spec for `lang` in numbering system `ns`. CLDR only ships\n\
         /// symbols for a handful of systems per locale; anything else resolves to\n\
         /// the locale's `latn` block, which is ICU's `NumberElements` fallback.\n\
         pub(crate) fn spec(lang: &str, ns: &str) -> Option<NumberSpec> {{\n    \
         let i = locale_index(lang)?;\n    \
         if let Some(s) = other(i, ns) {{\n        return Some(s);\n    }}\n    \
         latn(i)\n}}\n\n\
         /// The `latn` spec for a table index (every locale has one).\n\
         const fn latn(i: u16) -> Option<NumberSpec> {{\n    Some(match i {{\n"
    );
    for (i, r) in records.iter().enumerate() {
        let _ = write!(out, "        {i} => {},\n", rust_spec(&r.specs[0]));
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    // The non-`latn` blocks: 26 of the 103 locales carry one, always for the
    // system named by `otherNumberingSystems.native`. Gated per arm so a build
    // that never asks for a non-Latin numbering system does not compile them.
    let _ = write!(
        out,
        "/// A non-`latn` numbering system's block, where CLDR ships one for this\n\
         /// locale. Compiled only with the `number-numsys` feature; without it a\n\
         /// `-u-nu-` request keeps the locale's `latn` separators (the digits are\n\
         /// still transliterated).\n\
         fn other(i: u16, ns: &str) -> Option<NumberSpec> {{\n    match (i, ns) {{\n"
    );
    for (i, r) in records.iter().enumerate() {
        for s in &r.specs[1..] {
            let _ = write!(
                out,
                "        #[cfg(feature = \"number-numsys\")]\n        ({i}, \"{}\") => Some({}),\n",
                s.ns,
                rust_spec(s)
            );
        }
    }
    let _ = write!(out, "        _ => None,\n    }}\n}}\n\n");

    let _ = write!(
        out,
        "/// `(defaultNumberingSystem, otherNumberingSystems.native)` for an exact\n\
         /// (lowercased) locale id. The two differ for e.g. `ar` (`latn` / `arab`).\n\
         pub(crate) fn numbering_systems(lang: &str) -> Option<(&'static str, &'static str)> {{\n    \
         Some(match locale_index(lang)? {{\n"
    );
    for (i, r) in records.iter().enumerate() {
        let _ = write!(
            out,
            "        {i} => ({}, {}),\n",
            rust_str(&r.default_ns),
            rust_str(&r.native_ns)
        );
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    let _ = write!(
        out,
        "/// The `miscPatterns` `(approximately, range)` forms for an exact\n\
         /// (lowercased) locale id, e.g. `en` `(\"~{{0}}\", \"{{0}}\\u{{2013}}{{1}}\")`.\n\
         /// Only `format_range` reads these, so they follow the `number-range`\n\
         /// feature.\n\
         #[cfg(feature = \"number-range\")]\n\
         pub(crate) fn misc_patterns(lang: &str) -> Option<(&'static str, &'static str)> {{\n    \
         Some(match locale_index(lang)? {{\n"
    );
    for (i, r) in records.iter().enumerate() {
        let _ = write!(
            out,
            "        {i} => ({}, {}),\n",
            rust_str(&r.approximately),
            rust_str(&r.range)
        );
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n");

    write_cldr_generated(cldr_dir, "numbers", &out);
}

fn alloc_format(locale: &str) -> String {
    let mut s = String::from(locale);
    s.push_str(".json");
    s
}

fn alloc_concat(a: &str, b: &str) -> String {
    let mut s = String::from(a);
    s.push_str(b);
    s
}

/// Write `cldr/lists.bin`: per-locale list connector patterns (and / or).
fn emit_lists(cldr_dir: &Path, lists_dir: &Path) {
    let mut locales = locale_files(lists_dir);
    locales.sort();
    let mut records = Vec::new();
    for locale in locales {
        let path = lists_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let lp = loc_obj.get("listPatterns").expect("listPatterns");
        let mut p = Vec::new();
        for style_key in ["listPattern-type-standard", "listPattern-type-or"] {
            let st = lp.get(style_key).expect("style");
            for k in ["start", "middle", "end", "2"] {
                enc_str(&mut p, st.get(k).and_then(Json::as_str).unwrap_or(""));
            }
        }
        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "lists", &records);
}

/// Sorted base locale names from a directory of `<locale>.json` files.
fn locale_files(dir: &Path) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap_or_else(|_| panic!("read {}", dir.display()))
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_suffix(".json")
                .map(String::from)
        })
        .collect()
}

/// Write `cldr/relative.bin`: per-locale relative-time strings (7 units).
fn emit_relative(cldr_dir: &Path, datefields_dir: &Path) {
    let units = ["year", "month", "week", "day", "hour", "minute", "second"];
    let cat_index = |key: &str| match key {
        "relativeTimePattern-count-zero" => 0,
        "relativeTimePattern-count-one" => 1,
        "relativeTimePattern-count-two" => 2,
        "relativeTimePattern-count-few" => 3,
        "relativeTimePattern-count-many" => 4,
        _ => 5,
    };
    let mut locales = locale_files(datefields_dir);
    locales.sort();
    let mut records = Vec::new();
    for locale in locales {
        let path = datefields_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let fields = loc_obj
            .get("dates")
            .and_then(|d| d.get("fields"))
            .expect("fields");
        let mut p = Vec::new();
        for u in units {
            let f = fields.get(u).expect("unit");
            enc_opt(&mut p, f.get("relative-type--1").and_then(Json::as_str));
            enc_opt(&mut p, f.get("relative-type-0").and_then(Json::as_str));
            enc_opt(&mut p, f.get("relative-type-1").and_then(Json::as_str));
            for tense in ["relativeTime-type-past", "relativeTime-type-future"] {
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
        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "relative", &records);
}

/// Write `cldr/currency.bin` (per-locale pattern + symbols) and
/// `cldr/currency_digits.bin` (per-currency fraction digits).
/// Write `cldr/currency.bin` and `cldr/currency_digits.bin` from raw CLDR:
/// per-locale `currencies.json` (symbol / narrow symbol / display name), the
/// currency pattern from `numbers.json`, and per-currency fraction digits from
/// the supplemental `currencyData.json`.
///
/// `currency.bin` payload: currency `Pattern`, then `[u16 count]` and, per
/// currency, `(code, symbol, narrow-symbol, display-name)`.
fn emit_currency(cldr_dir: &Path, currencies_dir: &Path, numbers_dir: &Path, currency_data: &Path) {
    // ---- currency_digits.bin from supplemental fractions ----
    let cd_text = fs::read_to_string(currency_data).expect("read currencyData.json");
    let cd = json_parse(&cd_text);
    let fractions = cd
        .get("supplemental")
        .and_then(|s| s.get("currencyData"))
        .and_then(|c| c.get("fractions"))
        .expect("fractions");
    let mut digit_records = Vec::new();
    for (code, info) in fractions.entries() {
        if code == "DEFAULT" {
            continue;
        }
        let digits: u8 = info
            .get("_digits")
            .and_then(Json::as_str)
            .and_then(|d| d.parse().ok())
            .unwrap_or(2);
        digit_records.push((code.clone(), vec![digits]));
    }
    write_blob(cldr_dir, "currency_digits", &digit_records);

    // ---- currency.bin per locale ----
    let mut files: Vec<String> = fs::read_dir(currencies_dir)
        .expect("read currencies-raw dir")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_suffix(".json")
                .map(String::from)
        })
        .collect();
    files.sort();

    let mut records = Vec::new();
    for locale in files {
        let cur_text = fs::read_to_string(currencies_dir.join(alloc_format(&locale)))
            .unwrap_or_else(|_| panic!("read currencies {locale}"));
        let cur_json = json_parse(&cur_text);
        let (_, cur_loc) = cur_json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let currencies = cur_loc
            .get("numbers")
            .and_then(|n| n.get("currencies"))
            .expect("currencies");

        let num_text = fs::read_to_string(numbers_dir.join(alloc_format(&locale)))
            .unwrap_or_else(|_| panic!("read numbers {locale}"));
        let num_json = json_parse(&num_text);
        let (_, num_loc) = num_json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let cur_fmt = num_loc
            .get("numbers")
            .and_then(|n| n.get("currencyFormats-numberSystem-latn"));
        let pat = cur_fmt
            .and_then(|f| f.get("standard"))
            .and_then(Json::as_str)
            .unwrap_or("");
        // The unit pattern (number + currency code/name), e.g. "{0} {1}".
        let unit_pat = cur_fmt
            .and_then(|f| f.get("unitPattern-count-other"))
            .and_then(Json::as_str)
            .unwrap_or("{0} {1}");

        let mut p = Vec::new();
        enc_pattern(&mut p, &parse_number_pattern(pat, ""));
        enc_str(&mut p, unit_pat);
        let entries = currencies.entries();
        p.extend_from_slice(&(entries.len() as u16).to_le_bytes());
        for (code, info) in entries {
            let sym = info.get("symbol").and_then(Json::as_str).unwrap_or(code);
            let narrow = info
                .get("symbol-alt-narrow")
                .and_then(Json::as_str)
                .unwrap_or(sym);
            // Prefer the plural "other" display name ("US dollars") over the
            // base ("US Dollar"); ECMA uses the plural-selected form and "other"
            // matches the common (non-1) case.
            let name = info
                .get("displayName-count-other")
                .or_else(|| info.get("displayName"))
                .and_then(Json::as_str)
                .unwrap_or(code);
            enc_str(&mut p, code);
            enc_str(&mut p, sym);
            enc_str(&mut p, narrow);
            enc_str(&mut p, name);
        }
        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "currency", &records);
}

/// Write `cldr/calendar.bin`: per-locale Gregorian month/day names, am/pm, and
/// date/time/combining patterns. Payload (all required strings): months_wide(12),
/// months_abbr(12), days_wide(7), days_abbr(7), am, pm, date(4), time(4),
/// datetime(4) — styles in full/long/medium/short order.
/// Write `cldr/calendar.bin` and `cldr/skeletons.bin` from the raw, verbatim
/// Unicode CLDR `cldr-dates-full` `ca-gregorian.json` files vendored under
/// `data/cldr/48/dates/<locale>/` (see that dir's README for provenance).
///
/// `calendar.bin` payload order (the leading block is unchanged from the prior
/// trimmed layout so the runtime reader's existing reads stay valid; the narrow
/// and era blocks are appended):
///   months_wide[12], months_abbr[12], days_wide[7], days_abbr[7], am, pm,
///   date[4], time[4], datetime[4],
///   months_narrow[12], days_narrow[7], eras_wide[2], eras_abbr[2], eras_narrow[2]
/// (date/time/datetime are full, long, medium, short; eras are indexed 0 = BCE,
/// 1 = CE.)
///
/// `skeletons.bin` payload: the locale's `availableFormats` map (canonical keys
/// only — the `-alt-ascii` and `-count-*` variant keys are dropped), as
/// `[u16 count]` then `(skeleton, pattern)` string pairs.
/// The flexible day-period keys, in the index order shared by codegen and the
/// runtime: midnight, noon, then the range periods.
const DAY_PERIOD_KEYS: [&str; 10] = [
    "midnight",
    "noon",
    "morning1",
    "morning2",
    "afternoon1",
    "afternoon2",
    "evening1",
    "evening2",
    "night1",
    "night2",
];

/// Parse a `"HH:mm"` day-period boundary to a minute-of-day 0..=1440. CLDR's
/// boundaries are currently all on the hour, but minutes are preserved so
/// sub-hour boundaries (should CLDR ever add them) resolve correctly.
fn dp_minutes(t: &str) -> u16 {
    let mut it = t.split(':');
    let h: u16 = it.next().and_then(|h| h.parse().ok()).unwrap_or(0);
    let m: u16 = it.next().and_then(|m| m.parse().ok()).unwrap_or(0);
    h * 60 + m
}

/// Build the minute-resolution day-period range rules for a locale from its
/// `_from`/`_before` rules (`_at` midnight/noon points are applied at runtime for
/// the exact instant). Each rule is `(from_minute, to_minute, period_index)` with
/// `from < to`; a rule that wraps past midnight is split into two non-wrapping
/// rules covering `[from, 1440)` and `[0, before)`.
fn day_period_rules(rules: Option<&Json>) -> Vec<(u16, u16, u8)> {
    let mut out = Vec::new();
    let Some(rules) = rules else { return out };
    for (idx, key) in DAY_PERIOD_KEYS.iter().enumerate() {
        let Some(rule) = rules.get(key) else { continue };
        let (from, before) = match (rule.get("_from"), rule.get("_before")) {
            (Some(f), Some(b)) => (
                dp_minutes(f.as_str().unwrap_or("0")),
                dp_minutes(b.as_str().unwrap_or("0")),
            ),
            _ => continue, // _at rule (midnight/noon): not a range
        };
        let end = if before == 0 { 1440 } else { before };
        if end > from {
            out.push((from, end, idx as u8));
        } else {
            // Wraps past midnight: split into [from, 1440) and [0, before).
            out.push((from, 1440, idx as u8));
            out.push((0, end, idx as u8));
        }
    }
    out
}

fn emit_dates(cldr_dir: &Path, dates_dir: &Path, day_periods_path: &Path) {
    let dp_text = fs::read_to_string(day_periods_path).expect("read dayPeriods.json");
    let dp_json = json_parse(&dp_text);
    let dp_rules = dp_json
        .get("supplemental")
        .and_then(|s| s.get("dayPeriodRuleSet"));
    let mut locales: Vec<String> = fs::read_dir(dates_dir)
        .expect("read dates dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    locales.sort();

    let mut cal_records = Vec::new();
    let mut skel_records = Vec::new();

    for locale in locales {
        let path = dates_dir.join(&locale).join("ca-gregorian.json");
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        // main -> <locale> -> dates -> calendars -> gregorian
        let main = json.get("main").expect("main");
        let (_, loc_obj) = main.entries().first().expect("locale entry");
        let greg = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get("gregorian"))
            .expect("gregorian");

        let months = greg
            .get("months")
            .and_then(|m| m.get("format"))
            .expect("months.format");
        let days = greg
            .get("days")
            .and_then(|d| d.get("format"))
            .expect("days.format");
        let day_keys = ["sun", "mon", "tue", "wed", "thu", "fri", "sat"];
        let push_months = |p: &mut Vec<u8>, width: &str| {
            let w = months
                .get(width)
                .unwrap_or_else(|| panic!("months.{width}"));
            for m in 1..=12u8 {
                enc_str(
                    p,
                    w.get(&m.to_string()).and_then(Json::as_str).unwrap_or(""),
                );
            }
        };
        let push_days = |p: &mut Vec<u8>, width: &str| {
            let w = days.get(width).unwrap_or_else(|| panic!("days.{width}"));
            for k in day_keys {
                enc_str(p, w.get(k).and_then(Json::as_str).unwrap_or(""));
            }
        };

        // ---- calendar.bin payload ----
        let mut p = Vec::new();
        push_months(&mut p, "wide");
        push_months(&mut p, "abbreviated");
        push_days(&mut p, "wide");
        push_days(&mut p, "abbreviated");

        let dp = greg
            .get("dayPeriods")
            .and_then(|d| d.get("format"))
            .and_then(|f| f.get("abbreviated"));
        enc_str(
            &mut p,
            dp.and_then(|d| d.get("am"))
                .and_then(Json::as_str)
                .unwrap_or("AM"),
        );
        enc_str(
            &mut p,
            dp.and_then(|d| d.get("pm"))
                .and_then(Json::as_str)
                .unwrap_or("PM"),
        );

        let order = ["full", "long", "medium", "short"];
        let push_patterns = |p: &mut Vec<u8>, group: &str| {
            let g = greg.get(group).unwrap_or_else(|| panic!("{group}"));
            for k in order {
                enc_str(p, g.get(k).and_then(Json::as_str).unwrap_or(""));
            }
        };
        push_patterns(&mut p, "dateFormats");
        push_patterns(&mut p, "timeFormats");
        push_patterns(&mut p, "dateTimeFormats");

        // appended blocks: narrow widths + eras
        push_months(&mut p, "narrow");
        push_days(&mut p, "narrow");
        let eras = greg.get("eras").expect("eras");
        for variant in ["eraNames", "eraAbbr", "eraNarrow"] {
            let e = eras
                .get(variant)
                .unwrap_or_else(|| panic!("eras.{variant}"));
            enc_str(&mut p, e.get("0").and_then(Json::as_str).unwrap_or(""));
            enc_str(&mut p, e.get("1").and_then(Json::as_str).unwrap_or(""));
        }

        // appended block: flexible day-period names (10 keys × 3 widths, each
        // optional) + the minute-resolution day-period range rules from the
        // supplemental rules.
        let dp_fmt = greg.get("dayPeriods").and_then(|d| d.get("format"));
        for width in ["wide", "abbreviated", "narrow"] {
            let w = dp_fmt.and_then(|f| f.get(width));
            for key in DAY_PERIOD_KEYS {
                enc_opt(&mut p, w.and_then(|x| x.get(key)).and_then(Json::as_str));
            }
        }
        let rules = dp_rules
            .and_then(|r| r.get(&locale))
            .or_else(|| dp_rules.and_then(|r| r.get(locale.split('-').next().unwrap_or(&locale))));
        // Minute-resolution day-period range rules: [u8 count] then count
        // records of [u16 from_minute][u16 to_minute][u8 period_index].
        let dp_ranges = day_period_rules(rules);
        p.push(dp_ranges.len() as u8);
        for (from, to, idx) in dp_ranges {
            p.extend_from_slice(&from.to_le_bytes());
            p.extend_from_slice(&to.to_le_bytes());
            p.push(idx);
        }
        cal_records.push((locale.to_ascii_lowercase(), p));

        // ---- skeletons.bin payload ----
        let avail = greg
            .get("dateTimeFormats")
            .and_then(|d| d.get("availableFormats"))
            .expect("availableFormats");
        let kept: Vec<(&str, &str)> = avail
            .entries()
            .iter()
            .filter(|(k, _)| !k.contains('-'))
            .filter_map(|(k, v)| v.as_str().map(|s| (k.as_str(), s)))
            .collect();
        let mut sk = Vec::new();
        sk.extend_from_slice(&(kept.len() as u16).to_le_bytes());
        for (k, v) in kept {
            enc_str(&mut sk, k);
            enc_str(&mut sk, v);
        }
        skel_records.push((locale.to_ascii_lowercase(), sk));
    }

    write_blob(cldr_dir, "calendar", &cal_records);
    write_blob(cldr_dir, "skeletons", &skel_records);
}

/// Write `cldr/intervals.bin`: per-locale CLDR `intervalFormats`. Payload is
///   [str fallback]
///   [u16 skeleton count]
///   skeleton × count: [str skeleton][u8 field count]
///                     field × count: [u8 field letter][str pattern]
/// Skeleton keys and field letters are sorted for deterministic output.
fn emit_intervals(cldr_dir: &Path, dates_dir: &Path) {
    let mut locales: Vec<String> = fs::read_dir(dates_dir)
        .expect("read dates dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    locales.sort();

    let mut records = Vec::new();
    for locale in locales {
        let path = dates_dir.join(&locale).join("ca-gregorian.json");
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let main = json.get("main").expect("main");
        let (_, loc_obj) = main.entries().first().expect("locale entry");
        let iv = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get("gregorian"))
            .and_then(|g| g.get("dateTimeFormats"))
            .and_then(|d| d.get("intervalFormats"));
        let Some(iv) = iv else { continue };

        let fallback = iv
            .get("intervalFormatFallback")
            .and_then(Json::as_str)
            .unwrap_or("{0} – {1}");

        // Collect (skeleton, [(field letter, pattern)]) for every skeleton entry
        // (every key except the fallback, whose value is a string not an object).
        let mut skeletons: Vec<(&str, Vec<(u8, &str)>)> = Vec::new();
        for (sk, val) in iv.entries() {
            if sk == "intervalFormatFallback" {
                continue;
            }
            let Json::Obj(_) = val else { continue };
            let mut fields: Vec<(u8, &str)> = val
                .entries()
                .iter()
                .filter_map(|(f, p)| {
                    let letter = *f.as_bytes().first()?;
                    p.as_str().map(|s| (letter, s))
                })
                .collect();
            fields.sort_by_key(|(f, _)| *f);
            skeletons.push((sk.as_str(), fields));
        }
        skeletons.sort_by(|a, b| a.0.cmp(b.0));

        let mut p = Vec::new();
        enc_str(&mut p, fallback);
        p.extend_from_slice(&(skeletons.len() as u16).to_le_bytes());
        for (sk, fields) in skeletons {
            enc_str(&mut p, sk);
            p.push(fields.len() as u8);
            for (letter, pat) in fields {
                p.push(letter);
                enc_str(&mut p, pat);
            }
        }
        records.push((locale.to_ascii_lowercase(), p));
    }

    write_blob(cldr_dir, "intervals", &records);
}

/// The ECMA-402 sanctioned measurement units — `(unit identifier, CLDR key)` —
/// in the order the runtime `Unit` enum expects. The CLDR category prefix is
/// irregular (`concentr-percent`, `angle-degree`, …), so the mapping has to be
/// tabulated rather than derived. Indices 24/25 are the two `speed-…` compounds
/// CLDR ships pre-composed; keeping them avoids deriving `"{0} mi/h"` where the
/// locale data says `"{0} mph"`.
const UNITS: [(&str, &str); 47] = [
    ("second", "duration-second"),
    ("minute", "duration-minute"),
    ("hour", "duration-hour"),
    ("day", "duration-day"),
    ("week", "duration-week"),
    ("month", "duration-month"),
    ("year", "duration-year"),
    ("millimeter", "length-millimeter"),
    ("centimeter", "length-centimeter"),
    ("meter", "length-meter"),
    ("kilometer", "length-kilometer"),
    ("inch", "length-inch"),
    ("foot", "length-foot"),
    ("mile", "length-mile"),
    ("gram", "mass-gram"),
    ("kilogram", "mass-kilogram"),
    ("ounce", "mass-ounce"),
    ("pound", "mass-pound"),
    ("byte", "digital-byte"),
    ("kilobyte", "digital-kilobyte"),
    ("megabyte", "digital-megabyte"),
    ("gigabyte", "digital-gigabyte"),
    ("celsius", "temperature-celsius"),
    ("fahrenheit", "temperature-fahrenheit"),
    ("kilometer-per-hour", "speed-kilometer-per-hour"),
    ("mile-per-hour", "speed-mile-per-hour"),
    ("liter", "volume-liter"),
    ("milliliter", "volume-milliliter"),
    ("acre", "area-acre"),
    ("bit", "digital-bit"),
    ("degree", "angle-degree"),
    ("fluid-ounce", "volume-fluid-ounce"),
    ("gallon", "volume-gallon"),
    ("gigabit", "digital-gigabit"),
    ("hectare", "area-hectare"),
    ("kilobit", "digital-kilobit"),
    ("megabit", "digital-megabit"),
    ("microsecond", "duration-microsecond"),
    ("mile-scandinavian", "length-mile-scandinavian"),
    ("millisecond", "duration-millisecond"),
    ("nanosecond", "duration-nanosecond"),
    ("percent", "concentr-percent"),
    ("petabyte", "digital-petabyte"),
    ("stone", "mass-stone"),
    ("terabit", "digital-terabit"),
    ("terabyte", "digital-terabyte"),
    ("yard", "length-yard"),
];

/// CLDR unit widths, in `crate::unit::UnitWidth` order. `narrow` is gated on the
/// `units-narrow` cargo feature so size-sensitive builds pay for long+short only.
const UNIT_WIDTHS: [(&str, &str, Option<&str>); 3] = [
    ("long", "long", None),
    ("short", "short", None),
    ("narrow", "narrow", Some("units-narrow")),
];

/// Emit `cldr/generated/units.rs`: every locale's unit patterns as `const fn`
/// `match` lookups (no runtime blob). One function per (locale, width) keyed by
/// `unit * 8 + slot`, where slot 0..=5 is the plural category and slot 6 is the
/// unit's `perUnitPattern`; the locale's `per` `compoundUnitPattern` sits at the
/// pseudo-unit index `UNITS.len()`, slot 0.
fn emit_units(cldr_dir: &Path, units_dir: &Path) {
    let counts = [
        "unitPattern-count-zero",
        "unitPattern-count-one",
        "unitPattern-count-two",
        "unitPattern-count-few",
        "unitPattern-count-many",
        "unitPattern-count-other",
    ];
    let mut locales = locale_files(units_dir);
    locales.sort();

    // [locale][width] -> sorted (key, pattern) slots.
    let mut table: Vec<[Vec<(u16, String)>; 3]> = Vec::with_capacity(locales.len());
    for locale in &locales {
        let path = units_dir.join(alloc_format(locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let units = loc_obj.get("units").expect("units");
        let widths = std::array::from_fn::<_, 3, _>(|wi| {
            let w = units.get(UNIT_WIDTHS[wi].1).expect("width");
            let mut slots: Vec<(u16, String)> = Vec::new();
            for (ui, (_, key)) in UNITS.iter().enumerate() {
                let Some(f) = w.get(key) else { continue };
                let base = (ui as u16) * 8;
                for (ci, count) in counts.iter().enumerate() {
                    if let Some(pat) = f.get(count).and_then(Json::as_str) {
                        slots.push((base + ci as u16, pat.to_string()));
                    }
                }
                if let Some(pat) = f.get("perUnitPattern").and_then(Json::as_str) {
                    slots.push((base + 6, pat.to_string()));
                }
            }
            // The `per` compound pattern ("{0} per {1}", "{0}/{1}", "{0}毎{1}").
            if let Some(pat) = w
                .get("per")
                .and_then(|p| p.get("compoundUnitPattern"))
                .and_then(Json::as_str)
            {
                slots.push(((UNITS.len() as u16) * 8, pat.to_string()));
            }
            slots.sort();
            slots
        });
        table.push(widths);
    }

    let en = locales
        .iter()
        .position(|l| l == "en")
        .expect("en units data");

    let mut out = String::new();
    write_header(&mut out);
    let _ = write!(
        out,
        "//! CLDR measurement-unit patterns (UTS #35 `units.json`).\n\
         //!\n\
         //! A slot key is `unit * 8 + slot`: slots 0..=5 are the plural categories\n\
         //! (zero, one, two, few, many, other) and slot 6 is the unit's\n\
         //! `perUnitPattern`. The pseudo-unit `UNIT_COUNT` holds the locale's `per`\n\
         //! `compoundUnitPattern` in slot 0.\n\n\
         /// Unit slots per (locale, width): the 45 ECMA-402 sanctioned units plus the\n\
         /// two `speed-…` compounds CLDR ships pre-composed.\n\
         pub(crate) const UNIT_COUNT: u16 = {};\n\n\
         /// Table index of `en`, the last-resort locale fallback.\n\
         pub(crate) const EN: u16 = {en};\n\n",
        UNITS.len()
    );

    // Locale key -> index. Keys are the CLDR locale ids, lowercased; the runtime
    // walks the fallback chain by trimming `-` subtags.
    let _ = write!(
        out,
        "/// Table index for an exact (lowercased) CLDR locale id.\n\
         pub(crate) fn locale_index(lang: &str) -> Option<u16> {{\n    \
         Some(match lang {{\n"
    );
    for (i, locale) in locales.iter().enumerate() {
        let _ = write!(out, "        \"{}\" => {i},\n", locale.to_ascii_lowercase());
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    let _ = write!(
        out,
        "/// The pattern for `(locale, width, key)`, or `None` when CLDR has no such\n\
         /// string. `width` is 0 (long), 1 (short) or 2 (narrow).\n\
         #[inline]\n\
         pub(crate) const fn pattern(loc: u16, width: usize, key: u16) -> Option<&'static str> {{\n    \
         match width {{\n"
    );
    for (wi, (name, _, feature)) in UNIT_WIDTHS.iter().enumerate() {
        if let Some(f) = feature {
            let _ = write!(out, "        #[cfg(feature = \"{f}\")]\n");
        }
        let _ = write!(out, "        {wi} => {name}(loc, key),\n");
    }
    let _ = write!(out, "        _ => None,\n    }}\n}}\n");

    for (wi, (name, _, feature)) in UNIT_WIDTHS.iter().enumerate() {
        let cfg = feature.map_or(String::new(), |f| format!("#[cfg(feature = \"{f}\")]\n"));
        let _ = write!(
            out,
            "\n{cfg}const fn {name}(loc: u16, key: u16) -> Option<&'static str> {{\n    match loc {{\n"
        );
        for (li, locale) in locales.iter().enumerate() {
            if table[li][wi].is_empty() {
                continue;
            }
            let _ = write!(out, "        {li} => u_{}_{name}(key),\n", ident(locale));
        }
        let _ = write!(out, "        _ => None,\n    }}\n}}\n");

        for (li, locale) in locales.iter().enumerate() {
            if table[li][wi].is_empty() {
                continue;
            }
            let _ = write!(
                out,
                "\n{cfg}const fn u_{}_{name}(key: u16) -> Option<&'static str> {{\n    match key {{\n",
                ident(locale)
            );
            for (key, pat) in &table[li][wi] {
                let _ = write!(out, "        {key} => Some({}),\n", rust_str(pat));
            }
            let _ = write!(out, "        _ => None,\n    }}\n}}\n");
        }
    }

    write_cldr_generated(cldr_dir, "units", &out);
}

/// The modules under `src/cldr/generated/`, with the cargo feature each is
/// gated on, so a disabled formatter drops its table — and the megabytes of
/// string data in it — from the build entirely.
const CLDR_GENERATED: [(&str, &str); 3] = [
    ("numbers", "number"),
    ("tz_names", "datetime"),
    ("units", "units"),
];

/// Write one `src/cldr/generated/<name>.rs`.
fn write_cldr_generated(cldr_dir: &Path, name: &str, src: &str) {
    let gen_dir = cldr_dir.join("generated");
    fs::create_dir_all(&gen_dir).expect("create src/cldr/generated");
    let path = gen_dir.join(format!("{name}.rs"));
    fs::write(&path, src).unwrap_or_else(|_| panic!("write {}", path.display()));
    rustfmt(&path);
}

/// Write `src/cldr/generated/mod.rs` declaring every generated CLDR module.
fn emit_cldr_generated_mod(cldr_dir: &Path) {
    let mut out = String::new();
    write_header(&mut out);
    for (module, feature) in CLDR_GENERATED {
        let _ = write!(
            out,
            "#[cfg(feature = \"{feature}\")]\npub(crate) mod {module};\n"
        );
    }
    let gen_dir = cldr_dir.join("generated");
    fs::create_dir_all(&gen_dir).expect("create src/cldr/generated");
    let path = gen_dir.join("mod.rs");
    fs::write(&path, &out).expect("write cldr/generated/mod.rs");
    rustfmt(&path);
}

/// tzdb areas, in the order of the runtime's area-id space. Each gets its own
/// `tz-names-<area>` cargo feature: the zone→metazone map, the zone-level names
/// and the metazone names of an area that is compiled out are simply not
/// emitted, and its zones fall back to the localized GMT offset — which is UTS
/// #35's own last resort, so the answer stays correct, just less specific. A
/// metazone reachable from several areas (`GMT`, `Europe_Central`, …) is emitted
/// into each of their tables; that overlap is ~10% of the metazone strings.
const TZ_AREAS: [&str; 11] = [
    "Africa",
    "America",
    "Antarctica",
    "Arctic",
    "Asia",
    "Atlantic",
    "Australia",
    "Etc",
    "Europe",
    "Indian",
    "Pacific",
];

/// Name slots in key order: UTS #35's `<long>`/`<short>` × `<generic>` /
/// `<standard>` / `<daylight>`.
const TZ_NAME_SLOTS: [(&str, &str); 6] = [
    ("long", "generic"),
    ("long", "standard"),
    ("long", "daylight"),
    ("short", "generic"),
    ("short", "standard"),
    ("short", "daylight"),
];

/// Per-locale time-zone format patterns, in key order.
const TZ_FORMATS: [&str; 7] = [
    "gmtFormat",
    "gmtZeroFormat",
    "hourFormat",
    "regionFormat",
    "regionFormat-type-standard",
    "regionFormat-type-daylight",
    "fallbackFormat",
];

/// Start of the metazone key space. Zone keys are `zone * 8 + slot` and the
/// largest area (America) holds ~150 zones, so 4096 leaves ample headroom.
const TZ_MZ_BASE: u16 = 4096;

/// A zone's metazone usage over time: `(metazone id, from, to)` in Unix seconds,
/// `None` meaning unbounded on that side.
type MetazoneUse = Vec<(String, Option<i64>, Option<i64>)>;

/// Unix seconds for a CLDR metazone boundary (`"1971-10-31 02:00"`, always UTC).
fn tz_instant(s: &str) -> i64 {
    let num = |r: core::ops::Range<usize>| s[r].parse::<i64>().expect("metazone timestamp");
    let (y, m, d) = (num(0..4), num(5..7), num(8..10));
    // days_from_civil (Howard Hinnant): March-based year, era of 400 years.
    let y = y - i64::from(m <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let doy = (153 * (m + if m > 2 { -3 } else { 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    (era * 146_097 + doe - 719_468) * 86_400 + num(11..13) * 3600 + num(14..16) * 60
}

/// `path/part`, or `part` at the root.
fn tz_join(path: &str, part: &str) -> String {
    if path.is_empty() {
        String::from(part)
    } else {
        format!("{path}/{part}")
    }
}

/// Collect `zone id -> [(metazone, from, to)]` from the nested
/// `metaZones.metazoneInfo.timezone` tree, whose leaves are the usage lists.
fn tz_meta_walk(node: &Json, path: &str, out: &mut BTreeMap<String, MetazoneUse>) {
    match node {
        Json::Arr(items) => {
            let segs = items
                .iter()
                .map(|it| {
                    let u = it.get("usesMetazone").expect("usesMetazone");
                    let mz = u.get("_mzone").and_then(Json::as_str).expect("_mzone");
                    (
                        String::from(mz),
                        u.get("_from").and_then(Json::as_str).map(tz_instant),
                        u.get("_to").and_then(Json::as_str).map(tz_instant),
                    )
                })
                .collect();
            out.insert(String::from(path), segs);
        }
        Json::Obj(entries) => {
            for (k, v) in entries {
                tz_meta_walk(v, &tz_join(path, k), out);
            }
        }
        _ => {}
    }
}

/// Collect a locale's `timeZoneNames.zone` tree: the six name slots plus the
/// `exemplarCity` in slot 6. Only zones the locale actually overrides appear —
/// CLDR trims anything inherited from root, so the runtime derives the default
/// city from the zone id itself.
fn tz_zone_walk(node: &Json, path: &str, out: &mut BTreeMap<String, [Option<String>; 7]>) {
    let Json::Obj(entries) = node else { return };
    if node.get("_type").and_then(Json::as_str) == Some("zone") {
        let mut slots: [Option<String>; 7] = core::array::from_fn(|_| None);
        for (i, (width, ty)) in TZ_NAME_SLOTS.iter().enumerate() {
            slots[i] = node
                .get(width)
                .and_then(|w| w.get(ty))
                .and_then(Json::as_str)
                .map(String::from);
        }
        slots[6] = node
            .get("exemplarCity")
            .and_then(Json::as_str)
            .map(String::from);
        if slots.iter().any(Option::is_some) {
            out.insert(String::from(path), slots);
        }
        return;
    }
    for (k, v) in entries {
        if !k.starts_with('_') {
            tz_zone_walk(v, &tz_join(path, k), out);
        }
    }
}

/// The `match unix { … }` body selecting a zone's metazone over time. CLDR's
/// ranges are contiguous, so only the `_to` boundaries need testing; a leading
/// `_from` means the zone had no metazone before it.
fn tz_metazone_expr(segs: &[(u16, Option<i64>, Option<i64>)]) -> String {
    let mut expr = String::new();
    if let Some(from) = segs[0].1 {
        let _ = write!(expr, "if unix < {from} {{ None }} else ");
    }
    for (i, (mz, _, to)) in segs.iter().enumerate() {
        match to {
            Some(t) => {
                let _ = write!(expr, "if unix < {t} {{ Some({mz}) }} else ");
                if i + 1 == segs.len() {
                    expr.push_str("{ None }");
                }
            }
            None => {
                let _ = write!(expr, "{{ Some({mz}) }}");
                break;
            }
        }
    }
    expr
}

/// Emit `cldr/generated/tz_names.rs`: the localized time-zone name data of
/// UTS #35 §4.8 — per-locale GMT/region/fallback formats, the tzdb alias map,
/// the zone→metazone map with its historical ranges, exemplar cities and
/// zone-level name overrides, and the metazone names themselves.
fn emit_tz_names(
    cldr_dir: &Path,
    names_dir: &Path,
    metazones: &Path,
    bcp47_tz: &Path,
    primary_zones: &Path,
) {
    // ---- supplemental: zone -> metazone history ----
    let mzjson = json_parse(&fs::read_to_string(metazones).expect("read metaZones.json"));
    let info = mzjson
        .get("supplemental")
        .and_then(|s| s.get("metaZones"))
        .and_then(|m| m.get("metazoneInfo"))
        .and_then(|m| m.get("timezone"))
        .expect("metazoneInfo.timezone");
    let mut zone_meta = BTreeMap::new();
    tz_meta_walk(info, "", &mut zone_meta);

    // ---- bcp47: canonical ids, the alias set, and each zone's tzdb region ----
    let xml = fs::read_to_string(bcp47_tz).expect("read bcp47/timezone.xml");
    let mut aliases: BTreeMap<String, String> = BTreeMap::new();
    let mut zone_region: BTreeMap<String, String> = BTreeMap::new();
    let mut region_zones: BTreeMap<String, usize> = BTreeMap::new();
    let mut canonical: BTreeSet<String> = BTreeSet::new();
    for attrs in xml_self_tags(&xml, "type") {
        // Deprecated types carry no alias list; their ids reach the canonical
        // zone through the preferred type's own aliases.
        if xml_attr(attrs, "deprecated") == Some("true") {
            continue;
        }
        let (Some(name), Some(alias)) = (xml_attr(attrs, "name"), xml_attr(attrs, "alias")) else {
            continue;
        };
        let canon = xml_attr(attrs, "iana")
            .unwrap_or_else(|| alias.split_whitespace().next().expect("alias token"));
        canonical.insert(String::from(canon));
        for tok in alias.split_whitespace() {
            if tok != canon {
                aliases.insert(String::from(tok), String::from(canon));
            }
        }
        // `Etc/…` are offset pseudo-zones with no territory; leaving them out of
        // the region map keeps them off the generic-location path, as in ICU.
        if !canon.starts_with("Etc/") {
            let region = xml_attr(attrs, "region")
                .map_or_else(|| name[..2].to_ascii_uppercase(), String::from);
            *region_zones.entry(region.clone()).or_default() += 1;
            zone_region.insert(String::from(canon), region);
        }
    }

    // CLDR's supplemental and locale trees still key some zones by a tzdb link
    // (`Asia/Calcutta`, `America/Buenos_Aires`); fold those onto the canonical id
    // the runtime resolves to, preferring an entry already keyed canonically.
    let canon = |z: &str| aliases.get(z).cloned().unwrap_or_else(|| String::from(z));

    // A country with several zones still names the *country* for the one CLDR
    // designates primary (UTS #35 §4.8: "China Time", not "Shanghai Time"), so
    // those join the single-zone regions below. Keyed by canonical zone, since
    // CLDR lists some by tzdb link (`Europe/Kiev` → `Europe/Kyiv`).
    let pz_text = fs::read_to_string(primary_zones).expect("read primaryZones.json");
    let pz_json = json_parse(&pz_text);
    let mut primary_zone: BTreeMap<String, String> = BTreeMap::new();
    if let Some(map) = pz_json
        .get("supplemental")
        .and_then(|s| s.get("primaryZones"))
    {
        for (region, zone) in map.entries() {
            if let Some(z) = zone.as_str() {
                primary_zone.insert(canon(z), region.clone());
            }
        }
    }
    let mut folded: BTreeMap<String, MetazoneUse> = BTreeMap::new();
    for pass_aliases in [false, true] {
        for (z, segs) in &zone_meta {
            if aliases.contains_key(z) == pass_aliases {
                folded.entry(canon(z)).or_insert_with(|| segs.clone());
            }
        }
    }
    let zone_meta = folded;

    // ---- per-locale names ----
    struct LocTz {
        fmt: [String; 7],
        zones: BTreeMap<String, [Option<String>; 7]>,
        mzs: BTreeMap<String, [Option<String>; 6]>,
    }
    let mut locales = locale_files(names_dir);
    locales.sort();
    let mut locdata: Vec<LocTz> = Vec::with_capacity(locales.len());
    for locale in &locales {
        let path = names_dir.join(alloc_format(locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let tzn = loc_obj
            .get("dates")
            .and_then(|d| d.get("timeZoneNames"))
            .expect("timeZoneNames");
        let fmt = core::array::from_fn(|i| {
            String::from(
                tzn.get(TZ_FORMATS[i])
                    .and_then(Json::as_str)
                    .expect("time-zone format"),
            )
        });
        let mut raw = BTreeMap::new();
        if let Some(z) = tzn.get("zone") {
            tz_zone_walk(z, "", &mut raw);
        }
        let mut zones: BTreeMap<String, [Option<String>; 7]> = BTreeMap::new();
        for pass_aliases in [false, true] {
            for (z, slots) in &raw {
                if aliases.contains_key(z) != pass_aliases {
                    continue;
                }
                let e = zones
                    .entry(canon(z))
                    .or_insert_with(|| core::array::from_fn(|_| None));
                for (i, v) in slots.iter().enumerate() {
                    if e[i].is_none() {
                        e[i].clone_from(v);
                    }
                }
            }
        }
        let mut mzs = BTreeMap::new();
        for (mz, e) in tzn.get("metazone").map(Json::entries).unwrap_or(&[]) {
            let slots: [Option<String>; 6] = core::array::from_fn(|i| {
                let (width, ty) = TZ_NAME_SLOTS[i];
                e.get(width)
                    .and_then(|w| w.get(ty))
                    .and_then(Json::as_str)
                    .map(String::from)
            });
            if slots.iter().any(Option::is_some) {
                mzs.insert(mz.clone(), slots);
            }
        }
        locdata.push(LocTz { fmt, zones, mzs });
    }

    // ---- index spaces ----
    let mut all_zones: BTreeSet<String> = BTreeSet::new();
    all_zones.extend(zone_meta.keys().cloned());
    all_zones.extend(canonical.iter().cloned());
    for l in &locdata {
        all_zones.extend(l.zones.keys().cloned());
    }
    // Partition by area. The stored key is the id minus its area prefix, so the
    // prefix is not repeated once per zone in the generated `match`.
    let mut area_zones: Vec<Vec<&str>> = vec![Vec::new(); TZ_AREAS.len()];
    let mut zone_slot: BTreeMap<&str, (usize, u16)> = BTreeMap::new();
    for z in &all_zones {
        let Some((area, _)) = z.split_once('/') else {
            continue;
        };
        let Some(ai) = TZ_AREAS.iter().position(|a| *a == area) else {
            continue;
        };
        zone_slot.insert(z.as_str(), (ai, area_zones[ai].len() as u16));
        area_zones[ai].push(z.as_str());
    }

    let mut mz_ids: BTreeSet<&str> = BTreeSet::new();
    for (z, segs) in &zone_meta {
        if zone_slot.contains_key(z.as_str()) {
            mz_ids.extend(segs.iter().map(|(m, _, _)| m.as_str()));
        }
    }
    let mz_ids: Vec<&str> = mz_ids.into_iter().collect();
    let mz_index: BTreeMap<&str, u16> = mz_ids
        .iter()
        .enumerate()
        .map(|(i, m)| (*m, i as u16))
        .collect();
    // The metazones reachable from each area's zones; an area's table carries
    // the names of exactly those.
    let mut area_mz: Vec<BTreeSet<u16>> = vec![BTreeSet::new(); TZ_AREAS.len()];
    for (z, segs) in &zone_meta {
        let Some(&(ai, _)) = zone_slot.get(z.as_str()) else {
            continue;
        };
        for (m, _, _) in segs {
            area_mz[ai].insert(mz_index[m.as_str()]);
        }
    }

    let en = locales
        .iter()
        .position(|l| l == "en")
        .expect("en time-zone names");

    // ---- emit ----
    let mut out = String::new();
    write_header(&mut out);
    let _ = write!(
        out,
        "#![allow(unused_variables)]\n\n\
         //! CLDR localized time-zone names (UTS #35 §4.8).\n\
         //!\n\
         //! Two key spaces share one index per (area, locale):\n\
         //! * `zone * 8 + slot` — zone-level names, `slot` 0..=5 being (long,\n\
         //!   short) × (generic, standard, daylight), and slot 6 the exemplar city.\n\
         //! * `MZ_BASE + metazone * 6 + slot` — metazone names, same slot order.\n\
         //!\n\
         //! The names themselves live in one deduplicated `&str` arena per area,\n\
         //! reached through parallel `keys`/`ids` arrays — see `name_lookup` for\n\
         //! why this is data and not a `match`.\n\
         //!\n\
         //! Zones are grouped by tzdb area and gated on `tz-names-<area>`; an area\n\
         //! that is not compiled in resolves to `None` throughout, and the caller\n\
         //! falls back to the localized GMT offset.\n\n\
         /// Start of the metazone key space.\n\
         pub(crate) const MZ_BASE: u16 = {TZ_MZ_BASE};\n\n\
         /// Table index of `en`, the last-resort locale fallback.\n\
         pub(crate) const EN: u16 = {en};\n\n\
         /// Table index for an exact (lowercased) CLDR locale id.\n\
         pub(crate) fn locale_index(lang: &str) -> Option<u16> {{\n    \
         Some(match lang {{\n"
    );
    for (i, locale) in locales.iter().enumerate() {
        let _ = write!(out, "        \"{}\" => {i},\n", locale.to_ascii_lowercase());
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    // Per-locale formats: always compiled (they are what the plain localized GMT
    // offset needs), keyed `loc * 8 + slot` over TZ_FORMATS.
    let _ = write!(
        out,
        "/// A locale's time-zone format pattern. `slot` indexes gmtFormat,\n\
         /// gmtZeroFormat, hourFormat, regionFormat, regionFormat-type-standard,\n\
         /// regionFormat-type-daylight, fallbackFormat.\n\
         pub(crate) const fn format(loc: u16, slot: u16) -> &'static str {{\n    \
         match loc * 8 + slot {{\n"
    );
    for (li, l) in locdata.iter().enumerate() {
        for (si, pat) in l.fmt.iter().enumerate() {
            let _ = write!(out, "        {} => {},\n", li * 8 + si, rust_str(pat));
        }
    }
    let _ = write!(out, "        _ => \"\",\n    }}\n}}\n\n");

    // Zone id canonicalization (tzdb links and legacy ids -> the CLDR zone).
    let _ = write!(
        out,
        "/// The canonical tzdb id for a zone id or link, e.g. `\"US/Pacific\"` →\n\
         /// `\"America/Los_Angeles\"`. Unknown ids pass through unchanged.\n\
         pub(crate) fn canonical(zone: &str) -> &str {{\n    match zone {{\n"
    );
    for (alias, canon) in &aliases {
        let _ = write!(out, "        {} => {},\n", rust_str(alias), rust_str(canon));
    }
    let _ = write!(out, "        _ => zone,\n    }}\n}}\n\n");

    // ---- area dispatchers ----
    let sfx: Vec<String> = TZ_AREAS.iter().map(|a| a.to_ascii_lowercase()).collect();
    let up: Vec<String> = TZ_AREAS.iter().map(|a| a.to_ascii_uppercase()).collect();
    let cfg: Vec<String> = sfx
        .iter()
        .map(|s| format!("        #[cfg(feature = \"tz-names-{s}\")]\n"))
        .collect();
    // Gate the two shared readers on "at least one area", so a build with no
    // area selected compiles neither them nor any table.
    let any_area = format!(
        "#[cfg(any(\n{}))]\n",
        sfx.iter()
            .map(|s| format!("    feature = \"tz-names-{s}\",\n"))
            .collect::<String>()
    );

    // The two readers every area's tables share. Their doc comments carry the
    // measurements behind this representation, because the obvious "cleanup" is
    // to fold the tables back into `match` arms like the rest of the generated
    // code — which is what cost 4.3 MB of compiled output for 1.5 MB of strings.
    let _ = write!(
        out,
        "/// Binary search of an area's zone-id table: `zones` is the area's zone ids\n\
         /// minus their `Area/` prefix (`\"Abidjan\"`, `\"Accra\"`, …) concatenated in\n\
         /// ascending order and `starts` their `N + 1` byte boundaries, so a hit's\n\
         /// position *is* the area's zone index.\n\
         ///\n\
         /// Same reason as `name_lookup` below: as a `match` on `&str` the eleven\n\
         /// areas' zone tables inlined into 16 KB of compare chains; as data they\n\
         /// are ~5 KB of arena plus offsets and ~1.5 KB of code, with nothing per\n\
         /// zone.\n\
         {any_area}\
         fn zone_lookup(zones: &str, starts: &[u16], rest: &str) -> Option<u16> {{\n    \
         let mut lo = 0usize;\n    \
         let mut hi = starts.len().checked_sub(1)?;\n    \
         while lo < hi {{\n        \
         let mid = (lo + hi) / 2;\n        \
         let at = zones.get(*starts.get(mid)? as usize..*starts.get(mid + 1)? as usize)?;\n        \
         if at < rest {{\n            \
         lo = mid + 1;\n        \
         }} else {{\n            \
         hi = mid;\n        \
         }}\n    \
         }}\n    \
         let at = zones.get(*starts.get(lo)? as usize..*starts.get(lo + 1)? as usize)?;\n    \
         if at == rest {{ Some(lo as u16) }} else {{ None }}\n\
         }}\n\n"
    );
    let _ = write!(
        out,
        "/// Resolve `key` in one (area, locale) name table.\n\
         ///\n\
         /// `runs` bounds locale `loc`'s slice of the parallel `keys`/`ids` arrays;\n\
         /// `keys` ascends within a run, so the lookup is a single binary search.\n\
         /// `ids[i]` names a string by its index into `strings`, the `D + 1` byte\n\
         /// boundaries of the area's deduplicated `arena`. Boundaries always fall on\n\
         /// char boundaries — the arena is whole strings concatenated — so the slice\n\
         /// never splits a codepoint; `get` is used anyway so a corrupt table would\n\
         /// return `None` rather than panic.\n\
         ///\n\
         /// These tables are deliberately *data*, not the `match` shape the rest of\n\
         /// the generated code uses. They hold 67 060 name references over 1 111\n\
         /// (area, locale) pairs, and LLVM compiles a `match` that wide into a jump\n\
         /// table plus one code block per arm to materialize the `&'static str` fat\n\
         /// pointer. Measured on a release probe (`datetime,alloc` vs the same plus\n\
         /// `tz-names`), that shape added 1 098 KB of `.text` and 3 182 KB of\n\
         /// `.rodata` — 4 254 KB — to hold 1 523 KB of distinct strings, and left\n\
         /// 2 954 indirect jumps in the binary. As arrays the same tables add\n\
         /// 2 024 KB, of which 12 KB is code, with 247 indirect jumps and no\n\
         /// relocations. The `match` shape is the right one for the Unicode property\n\
         /// tables (dense integer -> small enum, which LLVM turns into a byte\n\
         /// lookup); it does not transfer to a wide string table. Please do not\n\
         /// \"simplify\" it back.\n\
         {any_area}\
         fn name_lookup(\n    \
         arena: &'static str,\n    \
         strings: &[u32],\n    \
         runs: &[u32],\n    \
         keys: &[u16],\n    \
         ids: &[u16],\n    \
         loc: u16,\n    \
         key: u16,\n\
         ) -> Option<&'static str> {{\n    \
         let lo = *runs.get(loc as usize)? as usize;\n    \
         let hi = *runs.get(loc as usize + 1)? as usize;\n    \
         let at = lo + keys.get(lo..hi)?.binary_search(&key).ok()?;\n    \
         let id = *ids.get(at)? as usize;\n    \
         arena.get(*strings.get(id)? as usize..*strings.get(id + 1)? as usize)\n\
         }}\n\n"
    );

    let _ = write!(
        out,
        "/// `(area, index)` for a canonical zone id, or `None` when the zone is\n\
         /// unknown or its area is not compiled in.\n\
         pub(crate) fn zone_index(zone: &str) -> Option<(u8, u16)> {{\n    \
         let (area, rest) = zone.split_once('/')?;\n    \
         Some(match area {{\n"
    );
    for (ai, area) in TZ_AREAS.iter().enumerate() {
        let _ = write!(
            out,
            "{}        \"{area}\" => ({ai}, zone_lookup(ZONES_{}, &ZONE_STARTS_{}, rest)?),\n",
            cfg[ai], up[ai], up[ai]
        );
    }
    let _ = write!(out, "        _ => return None,\n    }})\n}}\n\n");

    let _ = write!(
        out,
        "/// The metazone in effect for a zone at the UTC instant `unix`, or `None`\n\
         /// when CLDR maps it to none.\n\
         pub(crate) const fn metazone(area: u8, zone: u16, unix: i64) -> Option<u16> {{\n    \
         match area {{\n"
    );
    for ai in 0..TZ_AREAS.len() {
        let _ = write!(
            out,
            "{}        {ai} => mz_{}(zone, unix),\n",
            cfg[ai], sfx[ai]
        );
    }
    let _ = write!(out, "        _ => None,\n    }}\n}}\n\n");

    let _ = write!(
        out,
        "/// A zone's tzdb region when that region has exactly one canonical zone —\n\
         /// UTS #35's condition for naming the *country* rather than the exemplar\n\
         /// city in the generic location format.\n\
         pub(crate) const fn single_zone_region(area: u8, zone: u16) -> Option<&'static str> {{\n    \
         match area {{\n"
    );
    for ai in 0..TZ_AREAS.len() {
        let _ = write!(out, "{}        {ai} => rg_{}(zone),\n", cfg[ai], sfx[ai]);
    }
    let _ = write!(out, "        _ => None,\n    }}\n}}\n\n");

    let _ = write!(
        out,
        "/// A zone-level or metazone name string; see the key spaces above.\n\
         pub(crate) fn name(area: u8, loc: u16, key: u16) -> Option<&'static str> {{\n    \
         match area {{\n"
    );
    for ai in 0..TZ_AREAS.len() {
        let _ = write!(
            out,
            "{}        {ai} => name_lookup(\n            \
             ARENA_{a},\n            \
             &STRINGS_{a},\n            \
             &RUNS_{a},\n            \
             &KEYS_{a},\n            \
             &IDS_{a},\n            \
             loc,\n            \
             key,\n        \
             ),\n",
            cfg[ai],
            a = up[ai]
        );
    }
    let _ = write!(out, "        _ => None,\n    }}\n}}\n");

    // ---- per-area tables ----
    let mut area_bytes = Vec::new();
    for (ai, area) in TZ_AREAS.iter().enumerate() {
        let start = out.len();
        let acfg = format!("#[cfg(feature = \"tz-names-{}\")]\n", sfx[ai]);
        let s = &sfx[ai];
        let a = &up[ai];
        let _ = write!(out, "\n// ---- {area} ----\n");

        // Zone ids, `Area/` prefix stripped. `all_zones` is a sorted set and the
        // prefix is shared inside an area, so the ids are already in ascending
        // order — which is what `zone_lookup` binary-searches, and what makes a
        // hit's position the zone index.
        let zones: Vec<&str> = area_zones[ai]
            .iter()
            .map(|z| z.split_once('/').expect("area prefix").1)
            .collect();
        assert!(
            zones.windows(2).all(|w| w[0] < w[1]),
            "{area}: zone ids must be sorted and unique for zone_lookup"
        );
        let zone_starts = offsets(&zones);
        assert!(
            *zone_starts.last().expect("sentinel") <= u32::from(u16::MAX),
            "{area}: zone-id arena outgrew the u16 offsets"
        );
        emit_arena(&mut out, &acfg, &format!("ZONES_{a}"), &zones);
        emit_array(
            &mut out,
            &acfg,
            &format!("ZONE_STARTS_{a}"),
            "u16",
            &zone_starts,
        );

        let _ = write!(
            out,
            "\n{acfg}const fn mz_{s}(zone: u16, unix: i64) -> Option<u16> {{\n    match zone {{\n"
        );
        for (zi, zid) in area_zones[ai].iter().enumerate() {
            let Some(segs) = zone_meta.get(*zid) else {
                continue;
            };
            let segs: Vec<(u16, Option<i64>, Option<i64>)> = segs
                .iter()
                .map(|(m, f, t)| (mz_index[m.as_str()], *f, *t))
                .collect();
            let _ = write!(out, "        {zi} => {},\n", tz_metazone_expr(&segs));
        }
        let _ = write!(out, "        _ => None,\n    }}\n}}\n");

        let _ = write!(
            out,
            "\n{acfg}const fn rg_{s}(zone: u16) -> Option<&'static str> {{\n    match zone {{\n"
        );
        for (zi, zid) in area_zones[ai].iter().enumerate() {
            let Some(region) = zone_region.get(*zid) else {
                continue;
            };
            if region_zones[region] == 1 || primary_zone.get(*zid) == Some(region) {
                let _ = write!(out, "        {zi} => Some(\"{region}\"),\n");
            }
        }
        let _ = write!(out, "        _ => None,\n    }}\n}}\n");

        // (area, locale) name tables: for each locale, its (key, string) pairs in
        // ascending key order.
        let mut slots: Vec<Vec<(u16, &str)>> = Vec::with_capacity(locdata.len());
        for l in &locdata {
            let mut v: Vec<(u16, &str)> = Vec::new();
            for (zi, zid) in area_zones[ai].iter().enumerate() {
                if let Some(entry) = l.zones.get(*zid) {
                    for (si, val) in entry.iter().enumerate() {
                        if let Some(val) = val {
                            v.push((zi as u16 * 8 + si as u16, val.as_str()));
                        }
                    }
                }
            }
            for &mi in &area_mz[ai] {
                if let Some(entry) = l.mzs.get(mz_ids[mi as usize]) {
                    for (si, val) in entry.iter().enumerate() {
                        if let Some(val) = val {
                            v.push((TZ_MZ_BASE + mi * 6 + si as u16, val.as_str()));
                        }
                    }
                }
            }
            v.sort();
            slots.push(v);
        }

        // One arena per area holding each distinct string once — a locale reuses
        // "GMT", a zone's name often repeats its metazone's, and so on: 67 060
        // references resolve to 55 339 strings. Per area rather than one global
        // arena, because an area that is `#[cfg]`-ed out has to cost nothing, and
        // cross-area sharing would drag every area's strings into a one-area build
        // for the ~150 KB it would save on the full set. Ids are assigned in sorted
        // order, so the arena is stable and reviewable as a list.
        let mut str_id: BTreeMap<&str, u16> = BTreeMap::new();
        for v in &slots {
            for (_, val) in v {
                str_id.insert(val, 0);
            }
        }
        let strings: Vec<&str> = str_id.keys().copied().collect();
        assert!(
            strings.len() <= u16::MAX as usize + 1,
            "{area}: more distinct names than the u16 id space"
        );
        for (i, id) in str_id.values_mut().enumerate() {
            *id = i as u16;
        }

        // `runs[loc]..runs[loc + 1]` is locale `loc`'s slice of `keys`/`ids`; a
        // locale CLDR has no names for at all gets an empty run, which is how the
        // reader returns `None` for it.
        let mut runs: Vec<u32> = Vec::with_capacity(slots.len() + 1);
        let mut keys: Vec<u16> = Vec::new();
        let mut ids: Vec<u16> = Vec::new();
        for v in &slots {
            runs.push(keys.len() as u32);
            for (key, val) in v {
                keys.push(*key);
                ids.push(str_id[val]);
            }
        }
        runs.push(keys.len() as u32);

        emit_arena(&mut out, &acfg, &format!("ARENA_{a}"), &strings);
        emit_array(
            &mut out,
            &acfg,
            &format!("STRINGS_{a}"),
            "u32",
            &offsets(&strings),
        );
        emit_array(&mut out, &acfg, &format!("RUNS_{a}"), "u32", &runs);
        emit_array(&mut out, &acfg, &format!("KEYS_{a}"), "u16", &keys);
        emit_array(&mut out, &acfg, &format!("IDS_{a}"), "u16", &ids);
        area_bytes.push((*area, out.len() - start));
    }

    write_cldr_generated(cldr_dir, "tz_names", &out);
    println!(
        "codegen: wrote tz_names.rs ({} zones, {} metazones, {} locales; {} KB total, {})",
        all_zones.len(),
        mz_ids.len(),
        locales.len(),
        out.len() / 1024,
        area_bytes
            .iter()
            .map(|(a, n)| format!("{a} {} KB", n / 1024))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

/// A locale id as a Rust identifier fragment (`pt-PT` → `pt_pt`), lowercased so
/// generated function names are stable across platforms.
fn ident(locale: &str) -> String {
    locale
        .to_ascii_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// Render `s` as a Rust string literal. Invisible characters (NBSP, narrow NBSP,
/// bidi marks, soft hyphen, ZWJ/ZWNJ) are spelled out as `\u{…}` escapes so the
/// generated source stays reviewable; other non-ASCII is left verbatim.
fn rust_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c == ' ' || c.is_ascii_graphic() => out.push(c),
            c if c.is_ascii()
                || matches!(c,
                    '\u{a0}' | '\u{ad}' | '\u{61c}' | '\u{feff}'
                    | '\u{200b}'..='\u{200f}'
                    | '\u{2028}'..='\u{202f}'
                    | '\u{2066}'..='\u{2069}') =>
            {
                let _ = write!(out, "\\u{{{:x}}}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// The `N + 1` byte boundaries of `parts` concatenated: entry `i` occupies
/// `[out[i], out[i + 1])`. The trailing sentinel is what lets a reader recover a
/// length without storing one.
fn offsets(parts: &[&str]) -> Vec<u32> {
    let mut out = Vec::with_capacity(parts.len() + 1);
    let mut off = 0u32;
    for p in parts {
        out.push(off);
        off += p.len() as u32;
    }
    out.push(off);
    out
}

/// Emit `parts` concatenated into one `&str` static, as a `\`-continued literal
/// with one entry per source line so a multi-hundred-KB arena still reviews and
/// diffs like a list. The continuation escape eats the newline *and* the next
/// line's leading whitespace, so an entry that begins with a space has to escape
/// that space or it would be silently dropped.
fn emit_arena(out: &mut String, cfg: &str, name: &str, parts: &[&str]) {
    let _ = write!(out, "\n{cfg}static {name}: &str = \"\\\n");
    for p in parts {
        let lit = rust_str(p);
        // Strip `rust_str`'s quotes; escaping the first space is enough, since a
        // second one no longer sits at the start of the line.
        match lit[1..lit.len() - 1].strip_prefix(' ') {
            Some(rest) => {
                let _ = write!(out, "\\u{{20}}{rest}\\\n");
            }
            None => {
                let _ = write!(out, "{}\\\n", &lit[1..lit.len() - 1]);
            }
        }
    }
    out.push_str("\";\n");
}

/// Emit a plain array static, which costs exactly `len * size_of::<T>()` bytes of
/// `.rodata` and no relocations. rustfmt wraps the elements.
fn emit_array<T: std::fmt::Display>(out: &mut String, cfg: &str, name: &str, ty: &str, vals: &[T]) {
    let _ = write!(out, "\n{cfg}static {name}: [{ty}; {}] = [", vals.len());
    for (i, v) in vals.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{v}");
    }
    out.push_str("];\n");
}

/// Write `cldr/<name>.bin` for a non-Gregorian calendar: per-locale month names
/// (wide + abbr), the era names (all three widths × indices 0/1), and date
/// patterns (full/long/medium/short). Used for the Islamic and Persian calendars
/// (same record shape).
fn emit_alt_calendar(cldr_dir: &Path, name: &str, raw_dir: &Path) {
    let mut locales = locale_files(raw_dir);
    locales.sort();
    let mut records = Vec::new();
    for locale in locales {
        let path = raw_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let cal = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get(name))
            .expect("calendar");
        let months = cal
            .get("months")
            .and_then(|m| m.get("format"))
            .expect("months");
        let mut p = Vec::new();
        for width in ["wide", "abbreviated"] {
            let w = months.get(width).expect("width");
            for m in 1..=12u8 {
                enc_str(
                    &mut p,
                    w.get(&m.to_string()).and_then(Json::as_str).unwrap_or(""),
                );
            }
        }
        // Era names in all three widths, indices 0 (current era: AH / AP) and
        // 1 (pre-era: BH / BP). Persian defines only index 0, so index 1 is "".
        let eras = cal.get("eras");
        for width in ["eraNames", "eraAbbr", "eraNarrow"] {
            let w = eras.and_then(|e| e.get(width));
            for idx in ["0", "1"] {
                enc_str(
                    &mut p,
                    w.and_then(|x| x.get(idx))
                        .and_then(Json::as_str)
                        .unwrap_or(""),
                );
            }
        }
        let df = cal.get("dateFormats").expect("dateFormats");
        for k in ["full", "long", "medium", "short"] {
            enc_str(&mut p, df.get(k).and_then(Json::as_str).unwrap_or(""));
        }
        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, name, &records);
}

/// Write `cldr/chinese.bin`: per-locale Chinese-calendar data. Each record holds
/// the 60 sexagenary (cyclic) year names (the `U` field), the 12 numeric month
/// names (wide + abbreviated), the leap-month marker pattern (wide + abbreviated,
/// e.g. `"闰{0}"` / `"{0}bis"`) and the 4 date patterns (full/long/medium/short).
/// The cyclic-year names are identical across CLDR widths, so only one set is
/// stored. A `dateFormats` value may be a plain string or an object carrying a
/// `_value` (e.g. `zh`/`ja`/`yue`, which annotate a per-field numbering system);
/// the `_value` is used and the numbering annotation dropped (days render with
/// ASCII digits, as the other alternate calendars do).
fn emit_chinese(cldr_dir: &Path, raw_dir: &Path) {
    // Extract a pattern that is either a plain string or a `{_value, _numbers}`
    // object.
    fn pat_str(v: &Json) -> &str {
        v.as_str()
            .or_else(|| v.get("_value").and_then(Json::as_str))
            .unwrap_or("")
    }

    let mut locales = locale_files(raw_dir);
    locales.sort();
    let mut records = Vec::new();
    for locale in locales {
        let path = raw_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let cal = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get("chinese"))
            .expect("chinese calendar");

        let mut p = Vec::new();

        // 60 cyclic (sexagenary) year names, keyed "1"..="60" (wide width).
        let years = cal
            .get("cyclicNameSets")
            .and_then(|c| c.get("years"))
            .and_then(|y| y.get("format"))
            .and_then(|f| f.get("wide"))
            .expect("cyclic years");
        for i in 1..=60u8 {
            enc_str(
                &mut p,
                years
                    .get(&i.to_string())
                    .and_then(Json::as_str)
                    .unwrap_or(""),
            );
        }

        // 12 numeric month names (wide, then abbreviated).
        let months = cal
            .get("months")
            .and_then(|m| m.get("format"))
            .expect("months");
        for width in ["wide", "abbreviated"] {
            let w = months.get(width).expect("month width");
            for m in 1..=12u8 {
                enc_str(
                    &mut p,
                    w.get(&m.to_string()).and_then(Json::as_str).unwrap_or(""),
                );
            }
        }

        // Leap-month marker pattern (wide, then abbreviated).
        let leap = cal
            .get("monthPatterns")
            .and_then(|m| m.get("format"))
            .expect("monthPatterns");
        for width in ["wide", "abbreviated"] {
            enc_str(
                &mut p,
                leap.get(width)
                    .and_then(|w| w.get("leap"))
                    .and_then(Json::as_str)
                    .unwrap_or("{0}"),
            );
        }

        // 4 date patterns.
        let df = cal.get("dateFormats").expect("dateFormats");
        for k in ["full", "long", "medium", "short"] {
            enc_str(&mut p, df.get(k).map(pat_str).unwrap_or(""));
        }

        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "chinese", &records);
}

/// Write `cldr/japanese.bin`: per-locale Japanese-calendar data for the 5 modern
/// eras (CLDR era indices 232=Meiji, 233=Taishō, 234=Shōwa, 235=Heisei,
/// 236=Reiwa). Each record holds, in this order: the 5 eras in each of the three
/// widths (`eraNames` wide, `eraAbbr` abbreviated, `eraNarrow` narrow), then the
/// 4 date patterns (full/long/medium/short), then one "gannen" bitmask byte whose
/// bit `i` (full=0, long=1, medium=2, short=3) is set when that pattern's year
/// field uses the `jpanyear` numbering system — the CLDR signal that year 1 in an
/// era prints as 元 (gannen) rather than "1". A `dateFormats` value may be a plain
/// string or an object carrying a `_value` (e.g. `ja`, which annotates the year
/// field with `y=jpanyear`); the `_value` is stored and the annotation captured
/// only in the gannen mask. The Japanese calendar shares the Gregorian
/// months/weekdays, so no month or day names are stored here.
fn emit_japanese(cldr_dir: &Path, raw_dir: &Path) {
    // The 5 modern-era CLDR indices, in chronological order.
    const ERA_KEYS: [&str; 5] = ["232", "233", "234", "235", "236"];

    let mut locales = locale_files(raw_dir);
    locales.sort();
    let mut records = Vec::new();
    for locale in locales {
        let path = raw_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let cal = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get("japanese"))
            .expect("japanese calendar");

        let mut p = Vec::new();

        // 5 modern eras in each of the three widths (wide, abbr, narrow).
        let eras = cal.get("eras").expect("eras");
        for width in ["eraNames", "eraAbbr", "eraNarrow"] {
            let w = eras.get(width).expect("era width");
            for key in ERA_KEYS {
                enc_str(&mut p, w.get(key).and_then(Json::as_str).unwrap_or(""));
            }
        }

        // 4 date patterns; a pattern may be a plain string or a `{_value,
        // _numbers}` object. Capture, per style, whether the year field uses the
        // `jpanyear` numbering system (the gannen signal).
        let df = cal.get("dateFormats").expect("dateFormats");
        let mut gannen: u8 = 0;
        for (bit, k) in ["full", "long", "medium", "short"].iter().enumerate() {
            let v = df.get(k).expect("date pattern");
            let value = v
                .as_str()
                .or_else(|| v.get("_value").and_then(Json::as_str))
                .unwrap_or("");
            enc_str(&mut p, value);
            let numbers = v.get("_numbers").and_then(Json::as_str).unwrap_or("");
            if numbers.contains("jpanyear") {
                gannen |= 1 << bit;
            }
        }
        p.push(gannen);

        records.push((locale.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "japanese", &records);
}

/// Number of pre-Meiji (historical) Japanese era names carried by
/// `japanese_hist.bin`: CLDR era indices 0 (Taika) .. 231 (Keiō).
const HIST_ERA_COUNT: usize = 232;

/// Write `cldr/japanese_hist.bin`: the localized pre-Meiji nengō (era) names for
/// CLDR era indices 0..=231, in all three widths (`eraNames` wide, `eraAbbr`,
/// `eraNarrow`). Because these are mostly identical Latin romanizations across
/// locales (only ~20 distinct sets over ~100 locales), the per-locale name sets
/// are de-duplicated: each locale maps to a shared set id.
///
/// Layout (little-endian):
/// ```text
///   u16 nloc
///   nloc × [u8 klen][key bytes][u16 set_id]        (sorted by key)
///   u16 nsets
///   nsets × u32 offset            (into the sets region, cumulative)
///   sets region: per set, 3 × 232 strings [u8 len][bytes], in the order
///                wide[0..232], abbr[0..232], narrow[0..232]
/// ```
/// The modern eras (232..=236) live in `japanese.bin`; the Gregorian era-start
/// dates that select an index live in the runtime (`datetime.rs`).
fn emit_japanese_hist(cldr_dir: &Path, raw_dir: &Path) {
    let mut locales = locale_files(raw_dir);
    locales.sort();

    let mut set_index: BTreeMap<Vec<u8>, u16> = BTreeMap::new();
    let mut sets: Vec<Vec<u8>> = Vec::new();
    let mut loc_map: Vec<(String, u16)> = Vec::new();

    for locale in locales {
        let path = raw_dir.join(alloc_format(&locale));
        let text = fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
        let json = json_parse(&text);
        let (_, loc_obj) = json
            .get("main")
            .expect("main")
            .entries()
            .first()
            .expect("locale");
        let eras = loc_obj
            .get("dates")
            .and_then(|d| d.get("calendars"))
            .and_then(|c| c.get("japanese"))
            .and_then(|j| j.get("eras"))
            .expect("japanese eras");

        let mut payload = Vec::new();
        for width in ["eraNames", "eraAbbr", "eraNarrow"] {
            let w = eras.get(width);
            for i in 0..HIST_ERA_COUNT {
                let s = w
                    .and_then(|x| x.get(&i.to_string()))
                    .and_then(Json::as_str)
                    .unwrap_or("");
                enc_str(&mut payload, s);
            }
        }
        let id = *set_index.entry(payload.clone()).or_insert_with(|| {
            sets.push(payload.clone());
            (sets.len() - 1) as u16
        });
        loc_map.push((locale.to_ascii_lowercase(), id));
    }
    loc_map.sort();

    let mut blob = Vec::new();
    blob.extend_from_slice(&(loc_map.len() as u16).to_le_bytes());
    for (k, id) in &loc_map {
        blob.push(k.len() as u8);
        blob.extend_from_slice(k.as_bytes());
        blob.extend_from_slice(&id.to_le_bytes());
    }
    blob.extend_from_slice(&(sets.len() as u16).to_le_bytes());
    let mut off = 0u32;
    for s in &sets {
        blob.extend_from_slice(&off.to_le_bytes());
        off += s.len() as u32;
    }
    for s in &sets {
        blob.extend_from_slice(s);
    }

    let path = cldr_dir.join("japanese_hist.bin");
    fs::write(&path, &blob).expect("write japanese_hist.bin");
    println!(
        "codegen: wrote japanese_hist.bin ({} locales, {} sets, {} KB)",
        loc_map.len(),
        sets.len(),
        blob.len() / 1024
    );
}

/// Write `cldr/ordsuffix.bin`: per-locale ordinal suffix for each plural
/// category (zero/one/two/few/many/other), filling absent categories with the
/// `other` suffix.
/// Locales whose CLDR `standard` rule is **not** shipped in the generated table.
/// Each entry is a rule the runtime `Tailoring` parser cannot represent, so
/// bundling it would sort text *wrong* rather than merely coarsely; the
/// `tests/collation_data_consistency` gate re-derives this list from the data, so
/// a rule that stops being problematic will show up as a gate failure here.
/// Excluded locales fall back to a hand-written rule in `Tailoring::for_locale`,
/// or to root DUCET.
///
/// This list is the *only* hand-curation in the collation pipeline: everything
/// else is the official CLDR rule verbatim (modulo comment/whitespace
/// distillation). It deliberately does **not** filter on rule syntax — the
/// parser's `[before]`/`[import]`/star-range/escape support means the syntax
/// filter the first vendoring applied (and which stranded sv/fi/da/no/is/… on
/// hand rules long after the parser caught up) is obsolete.
const COLLATION_SKIP: &[(&str, &str)] = &[
    // -- Reset anchor the engine cannot hang a tailoring on -------------------
    // The tailored sort key gives each letter its anchor's DUCET *primary* plus a
    // sub-weight. An anchor with no primary of its own leaves nothing to offset
    // from, and `Tailoring::parse` rejects the rule rather than invent one.
    (
        "bo",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "dz",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "ee",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "fa",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "lt",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "ml",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "pa",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "ps",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "th",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "vi",
        "anchored on a bare combining mark, which has no DUCET primary weight",
    ),
    (
        "cu",
        "anchored on a `[first|last … ignorable]` pseudo-anchor, which the parser \
         does not resolve to a weight",
    ),
    (
        "km",
        "anchored on a `[first|last … ignorable]` pseudo-anchor, which the parser \
         does not resolve to a weight",
    ),
    (
        "ur",
        "anchored on a `[first|last … ignorable]` pseudo-anchor, which the parser \
         does not resolve to a weight",
    ),
    (
        "ar",
        "`&[before 2]` — a reset *before* an anchor at the secondary level; the \
         engine models reset-before only at the primary level",
    ),
    (
        "fr-ca",
        "the whole rule is `[backwards 2]` (French secondary-from-the-end \
         ordering), an option with no orderings and no runtime support",
    ),
    ("fa-af", "`[import ps]`, and `ps` is itself excluded below"),
    (
        "ja",
        "rule is `[import ja-u-co-private-kana]` + a 6 KB kanji chain; the private \
         kana collation is not a CLDR locale, so the import cannot resolve. \
         `for_locale` ships a kana-collapse rule verified against V8 instead",
    ),
    // -- Parses, but the resulting tailoring contradicts its own rule ----------
    // Each line is the `tests/collation_data_consistency` finding verbatim: the
    // two elements, the relation the rule asks for, and the order we produce.
    // Shipping these would sort text *wrong*, which is worse than sorting it
    // coarsely, so they fall back to a hand rule or to root DUCET.
    ("af", "gate: \"N\" (rel 3) \"ŉ\" -> Greater"),
    ("as", "gate: \"ৎ\" (rel 0) \"ত\\u{9cd}\\u{200d}\" -> Less"),
    ("bal", "gate: \"ا\\u{653}\" (rel 1) \"ا\" -> Greater"),
    ("br", "gate: \"c'h\" (rel 0) \"cʼh\" -> Greater"),
    ("da", "gate: \"Å\" (rel 3) \"aa\" -> Greater"),
    (
        "en-us-posix",
        "gate: \"A\" (rel 1) \"\\\\u0020\" -> Greater",
    ),
    ("ff-adlm", "gate: \"𞤀\\u{1e944}\" (rel 0) \"𞤀𞤀\" -> Greater"),
    ("fo", "gate: \"Å\" (rel 3) \"aa\" -> Greater"),
    ("hr", "gate: \"Dž\" (rel 3) \"ǅ\" -> Greater"),
    ("hu", "gate: \"DZ\" (rel 1) \"dzs\" -> Greater"),
    ("my", "gate: \"အော\" (rel 3) \"ဩ\" -> Greater"),
    ("ro", "gate: \"ş\" (rel 0) \"ș\" -> Greater"),
    ("to", "gate: \"NG\" (rel 3) \"ŋ\" -> Greater"),
    ("uz", "gate: \"oʻ\" (rel 0) \"o‘\" -> Less"),
    ("yi", "gate: \"פ\\u{5bf}\" (rel 3) \"ף\" -> Greater"),
];

/// Write `cldr/collation.bin` (and the committed `data/cldr/<ver>/collation.json`
/// mirror the consistency gate reads): per-locale CLDR collation tailoring **rule
/// strings**, taken verbatim from each `common/collation/<locale>.xml`'s
/// `<collation type="standard">` `<cr>`.
///
/// `Tailoring::for_locale` parses the looked-up rule at runtime, so locale
/// coverage is data-driven from the official CLDR rules. Only [`COLLATION_SKIP`]
/// is hand-curated; `root`/`zh` are handled elsewhere (root needs no tailoring,
/// zh is distilled into a Han-weight table by [`emit_collation_zh`]).
fn emit_collation_rules(cldr_dir: &Path, json_out: &Path, xml_dir: &Path) {
    let skip: BTreeMap<&str, &str> = COLLATION_SKIP.iter().copied().collect();
    let mut records: Vec<(String, Vec<u8>)> = Vec::new();
    let mut skipped: Vec<String> = Vec::new();
    let mut untailored = 0usize;

    let mut files: Vec<PathBuf> = fs::read_dir(xml_dir)
        .expect("read collation dir")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "xml"))
        .collect();
    files.sort();

    for path in &files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("collation file stem");
        // `root` carries the DUCET order itself (no tailoring to apply), and the
        // Chinese variants are distilled into Han-weight tables, not rule strings.
        if matches!(stem, "root" | "zh" | "zh_Hant") {
            continue;
        }
        let lang = stem.replace('_', "-").to_ascii_lowercase();
        let xml = fs::read_to_string(path).expect("read collation xml");
        let Some(rule) = standard_collation_rule(&xml) else {
            // No `standard` collation: the locale sorts in root order (e.g. de,
            // ga, nb, nl — de tailors only `phonebook`, which is `de-u-co-phonebk`,
            // not plain `de`).
            untailored += 1;
            continue;
        };
        if let Some(why) = skip.get(lang.as_str()) {
            skipped.push(format!("{lang} ({why})"));
            continue;
        }
        records.push((lang, rule.into_bytes()));
    }
    records.sort();

    // Mirror the table as JSON so the consistency gate (and a human diff) can read
    // it without an XML parser.
    let mut json = String::from("{\n");
    for (i, (lang, rule)) in records.iter().enumerate() {
        let comma = if i + 1 == records.len() { "" } else { "," };
        json.push_str(&format!(
            " {}: {}{comma}\n",
            json_quote(lang),
            json_quote(core::str::from_utf8(rule).expect("utf-8 rule"))
        ));
    }
    json.push_str("}\n");
    fs::write(json_out, json).expect("write collation.json");

    write_blob(cldr_dir, "collation", &records);
    let bytes: usize = records.iter().map(|(k, v)| k.len() + v.len() + 3).sum();
    let names: Vec<&str> = skipped
        .iter()
        .map(|s| s.split_once(' ').map_or(s.as_str(), |(n, _)| n))
        .collect();
    println!(
        "codegen: wrote collation.bin ({} locales from CLDR, {} KB; {} skipped \
         [{}]; {untailored} with no `standard` tailoring -> root order)",
        records.len(),
        bytes / 1024,
        skipped.len(),
        names.join(" "),
    );
}

/// The `<collation type="standard">` rule of an LDML collation file, distilled by
/// [`distill_collation_rule`], or `None` when the locale has no standard
/// tailoring.
///
/// Hand-rolled rather than XML-parsed because CLDR's collation files are not
/// uniformly formatted — attributes span lines (`sa.xml`), close tags carry
/// padding (`</collation  >` in `pl.xml`), and the rule body is CDATA full of
/// `<`. So: strip comments, index the `<collation …>` open tags, and read the
/// `<cr>` that follows the `standard` one. A `standard` block carrying `alt=` is
/// a *proposed* alternative, not the winning value, and is ignored; `draft=` is
/// not filtered (CLDR ships draft collations and ICU builds them).
fn standard_collation_rule(xml: &str) -> Option<String> {
    let text = strip_xml_comments(xml);
    // (tag start, body start, tag text) for every `<collation …>` — not
    // `<collations>`, whose 11th byte is `s` rather than space or `>`.
    let mut tags: Vec<(usize, usize, &str)> = Vec::new();
    let mut i = 0;
    while let Some(off) = text[i..].find("<collation") {
        let start = i + off;
        let rest = &text[start + "<collation".len()..];
        if rest.starts_with([' ', '\t', '\r', '\n', '>']) {
            let end = start + rest.find('>').expect("unterminated <collation") + "<collation".len();
            tags.push((start, end + 1, &text[start..=end]));
        }
        i = start + "<collation".len();
    }
    for (idx, &(_, body, tag)) in tags.iter().enumerate() {
        if collation_attr(tag, "type").as_deref() != Some("standard")
            || collation_attr(tag, "alt").is_some()
        {
            continue;
        }
        let stop = tags.get(idx + 1).map_or(text.len(), |n| n.0);
        let seg = &text[body..stop];
        let open = seg.find("<cr><![CDATA[")? + "<cr><![CDATA[".len();
        let close = seg[open..].find("]]>")? + open;
        return Some(distill_collation_rule(&seg[open..close]));
    }
    None
}

/// The value of attribute `name` in an XML open tag, or `None`. Unlike
/// [`xml_attr`] this accepts either quote style and requires a whole-name match,
/// both of which CLDR's collation files need.
fn collation_attr(tag: &str, name: &str) -> Option<String> {
    let mut i = 0;
    while let Some(off) = tag[i..].find(name) {
        let at = i + off;
        i = at + name.len();
        // Must be a whole attribute name: preceded by whitespace, followed by `=`.
        if !tag[..at].ends_with([' ', '\t', '\r', '\n']) {
            continue;
        }
        let rest = tag[i..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim_start();
        let quote = rest.chars().next()?;
        if quote != '"' && quote != '\'' {
            continue;
        }
        let val = &rest[1..];
        return Some(val[..val.find(quote)?].to_string());
    }
    None
}

/// Remove `<!-- … -->` comments.
fn strip_xml_comments(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len());
    let mut rest = xml;
    while let Some(open) = rest.find("<!--") {
        out.push_str(&rest[..open]);
        match rest[open..].find("-->") {
            Some(close) => rest = &rest[open + close + 3..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// Shrink a CLDR rule to what the runtime parser needs, without changing what it
/// means: drop `#` line comments and collapse whitespace runs to one space.
///
/// Both are done **quote-aware** (`'…'`, with `''` an escaped apostrophe) so a
/// quoted literal keeps its spaces and a quoted `#` (root's `'#⃣'`) is not read as
/// a comment. Comment removal is what makes the whitespace collapse safe — the
/// parser ends a `#` comment at the newline, so newlines cannot be dropped first.
/// `[optimize …]` and `[suppressContractions …]` are ICU performance hints that
/// carry no ordering (ko's `[optimize …]` alone is 4 KB), so they go too.
fn distill_collation_rule(rule: &str) -> String {
    let mut out = String::with_capacity(rule.len());
    let mut chars = rule.char_indices().peekable();
    let mut quoted = false;
    let mut pending_space = false;
    while let Some((i, c)) = chars.next() {
        match c {
            '\'' if chars.peek().map(|&(_, c)| c) == Some('\'') => {
                chars.next();
                out.push_str("''");
                pending_space = false;
            }
            '\'' => {
                quoted = !quoted;
                out.push('\'');
                pending_space = false;
            }
            '#' if !quoted => {
                for (_, c) in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            _ if !quoted && c.is_whitespace() => {
                pending_space = !out.is_empty();
            }
            '[' if !quoted
                && (rule[i..].starts_with("[optimize ")
                    || rule[i..].starts_with("[suppressContractions ")) =>
            {
                // Skip to the matching `]`, tracking the nested `[…]` of the
                // UnicodeSet argument.
                let mut depth = 1usize;
                for (_, c) in chars.by_ref() {
                    match c {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                if pending_space {
                    out.push(' ');
                    pending_space = false;
                }
                out.push(c);
            }
        }
    }
    out
}

/// Quote a string as a JSON scalar.
fn json_quote(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Whether `cp` is a Han ideograph we assign a pinyin rank to: the CJK Unified
/// Ideographs (URO, `4E00–9FFF`, incl. the high extension `9FA6–9FFF`) and the
/// CJK Compatibility Ideographs (`F900–FAFF`, which carry pinyin readings).
///
/// The Extension A/B/C/… blocks are deliberately EXCLUDED: ICU / V8's
/// `Intl.Collator('zh')` does not pinyin-tailor them — it sorts every Extension
/// ideograph *after* all pinyin chars, by radical-stroke. CLDR-48's rule instead
/// interleaves Extensions into the pinyin chain, but honoring those positions
/// mismatches V8 (verified: V8 places all Extensions after all URO). So we leave
/// every Extension to the runtime's unlisted-Han fallback, which band-places them
/// after all pinyin chars (before Latin) in code-point order — a proxy for ICU's
/// radical-stroke order, and the residual (rare-vs-rare) divergence class.
fn is_han_ideograph(cp: u32) -> bool {
    (0x4E00..=0x9FFF).contains(&cp) // CJK Unified Ideographs (URO)
        || (0xF900..=0xFAFF).contains(&cp) // CJK Compatibility Ideographs
}

/// Distill a zh `<collation>` rule (pinyin / stroke / zhuyin) into a dense
/// 1-based Han→rank map: lex the rule, bump a primary counter on every `<`
/// step, record each Han ideograph's counter at first appearance, then compact
/// to a gap-free rank. Only `is_han_ideograph` (URO + Compatibility) code
/// points are ranked; Extensions fall to the runtime radical-stroke fallback.
/// Extract a zh `<collation type='…'>` block's `<cr><![CDATA[ … ]]></cr>` rule.
fn zh_cdata<'a>(xml: &'a str, ty: &str) -> &'a str {
    let marker = format!("<collation type='{ty}'>");
    let start = xml
        .find(&marker)
        .unwrap_or_else(|| panic!("find {ty} collation"));
    let cdata_open = xml[start..].find("<![CDATA[").expect("find CDATA open") + start + 9;
    let cdata_close = xml[cdata_open..].find("]]>").expect("find CDATA close") + cdata_open;
    &xml[cdata_open..cdata_close]
}

fn zh_han_ranks(rule: &str) -> BTreeMap<u32, u16> {
    // ---- Lex (strip `#` comments; resolve quotes / `\u`,`\U` escapes). ----
    #[derive(Debug)]
    enum Tok {
        Amp,
        Opt,           // any `[ … ]` bracket (reorder / import / before / last regular)
        Rel(u8, bool), // (level 0..=3, star?)  level 0 = `=`
        Lit(char),
    }
    let mut toks: Vec<Tok> = Vec::new();
    let chars: Vec<char> = rule.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '#' => {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            _ if c.is_whitespace() => i += 1,
            '&' => {
                toks.push(Tok::Amp);
                i += 1;
            }
            '=' => {
                i += 1;
                let star = i < chars.len() && chars[i] == '*';
                if star {
                    i += 1;
                }
                toks.push(Tok::Rel(0, star));
            }
            '<' => {
                let mut lvl = 0u8;
                while i < chars.len() && chars[i] == '<' {
                    lvl += 1;
                    i += 1;
                }
                let star = i < chars.len() && chars[i] == '*';
                if star {
                    i += 1;
                }
                toks.push(Tok::Rel(lvl.min(3), star));
            }
            '[' => {
                let mut depth = 1;
                i += 1;
                while i < chars.len() && depth > 0 {
                    match chars[i] {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        _ => {}
                    }
                    i += 1;
                }
                toks.push(Tok::Opt);
            }
            '\'' => {
                i += 1;
                if i < chars.len() && chars[i] == '\'' {
                    toks.push(Tok::Lit('\''));
                    i += 1;
                } else {
                    while i < chars.len() && chars[i] != '\'' {
                        toks.push(Tok::Lit(chars[i]));
                        i += 1;
                    }
                    if i < chars.len() {
                        i += 1; // closing quote
                    }
                }
            }
            '\\' => {
                i += 1;
                match chars.get(i) {
                    Some('u') => {
                        let cp = u32::from_str_radix(
                            &chars[i + 1..i + 5].iter().collect::<String>(),
                            16,
                        )
                        .expect("\\u hex");
                        toks.push(Tok::Lit(char::from_u32(cp).expect("valid \\u")));
                        i += 5;
                    }
                    Some('U') => {
                        let cp = u32::from_str_radix(
                            &chars[i + 1..i + 9].iter().collect::<String>(),
                            16,
                        )
                        .expect("\\U hex");
                        toks.push(Tok::Lit(char::from_u32(cp).expect("valid \\U")));
                        i += 9;
                    }
                    Some(&other) => {
                        toks.push(Tok::Lit(other));
                        i += 1;
                    }
                    None => break,
                }
            }
            other => {
                toks.push(Tok::Lit(other));
                i += 1;
            }
        }
    }

    // ---- Interpret: assign each Han ideograph its pinyin primary rank. ----
    //
    // A running `prim` counter bumps on every primary step. Star runs (`<*abc`)
    // apply the relation to each character in turn; a non-star run is one target
    // (an expansion/contraction — we only assign it when it is a single char).
    // `<<` / `<<<` / `=` never introduce a Han primary distinction, but we still
    // advance no primary for them. First assignment wins.
    let mut rank: BTreeMap<u32, u32> = BTreeMap::new();
    let mut prim: u32 = 0;
    let mut i = 0;
    while i < toks.len() {
        match toks[i] {
            Tok::Rel(level, star) => {
                i += 1;
                let mut targets: Vec<char> = Vec::new();
                while let Some(Tok::Lit(ch)) = toks.get(i) {
                    targets.push(*ch);
                    i += 1;
                }
                if star {
                    for ch in targets {
                        if level == 1 {
                            prim += 1;
                        }
                        if is_han_ideograph(ch as u32) {
                            rank.entry(ch as u32).or_insert(prim);
                        }
                    }
                } else {
                    if level == 1 {
                        prim += 1;
                    }
                    if targets.len() == 1 {
                        let cp = targets[0] as u32;
                        if is_han_ideograph(cp) {
                            rank.entry(cp).or_insert(prim);
                        }
                    }
                }
            }
            // `&` reset and its anchor literals carry no new Han primary (the
            // anchor is either an option like `[last regular]` or an already-seen
            // character). Skip the `&` and any following anchor literals; the next
            // relation supplies its own primary bump from the current counter.
            Tok::Amp => {
                i += 1;
                while let Some(Tok::Opt) = toks.get(i) {
                    i += 1;
                }
                while let Some(Tok::Lit(_)) = toks.get(i) {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    // Compact the pinyin primaries to a dense 1-based rank (the running counter
    // has gaps where index markers and skipped chars consumed slots). Order is
    // preserved: sort the assigned ideographs by their counter value, then
    // renumber 1, 2, 3, …. Ranks stay well within `u16`.
    let mut ordered: Vec<(u32, u32)> = rank.iter().map(|(&cp, &p)| (cp, p)).collect();
    ordered.sort_by_key(|&(_, p)| p);
    assert!(
        ordered.len() <= u16::MAX as usize,
        "too many pinyin ranks for u16"
    );
    let mut final_rank: BTreeMap<u32, u16> = BTreeMap::new();
    for (idx, &(cp, _)) in ordered.iter().enumerate() {
        final_rank.insert(cp, idx as u16 + 1); // 1-based; 0 reserved
    }
    final_rank
}

/// Write `src/unicode/collation_zh.bin`: the distilled Chinese pinyin Han-weight
/// table, gated at runtime by the `collation-zh` feature.
///
/// The vendored CLDR `zh.xml` `<collation type='pinyin'>` rule establishes the
/// total pinyin order of ~44k Han ideographs. Its main chain is one continuous
/// `&[last regular] < … < …` block where every ordering step is a *primary*
/// (`<` / `<*` star runs and `<'﷐X'>` index markers) — so each listed Han
/// gets a distinct primary. We walk the rule token by token, keep a running
/// primary counter that bumps on every primary step, and record, for each Han
/// ideograph, the counter value at its first appearance (first assignment wins —
/// the later `&anchor<<<variant` compatibility lines and the `&x<han/ctx`
/// multi-reading context fixups never override a main-chain primary).
///
/// Secondary/tertiary are unused among Han (the main chain is all-primary), so
/// the table is simply `Han codepoint -> u16 pinyin rank`. Ranks are 1-based
/// (never 0) and fit in a `u16` (max ≈ 41k). The runtime slots these ranks into
/// a fixed primary base between the DUCET digit and Latin weights (matching
/// V8/ICU `[reorder Hani]`), disambiguating by rank via the tailoring sub-weight.
///
/// Blob layout (little-endian): `[u32 count]`, then `count` sorted `u32`
/// codepoints, then `count` `u16` ranks (parallel arrays; runtime binary-searches
/// the codepoint array). Unlisted ideographs are absent and fall back to DUCET.
fn emit_collation_zh(root: &Path, zh_xml: &Path) {
    let xml = fs::read_to_string(zh_xml).expect("read zh.xml");

    // Extract the `type='pinyin'` collation's `<cr><![CDATA[ … ]]></cr>`.
    let rule = zh_cdata(&xml, "pinyin");
    let final_rank = zh_han_ranks(rule);

    // Sorted parallel arrays: codepoints then ranks (BTreeMap iterates by cp).
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&(final_rank.len() as u32).to_le_bytes());
    for &cp in final_rank.keys() {
        buf.extend_from_slice(&cp.to_le_bytes());
    }
    for &rk in final_rank.values() {
        buf.extend_from_slice(&rk.to_le_bytes());
    }

    let path = root.join("src/unicode/collation_zh.bin");
    fs::write(&path, &buf).expect("write collation_zh.bin");
    eprintln!(
        "codegen: wrote collation_zh.bin ({} pinyin-ranked Han ideographs (URO + Compatibility), \
         {} KB)",
        final_rank.len(),
        buf.len() / 1024
    );
}

/// Write `src/unicode/collation_zh_<variant>.bin` for a non-default zh collation
/// (`stroke` / `zhuyin`), selectable at runtime via a `zh-u-co-<variant>` tag.
/// Same distillation and blob layout as [`emit_collation_zh`] (URO + Compatibility
/// Han → dense rank, little-endian `[u32 count]` + sorted `u32` cps + `u16`
/// ranks); Extensions again fall to the shared radical-stroke table. Gated at
/// runtime by `collation-zh`.
fn emit_collation_zh_variant(root: &Path, zh_xml: &Path, variant: &str) {
    let xml = fs::read_to_string(zh_xml).expect("read zh.xml");
    let final_rank = zh_han_ranks(zh_cdata(&xml, variant));

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&(final_rank.len() as u32).to_le_bytes());
    for &cp in final_rank.keys() {
        buf.extend_from_slice(&cp.to_le_bytes());
    }
    for &rk in final_rank.values() {
        buf.extend_from_slice(&rk.to_le_bytes());
    }
    let path = root.join(format!("src/unicode/collation_zh_{variant}.bin"));
    fs::write(&path, &buf).expect("write collation_zh_<variant>.bin");
    eprintln!(
        "codegen: wrote collation_zh_{variant}.bin ({} {variant}-ranked Han ideographs, {} KB)",
        final_rank.len(),
        buf.len() / 1024
    );
}

/// Write `src/unicode/collation_zh_rs.bin`: the Unihan **radical-stroke** order
/// for Han ideographs. Gated at runtime by `collation-zh`.
///
/// This table now covers **every** non-decomposing Han ideograph with a
/// `kRSUnicode` entry — the whole URO (`4E00–9FFF`) plus the CJK Extensions —
/// not just the reading-less ones. It serves two runtime paths:
///
///  * the default pinyin/stroke/zhuyin fallback — a Han *without* a pinyin (or
///    stroke/zhuyin) rank is sorted by radical-stroke, after all ranked chars
///    but before Latin. Pinyin-ranked URO chars never consult this table (the
///    ranked lookup wins first), so their inclusion here is inert for `zh`; it
///    only feeds the `unihan` variant below.
///  * the `zh-u-co-unihan` collation — which orders *all* Han purely by
///    radical-stroke (ignoring readings), so it needs URO coverage too.
///
/// Background: `Intl.Collator('zh')` (ICU/V8) sorts every Han *with* a Mandarin
/// reading by pinyin (the [`emit_collation_zh`] table, URO + Compatibility), and
/// every *other* Han by **radical-stroke** order — radical number, then residual
/// stroke count, then code point — placed after all pinyin chars but before Latin.
/// `Intl.Collator('zh',{collation:'unihan'})` instead sorts *all* Han by that
/// radical-stroke order.
///
/// Source: the vendored `data/ucd/<v>/Unihan_kRSUnicode.txt` (distilled
/// `kRSUnicode` field of Unihan_IRGSources.txt) — `radical[']␟.residual`, where a
/// trailing `'` marks a simplified-radical variant (sorts just after the base
/// radical). Only the first value is used when a char lists two.
///
/// Blob layout (little-endian): `[u32 count]`, then `count` sorted `u32` code
/// points, then `count` `u16` packed keys. Each key packs
/// `radical << 8 | (residual + 16) << 1 | is_simplified` — the runtime unpacks it
/// into two ordering primaries (radical, then the residual+simplified low byte),
/// then appends the DUCET implicit primaries as a within-block tie-breaker.
/// Decomposing Compatibility ideographs are excluded (they normalize to their URO
/// form before collation).
fn emit_collation_zh_rs(root: &Path, krs_path: &Path) {
    let text = fs::read_to_string(krs_path).expect("read Unihan_kRSUnicode.txt");

    // Compatibility ideographs that canonically decompose never reach the runtime
    // fallback (NFD folds them to their URO form first), so skip the two compat
    // blocks wholesale — including a handful of non-decomposing outliers, which
    // are pinyin-ranked anyway when they carry a reading.
    let decomposes = |cp: u32| (0xF900..=0xFAFF).contains(&cp) || (0x2F800..=0x2FA1D).contains(&cp);

    let mut table: BTreeMap<u32, u16> = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((cp_s, val)) = line.split_once('\t') else {
            continue;
        };
        let Some(hex) = cp_s.strip_prefix("U+") else {
            continue;
        };
        let Ok(cp) = u32::from_str_radix(hex, 16) else {
            continue;
        };
        if decomposes(cp) {
            continue;
        }
        // `radical[']␟.residual`, first value only.
        let val = val.split_whitespace().next().unwrap_or(val);
        let simplified = val.contains('\'');
        let clean = val.replace('\'', "");
        let Some((rad_s, res_s)) = clean.split_once('.') else {
            continue;
        };
        let (Ok(radical), Ok(residual)) = (rad_s.parse::<u32>(), res_s.parse::<i32>()) else {
            continue;
        };
        // Key significance order (radical, residual, simplified) — matching V8,
        // which sorts a simplified-radical variant *after* the base radical only
        // when residual strokes tie, not ahead of the whole base radical. Layout:
        //   radical 1..=214            → high 8 bits (never zero);
        //   residual −9..=76 → +16     → 1..=127, shifted left 1 (7 bits);
        //   is_simplified              → low bit (variant just after its base).
        let resid = (residual + 16) as u32;
        assert!(
            radical < 256 && resid < 128,
            "RS key out of range: {radical}.{residual}"
        );
        let packed = ((radical << 8) | (resid << 1) | u32::from(simplified)) as u16;
        table.insert(cp, packed);
    }

    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&(table.len() as u32).to_le_bytes());
    for &cp in table.keys() {
        buf.extend_from_slice(&cp.to_le_bytes());
    }
    for &pk in table.values() {
        buf.extend_from_slice(&pk.to_le_bytes());
    }
    let path = root.join("src/unicode/collation_zh_rs.bin");
    fs::write(&path, &buf).expect("write collation_zh_rs.bin");
    eprintln!(
        "codegen: wrote collation_zh_rs.bin ({} radical-stroke Han ideographs (URO + Extensions), \
         {} KB)",
        table.len(),
        buf.len() / 1024
    );
}

fn emit_ordsuffix(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read ordsuffix.json");
    let json = json_parse(&text);
    let cats = ["zero", "one", "two", "few", "many", "other"];
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let other = loc.get("other").and_then(Json::as_str).unwrap_or("");
        let mut p = Vec::new();
        for cat in cats {
            let s = loc.get(cat).and_then(Json::as_str).unwrap_or(other);
            enc_str(&mut p, s);
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "ordsuffix", &records);
}

/// Write `cldr/numsys_digits.bin`: numbering system → its 10 digit glyphs. The
/// per-locale default/native system moved into `generated/numbers.rs`, which is
/// where the matching per-system symbol blocks live.
fn emit_numsys(cldr_dir: &Path, numbering_systems: &Path) {
    // Digit glyphs from the supplemental numbering-systems table (numeric only).
    let ns_text = fs::read_to_string(numbering_systems).expect("read numberingSystems.json");
    let ns = json_parse(&ns_text);
    let table = ns
        .get("supplemental")
        .and_then(|s| s.get("numberingSystems"))
        .expect("numberingSystems");
    let mut digits = Vec::new();
    for (sys, info) in table.entries() {
        if info.get("_type").and_then(Json::as_str) != Some("numeric") {
            continue;
        }
        if let Some(glyphs) = info.get("_digits").and_then(Json::as_str) {
            let mut p = Vec::new();
            enc_str(&mut p, glyphs);
            digits.push((sys.clone(), p));
        }
    }
    write_blob(cldr_dir, "numsys_digits", &digits);
}

/// Write `cldr/rbnf.bin`: per-locale RBNF spell-out rule sets. Payload is
/// `[start-name str][u8 ruleset_count]` then each ruleset
/// `[name str][u16 rule_count]` of `[key str][text str]` pairs.
fn emit_rbnf(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read rbnf.json");
    let json = json_parse(&text);
    let mut records = Vec::new();
    for (lang, loc) in json.get("locales").expect("locales").entries() {
        let start = loc.get("_start").and_then(Json::as_str).unwrap_or("");
        let rulesets: Vec<&(String, Json)> = loc
            .entries()
            .iter()
            .filter(|(k, _)| k != "_start")
            .collect();
        let mut p = Vec::new();
        enc_str(&mut p, start);
        p.push(rulesets.len() as u8);
        for (name, rules) in rulesets {
            enc_str(&mut p, name);
            p.extend_from_slice(&(rules.entries().len() as u16).to_le_bytes());
            for (key, txt) in rules.entries() {
                enc_str(&mut p, key);
                enc_str(&mut p, txt.as_str().unwrap_or(""));
            }
        }
        records.push((lang.to_ascii_lowercase(), p));
    }
    write_blob(cldr_dir, "rbnf", &records);
}

/// Write `cldr/likely.bin`: the likelySubtags table (locale key -> maximized
/// locale). Keys are kept verbatim (canonical case).
fn emit_likely(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read likely.json");
    let json = json_parse(&text);
    let map = json.get("map").expect("map");
    let mut records = Vec::new();
    for (key, val) in map.entries() {
        let mut p = Vec::new();
        enc_str(&mut p, val.as_str().unwrap_or(""));
        records.push((key.clone(), p));
    }
    write_blob(cldr_dir, "likely", &records);
}

/// Write `cldr/aliases.bin`: the CLDR deprecated-subtag alias tables used by
/// locale canonicalization. All kinds share one blob via a 1-char type prefix on
/// the key: `'l'` = language (and grandfathered/redundant whole tags), `'s'` =
/// script, `'t'` = territory, `'v'` = variant. Language keys are lowercased with
/// `-`→`_` (so grandfathered tags like `i-klingon` become `i_klingon`).
/// Replacement values keep CLDR casing (language lowercase, script Titlecase,
/// region UPPER); a multi-subtag replacement (e.g. `sh`→`sr-Latn`) is stored as
/// space-separated subtags, and a one→many territory replacement (e.g.
/// `SU`→`RU AM AZ …`) keeps CLDR's candidate order. Keys are sorted for
/// determinism. The `_reason`/`_replacement` metadata attributes are ignored
/// except for `_replacement`.
fn emit_aliases(cldr_dir: &Path, path: &Path) {
    let text = fs::read_to_string(path).expect("read aliases.json");
    let json = json_parse(&text);
    let alias = json
        .get("supplemental")
        .and_then(|s| s.get("metadata"))
        .and_then(|m| m.get("alias"))
        .expect("supplemental.metadata.alias");

    let mut records: Vec<(String, Vec<u8>)> = Vec::new();
    // languageAlias: keys lowercased with `-`→`_` (covers grandfathered tags).
    if let Some(la) = alias.get("languageAlias") {
        for (key, entry) in la.entries() {
            let k = key.to_ascii_lowercase().replace('-', "_");
            push_alias(&mut records, 'l', &k, entry);
        }
    }
    // scriptAlias: keys kept as-is (Titlecase).
    if let Some(sa) = alias.get("scriptAlias") {
        for (key, entry) in sa.entries() {
            push_alias(&mut records, 's', key, entry);
        }
    }
    // territoryAlias: keys kept as-is (UPPER / numeric).
    if let Some(ta) = alias.get("territoryAlias") {
        for (key, entry) in ta.entries() {
            push_alias(&mut records, 't', key, entry);
        }
    }
    // variantAlias: keys lowercased.
    if let Some(va) = alias.get("variantAlias") {
        for (key, entry) in va.entries() {
            push_alias(&mut records, 'v', &key.to_ascii_lowercase(), entry);
        }
    }
    records.sort();
    write_blob(cldr_dir, "aliases", &records);
}

/// Push one alias record: `prefix + key` → the entry's `_replacement`, with the
/// replacement's `-` subtag separators rewritten to spaces (so a multi-subtag
/// replacement is stored as space-separated subtags; one→many territory lists
/// already use spaces).
fn push_alias(records: &mut Vec<(String, Vec<u8>)>, prefix: char, key: &str, entry: &Json) {
    let repl = entry
        .get("_replacement")
        .and_then(Json::as_str)
        .unwrap_or("")
        .replace('-', " ");
    let mut p = Vec::new();
    enc_str(&mut p, &repl);
    records.push((format!("{prefix}{key}"), p));
}

/// Write `cldr/bcp47.bin`: the CLDR `-u-`/`-t-` extension type-value aliases used
/// by locale-extension canonicalization (UTS #35 §3.6.5 / ECMA-402
/// CanonicalizeUnicodeLocaleId). Reads every `data/cldr/48/bcp47/*.xml` keyword
/// file and, for each `<key>`/`<type>`, records the deprecated-value → canonical
/// mapping under the key `"<keyName>/<sourceValue>"` (both lowercased). The
/// canonical value keeps CLDR's `-` subtag separators (e.g. `islamic-civil`).
///
/// Two shapes are recorded:
/// - `deprecated="true" preferred="P"` on a `<type name=N>` → `N` → `P`.
/// - otherwise, each space-separated token of `alias="A1 A2 …"` → `Ai` → `N`.
///
/// Only BCP-47-legal source values are kept (every `-` subtag is 2–8 alphanum),
/// which drops overlong long-form aliases (`gregorian`, `phonebook`, …) and the
/// slash-bearing IANA timezone aliases that can never appear in a real tag. The
/// boolean `yes`/`no`/`true`/`false` type aliases are intentionally skipped: at
/// canonicalization time a `true`/`yes` keyword value is dropped and `false`/`no`
/// kept verbatim (matching V8 / ECMA-402), so those aliases are never consulted.
fn emit_bcp47(cldr_dir: &Path, bcp47_dir: &Path) {
    let is_bool = |s: &str| matches!(s, "true" | "false" | "yes" | "no");
    // A source value is BCP-47-legal when each `-` subtag is 2..=8 alphanumerics.
    let valid_src = |s: &str| {
        !s.is_empty()
            && s.split('-').all(|sub| {
                (2..=8).contains(&sub.len()) && sub.bytes().all(|b| b.is_ascii_alphanumeric())
            })
    };

    let mut files: Vec<PathBuf> = fs::read_dir(bcp47_dir)
        .expect("read bcp47 dir")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "xml"))
        .collect();
    files.sort();

    // De-duplicated map so determinism does not depend on file order.
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    for path in &files {
        let xml = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
        for (kattrs, body) in xml_blocks(&xml, "key") {
            let Some(kname) = xml_attr(kattrs, "name") else {
                continue;
            };
            let kname = kname.to_ascii_lowercase();
            for tattrs in xml_self_tags(body, "type") {
                let Some(name) = xml_attr(tattrs, "name") else {
                    continue;
                };
                let name = name.to_ascii_lowercase();
                let deprecated = xml_attr(tattrs, "deprecated") == Some("true");
                let preferred = xml_attr(tattrs, "preferred").map(str::to_ascii_lowercase);
                let alias = xml_attr(tattrs, "alias").map(str::to_ascii_lowercase);

                let mut add = |src: &str, tgt: &str| {
                    if src == tgt || is_bool(src) || is_bool(tgt) || !valid_src(src) {
                        return;
                    }
                    map.insert(format!("{kname}/{src}"), tgt.to_string());
                };
                match (deprecated, &preferred) {
                    (true, Some(p)) => add(&name, p),
                    _ => {
                        if let Some(a) = &alias {
                            for tok in a.split_whitespace() {
                                add(tok, &name);
                            }
                        }
                    }
                }
            }
        }
    }

    let mut records: Vec<(String, Vec<u8>)> = Vec::new();
    for (key, val) in &map {
        let mut p = Vec::new();
        enc_str(&mut p, val);
        records.push((key.clone(), p));
    }
    records.sort();
    write_blob(cldr_dir, "bcp47", &records);
    println!(
        "codegen: wrote bcp47.bin ({} type-value aliases)",
        records.len()
    );
}

/// True when byte `after` follows a tag name as a real delimiter (so `<key`
/// matches `<key …>` but not `<keyword>`).
fn tag_delim(after: Option<u8>) -> bool {
    matches!(after, Some(b) if b == b'>' || b == b'/' || b.is_ascii_whitespace())
}

/// Iterate `<tag …attrs…> …body… </tag>` blocks, yielding `(attrs, body)`.
fn xml_blocks<'a>(xml: &'a str, tag: &str) -> Vec<(&'a str, &'a str)> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let bytes = xml.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(rel) = xml[i..].find(&open) {
        let start = i + rel + open.len();
        if !tag_delim(bytes.get(start).copied()) {
            i = start; // e.g. `<keyword>` when scanning for `<key`
            continue;
        }
        // Attributes run to the closing `>` of the open tag.
        let Some(gt) = xml[start..].find('>') else {
            break;
        };
        let attrs = &xml[start..start + gt];
        let body_start = start + gt + 1;
        let Some(crel) = xml[body_start..].find(&close) else {
            break;
        };
        out.push((attrs, &xml[body_start..body_start + crel]));
        i = body_start + crel + close.len();
    }
    out
}

/// Iterate the attribute strings of self-contained `<tag …/>` (or `<tag …>`)
/// elements found in `xml`.
fn xml_self_tags<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let open = format!("<{tag}");
    let bytes = xml.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(rel) = xml[i..].find(&open) {
        let start = i + rel + open.len();
        if !tag_delim(bytes.get(start).copied()) {
            i = start;
            continue;
        }
        let Some(gt) = xml[start..].find('>') else {
            break;
        };
        let attrs = xml[start..start + gt].trim_end_matches('/');
        out.push(attrs);
        i = start + gt + 1;
    }
    out
}

/// Extract the value of `name="…"` from an XML attribute string.
fn xml_attr<'a>(attrs: &'a str, name: &str) -> Option<&'a str> {
    let pat = format!("{name}=\"");
    let idx = attrs.find(&pat)? + pat.len();
    let end = attrs[idx..].find('"')? + idx;
    Some(&attrs[idx..end])
}

/// Write `cldr/display_languages.bin` and `cldr/display_territories.bin`: for
/// each display locale, a nested `[u16 count]` table of `code -> name`.
fn emit_display(cldr_dir: &Path, localenames_dir: &Path) {
    let mut locales: Vec<String> = fs::read_dir(localenames_dir)
        .expect("read localenames-raw dir")
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            e.file_name()
                .to_string_lossy()
                .strip_suffix("-languages.json")
                .map(String::from)
        })
        .collect();
    locales.sort();

    for (section, suffix, blob) in [
        ("languages", "-languages.json", "display_languages"),
        ("territories", "-territories.json", "display_territories"),
    ] {
        let mut records = Vec::new();
        for locale in &locales {
            let path = localenames_dir.join(alloc_concat(locale, suffix));
            let text =
                fs::read_to_string(&path).unwrap_or_else(|_| panic!("read {}", path.display()));
            let json = json_parse(&text);
            let (_, loc_obj) = json
                .get("main")
                .expect("main")
                .entries()
                .first()
                .expect("locale");
            let table = loc_obj
                .get("localeDisplayNames")
                .and_then(|d| d.get(section))
                .expect("section");
            // Skip `-alt-` variant keys; the runtime looks up the bare code.
            let kept: Vec<(&str, &str)> = table
                .entries()
                .iter()
                .filter(|(code, _)| !code.contains("-alt-"))
                .filter_map(|(code, name)| name.as_str().map(|n| (code.as_str(), n)))
                .collect();
            let mut payload = Vec::new();
            payload.extend_from_slice(&(kept.len() as u16).to_le_bytes());
            for (code, name) in kept {
                enc_str(&mut payload, code);
                enc_str(&mut payload, name);
            }
            records.push((locale.to_ascii_lowercase(), payload));
        }
        write_blob(cldr_dir, blob, &records);
    }
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
        // Match the `intl` crate edition so regenerated files stay fmt-clean
        // under the crate's style (e.g. 2024 import ordering).
        .arg("2024")
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
         #![allow(dead_code)]\n\
         // A dispatch whose every arm is `#[cfg]`-ed out diverges, making the\n\
         // fallback after it unreachable. That is the point of the gating.\n\
         #![allow(unreachable_code)]\n\n",
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
