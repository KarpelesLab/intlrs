//! Size probe: references the core property lookups so the generated tables are
//! linked, letting CI measure the compiled footprint per range tier with `size`.
//! Not part of the published package (examples are excluded from `include`).
use intl::unicode::{east_asian_width, general_category, script};

fn main() {
    let mut acc = 0u64;
    for cp in 0u32..0x11_0000 {
        if let Some(c) = char::from_u32(cp) {
            acc = acc
                .wrapping_add(general_category(c) as u64)
                .wrapping_add(east_asian_width(c) as u64)
                .wrapping_add(script(c) as u64);
        }
    }
    // Print so the loop isn't optimized away.
    println!("{acc}");
}
