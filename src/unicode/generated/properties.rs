#[inline]
pub(crate) const fn age(cp: u32) -> Option<(u8, u8)> {
    match cp >> 8 {
        #[cfg(feature = "ascii")]
        0x000 => age_p0(cp as u8),
        #[cfg(feature = "bmp")]
        0x001 => age_p1(cp as u8),
        #[cfg(feature = "bmp")]
        0x002 => age_p2(cp as u8),
        #[cfg(feature = "bmp")]
        0x003 => age_p3(cp as u8),
        #[cfg(feature = "bmp")]
        0x004 => age_p4(cp as u8),
        #[cfg(feature = "bmp")]
        0x005 => age_p5(cp as u8),
        #[cfg(feature = "bmp")]
        0x006 => age_p6(cp as u8),
        #[cfg(feature = "bmp")]
        0x007 => age_p7(cp as u8),
        #[cfg(feature = "bmp")]
        0x008 => age_p8(cp as u8),
        #[cfg(feature = "bmp")]
        0x009 => age_p9(cp as u8),
        #[cfg(feature = "bmp")]
        0x00a => age_pa(cp as u8),
        #[cfg(feature = "bmp")]
        0x00b => age_pb(cp as u8),
        #[cfg(feature = "bmp")]
        0x00c => age_pc(cp as u8),
        #[cfg(feature = "bmp")]
        0x00d => age_pd(cp as u8),
        #[cfg(feature = "bmp")]
        0x00e => age_pe(cp as u8),
        #[cfg(feature = "bmp")]
        0x00f => age_pf(cp as u8),
        #[cfg(feature = "bmp")]
        0x010 => age_p10(cp as u8),
        #[cfg(feature = "bmp")]
        0x011 => age_p11(cp as u8),
        #[cfg(feature = "bmp")]
        0x012 => age_p12(cp as u8),
        #[cfg(feature = "bmp")]
        0x013 => age_p13(cp as u8),
        #[cfg(feature = "bmp")]
        0x014 => age_p14(cp as u8),
        #[cfg(feature = "bmp")]
        0x015 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x016 => age_p16(cp as u8),
        #[cfg(feature = "bmp")]
        0x017 => age_p17(cp as u8),
        #[cfg(feature = "bmp")]
        0x018 => age_p18(cp as u8),
        #[cfg(feature = "bmp")]
        0x019 => age_p19(cp as u8),
        #[cfg(feature = "bmp")]
        0x01a => age_p1a(cp as u8),
        #[cfg(feature = "bmp")]
        0x01b => age_p1b(cp as u8),
        #[cfg(feature = "bmp")]
        0x01c => age_p1c(cp as u8),
        #[cfg(feature = "bmp")]
        0x01d => age_p1d(cp as u8),
        #[cfg(feature = "bmp")]
        0x01e => age_p1e(cp as u8),
        #[cfg(feature = "bmp")]
        0x01f => age_p1f(cp as u8),
        #[cfg(feature = "bmp")]
        0x020 => age_p20(cp as u8),
        #[cfg(feature = "bmp")]
        0x021 => age_p21(cp as u8),
        #[cfg(feature = "bmp")]
        0x022 => age_p22(cp as u8),
        #[cfg(feature = "bmp")]
        0x023 => age_p23(cp as u8),
        #[cfg(feature = "bmp")]
        0x024 => age_p24(cp as u8),
        #[cfg(feature = "bmp")]
        0x025 => age_p25(cp as u8),
        #[cfg(feature = "bmp")]
        0x026 => age_p26(cp as u8),
        #[cfg(feature = "bmp")]
        0x027 => age_p27(cp as u8),
        #[cfg(feature = "bmp")]
        0x028 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x029 => Some((3, 2)),
        #[cfg(feature = "bmp")]
        0x02a => Some((3, 2)),
        #[cfg(feature = "bmp")]
        0x02b => age_p2b(cp as u8),
        #[cfg(feature = "bmp")]
        0x02c => age_p2c(cp as u8),
        #[cfg(feature = "bmp")]
        0x02d => age_p2d(cp as u8),
        #[cfg(feature = "bmp")]
        0x02e => age_p2e(cp as u8),
        #[cfg(feature = "bmp")]
        0x02f => age_p2f(cp as u8),
        #[cfg(feature = "bmp")]
        0x030 => age_p30(cp as u8),
        #[cfg(feature = "bmp")]
        0x031 => age_p31(cp as u8),
        #[cfg(feature = "bmp")]
        0x032 => age_p32(cp as u8),
        #[cfg(feature = "bmp")]
        0x033 => age_p33(cp as u8),
        #[cfg(feature = "bmp")]
        0x034 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x035 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x036 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x037 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x038 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x039 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03a => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03b => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03c => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03d => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03e => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x03f => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x040 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x041 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x042 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x043 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x044 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x045 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x046 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x047 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x048 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x049 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x04a => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x04b => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x04c => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x04d => age_p4d(cp as u8),
        #[cfg(feature = "bmp")]
        0x04e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x04f => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x050 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x051 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x052 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x053 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x054 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x055 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x056 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x057 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x058 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x059 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05a => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05b => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05c => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05d => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x05f => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x060 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x061 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x062 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x063 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x064 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x065 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x066 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x067 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x068 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x069 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06a => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06b => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06c => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06d => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x06f => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x070 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x071 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x072 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x073 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x074 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x075 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x076 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x077 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x078 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x079 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07a => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07b => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07c => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07d => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x07f => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x080 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x081 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x082 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x083 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x084 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x085 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x086 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x087 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x088 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x089 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08a => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08b => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08c => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08d => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x08f => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x090 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x091 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x092 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x093 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x094 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x095 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x096 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x097 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x098 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x099 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09a => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09b => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09c => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09d => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09e => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x09f => age_p9f(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a0 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x0a1 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x0a2 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x0a3 => Some((3, 0)),
        #[cfg(feature = "bmp")]
        0x0a4 => age_pa4(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a5 => Some((5, 1)),
        #[cfg(feature = "bmp")]
        0x0a6 => age_pa6(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a7 => age_pa7(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a8 => age_pa8(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a9 => age_pa9(cp as u8),
        #[cfg(feature = "bmp")]
        0x0aa => age_paa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ab => age_pab(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ac => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0ad => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0ae => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0af => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b0 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b1 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b2 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b3 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b4 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b5 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b6 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b7 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b8 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0b9 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0ba => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0bb => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0bc => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0bd => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0be => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0bf => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c0 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c1 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c2 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c3 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c4 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c5 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c6 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c7 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c8 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0c9 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0ca => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0cb => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0cc => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0cd => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0ce => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0cf => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d0 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d1 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d2 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d3 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d4 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d5 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d6 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d7 => age_pd7(cp as u8),
        #[cfg(feature = "bmp")]
        0x0d8 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0d9 => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0da => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0db => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0dc => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0dd => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0de => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0df => Some((2, 0)),
        #[cfg(feature = "bmp")]
        0x0e0 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e1 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e2 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e3 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e4 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e5 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e6 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e7 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e8 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0e9 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0ea => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0eb => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0ec => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0ed => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0ee => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0ef => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f0 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f1 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f2 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f3 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f4 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f5 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f6 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f7 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f8 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0f9 => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0fa => age_pfa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fb => age_pfb(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fc => Some((1, 1)),
        #[cfg(feature = "bmp")]
        0x0fd => age_pfd(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fe => age_pfe(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ff => age_pff(cp as u8),
        #[cfg(feature = "full")]
        0x100 => age_p100(cp as u8),
        #[cfg(feature = "full")]
        0x101 => age_p101(cp as u8),
        #[cfg(feature = "full")]
        0x102 => age_p102(cp as u8),
        #[cfg(feature = "full")]
        0x103 => age_p103(cp as u8),
        #[cfg(feature = "full")]
        0x104 => age_p104(cp as u8),
        #[cfg(feature = "full")]
        0x105 => age_p105(cp as u8),
        #[cfg(feature = "full")]
        0x106 => Some((7, 0)),
        #[cfg(feature = "full")]
        0x107 => age_p107(cp as u8),
        #[cfg(feature = "full")]
        0x108 => age_p108(cp as u8),
        #[cfg(feature = "full")]
        0x109 => age_p109(cp as u8),
        #[cfg(feature = "full")]
        0x10a => age_p10a(cp as u8),
        #[cfg(feature = "full")]
        0x10b => age_p10b(cp as u8),
        #[cfg(feature = "full")]
        0x10c => age_p10c(cp as u8),
        #[cfg(feature = "full")]
        0x10d => age_p10d(cp as u8),
        #[cfg(feature = "full")]
        0x10e => age_p10e(cp as u8),
        #[cfg(feature = "full")]
        0x10f => age_p10f(cp as u8),
        #[cfg(feature = "full")]
        0x110 => age_p110(cp as u8),
        #[cfg(feature = "full")]
        0x111 => age_p111(cp as u8),
        #[cfg(feature = "full")]
        0x112 => age_p112(cp as u8),
        #[cfg(feature = "full")]
        0x113 => age_p113(cp as u8),
        #[cfg(feature = "full")]
        0x114 => age_p114(cp as u8),
        #[cfg(feature = "full")]
        0x115 => age_p115(cp as u8),
        #[cfg(feature = "full")]
        0x116 => age_p116(cp as u8),
        #[cfg(feature = "full")]
        0x117 => age_p117(cp as u8),
        #[cfg(feature = "full")]
        0x118 => age_p118(cp as u8),
        #[cfg(feature = "full")]
        0x119 => age_p119(cp as u8),
        #[cfg(feature = "full")]
        0x11a => age_p11a(cp as u8),
        #[cfg(feature = "full")]
        0x11b => age_p11b(cp as u8),
        #[cfg(feature = "full")]
        0x11c => age_p11c(cp as u8),
        #[cfg(feature = "full")]
        0x11d => age_p11d(cp as u8),
        #[cfg(feature = "full")]
        0x11e => age_p11e(cp as u8),
        #[cfg(feature = "full")]
        0x11f => age_p11f(cp as u8),
        #[cfg(feature = "full")]
        0x120 => Some((5, 0)),
        #[cfg(feature = "full")]
        0x121 => Some((5, 0)),
        #[cfg(feature = "full")]
        0x122 => Some((5, 0)),
        #[cfg(feature = "full")]
        0x123 => age_p123(cp as u8),
        #[cfg(feature = "full")]
        0x124 => age_p124(cp as u8),
        #[cfg(feature = "full")]
        0x125 => age_p125(cp as u8),
        #[cfg(feature = "full")]
        0x12f => age_p12f(cp as u8),
        #[cfg(feature = "full")]
        0x130 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x131 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x132 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x133 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x134 => age_p134(cp as u8),
        #[cfg(feature = "full")]
        0x135 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x136 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x137 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x138 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x139 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13a => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13b => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13c => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13d => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13e => Some((16, 0)),
        #[cfg(feature = "full")]
        0x13f => Some((16, 0)),
        #[cfg(feature = "full")]
        0x140 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x141 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x142 => Some((16, 0)),
        #[cfg(feature = "full")]
        0x143 => age_p143(cp as u8),
        #[cfg(feature = "full")]
        0x144 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x145 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x146 => age_p146(cp as u8),
        #[cfg(feature = "full")]
        0x161 => age_p161(cp as u8),
        #[cfg(feature = "full")]
        0x168 => Some((6, 0)),
        #[cfg(feature = "full")]
        0x169 => Some((6, 0)),
        #[cfg(feature = "full")]
        0x16a => age_p16a(cp as u8),
        #[cfg(feature = "full")]
        0x16b => age_p16b(cp as u8),
        #[cfg(feature = "full")]
        0x16d => age_p16d(cp as u8),
        #[cfg(feature = "full")]
        0x16e => age_p16e(cp as u8),
        #[cfg(feature = "full")]
        0x16f => age_p16f(cp as u8),
        #[cfg(feature = "full")]
        0x170 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x171 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x172 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x173 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x174 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x175 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x176 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x177 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x178 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x179 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17a => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17b => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17c => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17d => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17e => Some((9, 0)),
        #[cfg(feature = "full")]
        0x17f => Some((9, 0)),
        #[cfg(feature = "full")]
        0x180 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x181 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x182 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x183 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x184 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x185 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x186 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x187 => age_p187(cp as u8),
        #[cfg(feature = "full")]
        0x188 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x189 => Some((9, 0)),
        #[cfg(feature = "full")]
        0x18a => age_p18a(cp as u8),
        #[cfg(feature = "full")]
        0x18b => Some((13, 0)),
        #[cfg(feature = "full")]
        0x18c => age_p18c(cp as u8),
        #[cfg(feature = "full")]
        0x18d => age_p18d(cp as u8),
        #[cfg(feature = "full")]
        0x1af => age_p1af(cp as u8),
        #[cfg(feature = "full")]
        0x1b0 => age_p1b0(cp as u8),
        #[cfg(feature = "full")]
        0x1b1 => age_p1b1(cp as u8),
        #[cfg(feature = "full")]
        0x1b2 => age_p1b2(cp as u8),
        #[cfg(feature = "full")]
        0x1bc => age_p1bc(cp as u8),
        #[cfg(feature = "full")]
        0x1cc => age_p1cc(cp as u8),
        #[cfg(feature = "full")]
        0x1cd => Some((16, 0)),
        #[cfg(feature = "full")]
        0x1ce => age_p1ce(cp as u8),
        #[cfg(feature = "full")]
        0x1cf => age_p1cf(cp as u8),
        #[cfg(feature = "full")]
        0x1d0 => age_p1d0(cp as u8),
        #[cfg(feature = "full")]
        0x1d1 => age_p1d1(cp as u8),
        #[cfg(feature = "full")]
        0x1d2 => age_p1d2(cp as u8),
        #[cfg(feature = "full")]
        0x1d3 => age_p1d3(cp as u8),
        #[cfg(feature = "full")]
        0x1d4 => age_p1d4(cp as u8),
        #[cfg(feature = "full")]
        0x1d5 => age_p1d5(cp as u8),
        #[cfg(feature = "full")]
        0x1d6 => age_p1d6(cp as u8),
        #[cfg(feature = "full")]
        0x1d7 => age_p1d7(cp as u8),
        #[cfg(feature = "full")]
        0x1d8 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x1d9 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x1da => age_p1da(cp as u8),
        #[cfg(feature = "full")]
        0x1df => age_p1df(cp as u8),
        #[cfg(feature = "full")]
        0x1e0 => age_p1e0(cp as u8),
        #[cfg(feature = "full")]
        0x1e1 => age_p1e1(cp as u8),
        #[cfg(feature = "full")]
        0x1e2 => age_p1e2(cp as u8),
        #[cfg(feature = "full")]
        0x1e4 => age_p1e4(cp as u8),
        #[cfg(feature = "full")]
        0x1e5 => age_p1e5(cp as u8),
        #[cfg(feature = "full")]
        0x1e6 => age_p1e6(cp as u8),
        #[cfg(feature = "full")]
        0x1e7 => age_p1e7(cp as u8),
        #[cfg(feature = "full")]
        0x1e8 => age_p1e8(cp as u8),
        #[cfg(feature = "full")]
        0x1e9 => age_p1e9(cp as u8),
        #[cfg(feature = "full")]
        0x1ec => age_p1ec(cp as u8),
        #[cfg(feature = "full")]
        0x1ed => age_p1ed(cp as u8),
        #[cfg(feature = "full")]
        0x1ee => age_p1ee(cp as u8),
        #[cfg(feature = "full")]
        0x1f0 => age_p1f0(cp as u8),
        #[cfg(feature = "full")]
        0x1f1 => age_p1f1(cp as u8),
        #[cfg(feature = "full")]
        0x1f2 => age_p1f2(cp as u8),
        #[cfg(feature = "full")]
        0x1f3 => age_p1f3(cp as u8),
        #[cfg(feature = "full")]
        0x1f4 => age_p1f4(cp as u8),
        #[cfg(feature = "full")]
        0x1f5 => age_p1f5(cp as u8),
        #[cfg(feature = "full")]
        0x1f6 => age_p1f6(cp as u8),
        #[cfg(feature = "full")]
        0x1f7 => age_p1f7(cp as u8),
        #[cfg(feature = "full")]
        0x1f8 => age_p1f8(cp as u8),
        #[cfg(feature = "full")]
        0x1f9 => age_p1f9(cp as u8),
        #[cfg(feature = "full")]
        0x1fa => age_p1fa(cp as u8),
        #[cfg(feature = "full")]
        0x1fb => age_p1fb(cp as u8),
        #[cfg(feature = "full")]
        0x1ff => age_p1ff(cp as u8),
        #[cfg(feature = "full")]
        0x200 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x201 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x202 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x203 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x204 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x205 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x206 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x207 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x208 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x209 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x20f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x210 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x211 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x212 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x213 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x214 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x215 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x216 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x217 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x218 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x219 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x21f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x220 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x221 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x222 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x223 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x224 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x225 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x226 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x227 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x228 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x229 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x22f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x230 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x231 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x232 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x233 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x234 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x235 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x236 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x237 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x238 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x239 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x23f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x240 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x241 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x242 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x243 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x244 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x245 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x246 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x247 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x248 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x249 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x24f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x250 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x251 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x252 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x253 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x254 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x255 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x256 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x257 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x258 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x259 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x25f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x260 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x261 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x262 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x263 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x264 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x265 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x266 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x267 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x268 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x269 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x26f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x270 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x271 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x272 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x273 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x274 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x275 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x276 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x277 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x278 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x279 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x27f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x280 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x281 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x282 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x283 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x284 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x285 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x286 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x287 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x288 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x289 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x28f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x290 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x291 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x292 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x293 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x294 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x295 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x296 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x297 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x298 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x299 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29a => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29b => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29c => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29d => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29e => Some((3, 1)),
        #[cfg(feature = "full")]
        0x29f => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a0 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a1 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a2 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a3 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a4 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a5 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2a6 => age_p2a6(cp as u8),
        #[cfg(feature = "full")]
        0x2a7 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2a8 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2a9 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2aa => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2ab => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2ac => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2ad => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2ae => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2af => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b0 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b1 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b2 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b3 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b4 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b5 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b6 => Some((5, 2)),
        #[cfg(feature = "full")]
        0x2b7 => age_p2b7(cp as u8),
        #[cfg(feature = "full")]
        0x2b8 => age_p2b8(cp as u8),
        #[cfg(feature = "full")]
        0x2b9 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2ba => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2bb => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2bc => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2bd => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2be => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2bf => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c0 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c1 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c2 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c3 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c4 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c5 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c6 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c7 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c8 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2c9 => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2ca => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2cb => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2cc => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2cd => Some((8, 0)),
        #[cfg(feature = "full")]
        0x2ce => age_p2ce(cp as u8),
        #[cfg(feature = "full")]
        0x2cf => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d0 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d1 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d2 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d3 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d4 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d5 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d6 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d7 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d8 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2d9 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2da => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2db => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2dc => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2dd => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2de => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2df => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e0 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e1 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e2 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e3 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e4 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e5 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e6 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e7 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e8 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2e9 => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2ea => Some((10, 0)),
        #[cfg(feature = "full")]
        0x2eb => age_p2eb(cp as u8),
        #[cfg(feature = "full")]
        0x2ec => Some((15, 1)),
        #[cfg(feature = "full")]
        0x2ed => Some((15, 1)),
        #[cfg(feature = "full")]
        0x2ee => age_p2ee(cp as u8),
        #[cfg(feature = "full")]
        0x2f8 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2f9 => Some((3, 1)),
        #[cfg(feature = "full")]
        0x2fa => age_p2fa(cp as u8),
        #[cfg(feature = "full")]
        0x2ff => age_p2ff(cp as u8),
        #[cfg(feature = "full")]
        0x300 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x301 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x302 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x303 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x304 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x305 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x306 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x307 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x308 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x309 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30a => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30b => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30c => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30d => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30e => Some((13, 0)),
        #[cfg(feature = "full")]
        0x30f => Some((13, 0)),
        #[cfg(feature = "full")]
        0x310 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x311 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x312 => Some((13, 0)),
        #[cfg(feature = "full")]
        0x313 => age_p313(cp as u8),
        #[cfg(feature = "full")]
        0x314 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x315 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x316 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x317 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x318 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x319 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31a => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31b => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31c => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31d => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31e => Some((15, 0)),
        #[cfg(feature = "full")]
        0x31f => Some((15, 0)),
        #[cfg(feature = "full")]
        0x320 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x321 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x322 => Some((15, 0)),
        #[cfg(feature = "full")]
        0x323 => age_p323(cp as u8),
        #[cfg(feature = "full")]
        0x324 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x325 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x326 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x327 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x328 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x329 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32a => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32b => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32c => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32d => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32e => Some((17, 0)),
        #[cfg(feature = "full")]
        0x32f => Some((17, 0)),
        #[cfg(feature = "full")]
        0x330 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x331 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x332 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x333 => Some((17, 0)),
        #[cfg(feature = "full")]
        0x334 => age_p334(cp as u8),
        #[cfg(feature = "full")]
        0x3ff => age_p3ff(cp as u8),
        #[cfg(feature = "full")]
        0x4ff => age_p4ff(cp as u8),
        #[cfg(feature = "full")]
        0x5ff => age_p5ff(cp as u8),
        #[cfg(feature = "full")]
        0x6ff => age_p6ff(cp as u8),
        #[cfg(feature = "full")]
        0x7ff => age_p7ff(cp as u8),
        #[cfg(feature = "full")]
        0x8ff => age_p8ff(cp as u8),
        #[cfg(feature = "full")]
        0x9ff => age_p9ff(cp as u8),
        #[cfg(feature = "full")]
        0xaff => age_paff(cp as u8),
        #[cfg(feature = "full")]
        0xbff => age_pbff(cp as u8),
        #[cfg(feature = "full")]
        0xcff => age_pcff(cp as u8),
        #[cfg(feature = "full")]
        0xdff => age_pdff(cp as u8),
        #[cfg(feature = "full")]
        0xe00 => age_pe00(cp as u8),
        #[cfg(feature = "full")]
        0xe01 => age_pe01(cp as u8),
        #[cfg(feature = "full")]
        0xeff => age_peff(cp as u8),
        #[cfg(feature = "full")]
        0xf00 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf01 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf02 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf03 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf04 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf05 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf06 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf07 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf08 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf09 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf0f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf10 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf11 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf12 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf13 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf14 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf15 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf16 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf17 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf18 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf19 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf1f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf20 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf21 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf22 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf23 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf24 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf25 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf26 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf27 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf28 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf29 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf2f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf30 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf31 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf32 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf33 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf34 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf35 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf36 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf37 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf38 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf39 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf3f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf40 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf41 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf42 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf43 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf44 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf45 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf46 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf47 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf48 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf49 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf4f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf50 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf51 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf52 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf53 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf54 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf55 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf56 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf57 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf58 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf59 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf5f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf60 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf61 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf62 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf63 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf64 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf65 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf66 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf67 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf68 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf69 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf6f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf70 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf71 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf72 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf73 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf74 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf75 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf76 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf77 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf78 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf79 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf7f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf80 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf81 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf82 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf83 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf84 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf85 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf86 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf87 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf88 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf89 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf8f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf90 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf91 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf92 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf93 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf94 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf95 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf96 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf97 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf98 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf99 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9a => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9b => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9c => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9d => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9e => Some((2, 0)),
        #[cfg(feature = "full")]
        0xf9f => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfa9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfaa => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfab => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfac => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfad => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfae => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfaf => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfb9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfba => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfbb => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfbc => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfbd => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfbe => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfbf => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfc9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfca => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfcb => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfcc => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfcd => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfce => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfcf => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfd9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfda => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfdb => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfdc => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfdd => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfde => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfdf => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfe9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfea => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfeb => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfec => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfed => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfee => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfef => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xff9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0xffa => Some((2, 0)),
        #[cfg(feature = "full")]
        0xffb => Some((2, 0)),
        #[cfg(feature = "full")]
        0xffc => Some((2, 0)),
        #[cfg(feature = "full")]
        0xffd => Some((2, 0)),
        #[cfg(feature = "full")]
        0xffe => Some((2, 0)),
        #[cfg(feature = "full")]
        0xfff => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1000 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1001 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1002 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1003 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1004 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1005 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1006 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1007 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1008 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1009 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x100f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1010 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1011 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1012 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1013 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1014 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1015 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1016 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1017 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1018 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1019 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x101f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1020 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1021 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1022 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1023 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1024 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1025 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1026 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1027 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1028 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1029 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x102f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1030 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1031 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1032 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1033 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1034 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1035 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1036 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1037 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1038 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1039 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x103f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1040 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1041 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1042 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1043 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1044 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1045 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1046 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1047 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1048 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1049 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x104f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1050 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1051 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1052 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1053 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1054 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1055 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1056 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1057 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1058 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1059 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x105f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1060 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1061 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1062 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1063 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1064 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1065 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1066 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1067 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1068 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1069 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x106f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1070 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1071 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1072 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1073 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1074 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1075 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1076 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1077 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1078 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1079 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x107f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1080 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1081 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1082 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1083 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1084 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1085 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1086 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1087 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1088 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1089 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x108f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1090 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1091 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1092 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1093 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1094 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1095 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1096 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1097 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1098 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x1099 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109a => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109b => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109c => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109d => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109e => Some((2, 0)),
        #[cfg(feature = "full")]
        0x109f => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10a9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10aa => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ab => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ac => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ad => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ae => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10af => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10b9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ba => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10bb => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10bc => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10bd => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10be => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10bf => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10c9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ca => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10cb => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10cc => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10cd => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ce => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10cf => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10d9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10da => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10db => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10dc => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10dd => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10de => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10df => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10e9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ea => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10eb => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ec => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ed => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ee => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ef => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f0 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f1 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f2 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f3 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f4 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f5 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f6 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f7 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f8 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10f9 => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10fa => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10fb => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10fc => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10fd => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10fe => Some((2, 0)),
        #[cfg(feature = "full")]
        0x10ff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "ascii")]
const fn age_p0(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x7f => Some((1, 1)),
        #[cfg(feature = "latin1")]
        0x80..=0xff => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xf5 => Some((1, 1)),
        0xf6..=0xf9 => Some((3, 0)),
        0xfa..=0xff => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x17 => Some((1, 1)),
        0x18..=0x1f => Some((3, 0)),
        0x20 => Some((3, 2)),
        0x21 => Some((4, 0)),
        0x22..=0x33 => Some((3, 0)),
        0x34..=0x36 => Some((4, 0)),
        0x37..=0x41 => Some((4, 1)),
        0x42..=0x4f => Some((5, 0)),
        0x50..=0xa8 => Some((1, 1)),
        0xa9..=0xad => Some((3, 0)),
        0xae..=0xaf => Some((4, 0)),
        0xb0..=0xde => Some((1, 1)),
        0xdf => Some((3, 0)),
        0xe0..=0xe9 => Some((1, 1)),
        0xea..=0xee => Some((3, 0)),
        0xef..=0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p3(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x45 => Some((1, 1)),
        0x46..=0x4e => Some((3, 0)),
        0x4f => Some((3, 2)),
        0x50..=0x57 => Some((4, 0)),
        0x58..=0x5c => Some((4, 1)),
        0x5d..=0x5f => Some((4, 0)),
        0x60..=0x61 => Some((1, 1)),
        0x62 => Some((3, 0)),
        0x63..=0x6f => Some((3, 2)),
        0x70..=0x73 => Some((5, 1)),
        0x74..=0x75 => Some((1, 1)),
        0x76..=0x77 => Some((5, 1)),
        0x7a => Some((1, 1)),
        0x7b..=0x7d => Some((5, 0)),
        0x7e => Some((1, 1)),
        0x7f => Some((7, 0)),
        0x84..=0x8a => Some((1, 1)),
        0x8c => Some((1, 1)),
        0x8e..=0xa1 => Some((1, 1)),
        0xa3..=0xce => Some((1, 1)),
        0xcf => Some((5, 1)),
        0xd0..=0xd6 => Some((1, 1)),
        0xd7 => Some((3, 0)),
        0xd8..=0xd9 => Some((3, 2)),
        0xda => Some((1, 1)),
        0xdb => Some((3, 0)),
        0xdc => Some((1, 1)),
        0xdd => Some((3, 0)),
        0xde => Some((1, 1)),
        0xdf => Some((3, 0)),
        0xe0 => Some((1, 1)),
        0xe1 => Some((3, 0)),
        0xe2..=0xf3 => Some((1, 1)),
        0xf4..=0xf5 => Some((3, 1)),
        0xf6 => Some((3, 2)),
        0xf7..=0xfb => Some((4, 0)),
        0xfc..=0xff => Some((4, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p4(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((3, 0)),
        0x01..=0x0c => Some((1, 1)),
        0x0d => Some((3, 0)),
        0x0e..=0x4f => Some((1, 1)),
        0x50 => Some((3, 0)),
        0x51..=0x5c => Some((1, 1)),
        0x5d => Some((3, 0)),
        0x5e..=0x86 => Some((1, 1)),
        0x87 => Some((5, 1)),
        0x88..=0x89 => Some((3, 0)),
        0x8a..=0x8b => Some((3, 2)),
        0x8c..=0x8f => Some((3, 0)),
        0x90..=0xc4 => Some((1, 1)),
        0xc5..=0xc6 => Some((3, 2)),
        0xc7..=0xc8 => Some((1, 1)),
        0xc9..=0xca => Some((3, 2)),
        0xcb..=0xcc => Some((1, 1)),
        0xcd..=0xce => Some((3, 2)),
        0xcf => Some((5, 0)),
        0xd0..=0xeb => Some((1, 1)),
        0xec..=0xed => Some((3, 0)),
        0xee..=0xf5 => Some((1, 1)),
        0xf6..=0xf7 => Some((4, 1)),
        0xf8..=0xf9 => Some((1, 1)),
        0xfa..=0xff => Some((5, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p5(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0f => Some((3, 2)),
        0x10..=0x13 => Some((5, 0)),
        0x14..=0x23 => Some((5, 1)),
        0x24..=0x25 => Some((5, 2)),
        0x26..=0x27 => Some((6, 0)),
        0x28..=0x2f => Some((7, 0)),
        0x31..=0x56 => Some((1, 1)),
        0x59..=0x5f => Some((1, 1)),
        0x60 => Some((11, 0)),
        0x61..=0x87 => Some((1, 1)),
        0x88 => Some((11, 0)),
        0x89 => Some((1, 1)),
        0x8a => Some((3, 0)),
        0x8d..=0x8e => Some((7, 0)),
        0x8f => Some((6, 1)),
        0x91..=0xa1 => Some((2, 0)),
        0xa2 => Some((4, 1)),
        0xa3..=0xaf => Some((2, 0)),
        0xb0..=0xb9 => Some((1, 1)),
        0xba => Some((5, 0)),
        0xbb..=0xc3 => Some((1, 1)),
        0xc4 => Some((2, 0)),
        0xc5..=0xc7 => Some((4, 1)),
        0xd0..=0xea => Some((1, 1)),
        0xef => Some((11, 0)),
        0xf0..=0xf4 => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p6(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x03 => Some((4, 0)),
        0x04 => Some((6, 1)),
        0x05 => Some((7, 0)),
        0x06..=0x0a => Some((5, 1)),
        0x0b => Some((4, 1)),
        0x0c => Some((1, 1)),
        0x0d..=0x15 => Some((4, 0)),
        0x16..=0x1a => Some((5, 1)),
        0x1b => Some((1, 1)),
        0x1c => Some((6, 3)),
        0x1d => Some((14, 0)),
        0x1e => Some((4, 1)),
        0x1f => Some((1, 1)),
        0x20 => Some((6, 0)),
        0x21..=0x3a => Some((1, 1)),
        0x3b..=0x3f => Some((5, 1)),
        0x40..=0x52 => Some((1, 1)),
        0x53..=0x55 => Some((3, 0)),
        0x56..=0x58 => Some((4, 0)),
        0x59..=0x5e => Some((4, 1)),
        0x5f => Some((6, 0)),
        0x60..=0x6d => Some((1, 1)),
        0x6e..=0x6f => Some((3, 2)),
        0x70..=0xb7 => Some((1, 1)),
        0xb8..=0xb9 => Some((3, 0)),
        0xba..=0xbe => Some((1, 1)),
        0xbf => Some((3, 0)),
        0xc0..=0xce => Some((1, 1)),
        0xcf => Some((3, 0)),
        0xd0..=0xed => Some((1, 1)),
        0xee..=0xef => Some((4, 0)),
        0xf0..=0xf9 => Some((1, 1)),
        0xfa..=0xfe => Some((3, 0)),
        0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0d => Some((3, 0)),
        0x0f..=0x2c => Some((3, 0)),
        0x2d..=0x2f => Some((4, 0)),
        0x30..=0x4a => Some((3, 0)),
        0x4d..=0x4f => Some((4, 0)),
        0x50..=0x6d => Some((4, 1)),
        0x6e..=0x7f => Some((5, 1)),
        0x80..=0xb0 => Some((3, 0)),
        0xb1 => Some((3, 2)),
        0xc0..=0xfa => Some((5, 0)),
        0xfd..=0xff => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p8(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2d => Some((5, 2)),
        0x30..=0x3e => Some((5, 2)),
        0x40..=0x5b => Some((6, 0)),
        0x5e => Some((6, 0)),
        0x60..=0x6a => Some((10, 0)),
        0x70..=0x8e => Some((14, 0)),
        0x8f => Some((17, 0)),
        0x90..=0x91 => Some((14, 0)),
        0x97 => Some((16, 0)),
        0x98..=0x9f => Some((14, 0)),
        0xa0 => Some((6, 1)),
        0xa1 => Some((7, 0)),
        0xa2..=0xac => Some((6, 1)),
        0xad..=0xb2 => Some((7, 0)),
        0xb3..=0xb4 => Some((8, 0)),
        0xb5 => Some((14, 0)),
        0xb6..=0xbd => Some((9, 0)),
        0xbe..=0xc7 => Some((13, 0)),
        0xc8..=0xd2 => Some((14, 0)),
        0xd3 => Some((11, 0)),
        0xd4..=0xe2 => Some((9, 0)),
        0xe3 => Some((8, 0)),
        0xe4..=0xfe => Some((6, 1)),
        0xff => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p9(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((5, 2)),
        0x01..=0x03 => Some((1, 1)),
        0x04 => Some((4, 0)),
        0x05..=0x39 => Some((1, 1)),
        0x3a..=0x3b => Some((6, 0)),
        0x3c..=0x4d => Some((1, 1)),
        0x4e => Some((5, 2)),
        0x4f => Some((6, 0)),
        0x50..=0x54 => Some((1, 1)),
        0x55 => Some((5, 2)),
        0x56..=0x57 => Some((6, 0)),
        0x58..=0x70 => Some((1, 1)),
        0x71..=0x72 => Some((5, 1)),
        0x73..=0x77 => Some((6, 0)),
        0x78 => Some((7, 0)),
        0x79..=0x7a => Some((5, 2)),
        0x7b..=0x7c => Some((5, 0)),
        0x7d => Some((4, 1)),
        0x7e..=0x7f => Some((5, 0)),
        0x80 => Some((7, 0)),
        0x81..=0x83 => Some((1, 1)),
        0x85..=0x8c => Some((1, 1)),
        0x8f..=0x90 => Some((1, 1)),
        0x93..=0xa8 => Some((1, 1)),
        0xaa..=0xb0 => Some((1, 1)),
        0xb2 => Some((1, 1)),
        0xb6..=0xb9 => Some((1, 1)),
        0xbc => Some((1, 1)),
        0xbd => Some((4, 0)),
        0xbe..=0xc4 => Some((1, 1)),
        0xc7..=0xc8 => Some((1, 1)),
        0xcb..=0xcd => Some((1, 1)),
        0xce => Some((4, 1)),
        0xd7 => Some((1, 1)),
        0xdc..=0xdd => Some((1, 1)),
        0xdf..=0xe3 => Some((1, 1)),
        0xe6..=0xfa => Some((1, 1)),
        0xfb => Some((5, 2)),
        0xfc..=0xfd => Some((10, 0)),
        0xfe => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01 => Some((4, 0)),
        0x02 => Some((1, 1)),
        0x03 => Some((4, 0)),
        0x05..=0x0a => Some((1, 1)),
        0x0f..=0x10 => Some((1, 1)),
        0x13..=0x28 => Some((1, 1)),
        0x2a..=0x30 => Some((1, 1)),
        0x32..=0x33 => Some((1, 1)),
        0x35..=0x36 => Some((1, 1)),
        0x38..=0x39 => Some((1, 1)),
        0x3c => Some((1, 1)),
        0x3e..=0x42 => Some((1, 1)),
        0x47..=0x48 => Some((1, 1)),
        0x4b..=0x4d => Some((1, 1)),
        0x51 => Some((5, 1)),
        0x59..=0x5c => Some((1, 1)),
        0x5e => Some((1, 1)),
        0x66..=0x74 => Some((1, 1)),
        0x75 => Some((5, 1)),
        0x76 => Some((11, 0)),
        0x81..=0x83 => Some((1, 1)),
        0x85..=0x8b => Some((1, 1)),
        0x8c => Some((4, 0)),
        0x8d => Some((1, 1)),
        0x8f..=0x91 => Some((1, 1)),
        0x93..=0xa8 => Some((1, 1)),
        0xaa..=0xb0 => Some((1, 1)),
        0xb2..=0xb3 => Some((1, 1)),
        0xb5..=0xb9 => Some((1, 1)),
        0xbc..=0xc5 => Some((1, 1)),
        0xc7..=0xc9 => Some((1, 1)),
        0xcb..=0xcd => Some((1, 1)),
        0xd0 => Some((1, 1)),
        0xe0 => Some((1, 1)),
        0xe1..=0xe3 => Some((4, 0)),
        0xe6..=0xef => Some((1, 1)),
        0xf0 => Some((6, 1)),
        0xf1 => Some((4, 0)),
        0xf9 => Some((8, 0)),
        0xfa..=0xff => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pb(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01..=0x03 => Some((1, 1)),
        0x05..=0x0c => Some((1, 1)),
        0x0f..=0x10 => Some((1, 1)),
        0x13..=0x28 => Some((1, 1)),
        0x2a..=0x30 => Some((1, 1)),
        0x32..=0x33 => Some((1, 1)),
        0x35 => Some((4, 0)),
        0x36..=0x39 => Some((1, 1)),
        0x3c..=0x43 => Some((1, 1)),
        0x44 => Some((5, 1)),
        0x47..=0x48 => Some((1, 1)),
        0x4b..=0x4d => Some((1, 1)),
        0x55 => Some((13, 0)),
        0x56..=0x57 => Some((1, 1)),
        0x5c..=0x5d => Some((1, 1)),
        0x5f..=0x61 => Some((1, 1)),
        0x62..=0x63 => Some((5, 1)),
        0x66..=0x70 => Some((1, 1)),
        0x71 => Some((4, 0)),
        0x72..=0x77 => Some((6, 0)),
        0x82..=0x83 => Some((1, 1)),
        0x85..=0x8a => Some((1, 1)),
        0x8e..=0x90 => Some((1, 1)),
        0x92..=0x95 => Some((1, 1)),
        0x99..=0x9a => Some((1, 1)),
        0x9c => Some((1, 1)),
        0x9e..=0x9f => Some((1, 1)),
        0xa3..=0xa4 => Some((1, 1)),
        0xa8..=0xaa => Some((1, 1)),
        0xae..=0xb5 => Some((1, 1)),
        0xb6 => Some((4, 1)),
        0xb7..=0xb9 => Some((1, 1)),
        0xbe..=0xc2 => Some((1, 1)),
        0xc6..=0xc8 => Some((1, 1)),
        0xca..=0xcd => Some((1, 1)),
        0xd0 => Some((5, 1)),
        0xd7 => Some((1, 1)),
        0xe6 => Some((4, 1)),
        0xe7..=0xf2 => Some((1, 1)),
        0xf3..=0xfa => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pc(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((7, 0)),
        0x01..=0x03 => Some((1, 1)),
        0x04 => Some((11, 0)),
        0x05..=0x0c => Some((1, 1)),
        0x0e..=0x10 => Some((1, 1)),
        0x12..=0x28 => Some((1, 1)),
        0x2a..=0x33 => Some((1, 1)),
        0x34 => Some((7, 0)),
        0x35..=0x39 => Some((1, 1)),
        0x3c => Some((14, 0)),
        0x3d => Some((5, 1)),
        0x3e..=0x44 => Some((1, 1)),
        0x46..=0x48 => Some((1, 1)),
        0x4a..=0x4d => Some((1, 1)),
        0x55..=0x56 => Some((1, 1)),
        0x58..=0x59 => Some((5, 1)),
        0x5a => Some((8, 0)),
        0x5c => Some((17, 0)),
        0x5d => Some((14, 0)),
        0x60..=0x61 => Some((1, 1)),
        0x62..=0x63 => Some((5, 1)),
        0x66..=0x6f => Some((1, 1)),
        0x77 => Some((12, 0)),
        0x78..=0x7f => Some((5, 1)),
        0x80 => Some((9, 0)),
        0x81 => Some((7, 0)),
        0x82..=0x83 => Some((1, 1)),
        0x84 => Some((11, 0)),
        0x85..=0x8c => Some((1, 1)),
        0x8e..=0x90 => Some((1, 1)),
        0x92..=0xa8 => Some((1, 1)),
        0xaa..=0xb3 => Some((1, 1)),
        0xb5..=0xb9 => Some((1, 1)),
        0xbc..=0xbd => Some((4, 0)),
        0xbe..=0xc4 => Some((1, 1)),
        0xc6..=0xc8 => Some((1, 1)),
        0xca..=0xcd => Some((1, 1)),
        0xd5..=0xd6 => Some((1, 1)),
        0xdc => Some((17, 0)),
        0xdd => Some((14, 0)),
        0xde => Some((1, 1)),
        0xe0..=0xe1 => Some((1, 1)),
        0xe2..=0xe3 => Some((5, 0)),
        0xe6..=0xef => Some((1, 1)),
        0xf1..=0xf2 => Some((5, 0)),
        0xf3 => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pd(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((10, 0)),
        0x01 => Some((7, 0)),
        0x02..=0x03 => Some((1, 1)),
        0x04 => Some((13, 0)),
        0x05..=0x0c => Some((1, 1)),
        0x0e..=0x10 => Some((1, 1)),
        0x12..=0x28 => Some((1, 1)),
        0x29 => Some((6, 0)),
        0x2a..=0x39 => Some((1, 1)),
        0x3a => Some((6, 0)),
        0x3b..=0x3c => Some((10, 0)),
        0x3d => Some((5, 1)),
        0x3e..=0x43 => Some((1, 1)),
        0x44 => Some((5, 1)),
        0x46..=0x48 => Some((1, 1)),
        0x4a..=0x4d => Some((1, 1)),
        0x4e => Some((6, 0)),
        0x4f => Some((9, 0)),
        0x54..=0x56 => Some((9, 0)),
        0x57 => Some((1, 1)),
        0x58..=0x5e => Some((9, 0)),
        0x5f => Some((8, 0)),
        0x60..=0x61 => Some((1, 1)),
        0x62..=0x63 => Some((5, 1)),
        0x66..=0x6f => Some((1, 1)),
        0x70..=0x75 => Some((5, 1)),
        0x76..=0x78 => Some((9, 0)),
        0x79..=0x7f => Some((5, 1)),
        0x81 => Some((13, 0)),
        0x82..=0x83 => Some((3, 0)),
        0x85..=0x96 => Some((3, 0)),
        0x9a..=0xb1 => Some((3, 0)),
        0xb3..=0xbb => Some((3, 0)),
        0xbd => Some((3, 0)),
        0xc0..=0xc6 => Some((3, 0)),
        0xca => Some((3, 0)),
        0xcf..=0xd4 => Some((3, 0)),
        0xd6 => Some((3, 0)),
        0xd8..=0xdf => Some((3, 0)),
        0xe6..=0xef => Some((7, 0)),
        0xf2..=0xf4 => Some((3, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pe(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01..=0x3a => Some((1, 1)),
        0x3f..=0x5b => Some((1, 1)),
        0x81..=0x82 => Some((1, 1)),
        0x84 => Some((1, 1)),
        0x86 => Some((12, 0)),
        0x87..=0x88 => Some((1, 1)),
        0x89 => Some((12, 0)),
        0x8a => Some((1, 1)),
        0x8c => Some((12, 0)),
        0x8d => Some((1, 1)),
        0x8e..=0x93 => Some((12, 0)),
        0x94..=0x97 => Some((1, 1)),
        0x98 => Some((12, 0)),
        0x99..=0x9f => Some((1, 1)),
        0xa0 => Some((12, 0)),
        0xa1..=0xa3 => Some((1, 1)),
        0xa5 => Some((1, 1)),
        0xa7 => Some((1, 1)),
        0xa8..=0xa9 => Some((12, 0)),
        0xaa..=0xab => Some((1, 1)),
        0xac => Some((12, 0)),
        0xad..=0xb9 => Some((1, 1)),
        0xba => Some((12, 0)),
        0xbb..=0xbd => Some((1, 1)),
        0xc0..=0xc4 => Some((1, 1)),
        0xc6 => Some((1, 1)),
        0xc8..=0xcd => Some((1, 1)),
        0xce => Some((15, 0)),
        0xd0..=0xd9 => Some((1, 1)),
        0xdc..=0xdd => Some((1, 1)),
        0xde..=0xdf => Some((6, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pf(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x47 => Some((2, 0)),
        0x49..=0x69 => Some((2, 0)),
        0x6a => Some((3, 0)),
        0x6b..=0x6c => Some((5, 1)),
        0x71..=0x8b => Some((2, 0)),
        0x8c..=0x8f => Some((6, 0)),
        0x90..=0x95 => Some((2, 0)),
        0x96 => Some((3, 0)),
        0x97 => Some((2, 0)),
        0x99..=0xad => Some((2, 0)),
        0xae..=0xb0 => Some((3, 0)),
        0xb1..=0xb7 => Some((2, 0)),
        0xb8 => Some((3, 0)),
        0xb9 => Some((2, 0)),
        0xba..=0xbc => Some((3, 0)),
        0xbe..=0xcc => Some((3, 0)),
        0xce => Some((5, 1)),
        0xcf => Some((3, 0)),
        0xd0..=0xd1 => Some((4, 1)),
        0xd2..=0xd4 => Some((5, 1)),
        0xd5..=0xd8 => Some((5, 2)),
        0xd9..=0xda => Some((6, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p10(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x21 => Some((3, 0)),
        0x22 => Some((5, 1)),
        0x23..=0x27 => Some((3, 0)),
        0x28 => Some((5, 1)),
        0x29..=0x2a => Some((3, 0)),
        0x2b => Some((5, 1)),
        0x2c..=0x32 => Some((3, 0)),
        0x33..=0x35 => Some((5, 1)),
        0x36..=0x39 => Some((3, 0)),
        0x3a..=0x3f => Some((5, 1)),
        0x40..=0x59 => Some((3, 0)),
        0x5a..=0x99 => Some((5, 1)),
        0x9a..=0x9d => Some((5, 2)),
        0x9e..=0x9f => Some((5, 1)),
        0xa0..=0xc5 => Some((1, 1)),
        0xc7 => Some((6, 1)),
        0xcd => Some((6, 1)),
        0xd0..=0xf6 => Some((1, 1)),
        0xf7..=0xf8 => Some((3, 2)),
        0xf9..=0xfa => Some((4, 1)),
        0xfb => Some((1, 1)),
        0xfc => Some((4, 1)),
        0xfd..=0xff => Some((6, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p11(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x59 => Some((1, 1)),
        0x5a..=0x5e => Some((5, 2)),
        0x5f..=0xa2 => Some((1, 1)),
        0xa3..=0xa7 => Some((5, 2)),
        0xa8..=0xf9 => Some((1, 1)),
        0xfa..=0xff => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p12(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x06 => Some((3, 0)),
        0x07 => Some((4, 1)),
        0x08..=0x46 => Some((3, 0)),
        0x47 => Some((4, 1)),
        0x48 => Some((3, 0)),
        0x4a..=0x4d => Some((3, 0)),
        0x50..=0x56 => Some((3, 0)),
        0x58 => Some((3, 0)),
        0x5a..=0x5d => Some((3, 0)),
        0x60..=0x86 => Some((3, 0)),
        0x87 => Some((4, 1)),
        0x88 => Some((3, 0)),
        0x8a..=0x8d => Some((3, 0)),
        0x90..=0xae => Some((3, 0)),
        0xaf => Some((4, 1)),
        0xb0 => Some((3, 0)),
        0xb2..=0xb5 => Some((3, 0)),
        0xb8..=0xbe => Some((3, 0)),
        0xc0 => Some((3, 0)),
        0xc2..=0xc5 => Some((3, 0)),
        0xc8..=0xce => Some((3, 0)),
        0xcf => Some((4, 1)),
        0xd0..=0xd6 => Some((3, 0)),
        0xd8..=0xee => Some((3, 0)),
        0xef => Some((4, 1)),
        0xf0..=0xff => Some((3, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p13(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0e => Some((3, 0)),
        0x0f => Some((4, 1)),
        0x10 => Some((3, 0)),
        0x12..=0x15 => Some((3, 0)),
        0x18..=0x1e => Some((3, 0)),
        0x1f => Some((4, 1)),
        0x20..=0x46 => Some((3, 0)),
        0x47 => Some((4, 1)),
        0x48..=0x5a => Some((3, 0)),
        0x5d..=0x5e => Some((6, 0)),
        0x5f..=0x60 => Some((4, 1)),
        0x61..=0x7c => Some((3, 0)),
        0x80..=0x99 => Some((4, 1)),
        0xa0..=0xf4 => Some((3, 0)),
        0xf5 => Some((8, 0)),
        0xf8..=0xfd => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p14(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((5, 2)),
        0x01..=0xff => Some((3, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p16(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x76 => Some((3, 0)),
        0x77..=0x7f => Some((5, 2)),
        0x80..=0x9c => Some((3, 0)),
        0xa0..=0xf0 => Some((3, 0)),
        0xf1..=0xf8 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p17(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0c => Some((3, 2)),
        0x0d => Some((14, 0)),
        0x0e..=0x14 => Some((3, 2)),
        0x15 => Some((14, 0)),
        0x1f => Some((14, 0)),
        0x20..=0x36 => Some((3, 2)),
        0x40..=0x53 => Some((3, 2)),
        0x60..=0x6c => Some((3, 2)),
        0x6e..=0x70 => Some((3, 2)),
        0x72..=0x73 => Some((3, 2)),
        0x80..=0xdc => Some((3, 0)),
        0xdd => Some((4, 0)),
        0xe0..=0xe9 => Some((3, 0)),
        0xf0..=0xf9 => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p18(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0e => Some((3, 0)),
        0x0f => Some((14, 0)),
        0x10..=0x19 => Some((3, 0)),
        0x20..=0x77 => Some((3, 0)),
        0x78 => Some((11, 0)),
        0x80..=0xa9 => Some((3, 0)),
        0xaa => Some((5, 1)),
        0xb0..=0xf5 => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p19(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1c => Some((4, 0)),
        0x1d..=0x1e => Some((7, 0)),
        0x20..=0x2b => Some((4, 0)),
        0x30..=0x3b => Some((4, 0)),
        0x40 => Some((4, 0)),
        0x44..=0x6d => Some((4, 0)),
        0x70..=0x74 => Some((4, 0)),
        0x80..=0xa9 => Some((4, 1)),
        0xaa..=0xab => Some((5, 2)),
        0xb0..=0xc9 => Some((4, 1)),
        0xd0..=0xd9 => Some((4, 1)),
        0xda => Some((5, 2)),
        0xde..=0xdf => Some((4, 1)),
        0xe0..=0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1a(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1b => Some((4, 1)),
        0x1e..=0x1f => Some((4, 1)),
        0x20..=0x5e => Some((5, 2)),
        0x60..=0x7c => Some((5, 2)),
        0x7f..=0x89 => Some((5, 2)),
        0x90..=0x99 => Some((5, 2)),
        0xa0..=0xad => Some((5, 2)),
        0xb0..=0xbe => Some((7, 0)),
        0xbf..=0xc0 => Some((13, 0)),
        0xc1..=0xce => Some((14, 0)),
        0xcf..=0xdd => Some((17, 0)),
        0xe0..=0xeb => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1b(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x4b => Some((5, 0)),
        0x4c => Some((14, 0)),
        0x4e..=0x4f => Some((16, 0)),
        0x50..=0x7c => Some((5, 0)),
        0x7d..=0x7e => Some((14, 0)),
        0x7f => Some((16, 0)),
        0x80..=0xaa => Some((5, 1)),
        0xab..=0xad => Some((6, 1)),
        0xae..=0xb9 => Some((5, 1)),
        0xba..=0xbf => Some((6, 1)),
        0xc0..=0xf3 => Some((6, 0)),
        0xfc..=0xff => Some((6, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1c(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x37 => Some((5, 1)),
        0x3b..=0x49 => Some((5, 1)),
        0x4d..=0x7f => Some((5, 1)),
        0x80..=0x88 => Some((9, 0)),
        0x89..=0x8a => Some((16, 0)),
        0x90..=0xba => Some((11, 0)),
        0xbd..=0xbf => Some((11, 0)),
        0xc0..=0xc7 => Some((6, 1)),
        0xd0..=0xf2 => Some((5, 2)),
        0xf3..=0xf6 => Some((6, 1)),
        0xf7 => Some((10, 0)),
        0xf8..=0xf9 => Some((7, 0)),
        0xfa => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x6b => Some((4, 0)),
        0x6c..=0xc3 => Some((4, 1)),
        0xc4..=0xca => Some((5, 0)),
        0xcb..=0xe6 => Some((5, 1)),
        0xe7..=0xf5 => Some((7, 0)),
        0xf6..=0xf9 => Some((10, 0)),
        0xfa => Some((14, 0)),
        0xfb => Some((9, 0)),
        0xfc => Some((6, 0)),
        0xfd => Some((5, 2)),
        0xfe..=0xff => Some((5, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1e(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x9a => Some((1, 1)),
        0x9b => Some((2, 0)),
        0x9c..=0x9f => Some((5, 1)),
        0xa0..=0xf9 => Some((1, 1)),
        0xfa..=0xff => Some((5, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p1f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x15 => Some((1, 1)),
        0x18..=0x1d => Some((1, 1)),
        0x20..=0x45 => Some((1, 1)),
        0x48..=0x4d => Some((1, 1)),
        0x50..=0x57 => Some((1, 1)),
        0x59 => Some((1, 1)),
        0x5b => Some((1, 1)),
        0x5d => Some((1, 1)),
        0x5f..=0x7d => Some((1, 1)),
        0x80..=0xb4 => Some((1, 1)),
        0xb6..=0xc4 => Some((1, 1)),
        0xc6..=0xd3 => Some((1, 1)),
        0xd6..=0xdb => Some((1, 1)),
        0xdd..=0xef => Some((1, 1)),
        0xf2..=0xf4 => Some((1, 1)),
        0xf6..=0xfe => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p20(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2e => Some((1, 1)),
        0x2f => Some((3, 0)),
        0x30..=0x46 => Some((1, 1)),
        0x47 => Some((3, 2)),
        0x48..=0x4d => Some((3, 0)),
        0x4e..=0x52 => Some((3, 2)),
        0x53..=0x54 => Some((4, 0)),
        0x55..=0x56 => Some((4, 1)),
        0x57 => Some((3, 2)),
        0x58..=0x5e => Some((4, 1)),
        0x5f..=0x63 => Some((3, 2)),
        0x64 => Some((5, 1)),
        0x66..=0x69 => Some((6, 3)),
        0x6a..=0x70 => Some((1, 1)),
        0x71 => Some((3, 2)),
        0x74..=0x8e => Some((1, 1)),
        0x90..=0x94 => Some((4, 1)),
        0x95..=0x9c => Some((6, 0)),
        0xa0..=0xaa => Some((1, 1)),
        0xab => Some((2, 0)),
        0xac => Some((2, 1)),
        0xad..=0xaf => Some((3, 0)),
        0xb0..=0xb1 => Some((3, 2)),
        0xb2..=0xb5 => Some((4, 1)),
        0xb6..=0xb8 => Some((5, 2)),
        0xb9 => Some((6, 0)),
        0xba => Some((6, 2)),
        0xbb..=0xbd => Some((7, 0)),
        0xbe => Some((8, 0)),
        0xbf => Some((10, 0)),
        0xc0 => Some((14, 0)),
        0xc1 => Some((17, 0)),
        0xd0..=0xe1 => Some((1, 1)),
        0xe2..=0xe3 => Some((3, 0)),
        0xe4..=0xea => Some((3, 2)),
        0xeb => Some((4, 1)),
        0xec..=0xef => Some((5, 0)),
        0xf0 => Some((5, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p21(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x38 => Some((1, 1)),
        0x39..=0x3a => Some((3, 0)),
        0x3b => Some((4, 0)),
        0x3c => Some((4, 1)),
        0x3d..=0x4b => Some((3, 2)),
        0x4c => Some((4, 1)),
        0x4d..=0x4e => Some((5, 0)),
        0x4f => Some((5, 1)),
        0x50..=0x52 => Some((5, 2)),
        0x53..=0x82 => Some((1, 1)),
        0x83 => Some((3, 0)),
        0x84 => Some((5, 0)),
        0x85..=0x88 => Some((5, 1)),
        0x89 => Some((5, 2)),
        0x8a..=0x8b => Some((8, 0)),
        0x90..=0xea => Some((1, 1)),
        0xeb..=0xf3 => Some((3, 0)),
        0xf4..=0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p22(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xf1 => Some((1, 1)),
        0xf2..=0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p23(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((1, 1)),
        0x01 => Some((3, 0)),
        0x02..=0x7a => Some((1, 1)),
        0x7b => Some((3, 0)),
        0x7c => Some((3, 2)),
        0x7d..=0x9a => Some((3, 0)),
        0x9b..=0xce => Some((3, 2)),
        0xcf..=0xd0 => Some((4, 0)),
        0xd1..=0xdb => Some((4, 1)),
        0xdc..=0xe7 => Some((5, 0)),
        0xe8 => Some((5, 2)),
        0xe9..=0xf3 => Some((6, 0)),
        0xf4..=0xfa => Some((7, 0)),
        0xfb..=0xfe => Some((9, 0)),
        0xff => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p24(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x24 => Some((1, 1)),
        0x25..=0x26 => Some((3, 0)),
        0x27..=0x29 => Some((16, 0)),
        0x40..=0x4a => Some((1, 1)),
        0x60..=0xea => Some((1, 1)),
        0xeb..=0xfe => Some((3, 2)),
        0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p25(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x95 => Some((1, 1)),
        0x96..=0x9f => Some((3, 2)),
        0xa0..=0xef => Some((1, 1)),
        0xf0..=0xf7 => Some((3, 0)),
        0xf8..=0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p26(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x13 => Some((1, 1)),
        0x14..=0x15 => Some((4, 0)),
        0x16..=0x17 => Some((3, 2)),
        0x18 => Some((4, 1)),
        0x19 => Some((3, 0)),
        0x1a..=0x6f => Some((1, 1)),
        0x70..=0x71 => Some((3, 0)),
        0x72..=0x7d => Some((3, 2)),
        0x7e..=0x7f => Some((4, 1)),
        0x80..=0x89 => Some((3, 2)),
        0x8a..=0x91 => Some((4, 0)),
        0x92..=0x9c => Some((4, 1)),
        0x9d => Some((5, 1)),
        0x9e..=0x9f => Some((5, 2)),
        0xa0..=0xa1 => Some((4, 0)),
        0xa2..=0xb1 => Some((4, 1)),
        0xb2 => Some((5, 0)),
        0xb3..=0xbc => Some((5, 1)),
        0xbd..=0xbf => Some((5, 2)),
        0xc0..=0xc3 => Some((5, 1)),
        0xc4..=0xcd => Some((5, 2)),
        0xce => Some((6, 0)),
        0xcf..=0xe1 => Some((5, 2)),
        0xe2 => Some((6, 0)),
        0xe3 => Some((5, 2)),
        0xe4..=0xe7 => Some((6, 0)),
        0xe8..=0xff => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p27(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((7, 0)),
        0x01..=0x04 => Some((1, 1)),
        0x05 => Some((6, 0)),
        0x06..=0x09 => Some((1, 1)),
        0x0a..=0x0b => Some((6, 0)),
        0x0c..=0x27 => Some((1, 1)),
        0x28 => Some((6, 0)),
        0x29..=0x4b => Some((1, 1)),
        0x4c => Some((6, 0)),
        0x4d => Some((1, 1)),
        0x4e => Some((6, 0)),
        0x4f..=0x52 => Some((1, 1)),
        0x53..=0x55 => Some((6, 0)),
        0x56 => Some((1, 1)),
        0x57 => Some((5, 2)),
        0x58..=0x5e => Some((1, 1)),
        0x5f..=0x60 => Some((6, 0)),
        0x61..=0x67 => Some((1, 1)),
        0x68..=0x75 => Some((3, 2)),
        0x76..=0x94 => Some((1, 1)),
        0x95..=0x97 => Some((6, 0)),
        0x98..=0xaf => Some((1, 1)),
        0xb0 => Some((6, 0)),
        0xb1..=0xbe => Some((1, 1)),
        0xbf => Some((6, 0)),
        0xc0..=0xc6 => Some((4, 1)),
        0xc7..=0xca => Some((5, 0)),
        0xcb => Some((6, 1)),
        0xcc => Some((5, 1)),
        0xcd => Some((6, 1)),
        0xce..=0xcf => Some((6, 0)),
        0xd0..=0xeb => Some((3, 2)),
        0xec..=0xef => Some((5, 1)),
        0xf0..=0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2b(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0d => Some((4, 0)),
        0x0e..=0x13 => Some((4, 1)),
        0x14..=0x1a => Some((5, 0)),
        0x1b..=0x1f => Some((5, 1)),
        0x20..=0x23 => Some((5, 0)),
        0x24..=0x4c => Some((5, 1)),
        0x4d..=0x4f => Some((7, 0)),
        0x50..=0x54 => Some((5, 1)),
        0x55..=0x59 => Some((5, 2)),
        0x5a..=0x73 => Some((7, 0)),
        0x76..=0x95 => Some((7, 0)),
        0x96 => Some((17, 0)),
        0x97 => Some((13, 0)),
        0x98..=0xb9 => Some((7, 0)),
        0xba..=0xbc => Some((11, 0)),
        0xbd..=0xc8 => Some((7, 0)),
        0xc9 => Some((12, 0)),
        0xca..=0xd1 => Some((7, 0)),
        0xd2 => Some((10, 0)),
        0xd3..=0xeb => Some((11, 0)),
        0xec..=0xef => Some((8, 0)),
        0xf0..=0xfe => Some((11, 0)),
        0xff => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2c(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2e => Some((4, 1)),
        0x2f => Some((14, 0)),
        0x30..=0x5e => Some((4, 1)),
        0x5f => Some((14, 0)),
        0x60..=0x6c => Some((5, 0)),
        0x6d..=0x6f => Some((5, 1)),
        0x70 => Some((5, 2)),
        0x71..=0x73 => Some((5, 1)),
        0x74..=0x77 => Some((5, 0)),
        0x78..=0x7d => Some((5, 1)),
        0x7e..=0x7f => Some((5, 2)),
        0x80..=0xea => Some((4, 1)),
        0xeb..=0xf1 => Some((5, 2)),
        0xf2..=0xf3 => Some((6, 1)),
        0xf9..=0xff => Some((4, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x25 => Some((4, 1)),
        0x27 => Some((6, 1)),
        0x2d => Some((6, 1)),
        0x30..=0x65 => Some((4, 1)),
        0x66..=0x67 => Some((6, 1)),
        0x6f => Some((4, 1)),
        0x70 => Some((6, 0)),
        0x7f => Some((6, 0)),
        0x80..=0x96 => Some((4, 1)),
        0xa0..=0xa6 => Some((4, 1)),
        0xa8..=0xae => Some((4, 1)),
        0xb0..=0xb6 => Some((4, 1)),
        0xb8..=0xbe => Some((4, 1)),
        0xc0..=0xc6 => Some((4, 1)),
        0xc8..=0xce => Some((4, 1)),
        0xd0..=0xd6 => Some((4, 1)),
        0xd8..=0xde => Some((4, 1)),
        0xe0..=0xff => Some((5, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2e(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x17 => Some((4, 1)),
        0x18..=0x1b => Some((5, 1)),
        0x1c..=0x1d => Some((4, 1)),
        0x1e..=0x30 => Some((5, 1)),
        0x31 => Some((5, 2)),
        0x32..=0x3b => Some((6, 1)),
        0x3c..=0x42 => Some((7, 0)),
        0x43..=0x44 => Some((9, 0)),
        0x45..=0x49 => Some((10, 0)),
        0x4a..=0x4e => Some((11, 0)),
        0x4f => Some((12, 0)),
        0x50..=0x52 => Some((13, 0)),
        0x53..=0x5d => Some((14, 0)),
        0x80..=0x99 => Some((3, 0)),
        0x9b..=0xf3 => Some((3, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p2f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xd5 => Some((3, 0)),
        0xf0..=0xfb => Some((3, 0)),
        0xfc..=0xff => Some((15, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p30(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x37 => Some((1, 1)),
        0x38..=0x3a => Some((3, 0)),
        0x3b..=0x3d => Some((3, 2)),
        0x3e => Some((3, 0)),
        0x3f => Some((1, 1)),
        0x41..=0x94 => Some((1, 1)),
        0x95..=0x96 => Some((3, 2)),
        0x99..=0x9e => Some((1, 1)),
        0x9f..=0xa0 => Some((3, 2)),
        0xa1..=0xfe => Some((1, 1)),
        0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p31(b: u8) -> Option<(u8, u8)> {
    match b {
        0x05..=0x2c => Some((1, 1)),
        0x2d => Some((5, 1)),
        0x2e => Some((10, 0)),
        0x2f => Some((11, 0)),
        0x31..=0x8e => Some((1, 1)),
        0x90..=0x9f => Some((1, 1)),
        0xa0..=0xb7 => Some((3, 0)),
        0xb8..=0xba => Some((6, 0)),
        0xbb..=0xbf => Some((13, 0)),
        0xc0..=0xcf => Some((4, 1)),
        0xd0..=0xe3 => Some((5, 1)),
        0xe4..=0xe5 => Some((16, 0)),
        0xef => Some((15, 1)),
        0xf0..=0xff => Some((3, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p32(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1c => Some((1, 1)),
        0x1d..=0x1e => Some((4, 0)),
        0x20..=0x43 => Some((1, 1)),
        0x44..=0x4f => Some((5, 2)),
        0x50 => Some((4, 0)),
        0x51..=0x5f => Some((3, 2)),
        0x60..=0x7b => Some((1, 1)),
        0x7c..=0x7d => Some((4, 0)),
        0x7e => Some((4, 1)),
        0x7f..=0xb0 => Some((1, 1)),
        0xb1..=0xbf => Some((3, 2)),
        0xc0..=0xcb => Some((1, 1)),
        0xcc..=0xcf => Some((4, 0)),
        0xd0..=0xfe => Some((1, 1)),
        0xff => Some((12, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p33(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x76 => Some((1, 1)),
        0x77..=0x7a => Some((4, 0)),
        0x7b..=0xdd => Some((1, 1)),
        0xde..=0xdf => Some((4, 0)),
        0xe0..=0xfe => Some((1, 1)),
        0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p4d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xb5 => Some((3, 0)),
        0xb6..=0xbf => Some((13, 0)),
        0xc0..=0xff => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_p9f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xa5 => Some((1, 1)),
        0xa6..=0xbb => Some((4, 1)),
        0xbc..=0xc3 => Some((5, 1)),
        0xc4..=0xcb => Some((5, 2)),
        0xcc => Some((6, 1)),
        0xcd..=0xd5 => Some((8, 0)),
        0xd6..=0xea => Some((10, 0)),
        0xeb..=0xef => Some((11, 0)),
        0xf0..=0xfc => Some((13, 0)),
        0xfd..=0xff => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa4(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x8c => Some((3, 0)),
        0x90..=0xa1 => Some((3, 0)),
        0xa2..=0xa3 => Some((3, 2)),
        0xa4..=0xb3 => Some((3, 0)),
        0xb4 => Some((3, 2)),
        0xb5..=0xc0 => Some((3, 0)),
        0xc1 => Some((3, 2)),
        0xc2..=0xc4 => Some((3, 0)),
        0xc5 => Some((3, 2)),
        0xc6 => Some((3, 0)),
        0xd0..=0xff => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa6(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2b => Some((5, 1)),
        0x40..=0x5f => Some((5, 1)),
        0x60..=0x61 => Some((6, 0)),
        0x62..=0x73 => Some((5, 1)),
        0x74..=0x7b => Some((6, 1)),
        0x7c..=0x97 => Some((5, 1)),
        0x98..=0x9d => Some((7, 0)),
        0x9e => Some((8, 0)),
        0x9f => Some((6, 1)),
        0xa0..=0xf7 => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x16 => Some((4, 1)),
        0x17..=0x1a => Some((5, 0)),
        0x1b..=0x1f => Some((5, 1)),
        0x20..=0x21 => Some((5, 0)),
        0x22..=0x8c => Some((5, 1)),
        0x8d..=0x8e => Some((6, 0)),
        0x8f => Some((8, 0)),
        0x90..=0x91 => Some((6, 0)),
        0x92..=0x93 => Some((6, 1)),
        0x94..=0x9f => Some((7, 0)),
        0xa0..=0xa9 => Some((6, 0)),
        0xaa => Some((6, 1)),
        0xab..=0xad => Some((7, 0)),
        0xae => Some((9, 0)),
        0xaf => Some((11, 0)),
        0xb0..=0xb1 => Some((7, 0)),
        0xb2..=0xb7 => Some((8, 0)),
        0xb8..=0xb9 => Some((11, 0)),
        0xba..=0xbf => Some((12, 0)),
        0xc0..=0xc1 => Some((14, 0)),
        0xc2..=0xc6 => Some((12, 0)),
        0xc7..=0xca => Some((13, 0)),
        0xcb..=0xcd => Some((16, 0)),
        0xce..=0xcf => Some((17, 0)),
        0xd0..=0xd1 => Some((14, 0)),
        0xd2 => Some((17, 0)),
        0xd3 => Some((14, 0)),
        0xd4 => Some((17, 0)),
        0xd5..=0xd9 => Some((14, 0)),
        0xda..=0xdc => Some((16, 0)),
        0xf1 => Some((17, 0)),
        0xf2..=0xf4 => Some((14, 0)),
        0xf5..=0xf6 => Some((13, 0)),
        0xf7 => Some((7, 0)),
        0xf8..=0xf9 => Some((6, 1)),
        0xfa => Some((6, 0)),
        0xfb..=0xff => Some((5, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa8(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2b => Some((4, 1)),
        0x2c => Some((13, 0)),
        0x30..=0x39 => Some((5, 2)),
        0x40..=0x77 => Some((5, 0)),
        0x80..=0xc4 => Some((5, 1)),
        0xc5 => Some((9, 0)),
        0xce..=0xd9 => Some((5, 1)),
        0xe0..=0xfb => Some((5, 2)),
        0xfc..=0xfd => Some((8, 0)),
        0xfe..=0xff => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pa9(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x53 => Some((5, 1)),
        0x5f => Some((5, 1)),
        0x60..=0x7c => Some((5, 2)),
        0x80..=0xcd => Some((5, 2)),
        0xcf..=0xd9 => Some((5, 2)),
        0xde..=0xdf => Some((5, 2)),
        0xe0..=0xfe => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_paa(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x36 => Some((5, 1)),
        0x40..=0x4d => Some((5, 1)),
        0x50..=0x59 => Some((5, 1)),
        0x5c..=0x5f => Some((5, 1)),
        0x60..=0x7b => Some((5, 2)),
        0x7c..=0x7f => Some((7, 0)),
        0x80..=0xc2 => Some((5, 2)),
        0xdb..=0xdf => Some((5, 2)),
        0xe0..=0xf6 => Some((6, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pab(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01..=0x06 => Some((6, 0)),
        0x09..=0x0e => Some((6, 0)),
        0x11..=0x16 => Some((6, 0)),
        0x20..=0x26 => Some((6, 0)),
        0x28..=0x2e => Some((6, 0)),
        0x30..=0x5f => Some((7, 0)),
        0x60..=0x63 => Some((8, 0)),
        0x64..=0x65 => Some((7, 0)),
        0x66..=0x67 => Some((12, 0)),
        0x68..=0x6b => Some((13, 0)),
        0x70..=0xbf => Some((8, 0)),
        0xc0..=0xed => Some((5, 2)),
        0xf0..=0xf9 => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pd7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xa3 => Some((2, 0)),
        0xb0..=0xc6 => Some((5, 2)),
        0xcb..=0xfb => Some((5, 2)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pfa(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2d => Some((1, 1)),
        0x2e..=0x2f => Some((6, 1)),
        0x30..=0x6a => Some((3, 2)),
        0x6b..=0x6d => Some((5, 2)),
        0x70..=0xd9 => Some((4, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pfb(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x06 => Some((1, 1)),
        0x13..=0x17 => Some((1, 1)),
        0x1d => Some((3, 0)),
        0x1e..=0x36 => Some((1, 1)),
        0x38..=0x3c => Some((1, 1)),
        0x3e => Some((1, 1)),
        0x40..=0x41 => Some((1, 1)),
        0x43..=0x44 => Some((1, 1)),
        0x46..=0xb1 => Some((1, 1)),
        0xb2..=0xc1 => Some((6, 0)),
        0xc2 => Some((14, 0)),
        0xc3..=0xd2 => Some((17, 0)),
        0xd3..=0xff => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pfd(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x3f => Some((1, 1)),
        0x40..=0x4f => Some((14, 0)),
        0x50..=0x8f => Some((1, 1)),
        0x90..=0x91 => Some((17, 0)),
        0x92..=0xc7 => Some((1, 1)),
        0xc8..=0xce => Some((17, 0)),
        0xcf => Some((14, 0)),
        0xd0..=0xef => Some((3, 1)),
        0xf0..=0xfb => Some((1, 1)),
        0xfc => Some((3, 2)),
        0xfd => Some((4, 0)),
        0xfe..=0xff => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pfe(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0f => Some((3, 2)),
        0x10..=0x19 => Some((4, 1)),
        0x20..=0x23 => Some((1, 1)),
        0x24..=0x26 => Some((5, 1)),
        0x27..=0x2d => Some((7, 0)),
        0x2e..=0x2f => Some((8, 0)),
        0x30..=0x44 => Some((1, 1)),
        0x45..=0x46 => Some((3, 2)),
        0x47..=0x48 => Some((4, 0)),
        0x49..=0x52 => Some((1, 1)),
        0x54..=0x66 => Some((1, 1)),
        0x68..=0x6b => Some((1, 1)),
        0x70..=0x72 => Some((1, 1)),
        0x73 => Some((3, 2)),
        0x74 => Some((1, 1)),
        0x76..=0xfc => Some((1, 1)),
        0xff => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "bmp")]
const fn age_pff(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01..=0x5e => Some((1, 1)),
        0x5f..=0x60 => Some((3, 2)),
        0x61..=0xbe => Some((1, 1)),
        0xc2..=0xc7 => Some((1, 1)),
        0xca..=0xcf => Some((1, 1)),
        0xd2..=0xd7 => Some((1, 1)),
        0xda..=0xdc => Some((1, 1)),
        0xe0..=0xe6 => Some((1, 1)),
        0xe8..=0xee => Some((1, 1)),
        0xf9..=0xfb => Some((3, 0)),
        0xfc => Some((2, 1)),
        0xfd..=0xff => Some((1, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p100(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0b => Some((4, 0)),
        0x0d..=0x26 => Some((4, 0)),
        0x28..=0x3a => Some((4, 0)),
        0x3c..=0x3d => Some((4, 0)),
        0x3f..=0x4d => Some((4, 0)),
        0x50..=0x5d => Some((4, 0)),
        0x80..=0xfa => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p101(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x02 => Some((4, 0)),
        0x07..=0x33 => Some((4, 0)),
        0x37..=0x3f => Some((4, 0)),
        0x40..=0x8a => Some((4, 1)),
        0x8b..=0x8c => Some((7, 0)),
        0x8d..=0x8e => Some((9, 0)),
        0x90..=0x9b => Some((5, 1)),
        0x9c => Some((13, 0)),
        0xa0 => Some((7, 0)),
        0xd0..=0xfd => Some((5, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p102(b: u8) -> Option<(u8, u8)> {
    match b {
        0x80..=0x9c => Some((5, 1)),
        0xa0..=0xd0 => Some((5, 1)),
        0xe0..=0xfb => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p103(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1e => Some((3, 1)),
        0x1f => Some((7, 0)),
        0x20..=0x23 => Some((3, 1)),
        0x2d..=0x2f => Some((10, 0)),
        0x30..=0x4a => Some((3, 1)),
        0x50..=0x7a => Some((7, 0)),
        0x80..=0x9d => Some((4, 0)),
        0x9f => Some((4, 0)),
        0xa0..=0xc3 => Some((4, 1)),
        0xc8..=0xd5 => Some((4, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p104(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x25 => Some((3, 1)),
        0x26..=0x27 => Some((4, 0)),
        0x28..=0x4d => Some((3, 1)),
        0x4e..=0x9d => Some((4, 0)),
        0xa0..=0xa9 => Some((4, 0)),
        0xb0..=0xd3 => Some((9, 0)),
        0xd8..=0xfb => Some((9, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p105(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x27 => Some((7, 0)),
        0x30..=0x63 => Some((7, 0)),
        0x6f => Some((7, 0)),
        0x70..=0x7a => Some((14, 0)),
        0x7c..=0x8a => Some((14, 0)),
        0x8c..=0x92 => Some((14, 0)),
        0x94..=0x95 => Some((14, 0)),
        0x97..=0xa1 => Some((14, 0)),
        0xa3..=0xb1 => Some((14, 0)),
        0xb3..=0xb9 => Some((14, 0)),
        0xbb..=0xbc => Some((14, 0)),
        0xc0..=0xf3 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p107(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x36 => Some((7, 0)),
        0x40..=0x55 => Some((7, 0)),
        0x60..=0x67 => Some((7, 0)),
        0x80..=0x85 => Some((14, 0)),
        0x87..=0xb0 => Some((14, 0)),
        0xb2..=0xba => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p108(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x05 => Some((4, 0)),
        0x08 => Some((4, 0)),
        0x0a..=0x35 => Some((4, 0)),
        0x37..=0x38 => Some((4, 0)),
        0x3c => Some((4, 0)),
        0x3f => Some((4, 0)),
        0x40..=0x55 => Some((5, 2)),
        0x57..=0x5f => Some((5, 2)),
        0x60..=0x9e => Some((7, 0)),
        0xa7..=0xaf => Some((7, 0)),
        0xe0..=0xf2 => Some((8, 0)),
        0xf4..=0xf5 => Some((8, 0)),
        0xfb..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p109(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x19 => Some((5, 0)),
        0x1a..=0x1b => Some((5, 2)),
        0x1f => Some((5, 0)),
        0x20..=0x39 => Some((5, 1)),
        0x3f => Some((5, 1)),
        0x40..=0x59 => Some((17, 0)),
        0x80..=0xb7 => Some((6, 1)),
        0xbc..=0xbd => Some((8, 0)),
        0xbe..=0xbf => Some((6, 1)),
        0xc0..=0xcf => Some((8, 0)),
        0xd2..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10a(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x03 => Some((4, 1)),
        0x05..=0x06 => Some((4, 1)),
        0x0c..=0x13 => Some((4, 1)),
        0x15..=0x17 => Some((4, 1)),
        0x19..=0x33 => Some((4, 1)),
        0x34..=0x35 => Some((11, 0)),
        0x38..=0x3a => Some((4, 1)),
        0x3f..=0x47 => Some((4, 1)),
        0x48 => Some((11, 0)),
        0x50..=0x58 => Some((4, 1)),
        0x60..=0x7f => Some((5, 2)),
        0x80..=0x9f => Some((7, 0)),
        0xc0..=0xe6 => Some((7, 0)),
        0xeb..=0xf6 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10b(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x35 => Some((5, 2)),
        0x39..=0x55 => Some((5, 2)),
        0x58..=0x72 => Some((5, 2)),
        0x78..=0x7f => Some((5, 2)),
        0x80..=0x91 => Some((7, 0)),
        0x99..=0x9c => Some((7, 0)),
        0xa9..=0xaf => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10c(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x48 => Some((5, 2)),
        0x80..=0xb2 => Some((8, 0)),
        0xc0..=0xf2 => Some((8, 0)),
        0xfa..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x27 => Some((11, 0)),
        0x30..=0x39 => Some((11, 0)),
        0x40..=0x65 => Some((16, 0)),
        0x69..=0x85 => Some((16, 0)),
        0x8e..=0x8f => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10e(b: u8) -> Option<(u8, u8)> {
    match b {
        0x60..=0x7e => Some((5, 2)),
        0x80..=0xa9 => Some((13, 0)),
        0xab..=0xad => Some((13, 0)),
        0xb0..=0xb1 => Some((13, 0)),
        0xc2..=0xc4 => Some((16, 0)),
        0xc5..=0xc7 => Some((17, 0)),
        0xd0..=0xd8 => Some((17, 0)),
        0xfa..=0xfb => Some((17, 0)),
        0xfc => Some((16, 0)),
        0xfd..=0xff => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p10f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x27 => Some((11, 0)),
        0x30..=0x59 => Some((11, 0)),
        0x70..=0x89 => Some((14, 0)),
        0xb0..=0xcb => Some((13, 0)),
        0xe0..=0xf6 => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p110(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x4d => Some((6, 0)),
        0x52..=0x6f => Some((6, 0)),
        0x70..=0x75 => Some((14, 0)),
        0x7f => Some((7, 0)),
        0x80..=0xc1 => Some((5, 2)),
        0xc2 => Some((14, 0)),
        0xcd => Some((11, 0)),
        0xd0..=0xe8 => Some((6, 1)),
        0xf0..=0xf9 => Some((6, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p111(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x34 => Some((6, 1)),
        0x36..=0x43 => Some((6, 1)),
        0x44..=0x46 => Some((11, 0)),
        0x47 => Some((13, 0)),
        0x50..=0x76 => Some((7, 0)),
        0x80..=0xc8 => Some((6, 1)),
        0xc9..=0xcc => Some((8, 0)),
        0xcd => Some((7, 0)),
        0xce..=0xcf => Some((13, 0)),
        0xd0..=0xd9 => Some((6, 1)),
        0xda => Some((7, 0)),
        0xdb..=0xdf => Some((8, 0)),
        0xe1..=0xf4 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p112(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x11 => Some((7, 0)),
        0x13..=0x3d => Some((7, 0)),
        0x3e => Some((9, 0)),
        0x3f..=0x41 => Some((15, 0)),
        0x80..=0x86 => Some((8, 0)),
        0x88 => Some((8, 0)),
        0x8a..=0x8d => Some((8, 0)),
        0x8f..=0x9d => Some((8, 0)),
        0x9f..=0xa9 => Some((8, 0)),
        0xb0..=0xea => Some((7, 0)),
        0xf0..=0xf9 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p113(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((8, 0)),
        0x01..=0x03 => Some((7, 0)),
        0x05..=0x0c => Some((7, 0)),
        0x0f..=0x10 => Some((7, 0)),
        0x13..=0x28 => Some((7, 0)),
        0x2a..=0x30 => Some((7, 0)),
        0x32..=0x33 => Some((7, 0)),
        0x35..=0x39 => Some((7, 0)),
        0x3b => Some((11, 0)),
        0x3c..=0x44 => Some((7, 0)),
        0x47..=0x48 => Some((7, 0)),
        0x4b..=0x4d => Some((7, 0)),
        0x50 => Some((8, 0)),
        0x57 => Some((7, 0)),
        0x5d..=0x63 => Some((7, 0)),
        0x66..=0x6c => Some((7, 0)),
        0x70..=0x74 => Some((7, 0)),
        0x80..=0x89 => Some((16, 0)),
        0x8b => Some((16, 0)),
        0x8e => Some((16, 0)),
        0x90..=0xb5 => Some((16, 0)),
        0xb7..=0xc0 => Some((16, 0)),
        0xc2 => Some((16, 0)),
        0xc5 => Some((16, 0)),
        0xc7..=0xca => Some((16, 0)),
        0xcc..=0xd5 => Some((16, 0)),
        0xd7..=0xd8 => Some((16, 0)),
        0xe1..=0xe2 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p114(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x59 => Some((9, 0)),
        0x5a => Some((13, 0)),
        0x5b => Some((9, 0)),
        0x5d => Some((9, 0)),
        0x5e => Some((11, 0)),
        0x5f => Some((12, 0)),
        0x60..=0x61 => Some((13, 0)),
        0x80..=0xc7 => Some((7, 0)),
        0xd0..=0xd9 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p115(b: u8) -> Option<(u8, u8)> {
    match b {
        0x80..=0xb5 => Some((7, 0)),
        0xb8..=0xc9 => Some((7, 0)),
        0xca..=0xdd => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p116(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x44 => Some((7, 0)),
        0x50..=0x59 => Some((7, 0)),
        0x60..=0x6c => Some((9, 0)),
        0x80..=0xb7 => Some((6, 1)),
        0xb8 => Some((12, 0)),
        0xb9 => Some((14, 0)),
        0xc0..=0xc9 => Some((6, 1)),
        0xd0..=0xe3 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p117(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x19 => Some((8, 0)),
        0x1a => Some((11, 0)),
        0x1d..=0x2b => Some((8, 0)),
        0x30..=0x3f => Some((8, 0)),
        0x40..=0x46 => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p118(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x3b => Some((11, 0)),
        0xa0..=0xf2 => Some((7, 0)),
        0xff => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p119(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x06 => Some((13, 0)),
        0x09 => Some((13, 0)),
        0x0c..=0x13 => Some((13, 0)),
        0x15..=0x16 => Some((13, 0)),
        0x18..=0x35 => Some((13, 0)),
        0x37..=0x38 => Some((13, 0)),
        0x3b..=0x46 => Some((13, 0)),
        0x50..=0x59 => Some((13, 0)),
        0xa0..=0xa7 => Some((12, 0)),
        0xaa..=0xd7 => Some((12, 0)),
        0xda..=0xe4 => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11a(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x47 => Some((10, 0)),
        0x50..=0x83 => Some((10, 0)),
        0x84..=0x85 => Some((12, 0)),
        0x86..=0x9c => Some((10, 0)),
        0x9d => Some((11, 0)),
        0x9e..=0xa2 => Some((10, 0)),
        0xb0..=0xbf => Some((14, 0)),
        0xc0..=0xf8 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11b(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x09 => Some((15, 0)),
        0x60..=0x67 => Some((17, 0)),
        0xc0..=0xe1 => Some((16, 0)),
        0xf0..=0xf9 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11c(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x08 => Some((9, 0)),
        0x0a..=0x36 => Some((9, 0)),
        0x38..=0x45 => Some((9, 0)),
        0x50..=0x6c => Some((9, 0)),
        0x70..=0x8f => Some((9, 0)),
        0x92..=0xa7 => Some((9, 0)),
        0xa9..=0xb6 => Some((9, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x06 => Some((10, 0)),
        0x08..=0x09 => Some((10, 0)),
        0x0b..=0x36 => Some((10, 0)),
        0x3a => Some((10, 0)),
        0x3c..=0x3d => Some((10, 0)),
        0x3f..=0x47 => Some((10, 0)),
        0x50..=0x59 => Some((10, 0)),
        0x60..=0x65 => Some((11, 0)),
        0x67..=0x68 => Some((11, 0)),
        0x6a..=0x8e => Some((11, 0)),
        0x90..=0x91 => Some((11, 0)),
        0x93..=0x98 => Some((11, 0)),
        0xa0..=0xa9 => Some((11, 0)),
        0xb0..=0xdb => Some((17, 0)),
        0xe0..=0xe9 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11e(b: u8) -> Option<(u8, u8)> {
    match b {
        0xe0..=0xf8 => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p11f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x10 => Some((15, 0)),
        0x12..=0x3a => Some((15, 0)),
        0x3e..=0x59 => Some((15, 0)),
        0x5a => Some((16, 0)),
        0xb0 => Some((13, 0)),
        0xc0..=0xf1 => Some((12, 0)),
        0xff => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p123(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x6e => Some((5, 0)),
        0x6f..=0x98 => Some((7, 0)),
        0x99 => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p124(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x62 => Some((5, 0)),
        0x63..=0x6e => Some((7, 0)),
        0x70..=0x73 => Some((5, 0)),
        0x74 => Some((7, 0)),
        0x80..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p125(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x43 => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p12f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x90..=0xf2 => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p134(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2e => Some((5, 2)),
        0x2f => Some((15, 0)),
        0x30..=0x38 => Some((12, 0)),
        0x39..=0x55 => Some((15, 0)),
        0x60..=0xff => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p143(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xfa => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p146(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x46 => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p161(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x39 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p16a(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x38 => Some((6, 0)),
        0x40..=0x5e => Some((7, 0)),
        0x60..=0x69 => Some((7, 0)),
        0x6e..=0x6f => Some((7, 0)),
        0x70..=0xbe => Some((14, 0)),
        0xc0..=0xc9 => Some((14, 0)),
        0xd0..=0xed => Some((7, 0)),
        0xf0..=0xf5 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p16b(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x45 => Some((7, 0)),
        0x50..=0x59 => Some((7, 0)),
        0x5b..=0x61 => Some((7, 0)),
        0x63..=0x77 => Some((7, 0)),
        0x7d..=0x8f => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p16d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x40..=0x79 => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p16e(b: u8) -> Option<(u8, u8)> {
    match b {
        0x40..=0x9a => Some((11, 0)),
        0xa0..=0xb8 => Some((17, 0)),
        0xbb..=0xd3 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p16f(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x44 => Some((6, 1)),
        0x45..=0x4a => Some((12, 0)),
        0x4f => Some((12, 0)),
        0x50..=0x7e => Some((6, 1)),
        0x7f..=0x87 => Some((12, 0)),
        0x8f..=0x9f => Some((6, 1)),
        0xe0 => Some((9, 0)),
        0xe1 => Some((10, 0)),
        0xe2..=0xe3 => Some((12, 0)),
        0xe4 => Some((13, 0)),
        0xf0..=0xf1 => Some((13, 0)),
        0xf2..=0xf6 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p187(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xec => Some((9, 0)),
        0xed..=0xf1 => Some((11, 0)),
        0xf2..=0xf7 => Some((12, 0)),
        0xf8..=0xff => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p18a(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xf2 => Some((9, 0)),
        0xf3..=0xff => Some((13, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p18c(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xd5 => Some((13, 0)),
        0xff => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p18d(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x08 => Some((13, 0)),
        0x09..=0x1e => Some((17, 0)),
        0x80..=0xf2 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1af(b: u8) -> Option<(u8, u8)> {
    match b {
        0xf0..=0xf3 => Some((14, 0)),
        0xf5..=0xfb => Some((14, 0)),
        0xfd..=0xfe => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1b0(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x01 => Some((6, 0)),
        0x02..=0xff => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1b1(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1e => Some((10, 0)),
        0x1f..=0x22 => Some((14, 0)),
        0x32 => Some((15, 0)),
        0x50..=0x52 => Some((12, 0)),
        0x55 => Some((15, 0)),
        0x64..=0x67 => Some((12, 0)),
        0x70..=0xff => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1b2(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xfb => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1bc(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x6a => Some((7, 0)),
        0x70..=0x7c => Some((7, 0)),
        0x80..=0x88 => Some((7, 0)),
        0x90..=0x99 => Some((7, 0)),
        0x9c..=0xa3 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1cc(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xf9 => Some((16, 0)),
        0xfa..=0xfc => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1ce(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xb3 => Some((16, 0)),
        0xba..=0xd0 => Some((17, 0)),
        0xe0..=0xf0 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1cf(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2d => Some((14, 0)),
        0x30..=0x46 => Some((14, 0)),
        0x50..=0xc3 => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d0(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xf5 => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d1(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x26 => Some((3, 1)),
        0x29 => Some((5, 1)),
        0x2a..=0xdd => Some((3, 1)),
        0xde..=0xe8 => Some((8, 0)),
        0xe9..=0xea => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d2(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x45 => Some((4, 1)),
        0xc0..=0xd3 => Some((15, 0)),
        0xe0..=0xf3 => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d3(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x56 => Some((4, 0)),
        0x60..=0x71 => Some((5, 0)),
        0x72..=0x78 => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d4(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x54 => Some((3, 1)),
        0x56..=0x9c => Some((3, 1)),
        0x9e..=0x9f => Some((3, 1)),
        0xa2 => Some((3, 1)),
        0xa5..=0xa6 => Some((3, 1)),
        0xa9..=0xac => Some((3, 1)),
        0xae..=0xb9 => Some((3, 1)),
        0xbb => Some((3, 1)),
        0xbd..=0xc0 => Some((3, 1)),
        0xc1 => Some((4, 0)),
        0xc2..=0xc3 => Some((3, 1)),
        0xc5..=0xff => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d5(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x05 => Some((3, 1)),
        0x07..=0x0a => Some((3, 1)),
        0x0d..=0x14 => Some((3, 1)),
        0x16..=0x1c => Some((3, 1)),
        0x1e..=0x39 => Some((3, 1)),
        0x3b..=0x3e => Some((3, 1)),
        0x40..=0x44 => Some((3, 1)),
        0x46 => Some((3, 1)),
        0x4a..=0x50 => Some((3, 1)),
        0x52..=0xff => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d6(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xa3 => Some((3, 1)),
        0xa4..=0xa5 => Some((4, 1)),
        0xa8..=0xff => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1d7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xc9 => Some((3, 1)),
        0xca..=0xcb => Some((5, 0)),
        0xce..=0xff => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1da(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x8b => Some((8, 0)),
        0x9b..=0x9f => Some((8, 0)),
        0xa1..=0xaf => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1df(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1e => Some((14, 0)),
        0x25..=0x2a => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e0(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x06 => Some((9, 0)),
        0x08..=0x18 => Some((9, 0)),
        0x1b..=0x21 => Some((9, 0)),
        0x23..=0x24 => Some((9, 0)),
        0x26..=0x2a => Some((9, 0)),
        0x30..=0x6d => Some((15, 0)),
        0x8f => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e1(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2c => Some((12, 0)),
        0x30..=0x3d => Some((12, 0)),
        0x40..=0x49 => Some((12, 0)),
        0x4e..=0x4f => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e2(b: u8) -> Option<(u8, u8)> {
    match b {
        0x90..=0xae => Some((14, 0)),
        0xc0..=0xf9 => Some((12, 0)),
        0xff => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e4(b: u8) -> Option<(u8, u8)> {
    match b {
        0xd0..=0xf9 => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e5(b: u8) -> Option<(u8, u8)> {
    match b {
        0xd0..=0xfa => Some((16, 0)),
        0xff => Some((16, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e6(b: u8) -> Option<(u8, u8)> {
    match b {
        0xc0..=0xde => Some((17, 0)),
        0xe0..=0xf5 => Some((17, 0)),
        0xfe..=0xff => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e7(b: u8) -> Option<(u8, u8)> {
    match b {
        0xe0..=0xe6 => Some((14, 0)),
        0xe8..=0xeb => Some((14, 0)),
        0xed..=0xee => Some((14, 0)),
        0xf0..=0xfe => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e8(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xc4 => Some((7, 0)),
        0xc7..=0xd6 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1e9(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x4a => Some((9, 0)),
        0x4b => Some((12, 0)),
        0x50..=0x59 => Some((9, 0)),
        0x5e..=0x5f => Some((9, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1ec(b: u8) -> Option<(u8, u8)> {
    match b {
        0x71..=0xb4 => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1ed(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01..=0x3d => Some((12, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1ee(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x03 => Some((6, 1)),
        0x05..=0x1f => Some((6, 1)),
        0x21..=0x22 => Some((6, 1)),
        0x24 => Some((6, 1)),
        0x27 => Some((6, 1)),
        0x29..=0x32 => Some((6, 1)),
        0x34..=0x37 => Some((6, 1)),
        0x39 => Some((6, 1)),
        0x3b => Some((6, 1)),
        0x42 => Some((6, 1)),
        0x47 => Some((6, 1)),
        0x49 => Some((6, 1)),
        0x4b => Some((6, 1)),
        0x4d..=0x4f => Some((6, 1)),
        0x51..=0x52 => Some((6, 1)),
        0x54 => Some((6, 1)),
        0x57 => Some((6, 1)),
        0x59 => Some((6, 1)),
        0x5b => Some((6, 1)),
        0x5d => Some((6, 1)),
        0x5f => Some((6, 1)),
        0x61..=0x62 => Some((6, 1)),
        0x64 => Some((6, 1)),
        0x67..=0x6a => Some((6, 1)),
        0x6c..=0x72 => Some((6, 1)),
        0x74..=0x77 => Some((6, 1)),
        0x79..=0x7c => Some((6, 1)),
        0x7e => Some((6, 1)),
        0x80..=0x89 => Some((6, 1)),
        0x8b..=0x9b => Some((6, 1)),
        0xa1..=0xa3 => Some((6, 1)),
        0xa5..=0xa9 => Some((6, 1)),
        0xab..=0xbb => Some((6, 1)),
        0xf0..=0xf1 => Some((6, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f0(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x2b => Some((5, 1)),
        0x30..=0x93 => Some((5, 1)),
        0xa0..=0xae => Some((6, 0)),
        0xb1..=0xbe => Some((6, 0)),
        0xbf => Some((7, 0)),
        0xc1..=0xcf => Some((6, 0)),
        0xd1..=0xdf => Some((6, 0)),
        0xe0..=0xf5 => Some((7, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f1(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0a => Some((5, 2)),
        0x0b..=0x0c => Some((7, 0)),
        0x0d..=0x0f => Some((13, 0)),
        0x10..=0x2e => Some((5, 2)),
        0x2f => Some((11, 0)),
        0x30 => Some((6, 0)),
        0x31 => Some((5, 2)),
        0x32..=0x3c => Some((6, 0)),
        0x3d => Some((5, 2)),
        0x3e => Some((6, 0)),
        0x3f => Some((5, 2)),
        0x40..=0x41 => Some((6, 0)),
        0x42 => Some((5, 2)),
        0x43..=0x45 => Some((6, 0)),
        0x46 => Some((5, 2)),
        0x47..=0x49 => Some((6, 0)),
        0x4a..=0x4e => Some((5, 2)),
        0x4f..=0x56 => Some((6, 0)),
        0x57 => Some((5, 2)),
        0x58..=0x5e => Some((6, 0)),
        0x5f => Some((5, 2)),
        0x60..=0x69 => Some((6, 0)),
        0x6a..=0x6b => Some((6, 1)),
        0x6c => Some((12, 0)),
        0x6d..=0x6f => Some((13, 0)),
        0x70..=0x78 => Some((6, 0)),
        0x79 => Some((5, 2)),
        0x7a => Some((6, 0)),
        0x7b..=0x7c => Some((5, 2)),
        0x7d..=0x7e => Some((6, 0)),
        0x7f => Some((5, 2)),
        0x80..=0x89 => Some((6, 0)),
        0x8a..=0x8d => Some((5, 2)),
        0x8e..=0x8f => Some((6, 0)),
        0x90 => Some((5, 2)),
        0x91..=0x9a => Some((6, 0)),
        0x9b..=0xac => Some((9, 0)),
        0xad => Some((13, 0)),
        0xe6..=0xff => Some((6, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f2(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((5, 2)),
        0x01..=0x02 => Some((6, 0)),
        0x10..=0x31 => Some((5, 2)),
        0x32..=0x3a => Some((6, 0)),
        0x3b => Some((9, 0)),
        0x40..=0x48 => Some((5, 2)),
        0x50..=0x51 => Some((6, 0)),
        0x60..=0x65 => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f3(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x20 => Some((6, 0)),
        0x21..=0x2c => Some((7, 0)),
        0x2d..=0x2f => Some((8, 0)),
        0x30..=0x35 => Some((6, 0)),
        0x36 => Some((7, 0)),
        0x37..=0x7c => Some((6, 0)),
        0x7d => Some((7, 0)),
        0x7e..=0x7f => Some((8, 0)),
        0x80..=0x93 => Some((6, 0)),
        0x94..=0x9f => Some((7, 0)),
        0xa0..=0xc4 => Some((6, 0)),
        0xc5 => Some((7, 0)),
        0xc6..=0xca => Some((6, 0)),
        0xcb..=0xce => Some((7, 0)),
        0xcf..=0xd3 => Some((8, 0)),
        0xd4..=0xdf => Some((7, 0)),
        0xe0..=0xf0 => Some((6, 0)),
        0xf1..=0xf7 => Some((7, 0)),
        0xf8..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f4(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x3e => Some((6, 0)),
        0x3f => Some((7, 0)),
        0x40 => Some((6, 0)),
        0x41 => Some((7, 0)),
        0x42..=0xf7 => Some((6, 0)),
        0xf8 => Some((7, 0)),
        0xf9..=0xfc => Some((6, 0)),
        0xfd..=0xfe => Some((7, 0)),
        0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f5(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x3d => Some((6, 0)),
        0x3e..=0x3f => Some((7, 0)),
        0x40..=0x43 => Some((6, 1)),
        0x44..=0x4a => Some((7, 0)),
        0x4b..=0x4f => Some((8, 0)),
        0x50..=0x67 => Some((6, 0)),
        0x68..=0x79 => Some((7, 0)),
        0x7a => Some((9, 0)),
        0x7b..=0xa3 => Some((7, 0)),
        0xa4 => Some((9, 0)),
        0xa5..=0xfa => Some((7, 0)),
        0xfb..=0xff => Some((6, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f6(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00 => Some((6, 1)),
        0x01..=0x10 => Some((6, 0)),
        0x11 => Some((6, 1)),
        0x12..=0x14 => Some((6, 0)),
        0x15 => Some((6, 1)),
        0x16 => Some((6, 0)),
        0x17 => Some((6, 1)),
        0x18 => Some((6, 0)),
        0x19 => Some((6, 1)),
        0x1a => Some((6, 0)),
        0x1b => Some((6, 1)),
        0x1c..=0x1e => Some((6, 0)),
        0x1f => Some((6, 1)),
        0x20..=0x25 => Some((6, 0)),
        0x26..=0x27 => Some((6, 1)),
        0x28..=0x2b => Some((6, 0)),
        0x2c => Some((6, 1)),
        0x2d => Some((6, 0)),
        0x2e..=0x2f => Some((6, 1)),
        0x30..=0x33 => Some((6, 0)),
        0x34 => Some((6, 1)),
        0x35..=0x40 => Some((6, 0)),
        0x41..=0x42 => Some((7, 0)),
        0x43..=0x44 => Some((8, 0)),
        0x45..=0x4f => Some((6, 0)),
        0x50..=0x7f => Some((7, 0)),
        0x80..=0xc5 => Some((6, 0)),
        0xc6..=0xcf => Some((7, 0)),
        0xd0 => Some((8, 0)),
        0xd1..=0xd2 => Some((9, 0)),
        0xd3..=0xd4 => Some((10, 0)),
        0xd5 => Some((12, 0)),
        0xd6..=0xd7 => Some((13, 0)),
        0xd8 => Some((17, 0)),
        0xdc => Some((15, 0)),
        0xdd..=0xdf => Some((14, 0)),
        0xe0..=0xec => Some((7, 0)),
        0xf0..=0xf3 => Some((7, 0)),
        0xf4..=0xf6 => Some((9, 0)),
        0xf7..=0xf8 => Some((10, 0)),
        0xf9 => Some((11, 0)),
        0xfa => Some((12, 0)),
        0xfb..=0xfc => Some((13, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x73 => Some((6, 0)),
        0x74..=0x76 => Some((15, 0)),
        0x77..=0x7a => Some((17, 0)),
        0x7b..=0x7f => Some((15, 0)),
        0x80..=0xd4 => Some((7, 0)),
        0xd5..=0xd8 => Some((11, 0)),
        0xd9 => Some((15, 0)),
        0xe0..=0xeb => Some((12, 0)),
        0xf0 => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f8(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0b => Some((7, 0)),
        0x10..=0x47 => Some((7, 0)),
        0x50..=0x59 => Some((7, 0)),
        0x60..=0x87 => Some((7, 0)),
        0x90..=0xad => Some((7, 0)),
        0xb0..=0xb1 => Some((13, 0)),
        0xb2..=0xbb => Some((16, 0)),
        0xc0..=0xc1 => Some((16, 0)),
        0xd0..=0xd8 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1f9(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x0b => Some((10, 0)),
        0x0c => Some((13, 0)),
        0x0d..=0x0f => Some((12, 0)),
        0x10..=0x18 => Some((8, 0)),
        0x19..=0x1e => Some((9, 0)),
        0x1f => Some((10, 0)),
        0x20..=0x27 => Some((9, 0)),
        0x28..=0x2f => Some((10, 0)),
        0x30 => Some((9, 0)),
        0x31..=0x32 => Some((10, 0)),
        0x33..=0x3e => Some((9, 0)),
        0x3f => Some((12, 0)),
        0x40..=0x4b => Some((9, 0)),
        0x4c => Some((10, 0)),
        0x4d..=0x4f => Some((11, 0)),
        0x50..=0x5e => Some((9, 0)),
        0x5f..=0x6b => Some((10, 0)),
        0x6c..=0x70 => Some((11, 0)),
        0x71 => Some((12, 0)),
        0x72 => Some((13, 0)),
        0x73..=0x76 => Some((11, 0)),
        0x77..=0x78 => Some((13, 0)),
        0x79 => Some((14, 0)),
        0x7a => Some((11, 0)),
        0x7b => Some((12, 0)),
        0x7c..=0x7f => Some((11, 0)),
        0x80..=0x84 => Some((8, 0)),
        0x85..=0x91 => Some((9, 0)),
        0x92..=0x97 => Some((10, 0)),
        0x98..=0xa2 => Some((11, 0)),
        0xa3..=0xa4 => Some((13, 0)),
        0xa5..=0xaa => Some((12, 0)),
        0xab..=0xad => Some((13, 0)),
        0xae..=0xaf => Some((12, 0)),
        0xb0..=0xb9 => Some((11, 0)),
        0xba..=0xbf => Some((12, 0)),
        0xc0 => Some((8, 0)),
        0xc1..=0xc2 => Some((11, 0)),
        0xc3..=0xca => Some((12, 0)),
        0xcb => Some((13, 0)),
        0xcc => Some((14, 0)),
        0xcd..=0xcf => Some((12, 0)),
        0xd0..=0xe6 => Some((10, 0)),
        0xe7..=0xff => Some((11, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1fa(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x53 => Some((12, 0)),
        0x54..=0x57 => Some((17, 0)),
        0x60..=0x6d => Some((11, 0)),
        0x70..=0x73 => Some((12, 0)),
        0x74 => Some((13, 0)),
        0x75..=0x77 => Some((15, 0)),
        0x78..=0x7a => Some((12, 0)),
        0x7b..=0x7c => Some((14, 0)),
        0x80..=0x82 => Some((12, 0)),
        0x83..=0x86 => Some((13, 0)),
        0x87..=0x88 => Some((15, 0)),
        0x89 => Some((16, 0)),
        0x8a => Some((17, 0)),
        0x8e => Some((17, 0)),
        0x8f => Some((16, 0)),
        0x90..=0x95 => Some((12, 0)),
        0x96..=0xa8 => Some((13, 0)),
        0xa9..=0xac => Some((14, 0)),
        0xad..=0xaf => Some((15, 0)),
        0xb0..=0xb6 => Some((13, 0)),
        0xb7..=0xba => Some((14, 0)),
        0xbb..=0xbd => Some((15, 0)),
        0xbe => Some((16, 0)),
        0xbf => Some((15, 0)),
        0xc0..=0xc2 => Some((13, 0)),
        0xc3..=0xc5 => Some((14, 0)),
        0xc6 => Some((16, 0)),
        0xc8 => Some((17, 0)),
        0xcd => Some((17, 0)),
        0xce..=0xcf => Some((15, 0)),
        0xd0..=0xd6 => Some((13, 0)),
        0xd7..=0xd9 => Some((14, 0)),
        0xda..=0xdb => Some((15, 0)),
        0xdc => Some((16, 0)),
        0xdf => Some((16, 0)),
        0xe0..=0xe7 => Some((14, 0)),
        0xe8 => Some((15, 0)),
        0xe9 => Some((16, 0)),
        0xea => Some((17, 0)),
        0xef => Some((17, 0)),
        0xf0..=0xf6 => Some((14, 0)),
        0xf7..=0xf8 => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1fb(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x92 => Some((13, 0)),
        0x94..=0xca => Some((13, 0)),
        0xcb..=0xef => Some((16, 0)),
        0xf0..=0xf9 => Some((13, 0)),
        0xfa => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p1ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2a6(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xd6 => Some((3, 1)),
        0xd7..=0xdd => Some((13, 0)),
        0xde..=0xdf => Some((14, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2b7(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x34 => Some((5, 2)),
        0x35..=0x38 => Some((14, 0)),
        0x39 => Some((15, 0)),
        0x3a..=0x3f => Some((17, 0)),
        0x40..=0xff => Some((6, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2b8(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1d => Some((6, 0)),
        0x20..=0xff => Some((8, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2ce(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xa1 => Some((8, 0)),
        0xa2..=0xad => Some((17, 0)),
        0xb0..=0xff => Some((10, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2eb(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xe0 => Some((10, 0)),
        0xf0..=0xff => Some((15, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2ee(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x5d => Some((15, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2fa(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x1d => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p2ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p313(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x4a => Some((13, 0)),
        0x50..=0xff => Some((15, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p323(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xaf => Some((15, 0)),
        0xb0..=0xff => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p334(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0x79 => Some((17, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p3ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p4ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p5ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p6ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p7ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p8ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_p9ff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_paff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_pbff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_pcff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_pdff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_pe00(b: u8) -> Option<(u8, u8)> {
    match b {
        0x01 => Some((3, 1)),
        0x20..=0x7f => Some((3, 1)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_pe01(b: u8) -> Option<(u8, u8)> {
    match b {
        0x00..=0xef => Some((4, 0)),
        _ => None,
    }
}

#[cfg(feature = "full")]
const fn age_peff(b: u8) -> Option<(u8, u8)> {
    match b {
        0xfe..=0xff => Some((2, 0)),
        _ => None,
    }
}

#[inline]
pub(crate) const fn block(cp: u32) -> &'static str {
    match cp >> 8 {
        #[cfg(feature = "ascii")]
        0x000 => block_p0(cp as u8),
        #[cfg(feature = "bmp")]
        0x001 => block_p1(cp as u8),
        #[cfg(feature = "bmp")]
        0x002 => block_p2(cp as u8),
        #[cfg(feature = "bmp")]
        0x003 => block_p3(cp as u8),
        #[cfg(feature = "bmp")]
        0x004 => "Cyrillic",
        #[cfg(feature = "bmp")]
        0x005 => block_p5(cp as u8),
        #[cfg(feature = "bmp")]
        0x006 => "Arabic",
        #[cfg(feature = "bmp")]
        0x007 => block_p7(cp as u8),
        #[cfg(feature = "bmp")]
        0x008 => block_p8(cp as u8),
        #[cfg(feature = "bmp")]
        0x009 => block_p9(cp as u8),
        #[cfg(feature = "bmp")]
        0x00a => block_pa(cp as u8),
        #[cfg(feature = "bmp")]
        0x00b => block_pb(cp as u8),
        #[cfg(feature = "bmp")]
        0x00c => block_pc(cp as u8),
        #[cfg(feature = "bmp")]
        0x00d => block_pd(cp as u8),
        #[cfg(feature = "bmp")]
        0x00e => block_pe(cp as u8),
        #[cfg(feature = "bmp")]
        0x00f => "Tibetan",
        #[cfg(feature = "bmp")]
        0x010 => block_p10(cp as u8),
        #[cfg(feature = "bmp")]
        0x011 => "Hangul Jamo",
        #[cfg(feature = "bmp")]
        0x012 => "Ethiopic",
        #[cfg(feature = "bmp")]
        0x013 => block_p13(cp as u8),
        #[cfg(feature = "bmp")]
        0x014 => "Unified Canadian Aboriginal Syllabics",
        #[cfg(feature = "bmp")]
        0x015 => "Unified Canadian Aboriginal Syllabics",
        #[cfg(feature = "bmp")]
        0x016 => block_p16(cp as u8),
        #[cfg(feature = "bmp")]
        0x017 => block_p17(cp as u8),
        #[cfg(feature = "bmp")]
        0x018 => block_p18(cp as u8),
        #[cfg(feature = "bmp")]
        0x019 => block_p19(cp as u8),
        #[cfg(feature = "bmp")]
        0x01a => block_p1a(cp as u8),
        #[cfg(feature = "bmp")]
        0x01b => block_p1b(cp as u8),
        #[cfg(feature = "bmp")]
        0x01c => block_p1c(cp as u8),
        #[cfg(feature = "bmp")]
        0x01d => block_p1d(cp as u8),
        #[cfg(feature = "bmp")]
        0x01e => "Latin Extended Additional",
        #[cfg(feature = "bmp")]
        0x01f => "Greek Extended",
        #[cfg(feature = "bmp")]
        0x020 => block_p20(cp as u8),
        #[cfg(feature = "bmp")]
        0x021 => block_p21(cp as u8),
        #[cfg(feature = "bmp")]
        0x022 => "Mathematical Operators",
        #[cfg(feature = "bmp")]
        0x023 => "Miscellaneous Technical",
        #[cfg(feature = "bmp")]
        0x024 => block_p24(cp as u8),
        #[cfg(feature = "bmp")]
        0x025 => block_p25(cp as u8),
        #[cfg(feature = "bmp")]
        0x026 => "Miscellaneous Symbols",
        #[cfg(feature = "bmp")]
        0x027 => block_p27(cp as u8),
        #[cfg(feature = "bmp")]
        0x028 => "Braille Patterns",
        #[cfg(feature = "bmp")]
        0x029 => block_p29(cp as u8),
        #[cfg(feature = "bmp")]
        0x02a => "Supplemental Mathematical Operators",
        #[cfg(feature = "bmp")]
        0x02b => "Miscellaneous Symbols and Arrows",
        #[cfg(feature = "bmp")]
        0x02c => block_p2c(cp as u8),
        #[cfg(feature = "bmp")]
        0x02d => block_p2d(cp as u8),
        #[cfg(feature = "bmp")]
        0x02e => block_p2e(cp as u8),
        #[cfg(feature = "bmp")]
        0x02f => block_p2f(cp as u8),
        #[cfg(feature = "bmp")]
        0x030 => block_p30(cp as u8),
        #[cfg(feature = "bmp")]
        0x031 => block_p31(cp as u8),
        #[cfg(feature = "bmp")]
        0x032 => "Enclosed CJK Letters and Months",
        #[cfg(feature = "bmp")]
        0x033 => "CJK Compatibility",
        #[cfg(feature = "bmp")]
        0x034 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x035 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x036 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x037 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x038 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x039 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03a => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03b => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03c => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03d => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03e => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x03f => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x040 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x041 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x042 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x043 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x044 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x045 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x046 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x047 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x048 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x049 => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x04a => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x04b => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x04c => "CJK Unified Ideographs Extension A",
        #[cfg(feature = "bmp")]
        0x04d => block_p4d(cp as u8),
        #[cfg(feature = "bmp")]
        0x04e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x04f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x050 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x051 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x052 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x053 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x054 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x055 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x056 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x057 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x058 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x059 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05a => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05b => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05c => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05d => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x05f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x060 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x061 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x062 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x063 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x064 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x065 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x066 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x067 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x068 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x069 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06a => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06b => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06c => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06d => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x06f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x070 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x071 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x072 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x073 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x074 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x075 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x076 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x077 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x078 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x079 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07a => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07b => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07c => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07d => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x07f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x080 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x081 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x082 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x083 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x084 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x085 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x086 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x087 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x088 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x089 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08a => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08b => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08c => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08d => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x08f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x090 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x091 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x092 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x093 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x094 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x095 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x096 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x097 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x098 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x099 => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09a => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09b => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09c => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09d => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09e => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x09f => "CJK Unified Ideographs",
        #[cfg(feature = "bmp")]
        0x0a0 => "Yi Syllables",
        #[cfg(feature = "bmp")]
        0x0a1 => "Yi Syllables",
        #[cfg(feature = "bmp")]
        0x0a2 => "Yi Syllables",
        #[cfg(feature = "bmp")]
        0x0a3 => "Yi Syllables",
        #[cfg(feature = "bmp")]
        0x0a4 => block_pa4(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a5 => "Vai",
        #[cfg(feature = "bmp")]
        0x0a6 => block_pa6(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a7 => block_pa7(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a8 => block_pa8(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a9 => block_pa9(cp as u8),
        #[cfg(feature = "bmp")]
        0x0aa => block_paa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ab => block_pab(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ac => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0ad => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0ae => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0af => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b0 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b1 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b2 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b3 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b4 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b5 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b6 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b7 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b8 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0b9 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0ba => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0bb => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0bc => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0bd => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0be => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0bf => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c0 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c1 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c2 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c3 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c4 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c5 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c6 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c7 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c8 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0c9 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0ca => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0cb => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0cc => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0cd => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0ce => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0cf => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d0 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d1 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d2 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d3 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d4 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d5 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d6 => "Hangul Syllables",
        #[cfg(feature = "bmp")]
        0x0d7 => block_pd7(cp as u8),
        #[cfg(feature = "bmp")]
        0x0d8 => "High Surrogates",
        #[cfg(feature = "bmp")]
        0x0d9 => "High Surrogates",
        #[cfg(feature = "bmp")]
        0x0da => "High Surrogates",
        #[cfg(feature = "bmp")]
        0x0db => block_pdb(cp as u8),
        #[cfg(feature = "bmp")]
        0x0dc => "Low Surrogates",
        #[cfg(feature = "bmp")]
        0x0dd => "Low Surrogates",
        #[cfg(feature = "bmp")]
        0x0de => "Low Surrogates",
        #[cfg(feature = "bmp")]
        0x0df => "Low Surrogates",
        #[cfg(feature = "bmp")]
        0x0e0 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e1 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e2 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e3 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e4 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e5 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e6 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e7 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e8 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0e9 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0ea => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0eb => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0ec => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0ed => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0ee => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0ef => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f0 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f1 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f2 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f3 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f4 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f5 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f6 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f7 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f8 => "Private Use Area",
        #[cfg(feature = "bmp")]
        0x0f9 => "CJK Compatibility Ideographs",
        #[cfg(feature = "bmp")]
        0x0fa => "CJK Compatibility Ideographs",
        #[cfg(feature = "bmp")]
        0x0fb => block_pfb(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fc => "Arabic Presentation Forms-A",
        #[cfg(feature = "bmp")]
        0x0fd => "Arabic Presentation Forms-A",
        #[cfg(feature = "bmp")]
        0x0fe => block_pfe(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ff => block_pff(cp as u8),
        #[cfg(feature = "full")]
        0x100 => block_p100(cp as u8),
        #[cfg(feature = "full")]
        0x101 => block_p101(cp as u8),
        #[cfg(feature = "full")]
        0x102 => block_p102(cp as u8),
        #[cfg(feature = "full")]
        0x103 => block_p103(cp as u8),
        #[cfg(feature = "full")]
        0x104 => block_p104(cp as u8),
        #[cfg(feature = "full")]
        0x105 => block_p105(cp as u8),
        #[cfg(feature = "full")]
        0x106 => "Linear A",
        #[cfg(feature = "full")]
        0x107 => block_p107(cp as u8),
        #[cfg(feature = "full")]
        0x108 => block_p108(cp as u8),
        #[cfg(feature = "full")]
        0x109 => block_p109(cp as u8),
        #[cfg(feature = "full")]
        0x10a => block_p10a(cp as u8),
        #[cfg(feature = "full")]
        0x10b => block_p10b(cp as u8),
        #[cfg(feature = "full")]
        0x10c => block_p10c(cp as u8),
        #[cfg(feature = "full")]
        0x10d => block_p10d(cp as u8),
        #[cfg(feature = "full")]
        0x10e => block_p10e(cp as u8),
        #[cfg(feature = "full")]
        0x10f => block_p10f(cp as u8),
        #[cfg(feature = "full")]
        0x110 => block_p110(cp as u8),
        #[cfg(feature = "full")]
        0x111 => block_p111(cp as u8),
        #[cfg(feature = "full")]
        0x112 => block_p112(cp as u8),
        #[cfg(feature = "full")]
        0x113 => block_p113(cp as u8),
        #[cfg(feature = "full")]
        0x114 => block_p114(cp as u8),
        #[cfg(feature = "full")]
        0x115 => block_p115(cp as u8),
        #[cfg(feature = "full")]
        0x116 => block_p116(cp as u8),
        #[cfg(feature = "full")]
        0x117 => block_p117(cp as u8),
        #[cfg(feature = "full")]
        0x118 => block_p118(cp as u8),
        #[cfg(feature = "full")]
        0x119 => block_p119(cp as u8),
        #[cfg(feature = "full")]
        0x11a => block_p11a(cp as u8),
        #[cfg(feature = "full")]
        0x11b => block_p11b(cp as u8),
        #[cfg(feature = "full")]
        0x11c => block_p11c(cp as u8),
        #[cfg(feature = "full")]
        0x11d => block_p11d(cp as u8),
        #[cfg(feature = "full")]
        0x11e => block_p11e(cp as u8),
        #[cfg(feature = "full")]
        0x11f => block_p11f(cp as u8),
        #[cfg(feature = "full")]
        0x120 => "Cuneiform",
        #[cfg(feature = "full")]
        0x121 => "Cuneiform",
        #[cfg(feature = "full")]
        0x122 => "Cuneiform",
        #[cfg(feature = "full")]
        0x123 => "Cuneiform",
        #[cfg(feature = "full")]
        0x124 => block_p124(cp as u8),
        #[cfg(feature = "full")]
        0x125 => block_p125(cp as u8),
        #[cfg(feature = "full")]
        0x12f => block_p12f(cp as u8),
        #[cfg(feature = "full")]
        0x130 => "Egyptian Hieroglyphs",
        #[cfg(feature = "full")]
        0x131 => "Egyptian Hieroglyphs",
        #[cfg(feature = "full")]
        0x132 => "Egyptian Hieroglyphs",
        #[cfg(feature = "full")]
        0x133 => "Egyptian Hieroglyphs",
        #[cfg(feature = "full")]
        0x134 => block_p134(cp as u8),
        #[cfg(feature = "full")]
        0x135 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x136 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x137 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x138 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x139 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13a => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13b => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13c => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13d => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13e => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x13f => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x140 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x141 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x142 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x143 => "Egyptian Hieroglyphs Extended-A",
        #[cfg(feature = "full")]
        0x144 => "Anatolian Hieroglyphs",
        #[cfg(feature = "full")]
        0x145 => "Anatolian Hieroglyphs",
        #[cfg(feature = "full")]
        0x146 => block_p146(cp as u8),
        #[cfg(feature = "full")]
        0x161 => block_p161(cp as u8),
        #[cfg(feature = "full")]
        0x168 => "Bamum Supplement",
        #[cfg(feature = "full")]
        0x169 => "Bamum Supplement",
        #[cfg(feature = "full")]
        0x16a => block_p16a(cp as u8),
        #[cfg(feature = "full")]
        0x16b => block_p16b(cp as u8),
        #[cfg(feature = "full")]
        0x16d => block_p16d(cp as u8),
        #[cfg(feature = "full")]
        0x16e => block_p16e(cp as u8),
        #[cfg(feature = "full")]
        0x16f => block_p16f(cp as u8),
        #[cfg(feature = "full")]
        0x170 => "Tangut",
        #[cfg(feature = "full")]
        0x171 => "Tangut",
        #[cfg(feature = "full")]
        0x172 => "Tangut",
        #[cfg(feature = "full")]
        0x173 => "Tangut",
        #[cfg(feature = "full")]
        0x174 => "Tangut",
        #[cfg(feature = "full")]
        0x175 => "Tangut",
        #[cfg(feature = "full")]
        0x176 => "Tangut",
        #[cfg(feature = "full")]
        0x177 => "Tangut",
        #[cfg(feature = "full")]
        0x178 => "Tangut",
        #[cfg(feature = "full")]
        0x179 => "Tangut",
        #[cfg(feature = "full")]
        0x17a => "Tangut",
        #[cfg(feature = "full")]
        0x17b => "Tangut",
        #[cfg(feature = "full")]
        0x17c => "Tangut",
        #[cfg(feature = "full")]
        0x17d => "Tangut",
        #[cfg(feature = "full")]
        0x17e => "Tangut",
        #[cfg(feature = "full")]
        0x17f => "Tangut",
        #[cfg(feature = "full")]
        0x180 => "Tangut",
        #[cfg(feature = "full")]
        0x181 => "Tangut",
        #[cfg(feature = "full")]
        0x182 => "Tangut",
        #[cfg(feature = "full")]
        0x183 => "Tangut",
        #[cfg(feature = "full")]
        0x184 => "Tangut",
        #[cfg(feature = "full")]
        0x185 => "Tangut",
        #[cfg(feature = "full")]
        0x186 => "Tangut",
        #[cfg(feature = "full")]
        0x187 => "Tangut",
        #[cfg(feature = "full")]
        0x188 => "Tangut Components",
        #[cfg(feature = "full")]
        0x189 => "Tangut Components",
        #[cfg(feature = "full")]
        0x18a => "Tangut Components",
        #[cfg(feature = "full")]
        0x18b => "Khitan Small Script",
        #[cfg(feature = "full")]
        0x18c => "Khitan Small Script",
        #[cfg(feature = "full")]
        0x18d => block_p18d(cp as u8),
        #[cfg(feature = "full")]
        0x1af => block_p1af(cp as u8),
        #[cfg(feature = "full")]
        0x1b0 => "Kana Supplement",
        #[cfg(feature = "full")]
        0x1b1 => block_p1b1(cp as u8),
        #[cfg(feature = "full")]
        0x1b2 => "Nushu",
        #[cfg(feature = "full")]
        0x1bc => block_p1bc(cp as u8),
        #[cfg(feature = "full")]
        0x1cc => "Symbols for Legacy Computing Supplement",
        #[cfg(feature = "full")]
        0x1cd => "Symbols for Legacy Computing Supplement",
        #[cfg(feature = "full")]
        0x1ce => block_p1ce(cp as u8),
        #[cfg(feature = "full")]
        0x1cf => block_p1cf(cp as u8),
        #[cfg(feature = "full")]
        0x1d0 => "Byzantine Musical Symbols",
        #[cfg(feature = "full")]
        0x1d1 => "Musical Symbols",
        #[cfg(feature = "full")]
        0x1d2 => block_p1d2(cp as u8),
        #[cfg(feature = "full")]
        0x1d3 => block_p1d3(cp as u8),
        #[cfg(feature = "full")]
        0x1d4 => "Mathematical Alphanumeric Symbols",
        #[cfg(feature = "full")]
        0x1d5 => "Mathematical Alphanumeric Symbols",
        #[cfg(feature = "full")]
        0x1d6 => "Mathematical Alphanumeric Symbols",
        #[cfg(feature = "full")]
        0x1d7 => "Mathematical Alphanumeric Symbols",
        #[cfg(feature = "full")]
        0x1d8 => "Sutton SignWriting",
        #[cfg(feature = "full")]
        0x1d9 => "Sutton SignWriting",
        #[cfg(feature = "full")]
        0x1da => block_p1da(cp as u8),
        #[cfg(feature = "full")]
        0x1df => "Latin Extended-G",
        #[cfg(feature = "full")]
        0x1e0 => block_p1e0(cp as u8),
        #[cfg(feature = "full")]
        0x1e1 => block_p1e1(cp as u8),
        #[cfg(feature = "full")]
        0x1e2 => block_p1e2(cp as u8),
        #[cfg(feature = "full")]
        0x1e4 => block_p1e4(cp as u8),
        #[cfg(feature = "full")]
        0x1e5 => block_p1e5(cp as u8),
        #[cfg(feature = "full")]
        0x1e6 => block_p1e6(cp as u8),
        #[cfg(feature = "full")]
        0x1e7 => block_p1e7(cp as u8),
        #[cfg(feature = "full")]
        0x1e8 => block_p1e8(cp as u8),
        #[cfg(feature = "full")]
        0x1e9 => block_p1e9(cp as u8),
        #[cfg(feature = "full")]
        0x1ec => block_p1ec(cp as u8),
        #[cfg(feature = "full")]
        0x1ed => block_p1ed(cp as u8),
        #[cfg(feature = "full")]
        0x1ee => "Arabic Mathematical Alphabetic Symbols",
        #[cfg(feature = "full")]
        0x1f0 => block_p1f0(cp as u8),
        #[cfg(feature = "full")]
        0x1f1 => "Enclosed Alphanumeric Supplement",
        #[cfg(feature = "full")]
        0x1f2 => "Enclosed Ideographic Supplement",
        #[cfg(feature = "full")]
        0x1f3 => "Miscellaneous Symbols and Pictographs",
        #[cfg(feature = "full")]
        0x1f4 => "Miscellaneous Symbols and Pictographs",
        #[cfg(feature = "full")]
        0x1f5 => "Miscellaneous Symbols and Pictographs",
        #[cfg(feature = "full")]
        0x1f6 => block_p1f6(cp as u8),
        #[cfg(feature = "full")]
        0x1f7 => block_p1f7(cp as u8),
        #[cfg(feature = "full")]
        0x1f8 => "Supplemental Arrows-C",
        #[cfg(feature = "full")]
        0x1f9 => "Supplemental Symbols and Pictographs",
        #[cfg(feature = "full")]
        0x1fa => block_p1fa(cp as u8),
        #[cfg(feature = "full")]
        0x1fb => "Symbols for Legacy Computing",
        #[cfg(feature = "full")]
        0x200 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x201 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x202 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x203 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x204 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x205 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x206 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x207 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x208 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x209 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x20f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x210 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x211 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x212 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x213 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x214 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x215 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x216 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x217 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x218 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x219 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x21f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x220 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x221 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x222 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x223 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x224 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x225 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x226 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x227 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x228 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x229 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x22f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x230 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x231 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x232 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x233 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x234 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x235 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x236 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x237 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x238 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x239 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x23f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x240 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x241 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x242 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x243 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x244 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x245 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x246 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x247 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x248 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x249 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x24f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x250 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x251 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x252 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x253 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x254 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x255 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x256 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x257 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x258 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x259 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x25f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x260 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x261 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x262 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x263 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x264 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x265 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x266 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x267 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x268 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x269 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x26f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x270 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x271 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x272 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x273 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x274 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x275 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x276 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x277 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x278 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x279 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x27f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x280 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x281 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x282 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x283 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x284 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x285 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x286 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x287 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x288 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x289 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x28f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x290 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x291 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x292 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x293 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x294 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x295 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x296 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x297 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x298 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x299 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29a => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29b => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29c => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29d => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29e => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x29f => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a0 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a1 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a2 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a3 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a4 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a5 => "CJK Unified Ideographs Extension B",
        #[cfg(feature = "full")]
        0x2a6 => block_p2a6(cp as u8),
        #[cfg(feature = "full")]
        0x2a7 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2a8 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2a9 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2aa => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2ab => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2ac => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2ad => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2ae => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2af => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b0 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b1 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b2 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b3 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b4 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b5 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b6 => "CJK Unified Ideographs Extension C",
        #[cfg(feature = "full")]
        0x2b7 => block_p2b7(cp as u8),
        #[cfg(feature = "full")]
        0x2b8 => block_p2b8(cp as u8),
        #[cfg(feature = "full")]
        0x2b9 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2ba => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2bb => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2bc => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2bd => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2be => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2bf => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c0 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c1 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c2 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c3 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c4 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c5 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c6 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c7 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c8 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2c9 => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2ca => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2cb => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2cc => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2cd => "CJK Unified Ideographs Extension E",
        #[cfg(feature = "full")]
        0x2ce => block_p2ce(cp as u8),
        #[cfg(feature = "full")]
        0x2cf => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d0 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d1 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d2 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d3 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d4 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d5 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d6 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d7 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d8 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2d9 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2da => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2db => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2dc => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2dd => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2de => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2df => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e0 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e1 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e2 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e3 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e4 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e5 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e6 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e7 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e8 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2e9 => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2ea => "CJK Unified Ideographs Extension F",
        #[cfg(feature = "full")]
        0x2eb => block_p2eb(cp as u8),
        #[cfg(feature = "full")]
        0x2ec => "CJK Unified Ideographs Extension I",
        #[cfg(feature = "full")]
        0x2ed => "CJK Unified Ideographs Extension I",
        #[cfg(feature = "full")]
        0x2ee => block_p2ee(cp as u8),
        #[cfg(feature = "full")]
        0x2f8 => "CJK Compatibility Ideographs Supplement",
        #[cfg(feature = "full")]
        0x2f9 => "CJK Compatibility Ideographs Supplement",
        #[cfg(feature = "full")]
        0x2fa => block_p2fa(cp as u8),
        #[cfg(feature = "full")]
        0x300 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x301 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x302 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x303 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x304 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x305 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x306 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x307 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x308 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x309 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30a => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30b => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30c => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30d => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30e => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x30f => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x310 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x311 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x312 => "CJK Unified Ideographs Extension G",
        #[cfg(feature = "full")]
        0x313 => block_p313(cp as u8),
        #[cfg(feature = "full")]
        0x314 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x315 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x316 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x317 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x318 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x319 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31a => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31b => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31c => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31d => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31e => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x31f => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x320 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x321 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x322 => "CJK Unified Ideographs Extension H",
        #[cfg(feature = "full")]
        0x323 => block_p323(cp as u8),
        #[cfg(feature = "full")]
        0x324 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x325 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x326 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x327 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x328 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x329 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32a => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32b => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32c => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32d => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32e => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x32f => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x330 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x331 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x332 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x333 => "CJK Unified Ideographs Extension J",
        #[cfg(feature = "full")]
        0x334 => block_p334(cp as u8),
        #[cfg(feature = "full")]
        0xe00 => block_pe00(cp as u8),
        #[cfg(feature = "full")]
        0xe01 => block_pe01(cp as u8),
        #[cfg(feature = "full")]
        0xf00 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf01 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf02 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf03 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf04 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf05 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf06 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf07 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf08 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf09 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf0f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf10 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf11 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf12 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf13 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf14 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf15 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf16 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf17 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf18 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf19 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf1f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf20 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf21 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf22 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf23 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf24 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf25 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf26 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf27 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf28 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf29 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf2f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf30 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf31 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf32 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf33 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf34 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf35 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf36 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf37 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf38 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf39 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf3f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf40 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf41 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf42 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf43 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf44 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf45 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf46 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf47 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf48 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf49 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf4f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf50 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf51 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf52 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf53 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf54 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf55 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf56 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf57 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf58 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf59 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf5f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf60 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf61 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf62 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf63 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf64 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf65 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf66 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf67 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf68 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf69 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf6f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf70 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf71 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf72 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf73 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf74 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf75 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf76 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf77 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf78 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf79 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf7f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf80 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf81 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf82 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf83 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf84 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf85 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf86 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf87 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf88 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf89 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf8f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf90 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf91 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf92 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf93 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf94 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf95 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf96 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf97 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf98 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf99 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9a => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9b => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9c => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9d => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9e => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xf9f => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfa9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfaa => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfab => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfac => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfad => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfae => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfaf => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfb9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfba => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfbb => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfbc => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfbd => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfbe => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfbf => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfc9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfca => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfcb => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfcc => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfcd => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfce => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfcf => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfd9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfda => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfdb => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfdc => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfdd => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfde => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfdf => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfe9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfea => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfeb => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfec => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfed => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfee => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfef => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff0 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff1 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff2 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff3 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff4 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff5 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff6 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff7 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff8 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xff9 => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xffa => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xffb => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xffc => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xffd => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xffe => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0xfff => "Supplementary Private Use Area-A",
        #[cfg(feature = "full")]
        0x1000 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1001 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1002 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1003 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1004 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1005 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1006 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1007 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1008 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1009 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x100f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1010 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1011 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1012 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1013 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1014 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1015 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1016 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1017 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1018 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1019 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x101f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1020 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1021 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1022 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1023 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1024 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1025 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1026 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1027 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1028 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1029 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x102f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1030 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1031 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1032 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1033 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1034 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1035 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1036 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1037 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1038 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1039 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x103f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1040 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1041 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1042 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1043 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1044 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1045 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1046 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1047 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1048 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1049 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x104f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1050 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1051 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1052 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1053 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1054 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1055 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1056 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1057 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1058 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1059 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x105f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1060 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1061 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1062 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1063 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1064 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1065 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1066 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1067 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1068 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1069 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x106f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1070 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1071 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1072 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1073 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1074 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1075 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1076 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1077 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1078 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1079 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x107f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1080 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1081 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1082 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1083 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1084 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1085 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1086 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1087 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1088 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1089 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x108f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1090 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1091 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1092 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1093 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1094 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1095 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1096 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1097 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1098 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x1099 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109a => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109b => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109c => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109d => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109e => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x109f => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10a9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10aa => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ab => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ac => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ad => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ae => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10af => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10b9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ba => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10bb => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10bc => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10bd => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10be => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10bf => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10c9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ca => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10cb => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10cc => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10cd => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ce => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10cf => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10d9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10da => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10db => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10dc => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10dd => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10de => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10df => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10e9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ea => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10eb => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ec => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ed => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ee => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ef => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f0 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f1 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f2 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f3 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f4 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f5 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f6 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f7 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f8 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10f9 => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10fa => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10fb => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10fc => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10fd => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10fe => "Supplementary Private Use Area-B",
        #[cfg(feature = "full")]
        0x10ff => "Supplementary Private Use Area-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "ascii")]
const fn block_p0(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Basic Latin",
        #[cfg(feature = "latin1")]
        0x80..=0xff => "Latin-1 Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p1(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Latin Extended-A",
        0x80..=0xff => "Latin Extended-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p2(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Latin Extended-B",
        0x50..=0xaf => "IPA Extensions",
        0xb0..=0xff => "Spacing Modifier Letters",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p3(b: u8) -> &'static str {
    match b {
        0x00..=0x6f => "Combining Diacritical Marks",
        0x70..=0xff => "Greek and Coptic",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p5(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Cyrillic Supplement",
        0x30..=0x8f => "Armenian",
        0x90..=0xff => "Hebrew",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p7(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Syriac",
        0x50..=0x7f => "Arabic Supplement",
        0x80..=0xbf => "Thaana",
        0xc0..=0xff => "NKo",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p8(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Samaritan",
        0x40..=0x5f => "Mandaic",
        0x60..=0x6f => "Syriac Supplement",
        0x70..=0x9f => "Arabic Extended-B",
        0xa0..=0xff => "Arabic Extended-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p9(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Devanagari",
        0x80..=0xff => "Bengali",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Gurmukhi",
        0x80..=0xff => "Gujarati",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pb(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Oriya",
        0x80..=0xff => "Tamil",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pc(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Telugu",
        0x80..=0xff => "Kannada",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pd(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Malayalam",
        0x80..=0xff => "Sinhala",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pe(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Thai",
        0x80..=0xff => "Lao",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p10(b: u8) -> &'static str {
    match b {
        0x00..=0x9f => "Myanmar",
        0xa0..=0xff => "Georgian",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p13(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Ethiopic",
        0x80..=0x9f => "Ethiopic Supplement",
        0xa0..=0xff => "Cherokee",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p16(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Unified Canadian Aboriginal Syllabics",
        0x80..=0x9f => "Ogham",
        0xa0..=0xff => "Runic",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p17(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "Tagalog",
        0x20..=0x3f => "Hanunoo",
        0x40..=0x5f => "Buhid",
        0x60..=0x7f => "Tagbanwa",
        0x80..=0xff => "Khmer",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p18(b: u8) -> &'static str {
    match b {
        0x00..=0xaf => "Mongolian",
        0xb0..=0xff => "Unified Canadian Aboriginal Syllabics Extended",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p19(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Limbu",
        0x50..=0x7f => "Tai Le",
        0x80..=0xdf => "New Tai Lue",
        0xe0..=0xff => "Khmer Symbols",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p1a(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "Buginese",
        0x20..=0xaf => "Tai Tham",
        0xb0..=0xff => "Combining Diacritical Marks Extended",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p1b(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Balinese",
        0x80..=0xbf => "Sundanese",
        0xc0..=0xff => "Batak",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p1c(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Lepcha",
        0x50..=0x7f => "Ol Chiki",
        0x80..=0x8f => "Cyrillic Extended-C",
        0x90..=0xbf => "Georgian Extended",
        0xc0..=0xcf => "Sundanese Supplement",
        0xd0..=0xff => "Vedic Extensions",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p1d(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Phonetic Extensions",
        0x80..=0xbf => "Phonetic Extensions Supplement",
        0xc0..=0xff => "Combining Diacritical Marks Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p20(b: u8) -> &'static str {
    match b {
        0x00..=0x6f => "General Punctuation",
        0x70..=0x9f => "Superscripts and Subscripts",
        0xa0..=0xcf => "Currency Symbols",
        0xd0..=0xff => "Combining Diacritical Marks for Symbols",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p21(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Letterlike Symbols",
        0x50..=0x8f => "Number Forms",
        0x90..=0xff => "Arrows",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p24(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Control Pictures",
        0x40..=0x5f => "Optical Character Recognition",
        0x60..=0xff => "Enclosed Alphanumerics",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p25(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Box Drawing",
        0x80..=0x9f => "Block Elements",
        0xa0..=0xff => "Geometric Shapes",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p27(b: u8) -> &'static str {
    match b {
        0x00..=0xbf => "Dingbats",
        0xc0..=0xef => "Miscellaneous Mathematical Symbols-A",
        0xf0..=0xff => "Supplemental Arrows-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p29(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Supplemental Arrows-B",
        0x80..=0xff => "Miscellaneous Mathematical Symbols-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p2c(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Glagolitic",
        0x60..=0x7f => "Latin Extended-C",
        0x80..=0xff => "Coptic",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p2d(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Georgian Supplement",
        0x30..=0x7f => "Tifinagh",
        0x80..=0xdf => "Ethiopic Extended",
        0xe0..=0xff => "Cyrillic Extended-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p2e(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Supplemental Punctuation",
        0x80..=0xff => "CJK Radicals Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p2f(b: u8) -> &'static str {
    match b {
        0x00..=0xdf => "Kangxi Radicals",
        0xf0..=0xff => "Ideographic Description Characters",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p30(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "CJK Symbols and Punctuation",
        0x40..=0x9f => "Hiragana",
        0xa0..=0xff => "Katakana",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p31(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Bopomofo",
        0x30..=0x8f => "Hangul Compatibility Jamo",
        0x90..=0x9f => "Kanbun",
        0xa0..=0xbf => "Bopomofo Extended",
        0xc0..=0xef => "CJK Strokes",
        0xf0..=0xff => "Katakana Phonetic Extensions",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_p4d(b: u8) -> &'static str {
    match b {
        0x00..=0xbf => "CJK Unified Ideographs Extension A",
        0xc0..=0xff => "Yijing Hexagram Symbols",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa4(b: u8) -> &'static str {
    match b {
        0x00..=0x8f => "Yi Syllables",
        0x90..=0xcf => "Yi Radicals",
        0xd0..=0xff => "Lisu",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa6(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Vai",
        0x40..=0x9f => "Cyrillic Extended-B",
        0xa0..=0xff => "Bamum",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa7(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "Modifier Tone Letters",
        0x20..=0xff => "Latin Extended-D",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa8(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Syloti Nagri",
        0x30..=0x3f => "Common Indic Number Forms",
        0x40..=0x7f => "Phags-pa",
        0x80..=0xdf => "Saurashtra",
        0xe0..=0xff => "Devanagari Extended",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pa9(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Kayah Li",
        0x30..=0x5f => "Rejang",
        0x60..=0x7f => "Hangul Jamo Extended-A",
        0x80..=0xdf => "Javanese",
        0xe0..=0xff => "Myanmar Extended-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_paa(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Cham",
        0x60..=0x7f => "Myanmar Extended-A",
        0x80..=0xdf => "Tai Viet",
        0xe0..=0xff => "Meetei Mayek Extensions",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pab(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Ethiopic Extended-A",
        0x30..=0x6f => "Latin Extended-E",
        0x70..=0xbf => "Cherokee Supplement",
        0xc0..=0xff => "Meetei Mayek",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pd7(b: u8) -> &'static str {
    match b {
        0x00..=0xaf => "Hangul Syllables",
        0xb0..=0xff => "Hangul Jamo Extended-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pdb(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "High Surrogates",
        0x80..=0xff => "High Private Use Surrogates",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pfb(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Alphabetic Presentation Forms",
        0x50..=0xff => "Arabic Presentation Forms-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pfe(b: u8) -> &'static str {
    match b {
        0x00..=0x0f => "Variation Selectors",
        0x10..=0x1f => "Vertical Forms",
        0x20..=0x2f => "Combining Half Marks",
        0x30..=0x4f => "CJK Compatibility Forms",
        0x50..=0x6f => "Small Form Variants",
        0x70..=0xff => "Arabic Presentation Forms-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "bmp")]
const fn block_pff(b: u8) -> &'static str {
    match b {
        0x00..=0xef => "Halfwidth and Fullwidth Forms",
        0xf0..=0xff => "Specials",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p100(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Linear B Syllabary",
        0x80..=0xff => "Linear B Ideograms",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p101(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Aegean Numbers",
        0x40..=0x8f => "Ancient Greek Numbers",
        0x90..=0xcf => "Ancient Symbols",
        0xd0..=0xff => "Phaistos Disc",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p102(b: u8) -> &'static str {
    match b {
        0x80..=0x9f => "Lycian",
        0xa0..=0xdf => "Carian",
        0xe0..=0xff => "Coptic Epact Numbers",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p103(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Old Italic",
        0x30..=0x4f => "Gothic",
        0x50..=0x7f => "Old Permic",
        0x80..=0x9f => "Ugaritic",
        0xa0..=0xdf => "Old Persian",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p104(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Deseret",
        0x50..=0x7f => "Shavian",
        0x80..=0xaf => "Osmanya",
        0xb0..=0xff => "Osage",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p105(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Elbasan",
        0x30..=0x6f => "Caucasian Albanian",
        0x70..=0xbf => "Vithkuqi",
        0xc0..=0xff => "Todhri",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p107(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Linear A",
        0x80..=0xbf => "Latin Extended-F",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p108(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Cypriot Syllabary",
        0x40..=0x5f => "Imperial Aramaic",
        0x60..=0x7f => "Palmyrene",
        0x80..=0xaf => "Nabataean",
        0xe0..=0xff => "Hatran",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p109(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "Phoenician",
        0x20..=0x3f => "Lydian",
        0x40..=0x5f => "Sidetic",
        0x80..=0x9f => "Meroitic Hieroglyphs",
        0xa0..=0xff => "Meroitic Cursive",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10a(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Kharoshthi",
        0x60..=0x7f => "Old South Arabian",
        0x80..=0x9f => "Old North Arabian",
        0xc0..=0xff => "Manichaean",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10b(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Avestan",
        0x40..=0x5f => "Inscriptional Parthian",
        0x60..=0x7f => "Inscriptional Pahlavi",
        0x80..=0xaf => "Psalter Pahlavi",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10c(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Old Turkic",
        0x80..=0xff => "Old Hungarian",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10d(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Hanifi Rohingya",
        0x40..=0x8f => "Garay",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10e(b: u8) -> &'static str {
    match b {
        0x60..=0x7f => "Rumi Numeral Symbols",
        0x80..=0xbf => "Yezidi",
        0xc0..=0xff => "Arabic Extended-C",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p10f(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Old Sogdian",
        0x30..=0x6f => "Sogdian",
        0x70..=0xaf => "Old Uyghur",
        0xb0..=0xdf => "Chorasmian",
        0xe0..=0xff => "Elymaic",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p110(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Brahmi",
        0x80..=0xcf => "Kaithi",
        0xd0..=0xff => "Sora Sompeng",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p111(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Chakma",
        0x50..=0x7f => "Mahajani",
        0x80..=0xdf => "Sharada",
        0xe0..=0xff => "Sinhala Archaic Numbers",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p112(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Khojki",
        0x80..=0xaf => "Multani",
        0xb0..=0xff => "Khudawadi",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p113(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Grantha",
        0x80..=0xff => "Tulu-Tigalari",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p114(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Newa",
        0x80..=0xdf => "Tirhuta",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p115(b: u8) -> &'static str {
    match b {
        0x80..=0xff => "Siddham",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p116(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Modi",
        0x60..=0x7f => "Mongolian Supplement",
        0x80..=0xcf => "Takri",
        0xd0..=0xff => "Myanmar Extended-C",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p117(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Ahom",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p118(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Dogra",
        0xa0..=0xff => "Warang Citi",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p119(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Dives Akuru",
        0xa0..=0xff => "Nandinagari",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11a(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Zanabazar Square",
        0x50..=0xaf => "Soyombo",
        0xb0..=0xbf => "Unified Canadian Aboriginal Syllabics Extended-A",
        0xc0..=0xff => "Pau Cin Hau",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11b(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Devanagari Extended-A",
        0x60..=0x7f => "Sharada Supplement",
        0xc0..=0xff => "Sunuwar",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11c(b: u8) -> &'static str {
    match b {
        0x00..=0x6f => "Bhaiksuki",
        0x70..=0xbf => "Marchen",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11d(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Masaram Gondi",
        0x60..=0xaf => "Gunjala Gondi",
        0xb0..=0xef => "Tolong Siki",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11e(b: u8) -> &'static str {
    match b {
        0xe0..=0xff => "Makasar",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p11f(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Kawi",
        0xb0..=0xbf => "Lisu Supplement",
        0xc0..=0xff => "Tamil Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p124(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Cuneiform Numbers and Punctuation",
        0x80..=0xff => "Early Dynastic Cuneiform",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p125(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Early Dynastic Cuneiform",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p12f(b: u8) -> &'static str {
    match b {
        0x90..=0xff => "Cypro-Minoan",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p134(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Egyptian Hieroglyphs",
        0x30..=0x5f => "Egyptian Hieroglyph Format Controls",
        0x60..=0xff => "Egyptian Hieroglyphs Extended-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p146(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Anatolian Hieroglyphs",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p161(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Gurung Khema",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p16a(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "Bamum Supplement",
        0x40..=0x6f => "Mro",
        0x70..=0xcf => "Tangsa",
        0xd0..=0xff => "Bassa Vah",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p16b(b: u8) -> &'static str {
    match b {
        0x00..=0x8f => "Pahawh Hmong",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p16d(b: u8) -> &'static str {
    match b {
        0x40..=0x7f => "Kirat Rai",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p16e(b: u8) -> &'static str {
    match b {
        0x40..=0x9f => "Medefaidrin",
        0xa0..=0xdf => "Beria Erfe",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p16f(b: u8) -> &'static str {
    match b {
        0x00..=0x9f => "Miao",
        0xe0..=0xff => "Ideographic Symbols and Punctuation",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p18d(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Tangut Supplement",
        0x80..=0xff => "Tangut Components Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1af(b: u8) -> &'static str {
    match b {
        0xf0..=0xff => "Kana Extended-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1b1(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Kana Extended-A",
        0x30..=0x6f => "Small Kana Extension",
        0x70..=0xff => "Nushu",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1bc(b: u8) -> &'static str {
    match b {
        0x00..=0x9f => "Duployan",
        0xa0..=0xaf => "Shorthand Format Controls",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1ce(b: u8) -> &'static str {
    match b {
        0x00..=0xbf => "Symbols for Legacy Computing Supplement",
        0xc0..=0xff => "Miscellaneous Symbols Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1cf(b: u8) -> &'static str {
    match b {
        0x00..=0xcf => "Znamenny Musical Notation",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1d2(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Ancient Greek Musical Notation",
        0xc0..=0xdf => "Kaktovik Numerals",
        0xe0..=0xff => "Mayan Numerals",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1d3(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Tai Xuan Jing Symbols",
        0x60..=0x7f => "Counting Rod Numerals",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1da(b: u8) -> &'static str {
    match b {
        0x00..=0xaf => "Sutton SignWriting",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e0(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Glagolitic Supplement",
        0x30..=0x8f => "Cyrillic Extended-D",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e1(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Nyiakeng Puachue Hmong",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e2(b: u8) -> &'static str {
    match b {
        0x90..=0xbf => "Toto",
        0xc0..=0xff => "Wancho",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e4(b: u8) -> &'static str {
    match b {
        0xd0..=0xff => "Nag Mundari",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e5(b: u8) -> &'static str {
    match b {
        0xd0..=0xff => "Ol Onal",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e6(b: u8) -> &'static str {
    match b {
        0xc0..=0xff => "Tai Yo",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e7(b: u8) -> &'static str {
    match b {
        0xe0..=0xff => "Ethiopic Extended-B",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e8(b: u8) -> &'static str {
    match b {
        0x00..=0xdf => "Mende Kikakui",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1e9(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "Adlam",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1ec(b: u8) -> &'static str {
    match b {
        0x70..=0xbf => "Indic Siyaq Numbers",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1ed(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Ottoman Siyaq Numbers",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1f0(b: u8) -> &'static str {
    match b {
        0x00..=0x2f => "Mahjong Tiles",
        0x30..=0x9f => "Domino Tiles",
        0xa0..=0xff => "Playing Cards",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1f6(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "Emoticons",
        0x50..=0x7f => "Ornamental Dingbats",
        0x80..=0xff => "Transport and Map Symbols",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1f7(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Alchemical Symbols",
        0x80..=0xff => "Geometric Shapes Extended",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p1fa(b: u8) -> &'static str {
    match b {
        0x00..=0x6f => "Chess Symbols",
        0x70..=0xff => "Symbols and Pictographs Extended-A",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2a6(b: u8) -> &'static str {
    match b {
        0x00..=0xdf => "CJK Unified Ideographs Extension B",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2b7(b: u8) -> &'static str {
    match b {
        0x00..=0x3f => "CJK Unified Ideographs Extension C",
        0x40..=0xff => "CJK Unified Ideographs Extension D",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2b8(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "CJK Unified Ideographs Extension D",
        0x20..=0xff => "CJK Unified Ideographs Extension E",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2ce(b: u8) -> &'static str {
    match b {
        0x00..=0xaf => "CJK Unified Ideographs Extension E",
        0xb0..=0xff => "CJK Unified Ideographs Extension F",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2eb(b: u8) -> &'static str {
    match b {
        0x00..=0xef => "CJK Unified Ideographs Extension F",
        0xf0..=0xff => "CJK Unified Ideographs Extension I",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2ee(b: u8) -> &'static str {
    match b {
        0x00..=0x5f => "CJK Unified Ideographs Extension I",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p2fa(b: u8) -> &'static str {
    match b {
        0x00..=0x1f => "CJK Compatibility Ideographs Supplement",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p313(b: u8) -> &'static str {
    match b {
        0x00..=0x4f => "CJK Unified Ideographs Extension G",
        0x50..=0xff => "CJK Unified Ideographs Extension H",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p323(b: u8) -> &'static str {
    match b {
        0x00..=0xaf => "CJK Unified Ideographs Extension H",
        0xb0..=0xff => "CJK Unified Ideographs Extension J",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_p334(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "CJK Unified Ideographs Extension J",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_pe00(b: u8) -> &'static str {
    match b {
        0x00..=0x7f => "Tags",
        _ => "No_Block",
    }
}

#[cfg(feature = "full")]
const fn block_pe01(b: u8) -> &'static str {
    match b {
        0x00..=0xef => "Variation Selectors Supplement",
        _ => "No_Block",
    }
}

/// The `Joining_Type` property (Arabic/Syriac cursive joining, UAX #9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoiningType {
    /// `U` — does not join (the default).
    NonJoining,
    /// `C` — join-causing (e.g. ARABIC TATWEEL).
    JoinCausing,
    /// `D` — dual-joining.
    DualJoining,
    /// `L` — left-joining.
    LeftJoining,
    /// `R` — right-joining.
    RightJoining,
    /// `T` — transparent (combining marks, format chars).
    Transparent,
}

#[inline]
pub(crate) const fn joining_type(cp: u32) -> JoiningType {
    match cp >> 8 {
        #[cfg(feature = "ascii")]
        0x000 => jt_p0(cp as u8),
        #[cfg(feature = "bmp")]
        0x003 => jt_p3(cp as u8),
        #[cfg(feature = "bmp")]
        0x004 => jt_p4(cp as u8),
        #[cfg(feature = "bmp")]
        0x005 => jt_p5(cp as u8),
        #[cfg(feature = "bmp")]
        0x006 => jt_p6(cp as u8),
        #[cfg(feature = "bmp")]
        0x007 => jt_p7(cp as u8),
        #[cfg(feature = "bmp")]
        0x008 => jt_p8(cp as u8),
        #[cfg(feature = "bmp")]
        0x009 => jt_p9(cp as u8),
        #[cfg(feature = "bmp")]
        0x00a => jt_pa(cp as u8),
        #[cfg(feature = "bmp")]
        0x00b => jt_pb(cp as u8),
        #[cfg(feature = "bmp")]
        0x00c => jt_pc(cp as u8),
        #[cfg(feature = "bmp")]
        0x00d => jt_pd(cp as u8),
        #[cfg(feature = "bmp")]
        0x00e => jt_pe(cp as u8),
        #[cfg(feature = "bmp")]
        0x00f => jt_pf(cp as u8),
        #[cfg(feature = "bmp")]
        0x010 => jt_p10(cp as u8),
        #[cfg(feature = "bmp")]
        0x013 => jt_p13(cp as u8),
        #[cfg(feature = "bmp")]
        0x017 => jt_p17(cp as u8),
        #[cfg(feature = "bmp")]
        0x018 => jt_p18(cp as u8),
        #[cfg(feature = "bmp")]
        0x019 => jt_p19(cp as u8),
        #[cfg(feature = "bmp")]
        0x01a => jt_p1a(cp as u8),
        #[cfg(feature = "bmp")]
        0x01b => jt_p1b(cp as u8),
        #[cfg(feature = "bmp")]
        0x01c => jt_p1c(cp as u8),
        #[cfg(feature = "bmp")]
        0x01d => jt_p1d(cp as u8),
        #[cfg(feature = "bmp")]
        0x020 => jt_p20(cp as u8),
        #[cfg(feature = "bmp")]
        0x02c => jt_p2c(cp as u8),
        #[cfg(feature = "bmp")]
        0x02d => jt_p2d(cp as u8),
        #[cfg(feature = "bmp")]
        0x030 => jt_p30(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a6 => jt_pa6(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a8 => jt_pa8(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a9 => jt_pa9(cp as u8),
        #[cfg(feature = "bmp")]
        0x0aa => jt_paa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ab => jt_pab(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fb => jt_pfb(cp as u8),
        #[cfg(feature = "bmp")]
        0x0fe => jt_pfe(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ff => jt_pff(cp as u8),
        #[cfg(feature = "full")]
        0x101 => jt_p101(cp as u8),
        #[cfg(feature = "full")]
        0x102 => jt_p102(cp as u8),
        #[cfg(feature = "full")]
        0x103 => jt_p103(cp as u8),
        #[cfg(feature = "full")]
        0x10a => jt_p10a(cp as u8),
        #[cfg(feature = "full")]
        0x10b => jt_p10b(cp as u8),
        #[cfg(feature = "full")]
        0x10d => jt_p10d(cp as u8),
        #[cfg(feature = "full")]
        0x10e => jt_p10e(cp as u8),
        #[cfg(feature = "full")]
        0x10f => jt_p10f(cp as u8),
        #[cfg(feature = "full")]
        0x110 => jt_p110(cp as u8),
        #[cfg(feature = "full")]
        0x111 => jt_p111(cp as u8),
        #[cfg(feature = "full")]
        0x112 => jt_p112(cp as u8),
        #[cfg(feature = "full")]
        0x113 => jt_p113(cp as u8),
        #[cfg(feature = "full")]
        0x114 => jt_p114(cp as u8),
        #[cfg(feature = "full")]
        0x115 => jt_p115(cp as u8),
        #[cfg(feature = "full")]
        0x116 => jt_p116(cp as u8),
        #[cfg(feature = "full")]
        0x117 => jt_p117(cp as u8),
        #[cfg(feature = "full")]
        0x118 => jt_p118(cp as u8),
        #[cfg(feature = "full")]
        0x119 => jt_p119(cp as u8),
        #[cfg(feature = "full")]
        0x11a => jt_p11a(cp as u8),
        #[cfg(feature = "full")]
        0x11b => jt_p11b(cp as u8),
        #[cfg(feature = "full")]
        0x11c => jt_p11c(cp as u8),
        #[cfg(feature = "full")]
        0x11d => jt_p11d(cp as u8),
        #[cfg(feature = "full")]
        0x11e => jt_p11e(cp as u8),
        #[cfg(feature = "full")]
        0x11f => jt_p11f(cp as u8),
        #[cfg(feature = "full")]
        0x134 => jt_p134(cp as u8),
        #[cfg(feature = "full")]
        0x161 => jt_p161(cp as u8),
        #[cfg(feature = "full")]
        0x16a => jt_p16a(cp as u8),
        #[cfg(feature = "full")]
        0x16b => jt_p16b(cp as u8),
        #[cfg(feature = "full")]
        0x16f => jt_p16f(cp as u8),
        #[cfg(feature = "full")]
        0x1bc => jt_p1bc(cp as u8),
        #[cfg(feature = "full")]
        0x1cf => jt_p1cf(cp as u8),
        #[cfg(feature = "full")]
        0x1d1 => jt_p1d1(cp as u8),
        #[cfg(feature = "full")]
        0x1d2 => jt_p1d2(cp as u8),
        #[cfg(feature = "full")]
        0x1da => jt_p1da(cp as u8),
        #[cfg(feature = "full")]
        0x1e0 => jt_p1e0(cp as u8),
        #[cfg(feature = "full")]
        0x1e1 => jt_p1e1(cp as u8),
        #[cfg(feature = "full")]
        0x1e2 => jt_p1e2(cp as u8),
        #[cfg(feature = "full")]
        0x1e4 => jt_p1e4(cp as u8),
        #[cfg(feature = "full")]
        0x1e5 => jt_p1e5(cp as u8),
        #[cfg(feature = "full")]
        0x1e6 => jt_p1e6(cp as u8),
        #[cfg(feature = "full")]
        0x1e8 => jt_p1e8(cp as u8),
        #[cfg(feature = "full")]
        0x1e9 => jt_p1e9(cp as u8),
        #[cfg(feature = "full")]
        0xe00 => jt_pe00(cp as u8),
        #[cfg(feature = "full")]
        0xe01 => jt_pe01(cp as u8),
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "ascii")]
const fn jt_p0(b: u8) -> JoiningType {
    match b {
        #[cfg(feature = "latin1")]
        0xad => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p3(b: u8) -> JoiningType {
    match b {
        0x00..=0x6f => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p4(b: u8) -> JoiningType {
    match b {
        0x83..=0x89 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p5(b: u8) -> JoiningType {
    match b {
        0x91..=0xbd => JoiningType::Transparent,
        0xbf => JoiningType::Transparent,
        0xc1..=0xc2 => JoiningType::Transparent,
        0xc4..=0xc5 => JoiningType::Transparent,
        0xc7 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p6(b: u8) -> JoiningType {
    match b {
        0x10..=0x1a => JoiningType::Transparent,
        0x1c => JoiningType::Transparent,
        0x20 => JoiningType::DualJoining,
        0x22..=0x25 => JoiningType::RightJoining,
        0x26 => JoiningType::DualJoining,
        0x27 => JoiningType::RightJoining,
        0x28 => JoiningType::DualJoining,
        0x29 => JoiningType::RightJoining,
        0x2a..=0x2e => JoiningType::DualJoining,
        0x2f..=0x32 => JoiningType::RightJoining,
        0x33..=0x3f => JoiningType::DualJoining,
        0x40 => JoiningType::JoinCausing,
        0x41..=0x47 => JoiningType::DualJoining,
        0x48 => JoiningType::RightJoining,
        0x49..=0x4a => JoiningType::DualJoining,
        0x4b..=0x5f => JoiningType::Transparent,
        0x6e..=0x6f => JoiningType::DualJoining,
        0x70 => JoiningType::Transparent,
        0x71..=0x73 => JoiningType::RightJoining,
        0x75..=0x77 => JoiningType::RightJoining,
        0x78..=0x87 => JoiningType::DualJoining,
        0x88..=0x99 => JoiningType::RightJoining,
        0x9a..=0xbf => JoiningType::DualJoining,
        0xc0 => JoiningType::RightJoining,
        0xc1..=0xc2 => JoiningType::DualJoining,
        0xc3..=0xcb => JoiningType::RightJoining,
        0xcc => JoiningType::DualJoining,
        0xcd => JoiningType::RightJoining,
        0xce => JoiningType::DualJoining,
        0xcf => JoiningType::RightJoining,
        0xd0..=0xd1 => JoiningType::DualJoining,
        0xd2..=0xd3 => JoiningType::RightJoining,
        0xd5 => JoiningType::RightJoining,
        0xd6..=0xdc => JoiningType::Transparent,
        0xdf..=0xe4 => JoiningType::Transparent,
        0xe7..=0xe8 => JoiningType::Transparent,
        0xea..=0xed => JoiningType::Transparent,
        0xee..=0xef => JoiningType::RightJoining,
        0xfa..=0xfc => JoiningType::DualJoining,
        0xff => JoiningType::DualJoining,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p7(b: u8) -> JoiningType {
    match b {
        0x0f => JoiningType::Transparent,
        0x10 => JoiningType::RightJoining,
        0x11 => JoiningType::Transparent,
        0x12..=0x14 => JoiningType::DualJoining,
        0x15..=0x19 => JoiningType::RightJoining,
        0x1a..=0x1d => JoiningType::DualJoining,
        0x1e => JoiningType::RightJoining,
        0x1f..=0x27 => JoiningType::DualJoining,
        0x28 => JoiningType::RightJoining,
        0x29 => JoiningType::DualJoining,
        0x2a => JoiningType::RightJoining,
        0x2b => JoiningType::DualJoining,
        0x2c => JoiningType::RightJoining,
        0x2d..=0x2e => JoiningType::DualJoining,
        0x2f => JoiningType::RightJoining,
        0x30..=0x4a => JoiningType::Transparent,
        0x4d => JoiningType::RightJoining,
        0x4e..=0x58 => JoiningType::DualJoining,
        0x59..=0x5b => JoiningType::RightJoining,
        0x5c..=0x6a => JoiningType::DualJoining,
        0x6b..=0x6c => JoiningType::RightJoining,
        0x6d..=0x70 => JoiningType::DualJoining,
        0x71 => JoiningType::RightJoining,
        0x72 => JoiningType::DualJoining,
        0x73..=0x74 => JoiningType::RightJoining,
        0x75..=0x77 => JoiningType::DualJoining,
        0x78..=0x79 => JoiningType::RightJoining,
        0x7a..=0x7f => JoiningType::DualJoining,
        0xa6..=0xb0 => JoiningType::Transparent,
        0xca..=0xea => JoiningType::DualJoining,
        0xeb..=0xf3 => JoiningType::Transparent,
        0xfa => JoiningType::JoinCausing,
        0xfd => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p8(b: u8) -> JoiningType {
    match b {
        0x16..=0x19 => JoiningType::Transparent,
        0x1b..=0x23 => JoiningType::Transparent,
        0x25..=0x27 => JoiningType::Transparent,
        0x29..=0x2d => JoiningType::Transparent,
        0x40 => JoiningType::RightJoining,
        0x41..=0x45 => JoiningType::DualJoining,
        0x46..=0x47 => JoiningType::RightJoining,
        0x48 => JoiningType::DualJoining,
        0x49 => JoiningType::RightJoining,
        0x4a..=0x53 => JoiningType::DualJoining,
        0x54 => JoiningType::RightJoining,
        0x55 => JoiningType::DualJoining,
        0x56..=0x58 => JoiningType::RightJoining,
        0x59..=0x5b => JoiningType::Transparent,
        0x60 => JoiningType::DualJoining,
        0x62..=0x65 => JoiningType::DualJoining,
        0x67 => JoiningType::RightJoining,
        0x68 => JoiningType::DualJoining,
        0x69..=0x6a => JoiningType::RightJoining,
        0x70..=0x82 => JoiningType::RightJoining,
        0x83..=0x85 => JoiningType::JoinCausing,
        0x86 => JoiningType::DualJoining,
        0x89..=0x8d => JoiningType::DualJoining,
        0x8e => JoiningType::RightJoining,
        0x8f => JoiningType::DualJoining,
        0x97..=0x9f => JoiningType::Transparent,
        0xa0..=0xa9 => JoiningType::DualJoining,
        0xaa..=0xac => JoiningType::RightJoining,
        0xae => JoiningType::RightJoining,
        0xaf..=0xb0 => JoiningType::DualJoining,
        0xb1..=0xb2 => JoiningType::RightJoining,
        0xb3..=0xb8 => JoiningType::DualJoining,
        0xb9 => JoiningType::RightJoining,
        0xba..=0xc8 => JoiningType::DualJoining,
        0xca..=0xe1 => JoiningType::Transparent,
        0xe3..=0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p9(b: u8) -> JoiningType {
    match b {
        0x00..=0x02 => JoiningType::Transparent,
        0x3a => JoiningType::Transparent,
        0x3c => JoiningType::Transparent,
        0x41..=0x48 => JoiningType::Transparent,
        0x4d => JoiningType::Transparent,
        0x51..=0x57 => JoiningType::Transparent,
        0x62..=0x63 => JoiningType::Transparent,
        0x81 => JoiningType::Transparent,
        0xbc => JoiningType::Transparent,
        0xc1..=0xc4 => JoiningType::Transparent,
        0xcd => JoiningType::Transparent,
        0xe2..=0xe3 => JoiningType::Transparent,
        0xfe => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pa(b: u8) -> JoiningType {
    match b {
        0x01..=0x02 => JoiningType::Transparent,
        0x3c => JoiningType::Transparent,
        0x41..=0x42 => JoiningType::Transparent,
        0x47..=0x48 => JoiningType::Transparent,
        0x4b..=0x4d => JoiningType::Transparent,
        0x51 => JoiningType::Transparent,
        0x70..=0x71 => JoiningType::Transparent,
        0x75 => JoiningType::Transparent,
        0x81..=0x82 => JoiningType::Transparent,
        0xbc => JoiningType::Transparent,
        0xc1..=0xc5 => JoiningType::Transparent,
        0xc7..=0xc8 => JoiningType::Transparent,
        0xcd => JoiningType::Transparent,
        0xe2..=0xe3 => JoiningType::Transparent,
        0xfa..=0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pb(b: u8) -> JoiningType {
    match b {
        0x01 => JoiningType::Transparent,
        0x3c => JoiningType::Transparent,
        0x3f => JoiningType::Transparent,
        0x41..=0x44 => JoiningType::Transparent,
        0x4d => JoiningType::Transparent,
        0x55..=0x56 => JoiningType::Transparent,
        0x62..=0x63 => JoiningType::Transparent,
        0x82 => JoiningType::Transparent,
        0xc0 => JoiningType::Transparent,
        0xcd => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pc(b: u8) -> JoiningType {
    match b {
        0x00 => JoiningType::Transparent,
        0x04 => JoiningType::Transparent,
        0x3c => JoiningType::Transparent,
        0x3e..=0x40 => JoiningType::Transparent,
        0x46..=0x48 => JoiningType::Transparent,
        0x4a..=0x4d => JoiningType::Transparent,
        0x55..=0x56 => JoiningType::Transparent,
        0x62..=0x63 => JoiningType::Transparent,
        0x81 => JoiningType::Transparent,
        0xbc => JoiningType::Transparent,
        0xbf => JoiningType::Transparent,
        0xc6 => JoiningType::Transparent,
        0xcc..=0xcd => JoiningType::Transparent,
        0xe2..=0xe3 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pd(b: u8) -> JoiningType {
    match b {
        0x00..=0x01 => JoiningType::Transparent,
        0x3b..=0x3c => JoiningType::Transparent,
        0x41..=0x44 => JoiningType::Transparent,
        0x4d => JoiningType::Transparent,
        0x62..=0x63 => JoiningType::Transparent,
        0x81 => JoiningType::Transparent,
        0xca => JoiningType::Transparent,
        0xd2..=0xd4 => JoiningType::Transparent,
        0xd6 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pe(b: u8) -> JoiningType {
    match b {
        0x31 => JoiningType::Transparent,
        0x34..=0x3a => JoiningType::Transparent,
        0x47..=0x4e => JoiningType::Transparent,
        0xb1 => JoiningType::Transparent,
        0xb4..=0xbc => JoiningType::Transparent,
        0xc8..=0xce => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pf(b: u8) -> JoiningType {
    match b {
        0x18..=0x19 => JoiningType::Transparent,
        0x35 => JoiningType::Transparent,
        0x37 => JoiningType::Transparent,
        0x39 => JoiningType::Transparent,
        0x71..=0x7e => JoiningType::Transparent,
        0x80..=0x84 => JoiningType::Transparent,
        0x86..=0x87 => JoiningType::Transparent,
        0x8d..=0x97 => JoiningType::Transparent,
        0x99..=0xbc => JoiningType::Transparent,
        0xc6 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p10(b: u8) -> JoiningType {
    match b {
        0x2d..=0x30 => JoiningType::Transparent,
        0x32..=0x37 => JoiningType::Transparent,
        0x39..=0x3a => JoiningType::Transparent,
        0x3d..=0x3e => JoiningType::Transparent,
        0x58..=0x59 => JoiningType::Transparent,
        0x5e..=0x60 => JoiningType::Transparent,
        0x71..=0x74 => JoiningType::Transparent,
        0x82 => JoiningType::Transparent,
        0x85..=0x86 => JoiningType::Transparent,
        0x8d => JoiningType::Transparent,
        0x9d => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p13(b: u8) -> JoiningType {
    match b {
        0x5d..=0x5f => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p17(b: u8) -> JoiningType {
    match b {
        0x12..=0x14 => JoiningType::Transparent,
        0x32..=0x33 => JoiningType::Transparent,
        0x52..=0x53 => JoiningType::Transparent,
        0x72..=0x73 => JoiningType::Transparent,
        0xb4..=0xb5 => JoiningType::Transparent,
        0xb7..=0xbd => JoiningType::Transparent,
        0xc6 => JoiningType::Transparent,
        0xc9..=0xd3 => JoiningType::Transparent,
        0xdd => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p18(b: u8) -> JoiningType {
    match b {
        0x07 => JoiningType::DualJoining,
        0x0a => JoiningType::JoinCausing,
        0x0b..=0x0d => JoiningType::Transparent,
        0x0f => JoiningType::Transparent,
        0x20..=0x78 => JoiningType::DualJoining,
        0x85..=0x86 => JoiningType::Transparent,
        0x87..=0xa8 => JoiningType::DualJoining,
        0xa9 => JoiningType::Transparent,
        0xaa => JoiningType::DualJoining,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p19(b: u8) -> JoiningType {
    match b {
        0x20..=0x22 => JoiningType::Transparent,
        0x27..=0x28 => JoiningType::Transparent,
        0x32 => JoiningType::Transparent,
        0x39..=0x3b => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p1a(b: u8) -> JoiningType {
    match b {
        0x17..=0x18 => JoiningType::Transparent,
        0x1b => JoiningType::Transparent,
        0x56 => JoiningType::Transparent,
        0x58..=0x5e => JoiningType::Transparent,
        0x60 => JoiningType::Transparent,
        0x62 => JoiningType::Transparent,
        0x65..=0x6c => JoiningType::Transparent,
        0x73..=0x7c => JoiningType::Transparent,
        0x7f => JoiningType::Transparent,
        0xb0..=0xdd => JoiningType::Transparent,
        0xe0..=0xeb => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p1b(b: u8) -> JoiningType {
    match b {
        0x00..=0x03 => JoiningType::Transparent,
        0x34 => JoiningType::Transparent,
        0x36..=0x3a => JoiningType::Transparent,
        0x3c => JoiningType::Transparent,
        0x42 => JoiningType::Transparent,
        0x6b..=0x73 => JoiningType::Transparent,
        0x80..=0x81 => JoiningType::Transparent,
        0xa2..=0xa5 => JoiningType::Transparent,
        0xa8..=0xa9 => JoiningType::Transparent,
        0xab..=0xad => JoiningType::Transparent,
        0xe6 => JoiningType::Transparent,
        0xe8..=0xe9 => JoiningType::Transparent,
        0xed => JoiningType::Transparent,
        0xef..=0xf1 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p1c(b: u8) -> JoiningType {
    match b {
        0x2c..=0x33 => JoiningType::Transparent,
        0x36..=0x37 => JoiningType::Transparent,
        0xd0..=0xd2 => JoiningType::Transparent,
        0xd4..=0xe0 => JoiningType::Transparent,
        0xe2..=0xe8 => JoiningType::Transparent,
        0xed => JoiningType::Transparent,
        0xf4 => JoiningType::Transparent,
        0xf8..=0xf9 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p1d(b: u8) -> JoiningType {
    match b {
        0xc0..=0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p20(b: u8) -> JoiningType {
    match b {
        0x0b => JoiningType::Transparent,
        0x0d => JoiningType::JoinCausing,
        0x0e..=0x0f => JoiningType::Transparent,
        0x2a..=0x2e => JoiningType::Transparent,
        0x60..=0x64 => JoiningType::Transparent,
        0x6a..=0x6f => JoiningType::Transparent,
        0xd0..=0xf0 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p2c(b: u8) -> JoiningType {
    match b {
        0xef..=0xf1 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p2d(b: u8) -> JoiningType {
    match b {
        0x7f => JoiningType::Transparent,
        0xe0..=0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_p30(b: u8) -> JoiningType {
    match b {
        0x2a..=0x2d => JoiningType::Transparent,
        0x99..=0x9a => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pa6(b: u8) -> JoiningType {
    match b {
        0x6f..=0x72 => JoiningType::Transparent,
        0x74..=0x7d => JoiningType::Transparent,
        0x9e..=0x9f => JoiningType::Transparent,
        0xf0..=0xf1 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pa8(b: u8) -> JoiningType {
    match b {
        0x02 => JoiningType::Transparent,
        0x06 => JoiningType::Transparent,
        0x0b => JoiningType::Transparent,
        0x25..=0x26 => JoiningType::Transparent,
        0x2c => JoiningType::Transparent,
        0x40..=0x71 => JoiningType::DualJoining,
        0x72 => JoiningType::LeftJoining,
        0xc4..=0xc5 => JoiningType::Transparent,
        0xe0..=0xf1 => JoiningType::Transparent,
        0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pa9(b: u8) -> JoiningType {
    match b {
        0x26..=0x2d => JoiningType::Transparent,
        0x47..=0x51 => JoiningType::Transparent,
        0x80..=0x82 => JoiningType::Transparent,
        0xb3 => JoiningType::Transparent,
        0xb6..=0xb9 => JoiningType::Transparent,
        0xbc..=0xbd => JoiningType::Transparent,
        0xe5 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_paa(b: u8) -> JoiningType {
    match b {
        0x29..=0x2e => JoiningType::Transparent,
        0x31..=0x32 => JoiningType::Transparent,
        0x35..=0x36 => JoiningType::Transparent,
        0x43 => JoiningType::Transparent,
        0x4c => JoiningType::Transparent,
        0x7c => JoiningType::Transparent,
        0xb0 => JoiningType::Transparent,
        0xb2..=0xb4 => JoiningType::Transparent,
        0xb7..=0xb8 => JoiningType::Transparent,
        0xbe..=0xbf => JoiningType::Transparent,
        0xc1 => JoiningType::Transparent,
        0xec..=0xed => JoiningType::Transparent,
        0xf6 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pab(b: u8) -> JoiningType {
    match b {
        0xe5 => JoiningType::Transparent,
        0xe8 => JoiningType::Transparent,
        0xed => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pfb(b: u8) -> JoiningType {
    match b {
        0x1e => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pfe(b: u8) -> JoiningType {
    match b {
        0x00..=0x0f => JoiningType::Transparent,
        0x20..=0x2f => JoiningType::Transparent,
        0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "bmp")]
const fn jt_pff(b: u8) -> JoiningType {
    match b {
        0xf9..=0xfb => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p101(b: u8) -> JoiningType {
    match b {
        0xfd => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p102(b: u8) -> JoiningType {
    match b {
        0xe0 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p103(b: u8) -> JoiningType {
    match b {
        0x76..=0x7a => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p10a(b: u8) -> JoiningType {
    match b {
        0x01..=0x03 => JoiningType::Transparent,
        0x05..=0x06 => JoiningType::Transparent,
        0x0c..=0x0f => JoiningType::Transparent,
        0x38..=0x3a => JoiningType::Transparent,
        0x3f => JoiningType::Transparent,
        0xc0..=0xc4 => JoiningType::DualJoining,
        0xc5 => JoiningType::RightJoining,
        0xc7 => JoiningType::RightJoining,
        0xc9..=0xca => JoiningType::RightJoining,
        0xcd => JoiningType::LeftJoining,
        0xce..=0xd2 => JoiningType::RightJoining,
        0xd3..=0xd6 => JoiningType::DualJoining,
        0xd7 => JoiningType::LeftJoining,
        0xd8..=0xdc => JoiningType::DualJoining,
        0xdd => JoiningType::RightJoining,
        0xde..=0xe0 => JoiningType::DualJoining,
        0xe1 => JoiningType::RightJoining,
        0xe4 => JoiningType::RightJoining,
        0xe5..=0xe6 => JoiningType::Transparent,
        0xeb..=0xee => JoiningType::DualJoining,
        0xef => JoiningType::RightJoining,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p10b(b: u8) -> JoiningType {
    match b {
        0x80 => JoiningType::DualJoining,
        0x81 => JoiningType::RightJoining,
        0x82 => JoiningType::DualJoining,
        0x83..=0x85 => JoiningType::RightJoining,
        0x86..=0x88 => JoiningType::DualJoining,
        0x89 => JoiningType::RightJoining,
        0x8a..=0x8b => JoiningType::DualJoining,
        0x8c => JoiningType::RightJoining,
        0x8d => JoiningType::DualJoining,
        0x8e..=0x8f => JoiningType::RightJoining,
        0x90 => JoiningType::DualJoining,
        0x91 => JoiningType::RightJoining,
        0xa9..=0xac => JoiningType::RightJoining,
        0xad..=0xae => JoiningType::DualJoining,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p10d(b: u8) -> JoiningType {
    match b {
        0x00 => JoiningType::LeftJoining,
        0x01..=0x21 => JoiningType::DualJoining,
        0x22 => JoiningType::RightJoining,
        0x23 => JoiningType::DualJoining,
        0x24..=0x27 => JoiningType::Transparent,
        0x69..=0x6d => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p10e(b: u8) -> JoiningType {
    match b {
        0xab..=0xac => JoiningType::Transparent,
        0xc2 => JoiningType::RightJoining,
        0xc3..=0xc4 => JoiningType::DualJoining,
        0xc6..=0xc7 => JoiningType::DualJoining,
        0xfa..=0xff => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p10f(b: u8) -> JoiningType {
    match b {
        0x30..=0x32 => JoiningType::DualJoining,
        0x33 => JoiningType::RightJoining,
        0x34..=0x44 => JoiningType::DualJoining,
        0x46..=0x50 => JoiningType::Transparent,
        0x51..=0x53 => JoiningType::DualJoining,
        0x54 => JoiningType::RightJoining,
        0x70..=0x73 => JoiningType::DualJoining,
        0x74..=0x75 => JoiningType::RightJoining,
        0x76..=0x81 => JoiningType::DualJoining,
        0x82..=0x85 => JoiningType::Transparent,
        0xb0 => JoiningType::DualJoining,
        0xb2..=0xb3 => JoiningType::DualJoining,
        0xb4..=0xb6 => JoiningType::RightJoining,
        0xb8 => JoiningType::DualJoining,
        0xb9..=0xba => JoiningType::RightJoining,
        0xbb..=0xbc => JoiningType::DualJoining,
        0xbd => JoiningType::RightJoining,
        0xbe..=0xbf => JoiningType::DualJoining,
        0xc1 => JoiningType::DualJoining,
        0xc2..=0xc3 => JoiningType::RightJoining,
        0xc4 => JoiningType::DualJoining,
        0xc9 => JoiningType::RightJoining,
        0xca => JoiningType::DualJoining,
        0xcb => JoiningType::LeftJoining,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p110(b: u8) -> JoiningType {
    match b {
        0x01 => JoiningType::Transparent,
        0x38..=0x46 => JoiningType::Transparent,
        0x70 => JoiningType::Transparent,
        0x73..=0x74 => JoiningType::Transparent,
        0x7f..=0x81 => JoiningType::Transparent,
        0xb3..=0xb6 => JoiningType::Transparent,
        0xb9..=0xba => JoiningType::Transparent,
        0xc2 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p111(b: u8) -> JoiningType {
    match b {
        0x00..=0x02 => JoiningType::Transparent,
        0x27..=0x2b => JoiningType::Transparent,
        0x2d..=0x34 => JoiningType::Transparent,
        0x73 => JoiningType::Transparent,
        0x80..=0x81 => JoiningType::Transparent,
        0xb6..=0xbe => JoiningType::Transparent,
        0xc9..=0xcc => JoiningType::Transparent,
        0xcf => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p112(b: u8) -> JoiningType {
    match b {
        0x2f..=0x31 => JoiningType::Transparent,
        0x34 => JoiningType::Transparent,
        0x36..=0x37 => JoiningType::Transparent,
        0x3e => JoiningType::Transparent,
        0x41 => JoiningType::Transparent,
        0xdf => JoiningType::Transparent,
        0xe3..=0xea => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p113(b: u8) -> JoiningType {
    match b {
        0x00..=0x01 => JoiningType::Transparent,
        0x3b..=0x3c => JoiningType::Transparent,
        0x40 => JoiningType::Transparent,
        0x66..=0x6c => JoiningType::Transparent,
        0x70..=0x74 => JoiningType::Transparent,
        0xbb..=0xc0 => JoiningType::Transparent,
        0xce => JoiningType::Transparent,
        0xd0 => JoiningType::Transparent,
        0xd2 => JoiningType::Transparent,
        0xe1..=0xe2 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p114(b: u8) -> JoiningType {
    match b {
        0x38..=0x3f => JoiningType::Transparent,
        0x42..=0x44 => JoiningType::Transparent,
        0x46 => JoiningType::Transparent,
        0x5e => JoiningType::Transparent,
        0xb3..=0xb8 => JoiningType::Transparent,
        0xba => JoiningType::Transparent,
        0xbf..=0xc0 => JoiningType::Transparent,
        0xc2..=0xc3 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p115(b: u8) -> JoiningType {
    match b {
        0xb2..=0xb5 => JoiningType::Transparent,
        0xbc..=0xbd => JoiningType::Transparent,
        0xbf..=0xc0 => JoiningType::Transparent,
        0xdc..=0xdd => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p116(b: u8) -> JoiningType {
    match b {
        0x33..=0x3a => JoiningType::Transparent,
        0x3d => JoiningType::Transparent,
        0x3f..=0x40 => JoiningType::Transparent,
        0xab => JoiningType::Transparent,
        0xad => JoiningType::Transparent,
        0xb0..=0xb5 => JoiningType::Transparent,
        0xb7 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p117(b: u8) -> JoiningType {
    match b {
        0x1d => JoiningType::Transparent,
        0x1f => JoiningType::Transparent,
        0x22..=0x25 => JoiningType::Transparent,
        0x27..=0x2b => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p118(b: u8) -> JoiningType {
    match b {
        0x2f..=0x37 => JoiningType::Transparent,
        0x39..=0x3a => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p119(b: u8) -> JoiningType {
    match b {
        0x3b..=0x3c => JoiningType::Transparent,
        0x3e => JoiningType::Transparent,
        0x43 => JoiningType::Transparent,
        0xd4..=0xd7 => JoiningType::Transparent,
        0xda..=0xdb => JoiningType::Transparent,
        0xe0 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11a(b: u8) -> JoiningType {
    match b {
        0x01..=0x0a => JoiningType::Transparent,
        0x33..=0x38 => JoiningType::Transparent,
        0x3b..=0x3e => JoiningType::Transparent,
        0x47 => JoiningType::Transparent,
        0x51..=0x56 => JoiningType::Transparent,
        0x59..=0x5b => JoiningType::Transparent,
        0x8a..=0x96 => JoiningType::Transparent,
        0x98..=0x99 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11b(b: u8) -> JoiningType {
    match b {
        0x60 => JoiningType::Transparent,
        0x62..=0x64 => JoiningType::Transparent,
        0x66 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11c(b: u8) -> JoiningType {
    match b {
        0x30..=0x36 => JoiningType::Transparent,
        0x38..=0x3d => JoiningType::Transparent,
        0x3f => JoiningType::Transparent,
        0x92..=0xa7 => JoiningType::Transparent,
        0xaa..=0xb0 => JoiningType::Transparent,
        0xb2..=0xb3 => JoiningType::Transparent,
        0xb5..=0xb6 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11d(b: u8) -> JoiningType {
    match b {
        0x31..=0x36 => JoiningType::Transparent,
        0x3a => JoiningType::Transparent,
        0x3c..=0x3d => JoiningType::Transparent,
        0x3f..=0x45 => JoiningType::Transparent,
        0x47 => JoiningType::Transparent,
        0x90..=0x91 => JoiningType::Transparent,
        0x95 => JoiningType::Transparent,
        0x97 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11e(b: u8) -> JoiningType {
    match b {
        0xf3..=0xf4 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p11f(b: u8) -> JoiningType {
    match b {
        0x00..=0x01 => JoiningType::Transparent,
        0x36..=0x3a => JoiningType::Transparent,
        0x40 => JoiningType::Transparent,
        0x42 => JoiningType::Transparent,
        0x5a => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p134(b: u8) -> JoiningType {
    match b {
        0x30..=0x40 => JoiningType::Transparent,
        0x47..=0x55 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p161(b: u8) -> JoiningType {
    match b {
        0x1e..=0x29 => JoiningType::Transparent,
        0x2d..=0x2f => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p16a(b: u8) -> JoiningType {
    match b {
        0xf0..=0xf4 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p16b(b: u8) -> JoiningType {
    match b {
        0x30..=0x36 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p16f(b: u8) -> JoiningType {
    match b {
        0x4f => JoiningType::Transparent,
        0x8f..=0x92 => JoiningType::Transparent,
        0xe4 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1bc(b: u8) -> JoiningType {
    match b {
        0x9d..=0x9e => JoiningType::Transparent,
        0xa0..=0xa3 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1cf(b: u8) -> JoiningType {
    match b {
        0x00..=0x2d => JoiningType::Transparent,
        0x30..=0x46 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1d1(b: u8) -> JoiningType {
    match b {
        0x67..=0x69 => JoiningType::Transparent,
        0x73..=0x82 => JoiningType::Transparent,
        0x85..=0x8b => JoiningType::Transparent,
        0xaa..=0xad => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1d2(b: u8) -> JoiningType {
    match b {
        0x42..=0x44 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1da(b: u8) -> JoiningType {
    match b {
        0x00..=0x36 => JoiningType::Transparent,
        0x3b..=0x6c => JoiningType::Transparent,
        0x75 => JoiningType::Transparent,
        0x84 => JoiningType::Transparent,
        0x9b..=0x9f => JoiningType::Transparent,
        0xa1..=0xaf => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e0(b: u8) -> JoiningType {
    match b {
        0x00..=0x06 => JoiningType::Transparent,
        0x08..=0x18 => JoiningType::Transparent,
        0x1b..=0x21 => JoiningType::Transparent,
        0x23..=0x24 => JoiningType::Transparent,
        0x26..=0x2a => JoiningType::Transparent,
        0x8f => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e1(b: u8) -> JoiningType {
    match b {
        0x30..=0x36 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e2(b: u8) -> JoiningType {
    match b {
        0xae => JoiningType::Transparent,
        0xec..=0xef => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e4(b: u8) -> JoiningType {
    match b {
        0xec..=0xef => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e5(b: u8) -> JoiningType {
    match b {
        0xee..=0xef => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e6(b: u8) -> JoiningType {
    match b {
        0xe3 => JoiningType::Transparent,
        0xe6 => JoiningType::Transparent,
        0xee..=0xef => JoiningType::Transparent,
        0xf5 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e8(b: u8) -> JoiningType {
    match b {
        0xd0..=0xd6 => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_p1e9(b: u8) -> JoiningType {
    match b {
        0x00..=0x43 => JoiningType::DualJoining,
        0x44..=0x4b => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_pe00(b: u8) -> JoiningType {
    match b {
        0x01 => JoiningType::Transparent,
        0x20..=0x7f => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

#[cfg(feature = "full")]
const fn jt_pe01(b: u8) -> JoiningType {
    match b {
        0x00..=0xef => JoiningType::Transparent,
        _ => JoiningType::NonJoining,
    }
}

/// The `Indic_Syllabic_Category` property (UAX #44) for complex-script shaping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicSyllabicCategory {
    Other,
    Avagraha,
    Bindu,
    BrahmiJoiningNumber,
    CantillationMark,
    Consonant,
    ConsonantDead,
    ConsonantFinal,
    ConsonantHeadLetter,
    ConsonantInitialPostfixed,
    ConsonantKiller,
    ConsonantMedial,
    ConsonantPlaceholder,
    ConsonantPrecedingRepha,
    ConsonantPrefixed,
    ConsonantSubjoined,
    ConsonantSucceedingRepha,
    ConsonantWithStacker,
    GeminationMark,
    InvisibleStacker,
    Joiner,
    ModifyingLetter,
    NonJoiner,
    Nukta,
    Number,
    NumberJoiner,
    PureKiller,
    RegisterShifter,
    ReorderingKiller,
    SyllableModifier,
    ToneLetter,
    ToneMark,
    Virama,
    Visarga,
    Vowel,
    VowelDependent,
    VowelIndependent,
}

#[inline]
pub(crate) const fn indic_syllabic_category(cp: u32) -> IndicSyllabicCategory {
    match cp >> 8 {
        #[cfg(feature = "ascii")]
        0x000 => isc_p0(cp as u8),
        #[cfg(feature = "bmp")]
        0x009 => isc_p9(cp as u8),
        #[cfg(feature = "bmp")]
        0x00a => isc_pa(cp as u8),
        #[cfg(feature = "bmp")]
        0x00b => isc_pb(cp as u8),
        #[cfg(feature = "bmp")]
        0x00c => isc_pc(cp as u8),
        #[cfg(feature = "bmp")]
        0x00d => isc_pd(cp as u8),
        #[cfg(feature = "bmp")]
        0x00e => isc_pe(cp as u8),
        #[cfg(feature = "bmp")]
        0x00f => isc_pf(cp as u8),
        #[cfg(feature = "bmp")]
        0x010 => isc_p10(cp as u8),
        #[cfg(feature = "bmp")]
        0x017 => isc_p17(cp as u8),
        #[cfg(feature = "bmp")]
        0x019 => isc_p19(cp as u8),
        #[cfg(feature = "bmp")]
        0x01a => isc_p1a(cp as u8),
        #[cfg(feature = "bmp")]
        0x01b => isc_p1b(cp as u8),
        #[cfg(feature = "bmp")]
        0x01c => isc_p1c(cp as u8),
        #[cfg(feature = "bmp")]
        0x01d => isc_p1d(cp as u8),
        #[cfg(feature = "bmp")]
        0x020 => isc_p20(cp as u8),
        #[cfg(feature = "bmp")]
        0x025 => isc_p25(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a8 => isc_pa8(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a9 => isc_pa9(cp as u8),
        #[cfg(feature = "bmp")]
        0x0aa => isc_paa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ab => isc_pab(cp as u8),
        #[cfg(feature = "full")]
        0x10a => isc_p10a(cp as u8),
        #[cfg(feature = "full")]
        0x110 => isc_p110(cp as u8),
        #[cfg(feature = "full")]
        0x111 => isc_p111(cp as u8),
        #[cfg(feature = "full")]
        0x112 => isc_p112(cp as u8),
        #[cfg(feature = "full")]
        0x113 => isc_p113(cp as u8),
        #[cfg(feature = "full")]
        0x114 => isc_p114(cp as u8),
        #[cfg(feature = "full")]
        0x115 => isc_p115(cp as u8),
        #[cfg(feature = "full")]
        0x116 => isc_p116(cp as u8),
        #[cfg(feature = "full")]
        0x117 => isc_p117(cp as u8),
        #[cfg(feature = "full")]
        0x118 => isc_p118(cp as u8),
        #[cfg(feature = "full")]
        0x119 => isc_p119(cp as u8),
        #[cfg(feature = "full")]
        0x11a => isc_p11a(cp as u8),
        #[cfg(feature = "full")]
        0x11b => isc_p11b(cp as u8),
        #[cfg(feature = "full")]
        0x11c => isc_p11c(cp as u8),
        #[cfg(feature = "full")]
        0x11d => isc_p11d(cp as u8),
        #[cfg(feature = "full")]
        0x11e => isc_p11e(cp as u8),
        #[cfg(feature = "full")]
        0x11f => isc_p11f(cp as u8),
        #[cfg(feature = "full")]
        0x161 => isc_p161(cp as u8),
        #[cfg(feature = "full")]
        0x16d => isc_p16d(cp as u8),
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "ascii")]
const fn isc_p0(b: u8) -> IndicSyllabicCategory {
    match b {
        0x2d => IndicSyllabicCategory::ConsonantPlaceholder,
        0x30..=0x39 => IndicSyllabicCategory::Number,
        #[cfg(feature = "latin1")]
        0xa0 => IndicSyllabicCategory::ConsonantPlaceholder,
        #[cfg(feature = "latin1")]
        0xb2..=0xb3 => IndicSyllabicCategory::SyllableModifier,
        #[cfg(feature = "latin1")]
        0xd7 => IndicSyllabicCategory::ConsonantPlaceholder,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p9(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x04..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x39 => IndicSyllabicCategory::Consonant,
        0x3a..=0x3b => IndicSyllabicCategory::VowelDependent,
        0x3c => IndicSyllabicCategory::Nukta,
        0x3d => IndicSyllabicCategory::Avagraha,
        0x3e..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x4e..=0x4f => IndicSyllabicCategory::VowelDependent,
        0x51..=0x52 => IndicSyllabicCategory::CantillationMark,
        0x55..=0x57 => IndicSyllabicCategory::VowelDependent,
        0x58..=0x5f => IndicSyllabicCategory::Consonant,
        0x60..=0x61 => IndicSyllabicCategory::VowelIndependent,
        0x62..=0x63 => IndicSyllabicCategory::VowelDependent,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x72..=0x77 => IndicSyllabicCategory::VowelIndependent,
        0x78..=0x7f => IndicSyllabicCategory::Consonant,
        0x80 => IndicSyllabicCategory::ConsonantPlaceholder,
        0x81..=0x82 => IndicSyllabicCategory::Bindu,
        0x83 => IndicSyllabicCategory::Visarga,
        0x85..=0x8c => IndicSyllabicCategory::VowelIndependent,
        0x8f..=0x90 => IndicSyllabicCategory::VowelIndependent,
        0x93..=0x94 => IndicSyllabicCategory::VowelIndependent,
        0x95..=0xa8 => IndicSyllabicCategory::Consonant,
        0xaa..=0xb0 => IndicSyllabicCategory::Consonant,
        0xb2 => IndicSyllabicCategory::Consonant,
        0xb6..=0xb9 => IndicSyllabicCategory::Consonant,
        0xbc => IndicSyllabicCategory::Nukta,
        0xbd => IndicSyllabicCategory::Avagraha,
        0xbe..=0xc4 => IndicSyllabicCategory::VowelDependent,
        0xc7..=0xc8 => IndicSyllabicCategory::VowelDependent,
        0xcb..=0xcc => IndicSyllabicCategory::VowelDependent,
        0xcd => IndicSyllabicCategory::Virama,
        0xce => IndicSyllabicCategory::ConsonantDead,
        0xd7 => IndicSyllabicCategory::VowelDependent,
        0xdc..=0xdd => IndicSyllabicCategory::Consonant,
        0xdf => IndicSyllabicCategory::Consonant,
        0xe0..=0xe1 => IndicSyllabicCategory::VowelIndependent,
        0xe2..=0xe3 => IndicSyllabicCategory::VowelDependent,
        0xe6..=0xef => IndicSyllabicCategory::Number,
        0xf0..=0xf1 => IndicSyllabicCategory::Consonant,
        0xfc => IndicSyllabicCategory::Bindu,
        0xfe => IndicSyllabicCategory::SyllableModifier,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pa(b: u8) -> IndicSyllabicCategory {
    match b {
        0x01..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x05..=0x0a => IndicSyllabicCategory::VowelIndependent,
        0x0f..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x13..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x28 => IndicSyllabicCategory::Consonant,
        0x2a..=0x30 => IndicSyllabicCategory::Consonant,
        0x32..=0x33 => IndicSyllabicCategory::Consonant,
        0x35..=0x36 => IndicSyllabicCategory::Consonant,
        0x38..=0x39 => IndicSyllabicCategory::Consonant,
        0x3c => IndicSyllabicCategory::Nukta,
        0x3e..=0x42 => IndicSyllabicCategory::VowelDependent,
        0x47..=0x48 => IndicSyllabicCategory::VowelDependent,
        0x4b..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x51 => IndicSyllabicCategory::CantillationMark,
        0x59..=0x5c => IndicSyllabicCategory::Consonant,
        0x5e => IndicSyllabicCategory::Consonant,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x70 => IndicSyllabicCategory::Bindu,
        0x71 => IndicSyllabicCategory::GeminationMark,
        0x72..=0x73 => IndicSyllabicCategory::ConsonantPlaceholder,
        0x75 => IndicSyllabicCategory::ConsonantMedial,
        0x81..=0x82 => IndicSyllabicCategory::Bindu,
        0x83 => IndicSyllabicCategory::Visarga,
        0x85..=0x8d => IndicSyllabicCategory::VowelIndependent,
        0x8f..=0x91 => IndicSyllabicCategory::VowelIndependent,
        0x93..=0x94 => IndicSyllabicCategory::VowelIndependent,
        0x95..=0xa8 => IndicSyllabicCategory::Consonant,
        0xaa..=0xb0 => IndicSyllabicCategory::Consonant,
        0xb2..=0xb3 => IndicSyllabicCategory::Consonant,
        0xb5..=0xb9 => IndicSyllabicCategory::Consonant,
        0xbc => IndicSyllabicCategory::Nukta,
        0xbd => IndicSyllabicCategory::Avagraha,
        0xbe..=0xc5 => IndicSyllabicCategory::VowelDependent,
        0xc7..=0xc9 => IndicSyllabicCategory::VowelDependent,
        0xcb..=0xcc => IndicSyllabicCategory::VowelDependent,
        0xcd => IndicSyllabicCategory::Virama,
        0xe0..=0xe1 => IndicSyllabicCategory::VowelIndependent,
        0xe2..=0xe3 => IndicSyllabicCategory::VowelDependent,
        0xe6..=0xef => IndicSyllabicCategory::Number,
        0xf9 => IndicSyllabicCategory::Consonant,
        0xfa => IndicSyllabicCategory::CantillationMark,
        0xfb => IndicSyllabicCategory::GeminationMark,
        0xfc => IndicSyllabicCategory::CantillationMark,
        0xfd..=0xff => IndicSyllabicCategory::Nukta,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pb(b: u8) -> IndicSyllabicCategory {
    match b {
        0x01..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x05..=0x0c => IndicSyllabicCategory::VowelIndependent,
        0x0f..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x13..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x28 => IndicSyllabicCategory::Consonant,
        0x2a..=0x30 => IndicSyllabicCategory::Consonant,
        0x32..=0x33 => IndicSyllabicCategory::Consonant,
        0x35..=0x39 => IndicSyllabicCategory::Consonant,
        0x3c => IndicSyllabicCategory::Nukta,
        0x3d => IndicSyllabicCategory::Avagraha,
        0x3e..=0x44 => IndicSyllabicCategory::VowelDependent,
        0x47..=0x48 => IndicSyllabicCategory::VowelDependent,
        0x4b..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x55..=0x57 => IndicSyllabicCategory::VowelDependent,
        0x5c..=0x5d => IndicSyllabicCategory::Consonant,
        0x5f => IndicSyllabicCategory::Consonant,
        0x60..=0x61 => IndicSyllabicCategory::VowelIndependent,
        0x62..=0x63 => IndicSyllabicCategory::VowelDependent,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x71 => IndicSyllabicCategory::Consonant,
        0x82 => IndicSyllabicCategory::Bindu,
        0x83 => IndicSyllabicCategory::ModifyingLetter,
        0x85..=0x8a => IndicSyllabicCategory::VowelIndependent,
        0x8e..=0x90 => IndicSyllabicCategory::VowelIndependent,
        0x92..=0x94 => IndicSyllabicCategory::VowelIndependent,
        0x95 => IndicSyllabicCategory::Consonant,
        0x99..=0x9a => IndicSyllabicCategory::Consonant,
        0x9c => IndicSyllabicCategory::Consonant,
        0x9e..=0x9f => IndicSyllabicCategory::Consonant,
        0xa3..=0xa4 => IndicSyllabicCategory::Consonant,
        0xa8..=0xaa => IndicSyllabicCategory::Consonant,
        0xae..=0xb9 => IndicSyllabicCategory::Consonant,
        0xbe..=0xc2 => IndicSyllabicCategory::VowelDependent,
        0xc6..=0xc8 => IndicSyllabicCategory::VowelDependent,
        0xca..=0xcc => IndicSyllabicCategory::VowelDependent,
        0xcd => IndicSyllabicCategory::Virama,
        0xd7 => IndicSyllabicCategory::VowelDependent,
        0xe6..=0xef => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pc(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x04 => IndicSyllabicCategory::Bindu,
        0x05..=0x0c => IndicSyllabicCategory::VowelIndependent,
        0x0e..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x12..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x28 => IndicSyllabicCategory::Consonant,
        0x2a..=0x39 => IndicSyllabicCategory::Consonant,
        0x3c => IndicSyllabicCategory::Nukta,
        0x3d => IndicSyllabicCategory::Avagraha,
        0x3e..=0x44 => IndicSyllabicCategory::VowelDependent,
        0x46..=0x48 => IndicSyllabicCategory::VowelDependent,
        0x4a..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x55..=0x56 => IndicSyllabicCategory::VowelDependent,
        0x58..=0x5a => IndicSyllabicCategory::Consonant,
        0x5d => IndicSyllabicCategory::ConsonantDead,
        0x60..=0x61 => IndicSyllabicCategory::VowelIndependent,
        0x62..=0x63 => IndicSyllabicCategory::VowelDependent,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x80..=0x82 => IndicSyllabicCategory::Bindu,
        0x83 => IndicSyllabicCategory::Visarga,
        0x85..=0x8c => IndicSyllabicCategory::VowelIndependent,
        0x8e..=0x90 => IndicSyllabicCategory::VowelIndependent,
        0x92..=0x94 => IndicSyllabicCategory::VowelIndependent,
        0x95..=0xa8 => IndicSyllabicCategory::Consonant,
        0xaa..=0xb3 => IndicSyllabicCategory::Consonant,
        0xb5..=0xb9 => IndicSyllabicCategory::Consonant,
        0xbc => IndicSyllabicCategory::Nukta,
        0xbd => IndicSyllabicCategory::Avagraha,
        0xbe..=0xc4 => IndicSyllabicCategory::VowelDependent,
        0xc6..=0xc8 => IndicSyllabicCategory::VowelDependent,
        0xca..=0xcc => IndicSyllabicCategory::VowelDependent,
        0xcd => IndicSyllabicCategory::Virama,
        0xd5..=0xd6 => IndicSyllabicCategory::VowelDependent,
        0xdd => IndicSyllabicCategory::ConsonantDead,
        0xde => IndicSyllabicCategory::Consonant,
        0xe0..=0xe1 => IndicSyllabicCategory::VowelIndependent,
        0xe2..=0xe3 => IndicSyllabicCategory::VowelDependent,
        0xe6..=0xef => IndicSyllabicCategory::Number,
        0xf1..=0xf2 => IndicSyllabicCategory::ConsonantWithStacker,
        0xf3 => IndicSyllabicCategory::Bindu,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pd(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x04 => IndicSyllabicCategory::Bindu,
        0x05..=0x0c => IndicSyllabicCategory::VowelIndependent,
        0x0e..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x12..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x3a => IndicSyllabicCategory::Consonant,
        0x3b..=0x3c => IndicSyllabicCategory::PureKiller,
        0x3d => IndicSyllabicCategory::Avagraha,
        0x3e..=0x44 => IndicSyllabicCategory::VowelDependent,
        0x46..=0x48 => IndicSyllabicCategory::VowelDependent,
        0x4a..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x4e => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0x54..=0x56 => IndicSyllabicCategory::ConsonantDead,
        0x57 => IndicSyllabicCategory::VowelDependent,
        0x5f..=0x61 => IndicSyllabicCategory::VowelIndependent,
        0x62..=0x63 => IndicSyllabicCategory::VowelDependent,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x7a..=0x7f => IndicSyllabicCategory::ConsonantDead,
        0x81..=0x82 => IndicSyllabicCategory::Bindu,
        0x83 => IndicSyllabicCategory::Visarga,
        0x85..=0x96 => IndicSyllabicCategory::VowelIndependent,
        0x9a..=0xb1 => IndicSyllabicCategory::Consonant,
        0xb3..=0xbb => IndicSyllabicCategory::Consonant,
        0xbd => IndicSyllabicCategory::Consonant,
        0xc0..=0xc6 => IndicSyllabicCategory::Consonant,
        0xca => IndicSyllabicCategory::Virama,
        0xcf..=0xd4 => IndicSyllabicCategory::VowelDependent,
        0xd6 => IndicSyllabicCategory::VowelDependent,
        0xd8..=0xdf => IndicSyllabicCategory::VowelDependent,
        0xe6..=0xef => IndicSyllabicCategory::Number,
        0xf2..=0xf3 => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pe(b: u8) -> IndicSyllabicCategory {
    match b {
        0x01..=0x2e => IndicSyllabicCategory::Consonant,
        0x30..=0x39 => IndicSyllabicCategory::VowelDependent,
        0x3a => IndicSyllabicCategory::PureKiller,
        0x40..=0x45 => IndicSyllabicCategory::VowelDependent,
        0x47 => IndicSyllabicCategory::VowelDependent,
        0x48..=0x4b => IndicSyllabicCategory::ToneMark,
        0x4c => IndicSyllabicCategory::ConsonantKiller,
        0x4d => IndicSyllabicCategory::Bindu,
        0x4e => IndicSyllabicCategory::PureKiller,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x81..=0x82 => IndicSyllabicCategory::Consonant,
        0x84 => IndicSyllabicCategory::Consonant,
        0x86..=0x8a => IndicSyllabicCategory::Consonant,
        0x8c..=0xa3 => IndicSyllabicCategory::Consonant,
        0xa5 => IndicSyllabicCategory::Consonant,
        0xa7..=0xae => IndicSyllabicCategory::Consonant,
        0xb0..=0xb9 => IndicSyllabicCategory::VowelDependent,
        0xba => IndicSyllabicCategory::PureKiller,
        0xbb => IndicSyllabicCategory::VowelDependent,
        0xbc..=0xbd => IndicSyllabicCategory::ConsonantMedial,
        0xc0..=0xc4 => IndicSyllabicCategory::VowelDependent,
        0xc8..=0xcb => IndicSyllabicCategory::ToneMark,
        0xcd => IndicSyllabicCategory::Bindu,
        0xce => IndicSyllabicCategory::SyllableModifier,
        0xd0..=0xd9 => IndicSyllabicCategory::Number,
        0xdc..=0xdf => IndicSyllabicCategory::Consonant,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pf(b: u8) -> IndicSyllabicCategory {
    match b {
        0x20..=0x33 => IndicSyllabicCategory::Number,
        0x35 => IndicSyllabicCategory::SyllableModifier,
        0x37 => IndicSyllabicCategory::SyllableModifier,
        0x39 => IndicSyllabicCategory::Nukta,
        0x40..=0x47 => IndicSyllabicCategory::Consonant,
        0x49..=0x6c => IndicSyllabicCategory::Consonant,
        0x71..=0x7d => IndicSyllabicCategory::VowelDependent,
        0x7e => IndicSyllabicCategory::Bindu,
        0x7f => IndicSyllabicCategory::Visarga,
        0x80..=0x81 => IndicSyllabicCategory::VowelDependent,
        0x82..=0x83 => IndicSyllabicCategory::Bindu,
        0x84 => IndicSyllabicCategory::PureKiller,
        0x85 => IndicSyllabicCategory::Avagraha,
        0x88..=0x8c => IndicSyllabicCategory::ConsonantHeadLetter,
        0x8d..=0x97 => IndicSyllabicCategory::ConsonantSubjoined,
        0x99..=0xbc => IndicSyllabicCategory::ConsonantSubjoined,
        0xc6 => IndicSyllabicCategory::SyllableModifier,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p10(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x20 => IndicSyllabicCategory::Consonant,
        0x21..=0x2a => IndicSyllabicCategory::VowelIndependent,
        0x2b..=0x35 => IndicSyllabicCategory::VowelDependent,
        0x36 => IndicSyllabicCategory::Bindu,
        0x37 => IndicSyllabicCategory::ToneMark,
        0x38 => IndicSyllabicCategory::Visarga,
        0x39 => IndicSyllabicCategory::InvisibleStacker,
        0x3a => IndicSyllabicCategory::PureKiller,
        0x3b..=0x3e => IndicSyllabicCategory::ConsonantMedial,
        0x3f => IndicSyllabicCategory::Consonant,
        0x40..=0x49 => IndicSyllabicCategory::Number,
        0x4b => IndicSyllabicCategory::ConsonantPlaceholder,
        0x4e => IndicSyllabicCategory::ConsonantPlaceholder,
        0x50..=0x51 => IndicSyllabicCategory::Consonant,
        0x52..=0x55 => IndicSyllabicCategory::VowelIndependent,
        0x56..=0x59 => IndicSyllabicCategory::VowelDependent,
        0x5a..=0x5d => IndicSyllabicCategory::Consonant,
        0x5e..=0x60 => IndicSyllabicCategory::ConsonantMedial,
        0x61 => IndicSyllabicCategory::Consonant,
        0x62 => IndicSyllabicCategory::VowelDependent,
        0x63..=0x64 => IndicSyllabicCategory::ToneMark,
        0x65..=0x66 => IndicSyllabicCategory::Consonant,
        0x67..=0x68 => IndicSyllabicCategory::VowelDependent,
        0x69..=0x6d => IndicSyllabicCategory::ToneMark,
        0x6e..=0x70 => IndicSyllabicCategory::Consonant,
        0x71..=0x74 => IndicSyllabicCategory::VowelDependent,
        0x75..=0x81 => IndicSyllabicCategory::Consonant,
        0x82 => IndicSyllabicCategory::ConsonantMedial,
        0x83..=0x86 => IndicSyllabicCategory::VowelDependent,
        0x87..=0x8d => IndicSyllabicCategory::ToneMark,
        0x8e => IndicSyllabicCategory::Consonant,
        0x8f => IndicSyllabicCategory::ToneMark,
        0x90..=0x99 => IndicSyllabicCategory::Number,
        0x9a..=0x9b => IndicSyllabicCategory::ToneMark,
        0x9c..=0x9d => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p17(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::VowelIndependent,
        0x03..=0x11 => IndicSyllabicCategory::Consonant,
        0x12..=0x13 => IndicSyllabicCategory::VowelDependent,
        0x14..=0x15 => IndicSyllabicCategory::PureKiller,
        0x1f => IndicSyllabicCategory::Consonant,
        0x20..=0x22 => IndicSyllabicCategory::VowelIndependent,
        0x23..=0x31 => IndicSyllabicCategory::Consonant,
        0x32..=0x33 => IndicSyllabicCategory::VowelDependent,
        0x34 => IndicSyllabicCategory::PureKiller,
        0x40..=0x42 => IndicSyllabicCategory::VowelIndependent,
        0x43..=0x51 => IndicSyllabicCategory::Consonant,
        0x52..=0x53 => IndicSyllabicCategory::VowelDependent,
        0x60..=0x62 => IndicSyllabicCategory::VowelIndependent,
        0x63..=0x6c => IndicSyllabicCategory::Consonant,
        0x6e..=0x70 => IndicSyllabicCategory::Consonant,
        0x72..=0x73 => IndicSyllabicCategory::VowelDependent,
        0x80..=0xa2 => IndicSyllabicCategory::Consonant,
        0xa3..=0xb3 => IndicSyllabicCategory::VowelIndependent,
        0xb6..=0xc5 => IndicSyllabicCategory::VowelDependent,
        0xc6 => IndicSyllabicCategory::Bindu,
        0xc7 => IndicSyllabicCategory::Visarga,
        0xc8 => IndicSyllabicCategory::VowelDependent,
        0xc9..=0xca => IndicSyllabicCategory::RegisterShifter,
        0xcb => IndicSyllabicCategory::SyllableModifier,
        0xcc => IndicSyllabicCategory::ConsonantSucceedingRepha,
        0xcd => IndicSyllabicCategory::ConsonantKiller,
        0xce..=0xd0 => IndicSyllabicCategory::SyllableModifier,
        0xd1 => IndicSyllabicCategory::PureKiller,
        0xd2 => IndicSyllabicCategory::InvisibleStacker,
        0xd3 => IndicSyllabicCategory::SyllableModifier,
        0xdc => IndicSyllabicCategory::Avagraha,
        0xdd => IndicSyllabicCategory::SyllableModifier,
        0xe0..=0xe9 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p19(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x1e => IndicSyllabicCategory::Consonant,
        0x20..=0x28 => IndicSyllabicCategory::VowelDependent,
        0x29..=0x2b => IndicSyllabicCategory::ConsonantSubjoined,
        0x30..=0x31 => IndicSyllabicCategory::ConsonantFinal,
        0x32 => IndicSyllabicCategory::Bindu,
        0x33..=0x39 => IndicSyllabicCategory::ConsonantFinal,
        0x3a => IndicSyllabicCategory::VowelDependent,
        0x3b => IndicSyllabicCategory::SyllableModifier,
        0x46..=0x4f => IndicSyllabicCategory::Number,
        0x50..=0x62 => IndicSyllabicCategory::Consonant,
        0x63..=0x6d => IndicSyllabicCategory::Vowel,
        0x70..=0x74 => IndicSyllabicCategory::ToneLetter,
        0x80..=0xab => IndicSyllabicCategory::Consonant,
        0xb0..=0xc0 => IndicSyllabicCategory::VowelDependent,
        0xc1..=0xc7 => IndicSyllabicCategory::ConsonantFinal,
        0xc8..=0xc9 => IndicSyllabicCategory::ToneMark,
        0xd0..=0xda => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p1a(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x16 => IndicSyllabicCategory::Consonant,
        0x17..=0x1b => IndicSyllabicCategory::VowelDependent,
        0x20..=0x4c => IndicSyllabicCategory::Consonant,
        0x4d..=0x52 => IndicSyllabicCategory::VowelIndependent,
        0x53..=0x54 => IndicSyllabicCategory::Consonant,
        0x55..=0x56 => IndicSyllabicCategory::ConsonantMedial,
        0x57 => IndicSyllabicCategory::ConsonantSubjoined,
        0x58..=0x59 => IndicSyllabicCategory::ConsonantFinal,
        0x5a => IndicSyllabicCategory::ConsonantInitialPostfixed,
        0x5b..=0x5e => IndicSyllabicCategory::ConsonantSubjoined,
        0x60 => IndicSyllabicCategory::InvisibleStacker,
        0x61..=0x73 => IndicSyllabicCategory::VowelDependent,
        0x74 => IndicSyllabicCategory::Bindu,
        0x75..=0x79 => IndicSyllabicCategory::ToneMark,
        0x7a => IndicSyllabicCategory::PureKiller,
        0x7b..=0x7c => IndicSyllabicCategory::SyllableModifier,
        0x7f => IndicSyllabicCategory::SyllableModifier,
        0x80..=0x89 => IndicSyllabicCategory::Number,
        0x90..=0x99 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p1b(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::ConsonantFinal,
        0x04 => IndicSyllabicCategory::Visarga,
        0x05..=0x12 => IndicSyllabicCategory::VowelIndependent,
        0x13..=0x33 => IndicSyllabicCategory::Consonant,
        0x34 => IndicSyllabicCategory::Nukta,
        0x35..=0x43 => IndicSyllabicCategory::VowelDependent,
        0x44 => IndicSyllabicCategory::Virama,
        0x45..=0x4c => IndicSyllabicCategory::Consonant,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x80 => IndicSyllabicCategory::Bindu,
        0x81 => IndicSyllabicCategory::ConsonantFinal,
        0x82 => IndicSyllabicCategory::Visarga,
        0x83..=0x89 => IndicSyllabicCategory::VowelIndependent,
        0x8a..=0xa0 => IndicSyllabicCategory::Consonant,
        0xa1..=0xa3 => IndicSyllabicCategory::ConsonantSubjoined,
        0xa4..=0xa9 => IndicSyllabicCategory::VowelDependent,
        0xaa => IndicSyllabicCategory::PureKiller,
        0xab => IndicSyllabicCategory::InvisibleStacker,
        0xac..=0xad => IndicSyllabicCategory::ConsonantSubjoined,
        0xae..=0xaf => IndicSyllabicCategory::Consonant,
        0xb0..=0xb9 => IndicSyllabicCategory::Number,
        0xba => IndicSyllabicCategory::Avagraha,
        0xbb..=0xbd => IndicSyllabicCategory::Consonant,
        0xbe..=0xbf => IndicSyllabicCategory::ConsonantFinal,
        0xc0..=0xe3 => IndicSyllabicCategory::Consonant,
        0xe4..=0xe5 => IndicSyllabicCategory::VowelIndependent,
        0xe6 => IndicSyllabicCategory::Nukta,
        0xe7..=0xef => IndicSyllabicCategory::VowelDependent,
        0xf0..=0xf1 => IndicSyllabicCategory::ConsonantFinal,
        0xf2..=0xf3 => IndicSyllabicCategory::ReorderingKiller,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p1c(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x23 => IndicSyllabicCategory::Consonant,
        0x24..=0x25 => IndicSyllabicCategory::ConsonantSubjoined,
        0x26..=0x2c => IndicSyllabicCategory::VowelDependent,
        0x2d..=0x33 => IndicSyllabicCategory::ConsonantFinal,
        0x34..=0x35 => IndicSyllabicCategory::Bindu,
        0x36 => IndicSyllabicCategory::SyllableModifier,
        0x37 => IndicSyllabicCategory::Nukta,
        0x40..=0x49 => IndicSyllabicCategory::Number,
        0x4d..=0x4f => IndicSyllabicCategory::Consonant,
        0xd0..=0xd2 => IndicSyllabicCategory::CantillationMark,
        0xd4..=0xe1 => IndicSyllabicCategory::CantillationMark,
        0xf2..=0xf3 => IndicSyllabicCategory::ConsonantDead,
        0xf4 => IndicSyllabicCategory::CantillationMark,
        0xf5..=0xf6 => IndicSyllabicCategory::ConsonantWithStacker,
        0xf7..=0xf9 => IndicSyllabicCategory::CantillationMark,
        0xfa => IndicSyllabicCategory::ConsonantPlaceholder,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p1d(b: u8) -> IndicSyllabicCategory {
    match b {
        0xfb => IndicSyllabicCategory::SyllableModifier,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p20(b: u8) -> IndicSyllabicCategory {
    match b {
        0x0c => IndicSyllabicCategory::NonJoiner,
        0x0d => IndicSyllabicCategory::Joiner,
        0x10..=0x14 => IndicSyllabicCategory::ConsonantPlaceholder,
        0x74 => IndicSyllabicCategory::SyllableModifier,
        0x82..=0x84 => IndicSyllabicCategory::SyllableModifier,
        0xf0 => IndicSyllabicCategory::CantillationMark,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_p25(b: u8) -> IndicSyllabicCategory {
    match b {
        0xcc => IndicSyllabicCategory::ConsonantPlaceholder,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pa8(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x01 => IndicSyllabicCategory::VowelIndependent,
        0x02 => IndicSyllabicCategory::VowelDependent,
        0x03..=0x05 => IndicSyllabicCategory::VowelIndependent,
        0x06 => IndicSyllabicCategory::Virama,
        0x07..=0x0a => IndicSyllabicCategory::Consonant,
        0x0b => IndicSyllabicCategory::Bindu,
        0x0c..=0x22 => IndicSyllabicCategory::Consonant,
        0x23..=0x27 => IndicSyllabicCategory::VowelDependent,
        0x2c => IndicSyllabicCategory::PureKiller,
        0x40..=0x5d => IndicSyllabicCategory::Consonant,
        0x5e..=0x61 => IndicSyllabicCategory::Vowel,
        0x62..=0x65 => IndicSyllabicCategory::Consonant,
        0x66 => IndicSyllabicCategory::Vowel,
        0x67..=0x68 => IndicSyllabicCategory::ConsonantSubjoined,
        0x69..=0x70 => IndicSyllabicCategory::Consonant,
        0x71 => IndicSyllabicCategory::ConsonantSubjoined,
        0x72 => IndicSyllabicCategory::Consonant,
        0x73 => IndicSyllabicCategory::Bindu,
        0x80 => IndicSyllabicCategory::Bindu,
        0x81 => IndicSyllabicCategory::Visarga,
        0x82..=0x91 => IndicSyllabicCategory::VowelIndependent,
        0x92..=0xb3 => IndicSyllabicCategory::Consonant,
        0xb4 => IndicSyllabicCategory::ConsonantMedial,
        0xb5..=0xc3 => IndicSyllabicCategory::VowelDependent,
        0xc4 => IndicSyllabicCategory::Virama,
        0xc5 => IndicSyllabicCategory::Bindu,
        0xd0..=0xd9 => IndicSyllabicCategory::Number,
        0xe0..=0xf1 => IndicSyllabicCategory::CantillationMark,
        0xf2..=0xf3 => IndicSyllabicCategory::Bindu,
        0xfe => IndicSyllabicCategory::VowelIndependent,
        0xff => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pa9(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x09 => IndicSyllabicCategory::Number,
        0x0a..=0x21 => IndicSyllabicCategory::Consonant,
        0x22..=0x2a => IndicSyllabicCategory::Vowel,
        0x2b..=0x2d => IndicSyllabicCategory::ToneMark,
        0x30..=0x46 => IndicSyllabicCategory::Consonant,
        0x47..=0x4e => IndicSyllabicCategory::VowelDependent,
        0x4f..=0x52 => IndicSyllabicCategory::ConsonantFinal,
        0x53 => IndicSyllabicCategory::PureKiller,
        0x80..=0x81 => IndicSyllabicCategory::Bindu,
        0x82 => IndicSyllabicCategory::ConsonantFinal,
        0x83 => IndicSyllabicCategory::Visarga,
        0x84..=0x88 => IndicSyllabicCategory::VowelIndependent,
        0x89..=0x8b => IndicSyllabicCategory::Consonant,
        0x8c..=0x8e => IndicSyllabicCategory::VowelIndependent,
        0x8f..=0xb2 => IndicSyllabicCategory::Consonant,
        0xb3 => IndicSyllabicCategory::Nukta,
        0xb4..=0xbc => IndicSyllabicCategory::VowelDependent,
        0xbd..=0xbf => IndicSyllabicCategory::ConsonantMedial,
        0xc0 => IndicSyllabicCategory::Virama,
        0xd0..=0xd9 => IndicSyllabicCategory::Number,
        0xe0..=0xe4 => IndicSyllabicCategory::Consonant,
        0xe5 => IndicSyllabicCategory::VowelDependent,
        0xe7..=0xef => IndicSyllabicCategory::Consonant,
        0xf0..=0xf9 => IndicSyllabicCategory::Number,
        0xfa..=0xfe => IndicSyllabicCategory::Consonant,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_paa(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x05 => IndicSyllabicCategory::VowelIndependent,
        0x06..=0x28 => IndicSyllabicCategory::Consonant,
        0x29..=0x32 => IndicSyllabicCategory::VowelDependent,
        0x33..=0x36 => IndicSyllabicCategory::ConsonantMedial,
        0x40..=0x4d => IndicSyllabicCategory::ConsonantFinal,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x60..=0x6f => IndicSyllabicCategory::Consonant,
        0x71..=0x73 => IndicSyllabicCategory::Consonant,
        0x74..=0x76 => IndicSyllabicCategory::ConsonantPlaceholder,
        0x7a => IndicSyllabicCategory::Consonant,
        0x7b..=0x7d => IndicSyllabicCategory::ToneMark,
        0x7e..=0xaf => IndicSyllabicCategory::Consonant,
        0xb0..=0xbe => IndicSyllabicCategory::VowelDependent,
        0xbf => IndicSyllabicCategory::ToneMark,
        0xc0 => IndicSyllabicCategory::ToneLetter,
        0xc1 => IndicSyllabicCategory::ToneMark,
        0xc2 => IndicSyllabicCategory::ToneLetter,
        0xe0..=0xe1 => IndicSyllabicCategory::VowelIndependent,
        0xe2..=0xea => IndicSyllabicCategory::Consonant,
        0xeb..=0xef => IndicSyllabicCategory::VowelDependent,
        0xf5 => IndicSyllabicCategory::Visarga,
        0xf6 => IndicSyllabicCategory::InvisibleStacker,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "bmp")]
const fn isc_pab(b: u8) -> IndicSyllabicCategory {
    match b {
        0xc0..=0xcd => IndicSyllabicCategory::Consonant,
        0xce..=0xcf => IndicSyllabicCategory::VowelIndependent,
        0xd0 => IndicSyllabicCategory::Consonant,
        0xd1 => IndicSyllabicCategory::VowelIndependent,
        0xd2..=0xda => IndicSyllabicCategory::Consonant,
        0xdb..=0xe2 => IndicSyllabicCategory::ConsonantFinal,
        0xe3..=0xea => IndicSyllabicCategory::VowelDependent,
        0xec => IndicSyllabicCategory::ToneMark,
        0xed => IndicSyllabicCategory::PureKiller,
        0xf0..=0xf9 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p10a(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00 => IndicSyllabicCategory::Consonant,
        0x01..=0x03 => IndicSyllabicCategory::VowelDependent,
        0x05..=0x06 => IndicSyllabicCategory::VowelDependent,
        0x0c..=0x0d => IndicSyllabicCategory::VowelDependent,
        0x0e => IndicSyllabicCategory::Bindu,
        0x0f => IndicSyllabicCategory::Visarga,
        0x10..=0x13 => IndicSyllabicCategory::Consonant,
        0x15..=0x17 => IndicSyllabicCategory::Consonant,
        0x19..=0x35 => IndicSyllabicCategory::Consonant,
        0x38..=0x3a => IndicSyllabicCategory::Nukta,
        0x3f => IndicSyllabicCategory::InvisibleStacker,
        0x40..=0x48 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p110(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x01 => IndicSyllabicCategory::Bindu,
        0x02 => IndicSyllabicCategory::Visarga,
        0x03..=0x04 => IndicSyllabicCategory::ConsonantWithStacker,
        0x05..=0x12 => IndicSyllabicCategory::VowelIndependent,
        0x13..=0x37 => IndicSyllabicCategory::Consonant,
        0x38..=0x45 => IndicSyllabicCategory::VowelDependent,
        0x46 => IndicSyllabicCategory::Virama,
        0x52..=0x65 => IndicSyllabicCategory::BrahmiJoiningNumber,
        0x66..=0x6f => IndicSyllabicCategory::Number,
        0x70 => IndicSyllabicCategory::PureKiller,
        0x71..=0x72 => IndicSyllabicCategory::VowelIndependent,
        0x73..=0x74 => IndicSyllabicCategory::VowelDependent,
        0x75 => IndicSyllabicCategory::Consonant,
        0x7f => IndicSyllabicCategory::NumberJoiner,
        0x80..=0x81 => IndicSyllabicCategory::Bindu,
        0x82 => IndicSyllabicCategory::Visarga,
        0x83..=0x8c => IndicSyllabicCategory::VowelIndependent,
        0x8d..=0xaf => IndicSyllabicCategory::Consonant,
        0xb0..=0xb8 => IndicSyllabicCategory::VowelDependent,
        0xb9 => IndicSyllabicCategory::Virama,
        0xba => IndicSyllabicCategory::Nukta,
        0xc2 => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p111(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x01 => IndicSyllabicCategory::Bindu,
        0x02 => IndicSyllabicCategory::Visarga,
        0x03..=0x06 => IndicSyllabicCategory::VowelIndependent,
        0x07..=0x26 => IndicSyllabicCategory::Consonant,
        0x27..=0x32 => IndicSyllabicCategory::VowelDependent,
        0x33 => IndicSyllabicCategory::InvisibleStacker,
        0x34 => IndicSyllabicCategory::PureKiller,
        0x36..=0x3f => IndicSyllabicCategory::Number,
        0x44 => IndicSyllabicCategory::Consonant,
        0x45..=0x46 => IndicSyllabicCategory::VowelDependent,
        0x47 => IndicSyllabicCategory::Consonant,
        0x50..=0x54 => IndicSyllabicCategory::Vowel,
        0x55..=0x72 => IndicSyllabicCategory::Consonant,
        0x73 => IndicSyllabicCategory::Nukta,
        0x80..=0x81 => IndicSyllabicCategory::Bindu,
        0x82 => IndicSyllabicCategory::Visarga,
        0x83..=0x90 => IndicSyllabicCategory::VowelIndependent,
        0x91..=0xb2 => IndicSyllabicCategory::Consonant,
        0xb3..=0xbf => IndicSyllabicCategory::VowelDependent,
        0xc0 => IndicSyllabicCategory::Virama,
        0xc1 => IndicSyllabicCategory::Avagraha,
        0xc2..=0xc3 => IndicSyllabicCategory::ConsonantPrefixed,
        0xc9 => IndicSyllabicCategory::SyllableModifier,
        0xca => IndicSyllabicCategory::Nukta,
        0xcb..=0xcc => IndicSyllabicCategory::VowelDependent,
        0xce => IndicSyllabicCategory::VowelDependent,
        0xcf => IndicSyllabicCategory::Bindu,
        0xd0..=0xd9 => IndicSyllabicCategory::Number,
        0xe1..=0xf4 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p112(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x07 => IndicSyllabicCategory::VowelIndependent,
        0x08..=0x11 => IndicSyllabicCategory::Consonant,
        0x13..=0x2b => IndicSyllabicCategory::Consonant,
        0x2c..=0x33 => IndicSyllabicCategory::VowelDependent,
        0x34 => IndicSyllabicCategory::Bindu,
        0x35 => IndicSyllabicCategory::Virama,
        0x36 => IndicSyllabicCategory::Nukta,
        0x37 => IndicSyllabicCategory::GeminationMark,
        0x3e => IndicSyllabicCategory::CantillationMark,
        0x3f => IndicSyllabicCategory::Consonant,
        0x40 => IndicSyllabicCategory::VowelIndependent,
        0x41 => IndicSyllabicCategory::VowelDependent,
        0x80..=0x83 => IndicSyllabicCategory::VowelIndependent,
        0x84..=0x86 => IndicSyllabicCategory::Consonant,
        0x88 => IndicSyllabicCategory::Consonant,
        0x8a..=0x8d => IndicSyllabicCategory::Consonant,
        0x8f..=0x9d => IndicSyllabicCategory::Consonant,
        0x9f..=0xa8 => IndicSyllabicCategory::Consonant,
        0xb0..=0xb9 => IndicSyllabicCategory::VowelIndependent,
        0xba..=0xde => IndicSyllabicCategory::Consonant,
        0xdf => IndicSyllabicCategory::Bindu,
        0xe0..=0xe8 => IndicSyllabicCategory::VowelDependent,
        0xe9 => IndicSyllabicCategory::Nukta,
        0xea => IndicSyllabicCategory::PureKiller,
        0xf0..=0xf9 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p113(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x02 => IndicSyllabicCategory::Bindu,
        0x03 => IndicSyllabicCategory::Visarga,
        0x05..=0x0c => IndicSyllabicCategory::VowelIndependent,
        0x0f..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x13..=0x14 => IndicSyllabicCategory::VowelIndependent,
        0x15..=0x28 => IndicSyllabicCategory::Consonant,
        0x2a..=0x30 => IndicSyllabicCategory::Consonant,
        0x32..=0x33 => IndicSyllabicCategory::Consonant,
        0x35..=0x39 => IndicSyllabicCategory::Consonant,
        0x3b..=0x3c => IndicSyllabicCategory::Nukta,
        0x3d => IndicSyllabicCategory::Avagraha,
        0x3e..=0x44 => IndicSyllabicCategory::VowelDependent,
        0x47..=0x48 => IndicSyllabicCategory::VowelDependent,
        0x4b..=0x4c => IndicSyllabicCategory::VowelDependent,
        0x4d => IndicSyllabicCategory::Virama,
        0x57 => IndicSyllabicCategory::VowelDependent,
        0x5e..=0x5f => IndicSyllabicCategory::Bindu,
        0x60..=0x61 => IndicSyllabicCategory::VowelIndependent,
        0x62..=0x63 => IndicSyllabicCategory::VowelDependent,
        0x66..=0x6c => IndicSyllabicCategory::CantillationMark,
        0x70..=0x74 => IndicSyllabicCategory::CantillationMark,
        0x80..=0x89 => IndicSyllabicCategory::VowelIndependent,
        0x8b => IndicSyllabicCategory::VowelIndependent,
        0x8e => IndicSyllabicCategory::VowelIndependent,
        0x90..=0x91 => IndicSyllabicCategory::VowelIndependent,
        0x92..=0xb5 => IndicSyllabicCategory::Consonant,
        0xb7 => IndicSyllabicCategory::Avagraha,
        0xb8..=0xc0 => IndicSyllabicCategory::VowelDependent,
        0xc2 => IndicSyllabicCategory::VowelDependent,
        0xc5 => IndicSyllabicCategory::VowelDependent,
        0xc7..=0xc9 => IndicSyllabicCategory::VowelDependent,
        0xca => IndicSyllabicCategory::Bindu,
        0xcc => IndicSyllabicCategory::Bindu,
        0xcd => IndicSyllabicCategory::Visarga,
        0xce..=0xcf => IndicSyllabicCategory::PureKiller,
        0xd0 => IndicSyllabicCategory::InvisibleStacker,
        0xd1 => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0xd2 => IndicSyllabicCategory::GeminationMark,
        0xe1..=0xe2 => IndicSyllabicCategory::CantillationMark,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p114(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x0d => IndicSyllabicCategory::VowelIndependent,
        0x0e..=0x34 => IndicSyllabicCategory::Consonant,
        0x35..=0x41 => IndicSyllabicCategory::VowelDependent,
        0x42 => IndicSyllabicCategory::Virama,
        0x43..=0x44 => IndicSyllabicCategory::Bindu,
        0x45 => IndicSyllabicCategory::Visarga,
        0x46 => IndicSyllabicCategory::Nukta,
        0x47 => IndicSyllabicCategory::Avagraha,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x5e => IndicSyllabicCategory::SyllableModifier,
        0x5f => IndicSyllabicCategory::Bindu,
        0x60..=0x61 => IndicSyllabicCategory::ConsonantWithStacker,
        0x81..=0x8e => IndicSyllabicCategory::VowelIndependent,
        0x8f..=0xaf => IndicSyllabicCategory::Consonant,
        0xb0..=0xbe => IndicSyllabicCategory::VowelDependent,
        0xbf..=0xc0 => IndicSyllabicCategory::Bindu,
        0xc1 => IndicSyllabicCategory::Visarga,
        0xc2 => IndicSyllabicCategory::Virama,
        0xc3 => IndicSyllabicCategory::Nukta,
        0xc4 => IndicSyllabicCategory::Avagraha,
        0xd0..=0xd9 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p115(b: u8) -> IndicSyllabicCategory {
    match b {
        0x80..=0x8d => IndicSyllabicCategory::VowelIndependent,
        0x8e..=0xae => IndicSyllabicCategory::Consonant,
        0xaf..=0xb5 => IndicSyllabicCategory::VowelDependent,
        0xb8..=0xbb => IndicSyllabicCategory::VowelDependent,
        0xbc..=0xbd => IndicSyllabicCategory::Bindu,
        0xbe => IndicSyllabicCategory::Visarga,
        0xbf => IndicSyllabicCategory::Virama,
        0xc0 => IndicSyllabicCategory::Nukta,
        0xd8..=0xdb => IndicSyllabicCategory::VowelIndependent,
        0xdc..=0xdd => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p116(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x0d => IndicSyllabicCategory::VowelIndependent,
        0x0e..=0x2f => IndicSyllabicCategory::Consonant,
        0x30..=0x3c => IndicSyllabicCategory::VowelDependent,
        0x3d => IndicSyllabicCategory::Bindu,
        0x3e => IndicSyllabicCategory::Visarga,
        0x3f => IndicSyllabicCategory::Virama,
        0x40 => IndicSyllabicCategory::VowelDependent,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x80..=0x89 => IndicSyllabicCategory::VowelIndependent,
        0x8a..=0xaa => IndicSyllabicCategory::Consonant,
        0xab => IndicSyllabicCategory::Bindu,
        0xac => IndicSyllabicCategory::Visarga,
        0xad..=0xb5 => IndicSyllabicCategory::VowelDependent,
        0xb6 => IndicSyllabicCategory::Virama,
        0xb7 => IndicSyllabicCategory::Nukta,
        0xb8 => IndicSyllabicCategory::Consonant,
        0xc0..=0xc9 => IndicSyllabicCategory::Number,
        0xd0..=0xe3 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p117(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x1a => IndicSyllabicCategory::Consonant,
        0x1d..=0x1f => IndicSyllabicCategory::ConsonantMedial,
        0x20..=0x2a => IndicSyllabicCategory::VowelDependent,
        0x2b => IndicSyllabicCategory::PureKiller,
        0x30..=0x3b => IndicSyllabicCategory::Number,
        0x40..=0x46 => IndicSyllabicCategory::Consonant,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p118(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x09 => IndicSyllabicCategory::VowelIndependent,
        0x0a..=0x2b => IndicSyllabicCategory::Consonant,
        0x2c..=0x36 => IndicSyllabicCategory::VowelDependent,
        0x37 => IndicSyllabicCategory::Bindu,
        0x38 => IndicSyllabicCategory::Visarga,
        0x39 => IndicSyllabicCategory::Virama,
        0x3a => IndicSyllabicCategory::Nukta,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p119(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x06 => IndicSyllabicCategory::VowelIndependent,
        0x09 => IndicSyllabicCategory::VowelIndependent,
        0x0c..=0x13 => IndicSyllabicCategory::Consonant,
        0x15..=0x16 => IndicSyllabicCategory::Consonant,
        0x18..=0x2f => IndicSyllabicCategory::Consonant,
        0x30..=0x35 => IndicSyllabicCategory::VowelDependent,
        0x37..=0x38 => IndicSyllabicCategory::VowelDependent,
        0x3b..=0x3c => IndicSyllabicCategory::Bindu,
        0x3d => IndicSyllabicCategory::PureKiller,
        0x3e => IndicSyllabicCategory::InvisibleStacker,
        0x3f => IndicSyllabicCategory::ConsonantPrefixed,
        0x40 => IndicSyllabicCategory::ConsonantMedial,
        0x41 => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0x42 => IndicSyllabicCategory::ConsonantMedial,
        0x43 => IndicSyllabicCategory::Nukta,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0xa0..=0xa7 => IndicSyllabicCategory::VowelIndependent,
        0xaa..=0xad => IndicSyllabicCategory::VowelIndependent,
        0xae..=0xd0 => IndicSyllabicCategory::Consonant,
        0xd1..=0xd7 => IndicSyllabicCategory::VowelDependent,
        0xda..=0xdd => IndicSyllabicCategory::VowelDependent,
        0xde => IndicSyllabicCategory::Bindu,
        0xdf => IndicSyllabicCategory::Visarga,
        0xe0 => IndicSyllabicCategory::Virama,
        0xe1 => IndicSyllabicCategory::Avagraha,
        0xe4 => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11a(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00 => IndicSyllabicCategory::Consonant,
        0x01..=0x0a => IndicSyllabicCategory::VowelDependent,
        0x0b..=0x32 => IndicSyllabicCategory::Consonant,
        0x33 => IndicSyllabicCategory::SyllableModifier,
        0x34 => IndicSyllabicCategory::PureKiller,
        0x35..=0x38 => IndicSyllabicCategory::Bindu,
        0x39 => IndicSyllabicCategory::Visarga,
        0x3a => IndicSyllabicCategory::ConsonantWithStacker,
        0x3b..=0x3e => IndicSyllabicCategory::ConsonantMedial,
        0x3f => IndicSyllabicCategory::ConsonantPlaceholder,
        0x45 => IndicSyllabicCategory::ConsonantPlaceholder,
        0x47 => IndicSyllabicCategory::InvisibleStacker,
        0x50 => IndicSyllabicCategory::Consonant,
        0x51..=0x5b => IndicSyllabicCategory::VowelDependent,
        0x5c..=0x83 => IndicSyllabicCategory::Consonant,
        0x84..=0x85 => IndicSyllabicCategory::ConsonantPrefixed,
        0x86 => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0x87..=0x89 => IndicSyllabicCategory::ConsonantPrefixed,
        0x8a..=0x95 => IndicSyllabicCategory::ConsonantFinal,
        0x96 => IndicSyllabicCategory::Bindu,
        0x97 => IndicSyllabicCategory::Visarga,
        0x98 => IndicSyllabicCategory::GeminationMark,
        0x99 => IndicSyllabicCategory::InvisibleStacker,
        0x9d => IndicSyllabicCategory::Avagraha,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11b(b: u8) -> IndicSyllabicCategory {
    match b {
        0x60..=0x67 => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11c(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x08 => IndicSyllabicCategory::VowelIndependent,
        0x0a..=0x0d => IndicSyllabicCategory::VowelIndependent,
        0x0e..=0x2e => IndicSyllabicCategory::Consonant,
        0x2f..=0x36 => IndicSyllabicCategory::VowelDependent,
        0x38..=0x3b => IndicSyllabicCategory::VowelDependent,
        0x3c..=0x3d => IndicSyllabicCategory::Bindu,
        0x3e => IndicSyllabicCategory::Visarga,
        0x3f => IndicSyllabicCategory::Virama,
        0x40 => IndicSyllabicCategory::Avagraha,
        0x50..=0x6c => IndicSyllabicCategory::Number,
        0x72..=0x8f => IndicSyllabicCategory::Consonant,
        0x92..=0xa7 => IndicSyllabicCategory::ConsonantSubjoined,
        0xa9..=0xaf => IndicSyllabicCategory::ConsonantSubjoined,
        0xb0..=0xb4 => IndicSyllabicCategory::VowelDependent,
        0xb5..=0xb6 => IndicSyllabicCategory::Bindu,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11d(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x06 => IndicSyllabicCategory::VowelIndependent,
        0x08..=0x09 => IndicSyllabicCategory::VowelIndependent,
        0x0b => IndicSyllabicCategory::VowelIndependent,
        0x0c..=0x30 => IndicSyllabicCategory::Consonant,
        0x31..=0x36 => IndicSyllabicCategory::VowelDependent,
        0x3a => IndicSyllabicCategory::VowelDependent,
        0x3c..=0x3d => IndicSyllabicCategory::VowelDependent,
        0x3f => IndicSyllabicCategory::VowelDependent,
        0x40 => IndicSyllabicCategory::Bindu,
        0x41 => IndicSyllabicCategory::Visarga,
        0x42 => IndicSyllabicCategory::Nukta,
        0x43 => IndicSyllabicCategory::VowelDependent,
        0x44 => IndicSyllabicCategory::PureKiller,
        0x45 => IndicSyllabicCategory::InvisibleStacker,
        0x46 => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0x47 => IndicSyllabicCategory::ConsonantMedial,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x60..=0x65 => IndicSyllabicCategory::VowelIndependent,
        0x67..=0x68 => IndicSyllabicCategory::VowelIndependent,
        0x6a..=0x6b => IndicSyllabicCategory::VowelIndependent,
        0x6c..=0x89 => IndicSyllabicCategory::Consonant,
        0x8a..=0x8e => IndicSyllabicCategory::VowelDependent,
        0x90..=0x91 => IndicSyllabicCategory::VowelDependent,
        0x93..=0x94 => IndicSyllabicCategory::VowelDependent,
        0x95 => IndicSyllabicCategory::Bindu,
        0x96 => IndicSyllabicCategory::Visarga,
        0x97 => IndicSyllabicCategory::InvisibleStacker,
        0xa0..=0xa9 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11e(b: u8) -> IndicSyllabicCategory {
    match b {
        0xe0..=0xf1 => IndicSyllabicCategory::Consonant,
        0xf2 => IndicSyllabicCategory::ConsonantPlaceholder,
        0xf3..=0xf6 => IndicSyllabicCategory::VowelDependent,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p11f(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00..=0x01 => IndicSyllabicCategory::Bindu,
        0x02 => IndicSyllabicCategory::ConsonantPrecedingRepha,
        0x03 => IndicSyllabicCategory::Visarga,
        0x04..=0x10 => IndicSyllabicCategory::VowelIndependent,
        0x12..=0x33 => IndicSyllabicCategory::Consonant,
        0x34..=0x3a => IndicSyllabicCategory::VowelDependent,
        0x3e..=0x40 => IndicSyllabicCategory::VowelDependent,
        0x41 => IndicSyllabicCategory::PureKiller,
        0x42 => IndicSyllabicCategory::InvisibleStacker,
        0x50..=0x59 => IndicSyllabicCategory::Number,
        0x5a => IndicSyllabicCategory::Nukta,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p161(b: u8) -> IndicSyllabicCategory {
    match b {
        0x00 => IndicSyllabicCategory::VowelIndependent,
        0x01..=0x1d => IndicSyllabicCategory::Consonant,
        0x1e..=0x29 => IndicSyllabicCategory::VowelDependent,
        0x2a..=0x2c => IndicSyllabicCategory::ConsonantMedial,
        0x2d => IndicSyllabicCategory::Bindu,
        0x2e => IndicSyllabicCategory::ConsonantMedial,
        0x2f => IndicSyllabicCategory::PureKiller,
        0x30..=0x39 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

#[cfg(feature = "full")]
const fn isc_p16d(b: u8) -> IndicSyllabicCategory {
    match b {
        0x40..=0x41 => IndicSyllabicCategory::Bindu,
        0x42 => IndicSyllabicCategory::Visarga,
        0x43..=0x62 => IndicSyllabicCategory::Consonant,
        0x63..=0x6a => IndicSyllabicCategory::VowelDependent,
        0x6b..=0x6c => IndicSyllabicCategory::PureKiller,
        0x70..=0x79 => IndicSyllabicCategory::Number,
        _ => IndicSyllabicCategory::Other,
    }
}

/// The `Indic_Positional_Category` property (UAX #44): where a dependent character is positioned relative to its base.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndicPositionalCategory {
    NotApplicable,
    Bottom,
    BottomAndLeft,
    BottomAndRight,
    Left,
    LeftAndRight,
    Overstruck,
    Right,
    Top,
    TopAndBottom,
    TopAndBottomAndLeft,
    TopAndBottomAndRight,
    TopAndLeft,
    TopAndLeftAndRight,
    TopAndRight,
    VisualOrderLeft,
}

#[inline]
pub(crate) const fn indic_positional_category(cp: u32) -> IndicPositionalCategory {
    match cp >> 8 {
        #[cfg(feature = "ascii")]
        0x000 => ipc_p0(cp as u8),
        #[cfg(feature = "bmp")]
        0x009 => ipc_p9(cp as u8),
        #[cfg(feature = "bmp")]
        0x00a => ipc_pa(cp as u8),
        #[cfg(feature = "bmp")]
        0x00b => ipc_pb(cp as u8),
        #[cfg(feature = "bmp")]
        0x00c => ipc_pc(cp as u8),
        #[cfg(feature = "bmp")]
        0x00d => ipc_pd(cp as u8),
        #[cfg(feature = "bmp")]
        0x00e => ipc_pe(cp as u8),
        #[cfg(feature = "bmp")]
        0x00f => ipc_pf(cp as u8),
        #[cfg(feature = "bmp")]
        0x010 => ipc_p10(cp as u8),
        #[cfg(feature = "bmp")]
        0x017 => ipc_p17(cp as u8),
        #[cfg(feature = "bmp")]
        0x019 => ipc_p19(cp as u8),
        #[cfg(feature = "bmp")]
        0x01a => ipc_p1a(cp as u8),
        #[cfg(feature = "bmp")]
        0x01b => ipc_p1b(cp as u8),
        #[cfg(feature = "bmp")]
        0x01c => ipc_p1c(cp as u8),
        #[cfg(feature = "bmp")]
        0x01d => ipc_p1d(cp as u8),
        #[cfg(feature = "bmp")]
        0x020 => ipc_p20(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a8 => ipc_pa8(cp as u8),
        #[cfg(feature = "bmp")]
        0x0a9 => ipc_pa9(cp as u8),
        #[cfg(feature = "bmp")]
        0x0aa => ipc_paa(cp as u8),
        #[cfg(feature = "bmp")]
        0x0ab => ipc_pab(cp as u8),
        #[cfg(feature = "full")]
        0x10a => ipc_p10a(cp as u8),
        #[cfg(feature = "full")]
        0x110 => ipc_p110(cp as u8),
        #[cfg(feature = "full")]
        0x111 => ipc_p111(cp as u8),
        #[cfg(feature = "full")]
        0x112 => ipc_p112(cp as u8),
        #[cfg(feature = "full")]
        0x113 => ipc_p113(cp as u8),
        #[cfg(feature = "full")]
        0x114 => ipc_p114(cp as u8),
        #[cfg(feature = "full")]
        0x115 => ipc_p115(cp as u8),
        #[cfg(feature = "full")]
        0x116 => ipc_p116(cp as u8),
        #[cfg(feature = "full")]
        0x117 => ipc_p117(cp as u8),
        #[cfg(feature = "full")]
        0x118 => ipc_p118(cp as u8),
        #[cfg(feature = "full")]
        0x119 => ipc_p119(cp as u8),
        #[cfg(feature = "full")]
        0x11a => ipc_p11a(cp as u8),
        #[cfg(feature = "full")]
        0x11b => ipc_p11b(cp as u8),
        #[cfg(feature = "full")]
        0x11c => ipc_p11c(cp as u8),
        #[cfg(feature = "full")]
        0x11d => ipc_p11d(cp as u8),
        #[cfg(feature = "full")]
        0x11e => ipc_p11e(cp as u8),
        #[cfg(feature = "full")]
        0x11f => ipc_p11f(cp as u8),
        #[cfg(feature = "full")]
        0x161 => ipc_p161(cp as u8),
        #[cfg(feature = "full")]
        0x16d => ipc_p16d(cp as u8),
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "ascii")]
const fn ipc_p0(b: u8) -> IndicPositionalCategory {
    match b {
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p9(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x02 => IndicPositionalCategory::Top,
        0x03 => IndicPositionalCategory::Right,
        0x3a => IndicPositionalCategory::Top,
        0x3b => IndicPositionalCategory::Right,
        0x3c => IndicPositionalCategory::Bottom,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Left,
        0x40 => IndicPositionalCategory::Right,
        0x41..=0x44 => IndicPositionalCategory::Bottom,
        0x45..=0x48 => IndicPositionalCategory::Top,
        0x49..=0x4c => IndicPositionalCategory::Right,
        0x4d => IndicPositionalCategory::Bottom,
        0x4e => IndicPositionalCategory::Left,
        0x4f => IndicPositionalCategory::Right,
        0x51 => IndicPositionalCategory::Top,
        0x52 => IndicPositionalCategory::Bottom,
        0x55 => IndicPositionalCategory::Top,
        0x56..=0x57 => IndicPositionalCategory::Bottom,
        0x62..=0x63 => IndicPositionalCategory::Bottom,
        0x81 => IndicPositionalCategory::Top,
        0x82..=0x83 => IndicPositionalCategory::Right,
        0xbc => IndicPositionalCategory::Bottom,
        0xbe => IndicPositionalCategory::Right,
        0xbf => IndicPositionalCategory::Left,
        0xc0 => IndicPositionalCategory::Right,
        0xc1..=0xc4 => IndicPositionalCategory::Bottom,
        0xc7..=0xc8 => IndicPositionalCategory::Left,
        0xcb..=0xcc => IndicPositionalCategory::LeftAndRight,
        0xcd => IndicPositionalCategory::Bottom,
        0xd7 => IndicPositionalCategory::Right,
        0xe2..=0xe3 => IndicPositionalCategory::Bottom,
        0xfe => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pa(b: u8) -> IndicPositionalCategory {
    match b {
        0x01..=0x02 => IndicPositionalCategory::Top,
        0x03 => IndicPositionalCategory::Right,
        0x3c => IndicPositionalCategory::Bottom,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Left,
        0x40 => IndicPositionalCategory::Right,
        0x41..=0x42 => IndicPositionalCategory::Bottom,
        0x47..=0x48 => IndicPositionalCategory::Top,
        0x4b..=0x4c => IndicPositionalCategory::Top,
        0x4d => IndicPositionalCategory::Bottom,
        0x51 => IndicPositionalCategory::Bottom,
        0x70..=0x71 => IndicPositionalCategory::Top,
        0x75 => IndicPositionalCategory::Bottom,
        0x81..=0x82 => IndicPositionalCategory::Top,
        0x83 => IndicPositionalCategory::Right,
        0xbc => IndicPositionalCategory::Bottom,
        0xbe => IndicPositionalCategory::Right,
        0xbf => IndicPositionalCategory::Left,
        0xc0 => IndicPositionalCategory::Right,
        0xc1..=0xc4 => IndicPositionalCategory::Bottom,
        0xc5 => IndicPositionalCategory::Top,
        0xc7..=0xc8 => IndicPositionalCategory::Top,
        0xc9 => IndicPositionalCategory::TopAndRight,
        0xcb..=0xcc => IndicPositionalCategory::Right,
        0xcd => IndicPositionalCategory::Bottom,
        0xe2..=0xe3 => IndicPositionalCategory::Bottom,
        0xfa..=0xff => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pb(b: u8) -> IndicPositionalCategory {
    match b {
        0x01 => IndicPositionalCategory::Top,
        0x02..=0x03 => IndicPositionalCategory::Right,
        0x3c => IndicPositionalCategory::Bottom,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Top,
        0x40 => IndicPositionalCategory::Right,
        0x41..=0x44 => IndicPositionalCategory::Bottom,
        0x47 => IndicPositionalCategory::Left,
        0x48 => IndicPositionalCategory::TopAndLeft,
        0x4b => IndicPositionalCategory::LeftAndRight,
        0x4c => IndicPositionalCategory::TopAndLeftAndRight,
        0x4d => IndicPositionalCategory::Bottom,
        0x55..=0x56 => IndicPositionalCategory::Top,
        0x57 => IndicPositionalCategory::TopAndRight,
        0x62..=0x63 => IndicPositionalCategory::Bottom,
        0x82 => IndicPositionalCategory::Top,
        0xbe..=0xbf => IndicPositionalCategory::Right,
        0xc0 => IndicPositionalCategory::Top,
        0xc1..=0xc2 => IndicPositionalCategory::Right,
        0xc6..=0xc8 => IndicPositionalCategory::Left,
        0xca..=0xcc => IndicPositionalCategory::LeftAndRight,
        0xcd => IndicPositionalCategory::Top,
        0xd7 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pc(b: u8) -> IndicPositionalCategory {
    match b {
        0x00 => IndicPositionalCategory::Top,
        0x01..=0x03 => IndicPositionalCategory::Right,
        0x04 => IndicPositionalCategory::Top,
        0x3c => IndicPositionalCategory::Bottom,
        0x3e..=0x40 => IndicPositionalCategory::Top,
        0x41..=0x44 => IndicPositionalCategory::Right,
        0x46..=0x47 => IndicPositionalCategory::Top,
        0x48 => IndicPositionalCategory::TopAndBottom,
        0x4a..=0x4d => IndicPositionalCategory::Top,
        0x55 => IndicPositionalCategory::Top,
        0x56 => IndicPositionalCategory::Bottom,
        0x62..=0x63 => IndicPositionalCategory::Bottom,
        0x81 => IndicPositionalCategory::Top,
        0x82..=0x83 => IndicPositionalCategory::Right,
        0xbc => IndicPositionalCategory::Bottom,
        0xbe => IndicPositionalCategory::Right,
        0xbf => IndicPositionalCategory::Top,
        0xc0 => IndicPositionalCategory::TopAndRight,
        0xc1..=0xc4 => IndicPositionalCategory::Right,
        0xc6 => IndicPositionalCategory::Top,
        0xc7..=0xc8 => IndicPositionalCategory::TopAndRight,
        0xca..=0xcb => IndicPositionalCategory::TopAndRight,
        0xcc..=0xcd => IndicPositionalCategory::Top,
        0xd5..=0xd6 => IndicPositionalCategory::Right,
        0xe2..=0xe3 => IndicPositionalCategory::Bottom,
        0xf3 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pd(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x01 => IndicPositionalCategory::Top,
        0x02..=0x03 => IndicPositionalCategory::Right,
        0x3b..=0x3c => IndicPositionalCategory::Top,
        0x3e..=0x40 => IndicPositionalCategory::Right,
        0x41..=0x44 => IndicPositionalCategory::Bottom,
        0x46..=0x48 => IndicPositionalCategory::Left,
        0x4a..=0x4c => IndicPositionalCategory::LeftAndRight,
        0x4d..=0x4e => IndicPositionalCategory::Top,
        0x57 => IndicPositionalCategory::Right,
        0x62..=0x63 => IndicPositionalCategory::Bottom,
        0x81 => IndicPositionalCategory::Top,
        0x82..=0x83 => IndicPositionalCategory::Right,
        0xca => IndicPositionalCategory::Top,
        0xcf..=0xd1 => IndicPositionalCategory::Right,
        0xd2..=0xd3 => IndicPositionalCategory::Top,
        0xd4 => IndicPositionalCategory::Bottom,
        0xd6 => IndicPositionalCategory::Bottom,
        0xd8 => IndicPositionalCategory::Right,
        0xd9 => IndicPositionalCategory::Left,
        0xda => IndicPositionalCategory::TopAndLeft,
        0xdb => IndicPositionalCategory::Left,
        0xdc => IndicPositionalCategory::LeftAndRight,
        0xdd => IndicPositionalCategory::TopAndLeftAndRight,
        0xde => IndicPositionalCategory::LeftAndRight,
        0xdf => IndicPositionalCategory::Right,
        0xf2..=0xf3 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pe(b: u8) -> IndicPositionalCategory {
    match b {
        0x30 => IndicPositionalCategory::Right,
        0x31 => IndicPositionalCategory::Top,
        0x32..=0x33 => IndicPositionalCategory::Right,
        0x34..=0x37 => IndicPositionalCategory::Top,
        0x38..=0x3a => IndicPositionalCategory::Bottom,
        0x40..=0x44 => IndicPositionalCategory::VisualOrderLeft,
        0x45 => IndicPositionalCategory::Right,
        0x47..=0x4e => IndicPositionalCategory::Top,
        0xb0 => IndicPositionalCategory::Right,
        0xb1 => IndicPositionalCategory::Top,
        0xb2..=0xb3 => IndicPositionalCategory::Right,
        0xb4..=0xb7 => IndicPositionalCategory::Top,
        0xb8..=0xba => IndicPositionalCategory::Bottom,
        0xbb => IndicPositionalCategory::Top,
        0xbc => IndicPositionalCategory::Bottom,
        0xc0..=0xc4 => IndicPositionalCategory::VisualOrderLeft,
        0xc8..=0xce => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pf(b: u8) -> IndicPositionalCategory {
    match b {
        0x18..=0x19 => IndicPositionalCategory::Bottom,
        0x35 => IndicPositionalCategory::Bottom,
        0x37 => IndicPositionalCategory::Bottom,
        0x39 => IndicPositionalCategory::Top,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Left,
        0x71 => IndicPositionalCategory::Bottom,
        0x72 => IndicPositionalCategory::Top,
        0x73 => IndicPositionalCategory::TopAndBottom,
        0x74..=0x75 => IndicPositionalCategory::Bottom,
        0x76..=0x79 => IndicPositionalCategory::TopAndBottom,
        0x7a..=0x7e => IndicPositionalCategory::Top,
        0x7f => IndicPositionalCategory::Right,
        0x80 => IndicPositionalCategory::Top,
        0x81 => IndicPositionalCategory::TopAndBottom,
        0x82..=0x83 => IndicPositionalCategory::Top,
        0x84 => IndicPositionalCategory::Bottom,
        0x86..=0x87 => IndicPositionalCategory::Top,
        0x8d..=0x97 => IndicPositionalCategory::Bottom,
        0x99..=0xbc => IndicPositionalCategory::Bottom,
        0xc6 => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p10(b: u8) -> IndicPositionalCategory {
    match b {
        0x2b..=0x2c => IndicPositionalCategory::Right,
        0x2d..=0x2e => IndicPositionalCategory::Top,
        0x2f..=0x30 => IndicPositionalCategory::Bottom,
        0x31 => IndicPositionalCategory::Left,
        0x32..=0x36 => IndicPositionalCategory::Top,
        0x37 => IndicPositionalCategory::Bottom,
        0x38 => IndicPositionalCategory::Right,
        0x3a => IndicPositionalCategory::Top,
        0x3b => IndicPositionalCategory::Right,
        0x3c => IndicPositionalCategory::TopAndBottomAndLeft,
        0x3d..=0x3e => IndicPositionalCategory::Bottom,
        0x56..=0x57 => IndicPositionalCategory::Right,
        0x58..=0x59 => IndicPositionalCategory::Bottom,
        0x5e..=0x60 => IndicPositionalCategory::Bottom,
        0x62..=0x64 => IndicPositionalCategory::Right,
        0x67..=0x6d => IndicPositionalCategory::Right,
        0x71..=0x74 => IndicPositionalCategory::Top,
        0x82 => IndicPositionalCategory::Bottom,
        0x83 => IndicPositionalCategory::Right,
        0x84 => IndicPositionalCategory::Left,
        0x85..=0x86 => IndicPositionalCategory::Top,
        0x87..=0x8c => IndicPositionalCategory::Right,
        0x8d => IndicPositionalCategory::Bottom,
        0x8f => IndicPositionalCategory::Right,
        0x9a..=0x9c => IndicPositionalCategory::Right,
        0x9d => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p17(b: u8) -> IndicPositionalCategory {
    match b {
        0x12 => IndicPositionalCategory::Top,
        0x13..=0x14 => IndicPositionalCategory::Bottom,
        0x15 => IndicPositionalCategory::Right,
        0x32 => IndicPositionalCategory::Top,
        0x33 => IndicPositionalCategory::Bottom,
        0x34 => IndicPositionalCategory::Right,
        0x52 => IndicPositionalCategory::Top,
        0x53 => IndicPositionalCategory::Bottom,
        0x72 => IndicPositionalCategory::Top,
        0x73 => IndicPositionalCategory::Bottom,
        0xb6 => IndicPositionalCategory::Right,
        0xb7..=0xba => IndicPositionalCategory::Top,
        0xbb..=0xbd => IndicPositionalCategory::Bottom,
        0xbe => IndicPositionalCategory::TopAndLeft,
        0xbf => IndicPositionalCategory::TopAndLeftAndRight,
        0xc0 => IndicPositionalCategory::LeftAndRight,
        0xc1..=0xc3 => IndicPositionalCategory::Left,
        0xc4..=0xc5 => IndicPositionalCategory::LeftAndRight,
        0xc6 => IndicPositionalCategory::Top,
        0xc7..=0xc8 => IndicPositionalCategory::Right,
        0xc9..=0xd1 => IndicPositionalCategory::Top,
        0xd3 => IndicPositionalCategory::Top,
        0xdd => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p19(b: u8) -> IndicPositionalCategory {
    match b {
        0x20..=0x21 => IndicPositionalCategory::Top,
        0x22 => IndicPositionalCategory::Bottom,
        0x23..=0x24 => IndicPositionalCategory::Right,
        0x25..=0x26 => IndicPositionalCategory::TopAndRight,
        0x27..=0x28 => IndicPositionalCategory::Top,
        0x29..=0x2b => IndicPositionalCategory::Right,
        0x30..=0x31 => IndicPositionalCategory::Right,
        0x32 => IndicPositionalCategory::Bottom,
        0x33..=0x38 => IndicPositionalCategory::Right,
        0x39 => IndicPositionalCategory::Bottom,
        0x3a => IndicPositionalCategory::Top,
        0x3b => IndicPositionalCategory::Bottom,
        0xb0..=0xb4 => IndicPositionalCategory::Right,
        0xb5..=0xb7 => IndicPositionalCategory::VisualOrderLeft,
        0xb8..=0xb9 => IndicPositionalCategory::Right,
        0xba => IndicPositionalCategory::VisualOrderLeft,
        0xbb..=0xc0 => IndicPositionalCategory::Right,
        0xc8..=0xc9 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p1a(b: u8) -> IndicPositionalCategory {
    match b {
        0x17 => IndicPositionalCategory::Top,
        0x18 => IndicPositionalCategory::Bottom,
        0x19 => IndicPositionalCategory::Left,
        0x1a => IndicPositionalCategory::Right,
        0x1b => IndicPositionalCategory::Top,
        0x55 => IndicPositionalCategory::Left,
        0x56 => IndicPositionalCategory::Bottom,
        0x57 => IndicPositionalCategory::Right,
        0x58..=0x5a => IndicPositionalCategory::Top,
        0x5b..=0x5e => IndicPositionalCategory::Bottom,
        0x61 => IndicPositionalCategory::Right,
        0x62 => IndicPositionalCategory::Top,
        0x63..=0x64 => IndicPositionalCategory::Right,
        0x65..=0x68 => IndicPositionalCategory::Top,
        0x69..=0x6a => IndicPositionalCategory::Bottom,
        0x6b => IndicPositionalCategory::Top,
        0x6c => IndicPositionalCategory::Bottom,
        0x6d => IndicPositionalCategory::Right,
        0x6e..=0x72 => IndicPositionalCategory::Left,
        0x73..=0x7c => IndicPositionalCategory::Top,
        0x7f => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p1b(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x03 => IndicPositionalCategory::Top,
        0x04 => IndicPositionalCategory::Right,
        0x34 => IndicPositionalCategory::Top,
        0x35 => IndicPositionalCategory::Right,
        0x36..=0x37 => IndicPositionalCategory::Top,
        0x38..=0x3a => IndicPositionalCategory::Bottom,
        0x3b => IndicPositionalCategory::BottomAndRight,
        0x3c => IndicPositionalCategory::TopAndBottom,
        0x3d => IndicPositionalCategory::TopAndBottomAndRight,
        0x3e..=0x3f => IndicPositionalCategory::Left,
        0x40..=0x41 => IndicPositionalCategory::LeftAndRight,
        0x42 => IndicPositionalCategory::Top,
        0x43 => IndicPositionalCategory::TopAndRight,
        0x44 => IndicPositionalCategory::Right,
        0x6b => IndicPositionalCategory::Top,
        0x6c => IndicPositionalCategory::Bottom,
        0x6d..=0x73 => IndicPositionalCategory::Top,
        0x80..=0x81 => IndicPositionalCategory::Top,
        0x82 => IndicPositionalCategory::Right,
        0xa1 => IndicPositionalCategory::Right,
        0xa2..=0xa3 => IndicPositionalCategory::Bottom,
        0xa4 => IndicPositionalCategory::Top,
        0xa5 => IndicPositionalCategory::Bottom,
        0xa6 => IndicPositionalCategory::Left,
        0xa7 => IndicPositionalCategory::Right,
        0xa8..=0xa9 => IndicPositionalCategory::Top,
        0xaa => IndicPositionalCategory::Right,
        0xac..=0xad => IndicPositionalCategory::Bottom,
        0xe6 => IndicPositionalCategory::Top,
        0xe7 => IndicPositionalCategory::Right,
        0xe8..=0xe9 => IndicPositionalCategory::Top,
        0xea..=0xec => IndicPositionalCategory::Right,
        0xed => IndicPositionalCategory::Top,
        0xee => IndicPositionalCategory::Right,
        0xef..=0xf1 => IndicPositionalCategory::Top,
        0xf2..=0xf3 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p1c(b: u8) -> IndicPositionalCategory {
    match b {
        0x24..=0x26 => IndicPositionalCategory::Right,
        0x27..=0x28 => IndicPositionalCategory::Left,
        0x29 => IndicPositionalCategory::TopAndLeft,
        0x2a..=0x2b => IndicPositionalCategory::Right,
        0x2c => IndicPositionalCategory::Bottom,
        0x2d..=0x33 => IndicPositionalCategory::Top,
        0x34..=0x35 => IndicPositionalCategory::Left,
        0x36 => IndicPositionalCategory::Top,
        0x37 => IndicPositionalCategory::Bottom,
        0xd0..=0xd2 => IndicPositionalCategory::Top,
        0xd4 => IndicPositionalCategory::Overstruck,
        0xd5..=0xd9 => IndicPositionalCategory::Bottom,
        0xda..=0xdb => IndicPositionalCategory::Top,
        0xdc..=0xdf => IndicPositionalCategory::Bottom,
        0xe0 => IndicPositionalCategory::Top,
        0xe1 => IndicPositionalCategory::Right,
        0xe2..=0xe8 => IndicPositionalCategory::Overstruck,
        0xed => IndicPositionalCategory::Bottom,
        0xf4 => IndicPositionalCategory::Top,
        0xf7 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p1d(b: u8) -> IndicPositionalCategory {
    match b {
        0xfb => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_p20(b: u8) -> IndicPositionalCategory {
    match b {
        0xf0 => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pa8(b: u8) -> IndicPositionalCategory {
    match b {
        0x02 => IndicPositionalCategory::Top,
        0x06 => IndicPositionalCategory::Top,
        0x0b => IndicPositionalCategory::Top,
        0x23..=0x24 => IndicPositionalCategory::Right,
        0x25 => IndicPositionalCategory::Bottom,
        0x26 => IndicPositionalCategory::Top,
        0x27 => IndicPositionalCategory::Right,
        0x2c => IndicPositionalCategory::Bottom,
        0x80..=0x81 => IndicPositionalCategory::Right,
        0xb4..=0xc3 => IndicPositionalCategory::Right,
        0xc4 => IndicPositionalCategory::Bottom,
        0xc5 => IndicPositionalCategory::Top,
        0xe0..=0xf1 => IndicPositionalCategory::Top,
        0xff => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pa9(b: u8) -> IndicPositionalCategory {
    match b {
        0x26..=0x2a => IndicPositionalCategory::Top,
        0x2b..=0x2d => IndicPositionalCategory::Bottom,
        0x47..=0x49 => IndicPositionalCategory::Bottom,
        0x4a => IndicPositionalCategory::Top,
        0x4b..=0x4e => IndicPositionalCategory::Bottom,
        0x4f..=0x51 => IndicPositionalCategory::Top,
        0x52..=0x53 => IndicPositionalCategory::Right,
        0x80..=0x82 => IndicPositionalCategory::Top,
        0x83 => IndicPositionalCategory::Right,
        0xb3 => IndicPositionalCategory::Top,
        0xb4..=0xb5 => IndicPositionalCategory::Right,
        0xb6..=0xb7 => IndicPositionalCategory::Top,
        0xb8..=0xb9 => IndicPositionalCategory::Bottom,
        0xba..=0xbb => IndicPositionalCategory::Left,
        0xbc => IndicPositionalCategory::Top,
        0xbd => IndicPositionalCategory::Bottom,
        0xbe => IndicPositionalCategory::BottomAndRight,
        0xbf => IndicPositionalCategory::BottomAndLeft,
        0xc0 => IndicPositionalCategory::BottomAndRight,
        0xe5 => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_paa(b: u8) -> IndicPositionalCategory {
    match b {
        0x29..=0x2c => IndicPositionalCategory::Top,
        0x2d => IndicPositionalCategory::Bottom,
        0x2e => IndicPositionalCategory::Top,
        0x2f..=0x30 => IndicPositionalCategory::Left,
        0x31 => IndicPositionalCategory::Top,
        0x32 => IndicPositionalCategory::Bottom,
        0x33 => IndicPositionalCategory::Right,
        0x34 => IndicPositionalCategory::Left,
        0x35..=0x36 => IndicPositionalCategory::Bottom,
        0x43 => IndicPositionalCategory::Top,
        0x4c => IndicPositionalCategory::Top,
        0x4d => IndicPositionalCategory::Right,
        0x7b => IndicPositionalCategory::Right,
        0x7c => IndicPositionalCategory::Top,
        0x7d => IndicPositionalCategory::Right,
        0xb0 => IndicPositionalCategory::Top,
        0xb1 => IndicPositionalCategory::Right,
        0xb2..=0xb3 => IndicPositionalCategory::Top,
        0xb4 => IndicPositionalCategory::Bottom,
        0xb5..=0xb6 => IndicPositionalCategory::VisualOrderLeft,
        0xb7..=0xb8 => IndicPositionalCategory::Top,
        0xb9 => IndicPositionalCategory::VisualOrderLeft,
        0xba => IndicPositionalCategory::Right,
        0xbb..=0xbc => IndicPositionalCategory::VisualOrderLeft,
        0xbd => IndicPositionalCategory::Right,
        0xbe..=0xbf => IndicPositionalCategory::Top,
        0xc1 => IndicPositionalCategory::Top,
        0xeb => IndicPositionalCategory::Left,
        0xec => IndicPositionalCategory::Bottom,
        0xed => IndicPositionalCategory::Top,
        0xee => IndicPositionalCategory::Left,
        0xef => IndicPositionalCategory::Right,
        0xf5 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "bmp")]
const fn ipc_pab(b: u8) -> IndicPositionalCategory {
    match b {
        0xe3..=0xe4 => IndicPositionalCategory::Right,
        0xe5 => IndicPositionalCategory::Top,
        0xe6..=0xe7 => IndicPositionalCategory::Right,
        0xe8 => IndicPositionalCategory::Bottom,
        0xe9..=0xea => IndicPositionalCategory::Right,
        0xec => IndicPositionalCategory::Right,
        0xed => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p10a(b: u8) -> IndicPositionalCategory {
    match b {
        0x01 => IndicPositionalCategory::Overstruck,
        0x02..=0x03 => IndicPositionalCategory::Bottom,
        0x05 => IndicPositionalCategory::Top,
        0x06 => IndicPositionalCategory::Overstruck,
        0x0c..=0x0e => IndicPositionalCategory::Bottom,
        0x0f => IndicPositionalCategory::Top,
        0x38 => IndicPositionalCategory::Top,
        0x39..=0x3a => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p110(b: u8) -> IndicPositionalCategory {
    match b {
        0x00 => IndicPositionalCategory::Right,
        0x01 => IndicPositionalCategory::Top,
        0x02 => IndicPositionalCategory::Right,
        0x38..=0x3b => IndicPositionalCategory::Top,
        0x3c..=0x41 => IndicPositionalCategory::Bottom,
        0x42..=0x46 => IndicPositionalCategory::Top,
        0x70 => IndicPositionalCategory::Top,
        0x73..=0x74 => IndicPositionalCategory::Top,
        0x80..=0x81 => IndicPositionalCategory::Top,
        0x82 => IndicPositionalCategory::Right,
        0xb0 => IndicPositionalCategory::Right,
        0xb1 => IndicPositionalCategory::Left,
        0xb2 => IndicPositionalCategory::Right,
        0xb3..=0xb4 => IndicPositionalCategory::Bottom,
        0xb5..=0xb6 => IndicPositionalCategory::Top,
        0xb7..=0xb8 => IndicPositionalCategory::Right,
        0xb9..=0xba => IndicPositionalCategory::Bottom,
        0xc2 => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p111(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x02 => IndicPositionalCategory::Top,
        0x27..=0x29 => IndicPositionalCategory::Top,
        0x2a..=0x2b => IndicPositionalCategory::Bottom,
        0x2c => IndicPositionalCategory::Left,
        0x2d => IndicPositionalCategory::Top,
        0x2e..=0x2f => IndicPositionalCategory::TopAndBottom,
        0x30 => IndicPositionalCategory::Top,
        0x31..=0x32 => IndicPositionalCategory::Bottom,
        0x34 => IndicPositionalCategory::Top,
        0x45..=0x46 => IndicPositionalCategory::Right,
        0x73 => IndicPositionalCategory::Bottom,
        0x80..=0x81 => IndicPositionalCategory::Top,
        0x82 => IndicPositionalCategory::Right,
        0xb3 => IndicPositionalCategory::Right,
        0xb4 => IndicPositionalCategory::Left,
        0xb5 => IndicPositionalCategory::Right,
        0xb6..=0xbb => IndicPositionalCategory::Bottom,
        0xbc..=0xbe => IndicPositionalCategory::Top,
        0xbf => IndicPositionalCategory::TopAndRight,
        0xc0 => IndicPositionalCategory::Right,
        0xc2..=0xc3 => IndicPositionalCategory::Top,
        0xc9..=0xca => IndicPositionalCategory::Bottom,
        0xcb => IndicPositionalCategory::Top,
        0xcc => IndicPositionalCategory::Bottom,
        0xce => IndicPositionalCategory::Left,
        0xcf => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p112(b: u8) -> IndicPositionalCategory {
    match b {
        0x2c..=0x2e => IndicPositionalCategory::Right,
        0x2f => IndicPositionalCategory::Bottom,
        0x30..=0x31 => IndicPositionalCategory::Top,
        0x32..=0x33 => IndicPositionalCategory::TopAndRight,
        0x34 => IndicPositionalCategory::Top,
        0x35 => IndicPositionalCategory::Right,
        0x36..=0x37 => IndicPositionalCategory::Top,
        0x3e => IndicPositionalCategory::Top,
        0x41 => IndicPositionalCategory::Bottom,
        0xdf => IndicPositionalCategory::Top,
        0xe0 => IndicPositionalCategory::Right,
        0xe1 => IndicPositionalCategory::Left,
        0xe2 => IndicPositionalCategory::Right,
        0xe3..=0xe4 => IndicPositionalCategory::Bottom,
        0xe5..=0xe8 => IndicPositionalCategory::Top,
        0xe9..=0xea => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p113(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x01 => IndicPositionalCategory::Top,
        0x02..=0x03 => IndicPositionalCategory::Right,
        0x3b..=0x3c => IndicPositionalCategory::Bottom,
        0x3e..=0x3f => IndicPositionalCategory::Right,
        0x40 => IndicPositionalCategory::Top,
        0x41..=0x44 => IndicPositionalCategory::Right,
        0x47..=0x48 => IndicPositionalCategory::Left,
        0x4b..=0x4c => IndicPositionalCategory::LeftAndRight,
        0x4d => IndicPositionalCategory::Right,
        0x57 => IndicPositionalCategory::Right,
        0x62..=0x63 => IndicPositionalCategory::Right,
        0x66..=0x6c => IndicPositionalCategory::Top,
        0x70..=0x74 => IndicPositionalCategory::Top,
        0xb8 => IndicPositionalCategory::Right,
        0xb9..=0xba => IndicPositionalCategory::TopAndRight,
        0xbb..=0xc0 => IndicPositionalCategory::Bottom,
        0xc2 => IndicPositionalCategory::Left,
        0xc5 => IndicPositionalCategory::Left,
        0xc7..=0xc8 => IndicPositionalCategory::LeftAndRight,
        0xc9..=0xca => IndicPositionalCategory::Right,
        0xcc..=0xcd => IndicPositionalCategory::Right,
        0xce => IndicPositionalCategory::Top,
        0xcf => IndicPositionalCategory::Right,
        0xd1 => IndicPositionalCategory::Top,
        0xd2 => IndicPositionalCategory::Bottom,
        0xe1 => IndicPositionalCategory::Top,
        0xe2 => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p114(b: u8) -> IndicPositionalCategory {
    match b {
        0x35 => IndicPositionalCategory::Right,
        0x36 => IndicPositionalCategory::Left,
        0x37 => IndicPositionalCategory::Right,
        0x38..=0x3d => IndicPositionalCategory::Bottom,
        0x3e..=0x3f => IndicPositionalCategory::Top,
        0x40..=0x41 => IndicPositionalCategory::Right,
        0x42 => IndicPositionalCategory::Bottom,
        0x43..=0x44 => IndicPositionalCategory::Top,
        0x45 => IndicPositionalCategory::Right,
        0x46 => IndicPositionalCategory::Bottom,
        0x5e => IndicPositionalCategory::Top,
        0xb0 => IndicPositionalCategory::Right,
        0xb1 => IndicPositionalCategory::Left,
        0xb2 => IndicPositionalCategory::Right,
        0xb3..=0xb8 => IndicPositionalCategory::Bottom,
        0xb9 => IndicPositionalCategory::Left,
        0xba => IndicPositionalCategory::Top,
        0xbb => IndicPositionalCategory::TopAndLeft,
        0xbc => IndicPositionalCategory::LeftAndRight,
        0xbd => IndicPositionalCategory::Right,
        0xbe => IndicPositionalCategory::LeftAndRight,
        0xbf..=0xc0 => IndicPositionalCategory::Top,
        0xc1 => IndicPositionalCategory::Right,
        0xc2..=0xc3 => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p115(b: u8) -> IndicPositionalCategory {
    match b {
        0xaf => IndicPositionalCategory::Right,
        0xb0 => IndicPositionalCategory::Left,
        0xb1 => IndicPositionalCategory::Right,
        0xb2..=0xb5 => IndicPositionalCategory::Bottom,
        0xb8 => IndicPositionalCategory::Left,
        0xb9 => IndicPositionalCategory::TopAndLeft,
        0xba => IndicPositionalCategory::LeftAndRight,
        0xbb => IndicPositionalCategory::TopAndLeftAndRight,
        0xbc..=0xbd => IndicPositionalCategory::Top,
        0xbe => IndicPositionalCategory::Right,
        0xbf..=0xc0 => IndicPositionalCategory::Bottom,
        0xdc..=0xdd => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p116(b: u8) -> IndicPositionalCategory {
    match b {
        0x30..=0x32 => IndicPositionalCategory::Right,
        0x33..=0x38 => IndicPositionalCategory::Bottom,
        0x39..=0x3a => IndicPositionalCategory::Top,
        0x3b..=0x3c => IndicPositionalCategory::Right,
        0x3d => IndicPositionalCategory::Top,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Bottom,
        0x40 => IndicPositionalCategory::Top,
        0xab => IndicPositionalCategory::Top,
        0xac => IndicPositionalCategory::Right,
        0xad => IndicPositionalCategory::Top,
        0xae => IndicPositionalCategory::Left,
        0xaf => IndicPositionalCategory::Right,
        0xb0..=0xb1 => IndicPositionalCategory::Bottom,
        0xb2..=0xb5 => IndicPositionalCategory::Top,
        0xb6 => IndicPositionalCategory::Right,
        0xb7 => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p117(b: u8) -> IndicPositionalCategory {
    match b {
        0x1d => IndicPositionalCategory::Bottom,
        0x1e => IndicPositionalCategory::TopAndBottomAndLeft,
        0x1f => IndicPositionalCategory::Top,
        0x20..=0x21 => IndicPositionalCategory::Right,
        0x22..=0x23 => IndicPositionalCategory::Top,
        0x24..=0x25 => IndicPositionalCategory::Bottom,
        0x26 => IndicPositionalCategory::Left,
        0x27 => IndicPositionalCategory::Top,
        0x28 => IndicPositionalCategory::Bottom,
        0x29..=0x2b => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p118(b: u8) -> IndicPositionalCategory {
    match b {
        0x2c => IndicPositionalCategory::Right,
        0x2d => IndicPositionalCategory::Left,
        0x2e => IndicPositionalCategory::Right,
        0x2f..=0x32 => IndicPositionalCategory::Bottom,
        0x33..=0x37 => IndicPositionalCategory::Top,
        0x38 => IndicPositionalCategory::Right,
        0x39..=0x3a => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p119(b: u8) -> IndicPositionalCategory {
    match b {
        0x30..=0x34 => IndicPositionalCategory::Right,
        0x35 => IndicPositionalCategory::Left,
        0x37 => IndicPositionalCategory::Left,
        0x38 => IndicPositionalCategory::LeftAndRight,
        0x3b..=0x3c => IndicPositionalCategory::Top,
        0x3d => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Top,
        0x40 => IndicPositionalCategory::Right,
        0x41 => IndicPositionalCategory::Top,
        0x42 => IndicPositionalCategory::BottomAndRight,
        0x43 => IndicPositionalCategory::Bottom,
        0xd1 => IndicPositionalCategory::Right,
        0xd2 => IndicPositionalCategory::Left,
        0xd3 => IndicPositionalCategory::Right,
        0xd4..=0xd7 => IndicPositionalCategory::Bottom,
        0xda..=0xdb => IndicPositionalCategory::Top,
        0xdc..=0xdf => IndicPositionalCategory::Right,
        0xe0 => IndicPositionalCategory::Bottom,
        0xe4 => IndicPositionalCategory::Left,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11a(b: u8) -> IndicPositionalCategory {
    match b {
        0x01 => IndicPositionalCategory::Top,
        0x02..=0x03 => IndicPositionalCategory::Bottom,
        0x04..=0x09 => IndicPositionalCategory::Top,
        0x0a => IndicPositionalCategory::Bottom,
        0x33..=0x34 => IndicPositionalCategory::Bottom,
        0x35..=0x38 => IndicPositionalCategory::Top,
        0x39 => IndicPositionalCategory::Right,
        0x3b..=0x3e => IndicPositionalCategory::Bottom,
        0x51 => IndicPositionalCategory::Top,
        0x52..=0x53 => IndicPositionalCategory::Bottom,
        0x54..=0x56 => IndicPositionalCategory::Top,
        0x57..=0x58 => IndicPositionalCategory::Right,
        0x59..=0x5b => IndicPositionalCategory::Bottom,
        0x84..=0x89 => IndicPositionalCategory::Top,
        0x8a..=0x95 => IndicPositionalCategory::Bottom,
        0x96 => IndicPositionalCategory::Top,
        0x97 => IndicPositionalCategory::Right,
        0x98 => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11b(b: u8) -> IndicPositionalCategory {
    match b {
        0x60 => IndicPositionalCategory::Top,
        0x61 => IndicPositionalCategory::Right,
        0x62..=0x63 => IndicPositionalCategory::Bottom,
        0x64 => IndicPositionalCategory::Top,
        0x65 => IndicPositionalCategory::Right,
        0x66 => IndicPositionalCategory::Top,
        0x67 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11c(b: u8) -> IndicPositionalCategory {
    match b {
        0x2f => IndicPositionalCategory::Right,
        0x30..=0x31 => IndicPositionalCategory::Top,
        0x32..=0x36 => IndicPositionalCategory::Bottom,
        0x38..=0x3d => IndicPositionalCategory::Top,
        0x3e => IndicPositionalCategory::Right,
        0x3f => IndicPositionalCategory::Bottom,
        0x92..=0xa7 => IndicPositionalCategory::Bottom,
        0xa9 => IndicPositionalCategory::Right,
        0xaa..=0xb0 => IndicPositionalCategory::Bottom,
        0xb1 => IndicPositionalCategory::Left,
        0xb2 => IndicPositionalCategory::Bottom,
        0xb3 => IndicPositionalCategory::Top,
        0xb4 => IndicPositionalCategory::Right,
        0xb5..=0xb6 => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11d(b: u8) -> IndicPositionalCategory {
    match b {
        0x31..=0x35 => IndicPositionalCategory::Top,
        0x36 => IndicPositionalCategory::Bottom,
        0x3a => IndicPositionalCategory::Top,
        0x3c..=0x3d => IndicPositionalCategory::Top,
        0x3f..=0x41 => IndicPositionalCategory::Top,
        0x42 => IndicPositionalCategory::Bottom,
        0x43 => IndicPositionalCategory::Top,
        0x44 => IndicPositionalCategory::Bottom,
        0x46 => IndicPositionalCategory::Right,
        0x47 => IndicPositionalCategory::Bottom,
        0x8a..=0x8e => IndicPositionalCategory::Right,
        0x90..=0x91 => IndicPositionalCategory::Top,
        0x93..=0x94 => IndicPositionalCategory::Right,
        0x95 => IndicPositionalCategory::Top,
        0x96 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11e(b: u8) -> IndicPositionalCategory {
    match b {
        0xf3 => IndicPositionalCategory::Top,
        0xf4 => IndicPositionalCategory::Bottom,
        0xf5 => IndicPositionalCategory::Left,
        0xf6 => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p11f(b: u8) -> IndicPositionalCategory {
    match b {
        0x00..=0x02 => IndicPositionalCategory::Top,
        0x03 => IndicPositionalCategory::Right,
        0x34..=0x35 => IndicPositionalCategory::Right,
        0x36..=0x37 => IndicPositionalCategory::Top,
        0x38..=0x3a => IndicPositionalCategory::Bottom,
        0x3e..=0x3f => IndicPositionalCategory::Left,
        0x40 => IndicPositionalCategory::Top,
        0x41 => IndicPositionalCategory::Right,
        0x5a => IndicPositionalCategory::Top,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p161(b: u8) -> IndicPositionalCategory {
    match b {
        0x1e..=0x29 => IndicPositionalCategory::Top,
        0x2a..=0x2b => IndicPositionalCategory::Left,
        0x2c => IndicPositionalCategory::Right,
        0x2d => IndicPositionalCategory::Top,
        0x2e..=0x2f => IndicPositionalCategory::Bottom,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

#[cfg(feature = "full")]
const fn ipc_p16d(b: u8) -> IndicPositionalCategory {
    match b {
        0x40..=0x42 => IndicPositionalCategory::Right,
        0x63..=0x6c => IndicPositionalCategory::Right,
        _ => IndicPositionalCategory::NotApplicable,
    }
}

/// The derived-`Name` prefix for an algorithmically-named ideograph codepoint
/// (the full name is this prefix followed by the uppercase hex codepoint), or
/// `None` if the codepoint is not in such a range.
pub(crate) const fn ideograph_name_prefix(cp: u32) -> Option<&'static str> {
    match cp {
        0x3400..=0x4dbf => Some("CJK UNIFIED IDEOGRAPH-"),
        0x4e00..=0x9fff => Some("CJK UNIFIED IDEOGRAPH-"),
        0x17000..=0x187ff => Some("TANGUT IDEOGRAPH-"),
        0x18d00..=0x18d1e => Some("TANGUT IDEOGRAPH-"),
        0x20000..=0x2a6df => Some("CJK UNIFIED IDEOGRAPH-"),
        0x2a700..=0x2b73f => Some("CJK UNIFIED IDEOGRAPH-"),
        0x2b740..=0x2b81d => Some("CJK UNIFIED IDEOGRAPH-"),
        0x2b820..=0x2cead => Some("CJK UNIFIED IDEOGRAPH-"),
        0x2ceb0..=0x2ebe0 => Some("CJK UNIFIED IDEOGRAPH-"),
        0x2ebf0..=0x2ee5d => Some("CJK UNIFIED IDEOGRAPH-"),
        0x30000..=0x3134a => Some("CJK UNIFIED IDEOGRAPH-"),
        0x31350..=0x323af => Some("CJK UNIFIED IDEOGRAPH-"),
        0x323b0..=0x33479 => Some("CJK UNIFIED IDEOGRAPH-"),
        _ => None,
    }
}
