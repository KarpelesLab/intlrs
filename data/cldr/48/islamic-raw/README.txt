Raw Unicode CLDR v48 Islamic (Hijri) calendar data, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-cal-islamic-full/main/<locale>/ca-islamic.json

These are the official Unicode CLDR `cldr-cal-islamic-full` files, committed
unmodified, for the same CLDR "modern" coverage base languages as the sibling
`dates/` directory (101 locales). `codegen` reads them to produce
src/cldr/alt_calendars.bin. Do not hand-edit; to refresh, re-download from the
same upstream path at the pinned CLDR version.

12 months and two eras (AH / BH). The four BCP-47 Islamic calendars
(islamic, islamic-civil, islamic-rgsa, islamic-tbla, islamic-umalqura) differ in
their arithmetic, not their names, and all read this one name set. Also supplies
the date patterns `datetime::format_islamic_date` renders.
