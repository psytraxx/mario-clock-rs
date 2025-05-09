// Define color constants
pub const SKY_COLOR: u16 = 0x000E;
pub const BLACK: u16 = 0x0000;
pub const _MASK: u16 = SKY_COLOR;

pub const M_RED: u16 = 0xF801;
pub const M_SKIN: u16 = 0xfd28;
pub const M_SHOES: u16 = 0xC300;
pub const M_SHIRT: u16 = 0x7BCF;
pub const M_HAIR: u16 = 0x0000;

// Sprite data arrays
pub const BLOCK: &[u16; 352] = &[
    _MASK, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40, 0x9A40,
    0x9A40, 0x9A40, 0x9A40, 0x9A40, // 0x0010 (16) pixels
    0x9A40, 0x9A40, _MASK, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0020 (32) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0x0000, 0x0000, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0030 (48) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x0000, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0x0000,
    0x0000, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0040 (64) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x0000, 0xE4E4, 0x0000,
    0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0050 (80) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0x0000, 0x9A40, // 0x0060 (96) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0070 (112) pixels
    0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0080 (128) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0090 (144) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x00A0 (160) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x00B0 (176) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0x0000, 0x9A40, 0xE4E4, // 0x00C0 (192) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x00D0 (208) pixels
    0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x00E0 (224) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x00F0 (240) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0100 (256) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, 0x9A40, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0110 (272) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0x0000, 0x9A40, 0xE4E4, 0x0000, // 0x0120 (288) pixels
    0x0000, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0x0000, 0x0000, 0xE4E4, 0x0000, // 0x0130 (304) pixels
    0x9A40, 0xE4E4, 0x0000, 0x0000, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0x0000, // 0x0140 (320) pixels
    0x0000, 0xE4E4, 0x0000, 0x9A40, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4,
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, // 0x0150 (336) pixels
    0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0xE4E4, 0x0000, _MASK, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000, // 0x0160 (352) pixels
];

pub const BUSH: &[u16; 189] = &[
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0000, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0010 (16) pixels
    _MASK, 0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000,
    0xBFE3, 0xBFE3, 0x0000, // 0x0020 (32) pixels
    _MASK, 0x0000, _MASK, _MASK, _MASK, 0x0000, 0xBFE3, 0xBFE3, 0x0000, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0030 (48) pixels
    0x0000, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0x0000, 0xBFE3, 0x0000, _MASK, 0x0000, 0xBFE3, 0xBFE3,
    0xBFE3, 0xBFE3, 0x0000, _MASK, // 0x0040 (64) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0xBFE3, 0xBFE3, 0xBFE3, 0x0560, 0xBFE3, 0xBFE3,
    0x0000, _MASK, 0x0000, 0xBFE3, // 0x0050 (80) pixels
    0xBFE3, 0xBFE3, 0x0560, 0xBFE3, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0xBFE3, 0x0560,
    0x0560, 0xBFE3, 0xBFE3, 0x0560, // 0x0060 (96) pixels
    0xBFE3, 0xBFE3, 0x0000, 0xBFE3, 0x0560, 0x0560, 0xBFE3, 0xBFE3, 0x0560, _MASK, _MASK, _MASK,
    0x0000, 0x0000, 0xBFE3, 0x0560, // 0x0070 (112) pixels
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0x0560, 0xBFE3, 0xBFE3, 0xBFE3,
    0xBFE3, 0xBFE3, _MASK, _MASK, // 0x0080 (128) pixels
    0x0000, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3,
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, // 0x0090 (144) pixels
    0xBFE3, 0xBFE3, 0xBFE3, _MASK, _MASK, 0x0000, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3,
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, // 0x00A0 (160) pixels
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, _MASK, 0x0000, 0xBFE3, 0xBFE3,
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, // 0x00B0 (176) pixels
    0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3, 0xBFE3,
    0xBFE3,
];

