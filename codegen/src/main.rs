//! Code generator for the `unicode` crate.
//!
//! Reads the vendored UCD text files under `data/ucd/<version>/` and emits
//! committed Rust source into `src/generated/`. The generated code is a
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
    let out_dir = root.join("src/generated");
    fs::create_dir_all(&out_dir).expect("create src/generated");

    let (vmaj, vmin, vpatch) = parse_version(&ucd.join("ReadMe.txt"));
    eprintln!(
        "codegen: Unicode {vmaj}.{vmin}.{vpatch} from {}",
        ucd.display()
    );

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
        "use crate::category::GeneralCategory;\n\n\
         /// The Unicode version this table was generated from.\n\
         pub const UNICODE_VERSION: (u8, u8, u8) = ({vmaj}, {vmin}, {vpatch});\n\n"
    );
    emit_lookup(
        &mut gc_out,
        "general_category",
        "gc",
        "GeneralCategory",
        &gc,
        GC_UNASSIGNED,
        &render_gc,
    );
    fs::write(out_dir.join("general_category.rs"), gc_out).expect("write general_category.rs");

    // ---- Binary properties ----
    let mut bp_out = String::new();
    write_header(&mut bp_out);

    let white_space = parse_binary_prop(&ucd.join("PropList.txt"), "White_Space");
    emit_bool_lookup(&mut bp_out, "white_space", "ws", &white_space);

    let alphabetic = parse_binary_prop(&ucd.join("DerivedCoreProperties.txt"), "Alphabetic");
    emit_bool_lookup(&mut bp_out, "alphabetic", "al", &alphabetic);

    let uppercase = parse_binary_prop(&ucd.join("DerivedCoreProperties.txt"), "Uppercase");
    emit_bool_lookup(&mut bp_out, "uppercase", "up", &uppercase);

    let lowercase = parse_binary_prop(&ucd.join("DerivedCoreProperties.txt"), "Lowercase");
    emit_bool_lookup(&mut bp_out, "lowercase", "lo", &lowercase);

    fs::write(out_dir.join("binary_props.rs"), bp_out).expect("write binary_props.rs");

    // ---- generated/mod.rs ----
    let mut mod_out = String::new();
    write_header(&mut mod_out);
    mod_out.push_str("pub(crate) mod binary_props;\npub(crate) mod general_category;\n");
    fs::write(out_dir.join("mod.rs"), mod_out).expect("write generated/mod.rs");

    // Format the generated files with stable rustfmt so the committed output is
    // fmt-clean and regeneration stays byte-for-byte deterministic.
    for f in ["general_category.rs", "binary_props.rs", "mod.rs"] {
        rustfmt(&out_dir.join(f));
    }

    eprintln!("codegen: wrote general_category.rs, binary_props.rs, mod.rs");
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
fn parse_unicode_data(path: &Path) -> Vec<u8> {
    let abbr_to_code: BTreeMap<&str, u8> = GC_ABBRS
        .iter()
        .enumerate()
        .map(|(i, &a)| (a, i as u8))
        .collect();

    let text = fs::read_to_string(path).expect("read UnicodeData.txt");
    let mut codes = vec![GC_UNASSIGNED; NUM_CODEPOINTS];

    let mut range_start: Option<u32> = None;
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        let mut fields = line.split(';');
        let cp = u32::from_str_radix(fields.next().unwrap(), 16).expect("hex codepoint");
        let name = fields.next().unwrap_or("");
        let cat_abbr = fields.next().unwrap_or("Cn");
        let cat = *abbr_to_code.get(cat_abbr).unwrap_or(&GC_UNASSIGNED);

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
fn parse_binary_prop(path: &Path, prop: &str) -> Vec<u8> {
    let text = fs::read_to_string(path).unwrap_or_else(|_| panic!("read {}", path.display()));
    let mut codes = vec![0u8; NUM_CODEPOINTS];
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
        let (start, end) = match range.split_once("..") {
            Some((a, b)) => (
                u32::from_str_radix(a.trim(), 16).unwrap(),
                u32::from_str_radix(b.trim(), 16).unwrap(),
            ),
            None => {
                let v = u32::from_str_radix(range, 16).unwrap();
                (v, v)
            }
        };
        for c in start..=end {
            codes[c as usize] = 1;
        }
    }
    codes
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
    codes: &[u8],
    default_code: u8,
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
fn emit_bool_lookup(out: &mut String, fn_name: &str, prefix: &str, codes: &[u8]) {
    let render = [String::from("false"), String::from("true")];
    emit_lookup(out, fn_name, prefix, "bool", codes, 0, &render);
}

/// Emit coalesced `match` arms for one (sub)slice of low-byte values, skipping
/// runs equal to the default. `base` is the low byte of `slice[0]`. Each arm is
/// prefixed with `arm_cfg` (e.g. a latin1 cfg, or empty).
fn emit_arms(
    out: &mut String,
    slice: &[u8],
    base: usize,
    default_code: u8,
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
