Unicode CLDR v48 collation tailorings, vendored verbatim.

Source: https://github.com/unicode-org/cldr  (tag release-48)
        common/collation/*.xml

Every file here is an official Unicode CLDR collation file, committed unmodified.
Do not hand-edit: the whole point of vendoring the XML is that the shipped tables
are reproducible from upstream. To refresh, re-download the same upstream
directory at the pinned CLDR version and re-run `codegen`.

--- Per-locale tailoring rules --------------------------------------------------

`codegen` (emit_collation_rules) reads each file's `<collation type="standard">`
`<cr>` rule and emits both `data/cldr/48/collation.json` (a readable mirror) and
`src/cldr/collation.bin` (the runtime blob behind `Tailoring::for_locale`). Rules
are copied verbatim; only `#` comments, redundant whitespace and the ordering-free
`[optimize …]` / `[suppressContractions …]` hints are stripped.

Locale coverage is therefore data-driven, NOT hand-curated. Two things are
filtered, both deliberately:

  * `codegen`'s COLLATION_SKIP — locales whose rule the runtime rule engine
    cannot represent (a reset onto a bare combining mark, a `[first|last …
    ignorable]` pseudo-anchor, a chained multi-char expansion like Danish
    `&å<<<aa`). Shipping those would sort text *wrong*, so they fall back to a
    hand-written approximation in `Tailoring::for_locale`, or to root order.
    `tests/collation_data_consistency` re-derives the list from the data, so an
    entry that becomes representable shows up as a gate failure.

  * A `<collation type="standard">` block carrying `alt=` is a proposed
    alternative rather than the winning value, and is ignored. `draft=` is NOT
    filtered — CLDR ships draft collations and ICU builds them.

A locale with no `standard` collation (German, which tailors only `phonebook` =
`de-u-co-phonebk`; Irish, Dutch, Catalan, Indonesian, …) gets no tailoring at all,
matching `Intl.Collator`, which sorts those in root order.

--- Chinese (zh.xml) ------------------------------------------------------------

`zh.xml` is handled separately and is excluded from the rule table above. It
defines several Chinese collation variants; the default is `pinyin`
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

The `stroke` and `zhuyin` variants are distilled the same way (selected by the
`-u-co-` keyword); `unihan` uses the Unihan radical-stroke table instead, and
`gb2312`/`big5` are out of scope.

Distributed by Unicode under the Unicode license
(https://www.unicode.org/copyright.html), compatible with this crate's MIT
license.
