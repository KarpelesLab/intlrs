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
//! Run with `cargo run -p codegen`. Output is deterministic.
#![allow(clippy::write_with_newline)]

use std::collections::BTreeMap;
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
         #![allow(unreachable_patterns)]\n\n",
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
