numbersSymbolOverrides.json — Unicode CLDR v48, DERIVED (not verbatim).

Source: https://github.com/unicode-org/cldr (tag release-48)
        common/main/<locale>.xml, the //ldml/numbers section.

Why this file exists
--------------------
cldr-json's `cldr-numbers-full` (which numbers-raw/ is vendored from) emits
`symbols-numberSystem-<ns>` only for the numbering systems a locale *declares* —
its `defaultNumberingSystem` and `otherNumberingSystems`. CLDR's LDML carries
more: most locales also override a handful of symbol fields for systems they do
not declare, precisely so that root's block does not leak in. `fa` has no `arab`
block in the JSON, but `fa.xml` overrides `arab`'s percentSign to "٪" and its nan
to "ناعدد"; the rest of the block is `↑↑↑` (inherit), and comes from root.

ICU resolves this per *field*, not per block (`DecimalFormatSymbols::initialize`
in dcfmtsym.cpp: the requested system's symbols are collected up the bundle chain
to root, then latn's are collected for whatever is still missing). Without these
overrides the crate's root fallback — added for issue #22 A4 — would hand
`fa-u-nu-arab` root's percentSign where `Intl.NumberFormat` gives fa's.

Scope
-----
Only the two numbering systems CLDR root defines real symbols for, `arab` and
`arabext`. For every other system root is an `<alias source="locale" …>` to
`latn`, so the locale's own `latn` block answers anyway and an override that
merely restates it changes nothing.

How to regenerate
-----------------
For each locale vendored under numbers-raw/, read common/main/<locale>.xml
(with `-` mapped to `_`) and, for `ns` in {arab, arabext} *not* already present
as `symbols-numberSystem-<ns>` in that locale's numbers.json:

  * from `<symbols numberSystem="ns">`, keep decimal, group, percentSign,
    plusSign, minusSign, nan and infinity whose text is not the CLDR inheritance
    marker `↑↑↑`;
  * from `<decimalFormats numberSystem="ns">` and `<percentFormats …>`, keep the
    `type`-less `<…FormatLength>`'s `<pattern>` unless it is `↑↑↑` or the block
    is an `<alias>`;
  * in both cases skip elements marked `draft="unconfirmed"` or
    `draft="provisional"`. CLDR ships those in the survey data but not in built
    data — ICU keeps approved + contributed — and honouring them would give
    `fr-u-nu-arab` a minus sign `Intl.NumberFormat` does not use.

Emit them under the same keys numbers.json uses. Read by codegen (emit_numbers),
which merges each block over root's before falling back to the locale's `latn`.
Do not hand-edit.

The vendored data is CLDR 48; a `node`/ICU built against an older CLDR will
disagree on entries CLDR changed since — `cv`'s `arab` percent pattern is `↑↑↑`
in CLDR 47 and `#,##0 %` in CLDR 48.
