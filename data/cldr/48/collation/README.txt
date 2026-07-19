Unicode CLDR v48 Chinese (zh) collation tailoring, vendored verbatim.

Source: https://github.com/unicode-org/cldr  (tag release-48)
        common/collation/zh.xml

This is the official Unicode CLDR `zh.xml` collation file, committed unmodified.
It defines several Chinese collation variants; the default is `pinyin`
(`<defaultCollation>pinyin</defaultCollation>`), the order used by
`Intl.Collator('zh')`. The `<collation type='pinyin'>` `<cr>` rule is a ~1.15 MB
chain that establishes the total pinyin order of ~44k Han ideographs
(`[reorder Hani]`, `&[last regular]`, then `<*`-separated runs sorted by
pinyin -> tone -> kTotalStrokes -> kRSUnicode).

Because the raw rule string is far too large to ship as a runtime tailoring, it
is DISTILLED by `codegen` (emit_collation_zh) into a compact
`src/unicode/collation_zh.bin` table: each listed Han ideograph -> its pinyin
primary rank (a running counter that bumps on each `<`). The runtime `zh` pinyin
collator (feature `collation-zh`) overrides the DUCET primary of each Han
character with this rank; unlisted ideographs fall back to DUCET
radical-stroke order, and non-Han uses the root (DUCET) order.

Only the `pinyin` variant is distilled. The `stroke`, `zhuyin`, `unihan`, and
`gb2312`/`big5` variants are out of scope. Distributed by Unicode under the
Unicode license (https://www.unicode.org/copyright.html), compatible with this
crate's MIT license. Do not hand-edit; to refresh, re-download from the same
upstream path at the pinned CLDR version.
