//! Fuzz the Unicode algorithms that take arbitrary text: they must never panic
//! and the round-trip / partition invariants must hold.
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _: String = intl::unicode::nfc(data.chars()).collect();
    let _: String = intl::unicode::nfd(data.chars()).collect();
    let _: String = intl::unicode::nfkc(data.chars()).collect();
    let _: String = intl::unicode::nfkd(data.chars()).collect();
    assert_eq!(intl::unicode::graphemes(data).collect::<String>(), data);
    assert_eq!(intl::unicode::words(data).collect::<String>(), data);
    assert_eq!(
        intl::unicode::line_breaks(data)
            .map(|b| b.text)
            .collect::<String>(),
        data
    );
    let info = intl::unicode::bidi::process(data, None);
    assert_eq!(info.levels.len(), data.chars().count());
    let _ = intl::unicode::idna::to_ascii(data);
    let _ = intl::unicode::spoof::skeleton(data);
    let _ = intl::unicode::lowercase_str(data);
    let _ = intl::unicode::collate::sort_key(data);
});
