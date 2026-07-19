//! Dictionary-based CJK (Chinese + Japanese) word segmentation
//! (feature `segmentation-dict-cjk`).
//!
//! Han/kana text carries no spaces, so UAX #29 alone leaves each ideograph its
//! own "word"; a language dictionary plus a cost model is required to recover
//! real words. This module is a `no_std` (but allocating) port of ICU's
//! `CjkBreakEngine::divideUpDictionaryRange` (`icu4c/source/common/dictbe.cpp`):
//! a Viterbi / dynamic-programming search for the minimum-total-cost
//! segmentation of a maximal run of CJK dictionary characters, where each
//! dictionary word contributes its stored self-negative-log-probability cost and
//! any non-dictionary code point falls back to a single-character word at the
//! maximum cost. Continuous Katakana runs get ICU's length-based cost heuristic.
//!
//! The dictionary is the committed `segment_dict_cjk.bin` DAWG built from ICU's
//! `cjdict.txt` (see `codegen::emit_cjk_dict` for the byte layout); unlike the
//! Thai DAWG its edges are full codepoints and each word-end node stores a `u8`
//! cost. The DP arrays are sized to the run length, hence the `alloc`
//! requirement (the Thai engine stays allocation-free).
//!
//! Like ICU, the run is **NFKC-normalized before dictionary matching** so that
//! half-width katakana (`ﾃｽﾄ`), full-width forms and the like segment as their
//! canonical equivalents; the resulting break positions are then mapped back to
//! byte offsets in the *original* run so the emitted tokens keep their original
//! spelling. Normalization is done segment-by-segment at NFKC boundaries, and
//! every normalized code point is mapped back to the byte offset of its
//! segment's first original code point, so a break is never introduced inside an
//! original code point (matching V8/ICU, which never splits e.g. `ﾃﾞ`→デ). On
//! already-NFKC text (the overwhelmingly common Han/kana case) the mapping is the
//! identity and the output is byte-for-byte the pre-normalization behavior.
//!
//! Divergences from ICU that are deliberately not reproduced: the
//! phrase-breaking / ML paths. See the module tests and the crate notes for the
//! honest parity picture.

use alloc::vec::Vec;

/// `kMaxKatakanaLength` (ICU): katakana words longer than this share one cost.
const MAX_KATAKANA_LENGTH: usize = 8;
/// `kMaxKatakanaGroupLength` (ICU): the longest katakana run given a group cost.
const MAX_KATAKANA_GROUP_LENGTH: usize = 20;
/// `maxSnlp` (ICU): the single-character fallback cost (least likely word).
const MAX_SNLP: u32 = 255;
/// `maxWordSize` (ICU): the dictionary is probed at most this many code points.
const MAX_WORD_SIZE: usize = 20;

/// ICU's `getKatakanaCost`: a continuous katakana run of `word_length` code
/// points is a candidate word with this cost.
#[inline]
fn katakana_cost(word_length: usize) -> u32 {
    const COST: [u32; MAX_KATAKANA_LENGTH + 1] = [8192, 984, 408, 240, 204, 252, 300, 372, 480];
    if word_length > MAX_KATAKANA_LENGTH {
        8192
    } else {
        COST[word_length]
    }
}

/// ICU's `isKatakana`.
#[inline]
fn is_katakana(c: char) -> bool {
    let v = c as u32;
    (0x30A1..=0x30FE).contains(&v) && v != 0x30FB || (0xFF66..=0xFF9F).contains(&v)
}

/// The character set ICU's `CjkBreakEngine` segments: `[[:Han:][:Hiragana:]`
/// `[:Katakana:]ーｰﾞﾟ]`. Hangul is intentionally excluded —
/// Korean uses spaces and is handled by the plain UAX #29 rules.
#[inline]
pub(crate) fn is_cjk_dict_char(c: char) -> bool {
    use super::script::Script;
    matches!(
        super::script::script(c),
        Script::Han | Script::Hiragana | Script::Katakana
    ) || matches!(c as u32, 0x30FC | 0xFF70 | 0xFF9E | 0xFF9F)
}

// ---- DAWG dictionary matcher (codepoint edges + per-word cost) ----

/// The committed CJK dictionary DAWG. Always embedded; only read when the
/// `segmentation-dict-cjk` feature is on. See `codegen::emit_cjk_dict` for the
/// byte layout.
const DICT: &[u8] = include_bytes!("segment_dict_cjk.bin");

