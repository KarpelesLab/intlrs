Raw Unicode CLDR v48 number data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-numbers-full/main/<locale>/numbers.json

These are the official Unicode CLDR `cldr-numbers-full` files, committed
unmodified. `codegen` (emit_numbers, emit_numsys, emit_currency) reads them to
produce src/cldr/numbers.bin, src/cldr/compact.bin and src/cldr/numsys_default.bin.
Do not hand-edit; to refresh, re-download from the same upstream path at the
pinned CLDR version.

The base set is the CLDR "modern" coverage base languages (language subtag only),
matching the other `*-raw` directories. Two non-base locales are vendored on top
because their number formatting differs from their base language and no fallback
can derive it:

  en-IN    Indian digit grouping (`#,##,##0.###`) and crore/lakh compact forms.
  zh-Hant  Traditional Chinese compact forms (億 rather than 亿).

`zh-Hant` has no region files upstream; `emit_numbers` derives `zh-TW`, `zh-HK`
and `zh-MO` from it using CLDR's own likelySubtags, since the runtime lookup does
no script inference.
