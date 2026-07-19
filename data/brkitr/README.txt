ICU break-iterator dictionaries, vendored verbatim.

Source: https://github.com/unicode-org/icu  (branch main, fetched 2026-07-19)
        icu4c/source/data/brkitr/dictionaries/thaidict.txt

thaidict.txt is ICU's Thai word-break dictionary: a plain, newline-separated
list of Thai words (leading `#` lines are comments; a UTF-8 BOM prefixes the
file). It is distributed by Unicode under the Unicode license
(https://www.unicode.org/copyright.html), which is compatible with this crate's
MIT license.

`codegen` (emit_segment_dict) reads this file and builds a minimized
deterministic acyclic word graph (DAWG), emitting src/unicode/segment_dict.bin.
That blob is embedded (behind the `segmentation-dict` Cargo feature) and drives
the ICU-style ThaiBreakEngine dictionary word segmentation in
src/unicode/segment.rs.

Do not hand-edit; to refresh, re-download from the same upstream path.
