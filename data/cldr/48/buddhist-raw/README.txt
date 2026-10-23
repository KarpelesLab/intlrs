Raw Unicode CLDR v48 Thai Buddhist calendar data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-cal-buddhist-full/main/<locale>/ca-buddhist.json

These are the official Unicode CLDR `cldr-cal-buddhist-full` files, committed
unmodified, for the same CLDR "modern" coverage base languages as the sibling
`dates/` directory. `codegen` reads them to produce
src/cldr/alt_calendars.bin. Do not hand-edit; to refresh, re-download from the
same upstream path at the pinned CLDR version.

Only the era names are compiled in: CLDR gives this calendar the locale's
Gregorian month names verbatim, which codegen checks per locale before deciding
not to store a second copy.

The CLDR hub and non-default-script sublocales (`en-001`, `pt-PT`, `zh-Hant`,
…) are vendored on top of the base languages; see ../README-sublocales.txt.
