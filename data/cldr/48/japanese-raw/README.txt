Raw Unicode CLDR v48 Japanese imperial calendar data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-cal-japanese-full/main/<locale>/ca-japanese.json

These are the official Unicode CLDR `cldr-cal-japanese-full` files, committed
unmodified, for the same CLDR "modern" coverage base languages as the sibling
`dates/` directory (101 locales). `codegen` reads them to produce
src/cldr/japanese.bin. Do not hand-edit; to refresh, re-download from the
same upstream path at the pinned CLDR version.

All 237 nengō in three widths. The 5 modern eras (CLDR indices 232 Meiji ..
236 Reiwa) plus the date patterns go into japanese.bin; the 232 pre-Meiji names
are deduplicated into japanese_hist.bin (only ~21 distinct sets across the 101
locales). The Japanese calendar shares the Gregorian month and weekday names, so
none are stored.
