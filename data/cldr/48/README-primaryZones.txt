primaryZones.json — Unicode CLDR v48, vendored verbatim.

Source: https://github.com/unicode-org/cldr-json  (tag 48.0.0)
        cldr-json/cldr-core/supplemental/primaryZones.json

Eleven territories whose "main" time zone CLDR designates explicitly. UTS #35
§4.8's generic location format names the *country* rather than the exemplar city
for a zone that is alone in its territory; a designated primary zone gets the
same treatment even though its territory has several zones, which is why
Asia/Shanghai reads "China Time" and not "Shanghai Time". Read by codegen
(emit_tz_names). Do not hand-edit.
