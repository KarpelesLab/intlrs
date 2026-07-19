ICU break-iterator dictionaries, vendored verbatim.

Source: https://github.com/unicode-org/icu  (branch main, fetched 2026-07-19)
        icu4c/source/data/brkitr/dictionaries/thaidict.txt
        icu4c/source/data/brkitr/dictionaries/laodict.txt
        icu4c/source/data/brkitr/dictionaries/khmerdict.txt
        icu4c/source/data/brkitr/dictionaries/burmesedict.txt
        icu4c/source/data/brkitr/dictionaries/cjdict.txt

thaidict.txt / laodict.txt / khmerdict.txt / burmesedict.txt are ICU's Thai, Lao,
Khmer and Burmese (Myanmar) word-break dictionaries: plain, newline-separated
lists of words (leading `#` lines are comments; a UTF-8 BOM prefixes each file).
They are distributed by Unicode under the Unicode license
(https://www.unicode.org/copyright.html), which is compatible with this crate's
MIT license.

cjdict.txt is ICU's Chinese/Japanese word-break dictionary: `word<TAB>value`
lines covering Han + Hiragana + Katakana (leading `#` lines are comments; a
UTF-8 BOM prefixes the file). The `value` is the self-negative-log-probability
cost (~27..251) that ICU's `gendict` stores verbatim and `CjkBreakEngine` feeds
into its Viterbi minimum-cost segmentation. It originates from the Chromium
project; see the header of cjdict.txt for the full BSD/Unicode license terms
(compatible with this crate's MIT license).

`codegen` builds minimized deterministic acyclic word graphs (DAWGs) from these
files:
  * emit_segment_dict  -> src/unicode/segment_dict.bin      (Thai; feature
                          `segmentation-dict`; edge symbols U+0E00-relative)
  *                    -> src/unicode/segment_dict_lao.bin  (Lao, same format;
                          feature `segmentation-dict-lao`)
  *                    -> src/unicode/segment_dict_km.bin   (Khmer, same format,
                          edge symbols U+1780-relative; large enough that node ids
                          widen to u32; feature `segmentation-dict-km`)
  *                    -> src/unicode/segment_dict_my.bin   (Burmese/Myanmar, same
                          format, edge symbols U+1000-relative, MAIN BLOCK ONLY —
                          Myanmar Extended-A/B code points do not fit a u8 of
                          U+1000 and are absent from burmesedict.txt anyway;
                          feature `segmentation-dict-my`)
  * emit_cjk_dict      -> src/unicode/segment_dict_cjk.bin  (CJK, with per-word
                          costs; feature `segmentation-dict-cjk`)
Those blobs are embedded (behind the respective Cargo features) and drive the
ICU-style ThaiBreakEngine / KhmerBreakEngine / BurmeseBreakEngine /
CjkBreakEngine dictionary word segmentation in src/unicode/segment.rs.

Do not hand-edit; to refresh, re-download from the same upstream paths.
