Raw Unicode CLDR v48 Chinese lunisolar calendar data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-cal-chinese-full/main/<locale>/ca-chinese.json

These are the official Unicode CLDR `cldr-cal-chinese-full` files, committed
unmodified, for the same CLDR "modern" coverage base languages as the sibling
`dates/` directory (101 locales). `codegen` reads them to produce
src/cldr/lunisolar.bin. Do not hand-edit; to refresh, re-download from the
same upstream path at the pinned CLDR version.

60 sexagenary cycle names, 12 numeric month names in three widths, the
leap-month marker (UTS #35 `monthPatterns`) and the date patterns
`datetime::format_chinese_date` renders. No eras: CLDR names these years by
cycle position, not by era.