pub const CLOUD1: &[u16; 156] = &[
    _MASK, 0x0000, 0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    0x0000, 0xFFFF, 0xFFFF, // 0x0010 (16) pixels
    0xFFFF, 0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0xFFFF,
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, // 0x0020 (32) pixels
    0x0000, _MASK, 0x0000, _MASK, _MASK, _MASK, _MASK, 0xFFFF, 0x3DFF, 0xFFFF, 0xFFFF, 0x3DFF,
    0xFFFF, 0xFFFF, 0x0000, 0xFFFF, // 0x0030 (48) pixels
    0x0000, _MASK, _MASK, _MASK, 0x3DFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    0xFFFF, 0xFFFF, 0x0000, _MASK, // 0x0040 (64) pixels
    _MASK, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    0x0000, _MASK, 0xFFFF, 0xFFFF, // 0x0050 (80) pixels
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, _MASK, 0xFFFF,
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, // 0x0060 (96) pixels
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, _MASK, _MASK, 0xFFFF, 0xFFFF, 0xFFFF, 0x3DFF,
    0x3DFF, 0xFFFF, 0x3DFF, 0xFFFF, // 0x0070 (112) pixels
    0xFFFF, 0xFFFF, 0xFFFF, 0x0000, _MASK, 0x3DFF, 0x3DFF, 0x3DFF, 0xFFFF, 0xFFFF, 0x3DFF, 0xFFFF,
    0xFFFF, 0xFFFF, 0xFFFF, 0x0000, // 0x0080 (128) pixels
    _MASK, _MASK, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, 0x0000, _MASK,
    _MASK, _MASK, _MASK, 0x0000, // 0x0090 (144) pixels
    0x0000, _MASK, 0x0000, 0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
];

pub const CLOUD2: &[u16; 156] = &[
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0000, 0x0000, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0010 (16) pixels
    _MASK, _MASK, _MASK, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, 0x0000, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, _MASK, // 0x0020 (32) pixels
    0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK,
    0x0000, 0xFFFF, 0x3DFF, 0xFFFF, // 0x0030 (48) pixels
    0xFFFF, 0x3DFF, 0xFFFF, 0xFFFF, _MASK, _MASK, _MASK, 0x0000, 0x0000, 0xFFFF, 0x3DFF, 0xFFFF,
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, // 0x0040 (64) pixels
    0xFFFF, _MASK, _MASK, 0x0000, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
    0xFFFF, 0xFFFF, _MASK, 0x0000, // 0x0050 (80) pixels
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, _MASK,
    0xFFFF, 0xFFFF, 0xFFFF, 0x3DFF, // 0x0060 (96) pixels
    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, _MASK, _MASK, 0x0000, 0xFFFF,
    0xFFFF, 0x3DFF, 0xFFFF, 0xFFFF, // 0x0070 (112) pixels
    0xFFFF, 0x3DFF, 0x3DFF, 0xFFFF, 0x3DFF, _MASK, _MASK, _MASK, 0x0000, 0xFFFF, 0xFFFF, 0x3DFF,
    0x3DFF, 0x3DFF, 0xFFFF, 0xFFFF, // 0x0080 (128) pixels
    0x3DFF, 0xFFFF, _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0000, 0xFFFF, 0xFFFF, 0x0000, 0xFFFF,
    0xFFFF, 0xFFFF, 0x0000, _MASK, // 0x0090 (144) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0000, _MASK, 0x0000, 0x0000, 0x0000, _MASK,
];

pub const GROUND: &[u16; 64] = &[
    0xE2C2, 0xF6B6, 0xF6B6, 0xF6B6, 0x0000, 0xE2C2, 0xF6B6, 0xE2C2, 0xF6B6, 0xE2C2, 0xE2C2, 0xE2C2,
    0x0000, 0xF6B6, 0xE2C2, 0x0000, // 0x0010 (16) pixels
    0xF6B6, 0xE2C2, 0xE2C2, 0xE2C2, 0x0000, 0xE2C2, 0x0000, 0xE2C2, 0x0000, 0xE2C2, 0xE2C2, 0xE2C2,
    0x0000, 0xF6B6, 0xF6B6, 0x0000, // 0x0020 (32) pixels
    0xF6B6, 0x0000, 0x0000, 0xE2C2, 0x0000, 0xF6B6, 0xE2C2, 0x0000, 0xF6B6, 0xF6B6, 0xF6B6, 0x0000,
    0xF6B6, 0xE2C2, 0xE2C2, 0x0000, // 0x0030 (48) pixels
    0xF6B6, 0xE2C2, 0xE2C2, 0xF6B6, 0xE2C2, 0xE2C2, 0xE2C2, 0x0000, 0xE2C2, 0x0000, 0x0000, 0xF6B6,
    0x0000, 0x0000, 0x0000, 0xE2C2, // 0x0040 (64) pixels
];