#[inline]
fn rd_u32(o: usize) -> Option<u32> {
    let b = DICT.get(o..o.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// A parsed view over the embedded DAWG. Every access is bounds-checked so a
/// truncated or inconsistent blob degrades to "no dictionary match" rather than
/// panicking.
struct Dict {
    n: usize,
    root: usize,
    values_base: usize,
    off_base: usize,
    edges_base: usize,
}

impl Dict {
    fn load() -> Option<Dict> {
        let n = rd_u32(0)? as usize;
        let e = rd_u32(4)? as usize;
        let root = rd_u32(8)? as usize;
        let values_base = 12usize;
        let off_base = values_base.checked_add(n)?;
        let edges_base = off_base.checked_add((n.checked_add(1)?).checked_mul(4)?)?;
        if edges_base.checked_add(e.checked_mul(5)?)? > DICT.len() || root >= n {
            return None;
        }
        Some(Dict {
            n,
            root,
            values_base,
            off_base,
            edges_base,
        })
    }

    /// The word-end cost stored at `node` (0 == not a word end).
    #[inline]
    fn cost(&self, node: usize) -> u8 {
        if node >= self.n {
            return 0;
        }
        DICT.get(self.values_base + node).copied().unwrap_or(0)
    }

    /// The `[start, end)` edge index range owned by `node`.
    #[inline]
    fn edge_range(&self, node: usize) -> (usize, usize) {
        let a = rd_u32(self.off_base + node * 4).unwrap_or(0) as usize;
        let b = rd_u32(self.off_base + (node + 1) * 4).unwrap_or(0) as usize;
        (a, b)
    }

    /// Follow the edge labelled `sym` out of `node`, if any (binary search).
    #[inline]
    fn child(&self, node: usize, sym: u16) -> Option<usize> {
        let (mut lo, mut hi) = self.edge_range(node);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let rec = self.edges_base + mid * 5;
            let s = u16::from_le_bytes([*DICT.get(rec)?, *DICT.get(rec + 1)?]);
            match s.cmp(&sym) {
                core::cmp::Ordering::Less => lo = mid + 1,
                core::cmp::Ordering::Greater => hi = mid,
                core::cmp::Ordering::Equal => {
                    let t = [
                        *DICT.get(rec + 2)?,
                        *DICT.get(rec + 3)?,
                        *DICT.get(rec + 4)?,
                        0,
                    ];
                    return Some(u32::from_le_bytes(t) as usize);
                }
            }
        }
        None
    }
}

/// The DAWG edge symbol for a codepoint, or `None` if it is outside the BMP
/// (cjdict is entirely within the BMP, so a supplementary code point simply has
/// no dictionary word and falls back to a single-character segment, exactly as
/// in ICU).
#[inline]
fn sym(c: char) -> Option<u16> {
    u16::try_from(c as u32).ok()
}

/// Whether `c` starts a fresh NFKC normalization segment (has a "boundary
/// before" it). A code point can be composed onto preceding text exactly when
/// the leading code point of its (compatibility) decomposition is a non-starter
/// — e.g. the half-width voiced sound marks U+FF9E/U+FF9F decompose to U+3099
/// (CCC 8) and so attach to the preceding kana (`ﾃﾞ` → デ). Everything else
/// begins a new segment. (Two starters only compose for Hangul L+V, and Hangul
/// is excluded from CJK runs, so the leading-CCC test is exact here.)
#[inline]
fn nfkc_boundary_before(c: char) -> bool {
    use super::normalize;
    let lead = normalize::nfkd(core::iter::once(c)).next().unwrap_or(c);
    normalize::canonical_combining_class(lead) == 0
}

/// Build the NFKC-normalized code point sequence of `run` together with a
/// mapping back into the original: `chars[i]` is the i-th normalized code point
/// and `byte_off[i]` is the byte offset (in `run`) of the first original code
/// point of the normalization segment it came from; `byte_off` has `ncp + 1`
/// entries with the last being `run.len()`.
///
/// Each maximal segment of original code points (one leading starter plus any
/// following combining/attaching code points) is normalized as a whole, so
/// composition across the join (`ﾃﾞ` → デ) is honored; every code point the
/// segment produces maps back to that segment's start, so a dictionary break can
/// only ever fall on an original code-point boundary, never inside one.
fn build_normalized(run: &str, chars: &mut Vec<char>, byte_off: &mut Vec<usize>) {
    use super::normalize::nfkc;
    let orig: Vec<(usize, char)> = run.char_indices().collect();
    let n = orig.len();
    let mut i = 0;
    while i < n {
        let start_byte = orig[i].0;
        i += 1;
        while i < n && !nfkc_boundary_before(orig[i].1) {
            i += 1;
        }
        let end_byte = if i < n { orig[i].0 } else { run.len() };
        for c in nfkc(run[start_byte..end_byte].chars()) {
            chars.push(c);
            byte_off.push(start_byte);
        }
    }
    byte_off.push(run.len());
}

/// Fill `out` with the byte offsets (within `run`) of every CJK word end, in
/// ascending order, the last being `run.len()`. `out` is cleared first.
///
/// This is one call of ICU's `CjkBreakEngine::divideUpDictionaryRange` over the
/// whole run: a minimum-total-cost Viterbi segmentation. `run` must be a maximal
/// run of [`is_cjk_dict_char`] characters. Like ICU, the run is NFKC-normalized
/// before matching and the resulting breaks are mapped back to original byte
/// offsets (see [`build_normalized`]).
pub(crate) fn segment(run: &str, out: &mut Vec<usize>) {
    out.clear();
    if run.is_empty() {
        return;
    }
    let Some(dict) = Dict::load() else {
        out.push(run.len());
        return;
    };

    // Code points to segment, with a byte-offset mapping back into `run`.
    // `byte_off` has `ncp + 1` entries so index `ncp` maps to `run.len()`.
    let mut chars: Vec<char> = Vec::new();
    let mut byte_off: Vec<usize> = Vec::new();
    if super::normalize::quick_check_nfkc(run.chars()) == super::normalize::IsNormalized::Yes {
        // Already NFKC: identity mapping — byte-for-byte the pre-normalization
        // behavior, and the common Han/kana case avoids all normalization work.
        for (i, c) in run.char_indices() {
            chars.push(c);
            byte_off.push(i);
        }
        byte_off.push(run.len());
    } else {
        // Segment the run over its NFKC-normalized code points, mapping each back
        // to an original byte offset.
        build_normalized(run, &mut chars, &mut byte_off);
    }
    let ncp = chars.len();
    if ncp == 0 {
        out.push(run.len());
        return;
    }

    // bestSnlp[i] = min cost of segmenting the first i code points; prev[i] =
    // the start code point of the last word in that best segmentation.
    let mut best = alloc::vec![u32::MAX; ncp + 1];
    best[0] = 0;
    let mut prev = alloc::vec![usize::MAX; ncp + 1];

    // Scratch candidate lists (word code-point length, cost), refilled per i.
    let mut cand_len: Vec<usize> = Vec::with_capacity(MAX_WORD_SIZE);
    let mut cand_cost: Vec<u32> = Vec::with_capacity(MAX_WORD_SIZE);

    let mut is_prev_katakana = false;
    for i in 0..ncp {
        if best[i] == u32::MAX {
            // Unreachable in practice (the single-char fallback keeps every
            // position reachable), but replicate ICU's early `continue` — it
            // also skips the katakana-state update.
            continue;
        }

        // Dictionary words starting at code point i (increasing length order).
        cand_len.clear();
        cand_cost.clear();
        let mut node = dict.root;
        let mut k = i;
        while k < ncp && (k - i) < MAX_WORD_SIZE {
            let Some(s) = sym(chars[k]) else { break };
            let Some(nx) = dict.child(node, s) else { break };
            node = nx;
            k += 1;
            let cost = dict.cost(node);
            if cost != 0 {
                cand_len.push(k - i);
                cand_cost.push(cost as u32);
            }
        }

        // Single-character fallback: if no dictionary word of length 1 begins
        // here, treat this code point as a 1-char word at the maximum cost.
        // (Hangul is never in a CJK run, so ICU's Hangul exclusion is moot.)
        if cand_len.first() != Some(&1) {
            cand_len.push(1);
            cand_cost.push(MAX_SNLP);
        }

        for (idx, &len) in cand_len.iter().enumerate() {
            let ni = i + len;
            let nsnlp = best[i].saturating_add(cand_cost[idx]);
            if nsnlp < best[ni] {
                best[ni] = nsnlp;
                prev[ni] = i;
            }
        }

        // Katakana heuristic: a continuous katakana run is a candidate word with
        // a length-based cost.
        let is_kata = is_katakana(chars[i]);
        if !is_prev_katakana && is_kata {
            let mut run_len = 1usize;
            let mut j = i + 1;
            while j < ncp && run_len < MAX_KATAKANA_GROUP_LENGTH && is_katakana(chars[j]) {
                j += 1;
                run_len += 1;
            }
            if run_len < MAX_KATAKANA_GROUP_LENGTH {
                let ni = i + run_len;
                let nsnlp = best[i].saturating_add(katakana_cost(run_len));
                if nsnlp < best[ni] {
                    best[ni] = nsnlp;
                    prev[ni] = i;
                }
            }
        }
        is_prev_katakana = is_kata;
    }

    // Backtrace the optimal boundaries (code point indices), collect descending
    // then reverse into ascending byte offsets. A boundary in normalized space
    // maps back through `byte_off`; two adjacent normalized breaks can land on
    // the same original offset (a break that would fall inside a normalization
    // segment), so drop those to keep the ends strictly ascending — that never
    // splits an original code point.
    if best[ncp] == u32::MAX {
        out.push(run.len());
        return;
    }
    let mut i = ncp;
    while i > 0 {
        let b = byte_off[i];
        if out.last() != Some(&b) {
            out.push(b);
        }
        let p = prev[i];
        if p == usize::MAX || p >= i {
            break; // defensive: never loop on a malformed prev chain
        }
        i = p;
    }
    out.reverse();
}