pub const HILL: &[u16; 439] = &[
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0010 (16) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0020 (32) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0030 (48) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0040 (64) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0050 (80) pixels
    0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0060 (96) pixels
    _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0000, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x0070 (112) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000,
    _MASK, _MASK, _MASK, // 0x0080 (128) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0560,
    0x0560, 0x0000, 0x0560, // 0x0090 (144) pixels
    0x0560, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, // 0x00A0 (160) pixels
    0x0560, 0x0560, 0x0000, 0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, _MASK, // 0x00B0 (176) pixels
    _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0560, 0x0000, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000,
    _MASK, _MASK, _MASK, _MASK, // 0x00C0 (192) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0000, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0560, // 0x00D0 (208) pixels
    0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0560,
    0x0560, 0x0560, 0x0560, // 0x00E0 (224) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, _MASK, _MASK, // 0x00F0 (240) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, _MASK,
    _MASK, _MASK, _MASK, _MASK, // 0x0100 (256) pixels
    _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0560, 0x0000, // 0x0110 (272) pixels
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0560, // 0x0120 (288) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    0x0560, 0x0560, 0x0560, 0x0560, // 0x0130 (304) pixels
    0x0560, 0x0560, 0x0000, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK,
    _MASK, _MASK, _MASK, _MASK, // 0x0140 (320) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0000, _MASK, // 0x0150 (336) pixels
    _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, 0x0560, 0x0000, 0x0560,
    0x0560, 0x0560, 0x0560, 0x0560, // 0x0160 (352) pixels
    0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK, _MASK, _MASK, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0000, 0x0560, 0x0560, 0x0560, // 0x0170 (368) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0000, _MASK, _MASK, _MASK,
    0x0560, 0x0560, 0x0560, 0x0560, // 0x0180 (384) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0000, _MASK, _MASK, // 0x0190 (400) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0560, 0x0560, // 0x01A0 (416) pixels
    0x0560, 0x0560, 0x0000, _MASK, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
    0x0560, 0x0560, 0x0560, 0x0560, // 0x01B0 (432) pixels
    0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560, 0x0560,
];

pub const MARIO_IDLE: &[u16; 208] = &[
    _MASK, _MASK, _MASK, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, _MASK,
    _MASK, _MASK, M_HAIR, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_HAIR, M_SKIN, M_SKIN, _MASK, _MASK,
    _MASK, _MASK, M_HAIR, M_SKIN, M_HAIR, M_SKIN, M_SKIN, M_SKIN, M_HAIR, M_SKIN, M_SKIN, M_SKIN,
    M_SKIN, _MASK, _MASK, M_HAIR, M_SKIN, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_SKIN, M_HAIR, M_SKIN,
    M_SKIN, M_SKIN, M_SKIN, _MASK, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_HAIR, M_HAIR,
    M_HAIR, M_HAIR, M_HAIR, _MASK, _MASK, _MASK, _MASK, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_SKIN,
    M_SKIN, M_SKIN, M_SKIN, _MASK, _MASK, _MASK, _MASK, M_SHIRT, M_SHIRT, M_RED, M_SHIRT, M_SHIRT,
    M_SHIRT, M_SHIRT, _MASK, _MASK, _MASK, _MASK, _MASK, M_SHIRT, M_SHIRT, M_SHIRT, M_RED, M_SHIRT,
    M_SHIRT, M_RED, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, _MASK, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT,
    M_RED, M_RED, M_RED, M_RED, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_SKIN, M_SKIN,
    M_SHIRT, M_RED, M_SKIN, M_RED, M_RED, M_SKIN, M_RED, M_SHIRT, M_SKIN, M_SKIN, M_SKIN, M_SKIN,
    M_SKIN, M_SKIN, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_SKIN, M_SKIN, M_SKIN, M_SKIN,
    M_SKIN, M_SKIN, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_SKIN, M_SKIN, M_SKIN,
    _MASK, _MASK, M_RED, M_RED, M_RED, M_RED, _MASK, M_RED, M_RED, M_RED, M_RED, _MASK, _MASK,
    _MASK, M_SHOES, M_SHOES, M_SHOES, M_SHOES, _MASK, _MASK, _MASK, M_SHOES, M_SHOES, M_SHOES,
    M_SHOES, _MASK, M_SHOES, M_SHOES, M_SHOES, M_SHOES, M_SHOES, _MASK, _MASK, _MASK, M_SHOES,
    M_SHOES, M_SHOES, M_SHOES, M_SHOES,
];

pub const MARIO_IDLE_SIZE: [u8; 2] = [13, 16];

pub const MARIO_JUMP: &[u16; 272] = &[
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
    M_SKIN, M_SKIN, M_SKIN, M_SKIN, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, M_RED, M_RED, M_RED,
    M_RED, M_RED, M_RED, _MASK, M_SKIN, M_SKIN, M_SKIN, M_SKIN, _MASK, _MASK, _MASK, _MASK, _MASK,
    M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_SKIN, M_SKIN, M_SKIN, _MASK,
    _MASK, _MASK, _MASK, _MASK, M_HAIR, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_HAIR, M_SKIN, M_SKIN,
    M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, _MASK, _MASK, _MASK, _MASK, M_HAIR, M_SKIN, M_HAIR, M_SKIN,
    M_SKIN, M_SKIN, M_HAIR, M_SKIN, M_SKIN, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, _MASK, _MASK,
    _MASK, _MASK, M_HAIR, M_SKIN, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_SKIN, M_HAIR, M_SKIN, M_SKIN,
    M_SKIN, M_SHIRT, M_SHIRT, _MASK, _MASK, _MASK, _MASK, M_HAIR, M_HAIR, M_SKIN, M_SKIN, M_SKIN,
    M_SKIN, M_HAIR, M_HAIR, M_HAIR, M_HAIR, M_SHIRT, M_SHIRT, _MASK, _MASK, _MASK, _MASK, _MASK,
    _MASK, _MASK, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_SKIN, M_SHIRT, M_SHIRT, _MASK,
    _MASK, _MASK, _MASK, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_RED, M_SHIRT, M_SHIRT,
    M_SHIRT, M_RED, M_SHIRT, M_SHIRT, _MASK, _MASK, _MASK, _MASK, M_SHIRT, M_SHIRT, M_SHIRT,
    M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_RED, M_SHIRT, M_SHIRT, M_SHIRT, M_RED, M_RED, _MASK,
    M_SHOES, M_SHOES, M_SKIN, M_SKIN, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_SHIRT, M_RED,
    M_RED, M_RED, M_RED, M_RED, M_RED, _MASK, M_SHOES, M_SHOES, M_SKIN, M_SKIN, M_SKIN, M_SKIN,
    M_RED, M_RED, M_SHIRT, M_RED, M_RED, M_SKIN, M_RED, M_RED, M_SKIN, M_RED, M_SHOES, M_SHOES,
    M_SHOES, _MASK, M_SKIN, M_SKIN, M_SHOES, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED,
    M_RED, M_RED, M_RED, M_SHOES, M_SHOES, M_SHOES, _MASK, _MASK, M_SHOES, M_SHOES, M_SHOES, M_RED,
    M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_SHOES, M_SHOES, M_SHOES, _MASK,
    M_SHOES, M_SHOES, M_SHOES, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, M_RED, _MASK,
    _MASK, _MASK, _MASK, _MASK, _MASK, M_SHOES, M_SHOES, _MASK, M_RED, M_RED, M_RED, M_RED, M_RED,
    _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK, _MASK,
];

pub const MARIO_JUMP_SIZE: [u8; 2] = [17, 16];
