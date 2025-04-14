use crate::engine::font::{GFXfont, GFXglyph};

/// Font bitmap data for Super Mario Bros font
pub const SUPER_MARIO_BROS_24PT_BITMAPS: &[u8] = &[
    0x00, 0xFF, 0xEC, 0x30, 0xDE, 0xF6, 0x6C, 0xDB, 0xFB, 0x6F, 0xED, 0x9B, 0x00, 0x10, 0xFB, 0x83,
    0xE0, 0xFF, 0x84, 0x00, 0xC7, 0x9C, 0x71, 0xC7, 0x1C, 0xF1, 0x80, 0x61, 0xA3, 0x5B, 0xED, 0x99,
    0x9D, 0x80, 0xFC, 0x36, 0xCC, 0xC6, 0x30, 0xC6, 0x33, 0x36, 0xC0, 0x6C, 0x73, 0xF9, 0xC6, 0xC0,
    0x30, 0xCF, 0xCC, 0x30, 0x6F, 0x00, 0xFF, 0xF0, 0xF0, 0x06, 0x1C, 0x71, 0xC7, 0x1C, 0x30, 0x00,
    0x7D, 0x9F, 0x3E, 0x7C, 0xF9, 0xDF, 0x00, 0x77, 0x9C, 0xE7, 0x3B, 0xE0, 0x7D, 0x9C, 0x39, 0xE7,
    0x1C, 0x3F, 0x80, 0x7E, 0x18, 0xE0, 0x60, 0xF9, 0xDF, 0x00, 0x1C, 0x79, 0xB6, 0x6F, 0xE1, 0x83,
    0x00, 0xFD, 0x83, 0xF0, 0x70, 0xF9, 0xDF, 0x00, 0x7D, 0x83, 0xF6, 0x7C, 0xF9, 0xDF, 0x00, 0xFF,
    0x9C, 0x30, 0xC3, 0x06, 0x0C, 0x00, 0x7D, 0x9F, 0x3B, 0xEC, 0xF9, 0xDF, 0x00, 0x7D, 0x9F, 0x3B,
    0xF0, 0xE1, 0xDF, 0x00, 0xF3, 0xC0, 0x6C, 0x37, 0x80, 0x19, 0x99, 0x86, 0x18, 0x60, 0xF8, 0x01,
    0xF0, 0xC3, 0x0C, 0x33, 0x33, 0x00, 0x7D, 0x8F, 0x18, 0xE3, 0x00, 0x0C, 0x00, 0x7D, 0x06, 0xED,
    0x5B, 0xF0, 0x1F, 0x00, 0x38, 0xFB, 0x9F, 0x3F, 0xFC, 0xF9, 0x80, 0xFD, 0xCF, 0x9F, 0xEE, 0x7C,
    0xFF, 0x00, 0x3C, 0xCF, 0x87, 0x0E, 0x0C, 0xCF, 0x00, 0xF9, 0xDB, 0x9F, 0x3E, 0x7D, 0xBE, 0x00,
    0xFF, 0xC3, 0x87, 0xEE, 0x1C, 0x3F, 0x80, 0xFF, 0xC3, 0x87, 0xEE, 0x1C, 0x38, 0x00, 0x3C, 0xC3,
    0x87, 0x7E, 0x6C, 0xCF, 0x80, 0xE7, 0xCF, 0x9F, 0xFE, 0x7C, 0xF9, 0x80, 0xFB, 0x9C, 0xE7, 0x3B,
    0xE0, 0x1E, 0x0C, 0x18, 0x3E, 0x7C, 0xDF, 0x00, 0xE7, 0xDB, 0xE7, 0x8F, 0x9D, 0xB9, 0x80, 0xE3,
    0x8E, 0x38, 0xE3, 0x8F, 0xC0, 0xC7, 0xDF, 0xFF, 0xFD, 0x78, 0xF1, 0x80, 0xC7, 0xCF, 0xDF, 0xFE,
    0xFC, 0xF9, 0x80, 0x7D, 0xCF, 0x9F, 0x3E, 0x7C, 0xDF, 0x00, 0xFD, 0xCF, 0x9F, 0x3F, 0xDC, 0x38,
    0x00, 0x7D, 0xCF, 0x9F, 0x3F, 0xFD, 0x9E, 0x80, 0xFD, 0xCF, 0x9F, 0x6F, 0x9D, 0xB9, 0x80, 0x79,
    0xDB, 0x83, 0xE0, 0x7C, 0xDF, 0x00, 0xFE, 0x70, 0xE1, 0xC3, 0x87, 0x0E, 0x00, 0xE7, 0xCF, 0x9F,
    0x3E, 0x7C, 0xDF, 0x00, 0xE7, 0xCF, 0x9F, 0x36, 0xC7, 0x04, 0x00, 0xC7, 0x8F, 0x5E, 0xBF, 0xFD,
    0xF1, 0x80, 0xC7, 0xDD, 0xF1, 0xC7, 0xDD, 0xF1, 0x80, 0xE7, 0xCF, 0x9B, 0xE3, 0x87, 0x0E, 0x00,
    0xFE, 0x1C, 0x71, 0xC7, 0x1C, 0x3F, 0x80, 0xFC, 0xCC, 0xCC, 0xF0, 0xC1, 0xC1, 0xC1, 0xC1, 0xC1,
    0xC1, 0x80, 0xF3, 0x33, 0x33, 0xF0, 0x76, 0xC0, 0xFE, 0x90, 0x38, 0xFB, 0x9F, 0x3F, 0xFC, 0xF9,
    0x80, 0xFD, 0xCF, 0x9F, 0xEE, 0x7C, 0xFF, 0x00, 0x3C, 0xCF, 0x87, 0x0E, 0x0C, 0xCF, 0x00, 0xF9,
    0xDB, 0x9F, 0x3E, 0x7D, 0xBE, 0x00, 0xFF, 0xC3, 0x87, 0xEE, 0x1C, 0x3F, 0x80, 0xFF, 0xC3, 0x87,
    0xEE, 0x1C, 0x38, 0x00, 0x3C, 0xC3, 0x87, 0x7E, 0x6C, 0xCF, 0x80, 0xE7, 0xCF, 0x9F, 0xFE, 0x7C,
    0xF9, 0x80, 0xFB, 0x9C, 0xE7, 0x3B, 0xE0, 0x1E, 0x0C, 0x18, 0x3E, 0x7C, 0xDF, 0x00, 0xE7, 0xDB,
    0xE7, 0x8F, 0x9D, 0xB9, 0x80, 0xE3, 0x8E, 0x38, 0xE3, 0x8F, 0xC0, 0xC7, 0xDF, 0xFF, 0xFD, 0x78,
    0xF1, 0x80, 0xC7, 0xCF, 0xDF, 0xFE, 0xFC, 0xF9, 0x80, 0x7D, 0xCF, 0x9F, 0x3E, 0x7C, 0xDF, 0x00,
    0xFD, 0xCF, 0x9F, 0x3F, 0xDC, 0x38, 0x00, 0x7D, 0xCF, 0x9F, 0x3F, 0xFD, 0x9E, 0x80, 0xFD, 0xCF,
    0x9F, 0x6F, 0x9D, 0xB9, 0x80, 0x79, 0xDB, 0x83, 0xE0, 0x7C, 0xDF, 0x00, 0xFE, 0x70, 0xE1, 0xC3,
    0x87, 0x0E, 0x00, 0xE7, 0xCF, 0x9F, 0x3E, 0x7C, 0xDF, 0x00, 0xE7, 0xCF, 0x9F, 0x36, 0xC7, 0x04,
    0x00, 0xC7, 0x8F, 0x5E, 0xBF, 0xFD, 0xF1, 0x80, 0xC7, 0xDD, 0xF1, 0xC7, 0xDD, 0xF1, 0x80, 0xE7,
    0xCF, 0x9B, 0xE3, 0x87, 0x0E, 0x00, 0xFE, 0x1C, 0x71, 0xC7, 0x1C, 0x3F, 0x80, 0x36, 0x6C, 0x66,
    0x30, 0xFF, 0xFC, 0xC6, 0x63, 0x66, 0xC0, 0x71, 0x74, 0x70,
];

/// Glyph definitions for each character
pub const SUPER_MARIO_BROS_24PT_GLYPHS: &[GFXglyph] = &[
    GFXglyph {
        bitmap_offset: 0,
        width: 1,
        height: 1,
        x_advance: 8,
        x_offset: 0,
        y_offset: 0,
    }, // 0x20 ' '
    GFXglyph {
        bitmap_offset: 1,
        width: 3,
        height: 7,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x21 '!'
    GFXglyph {
        bitmap_offset: 4,
        width: 5,
        height: 3,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x22 '"'
    GFXglyph {
        bitmap_offset: 6,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x23 '#'
    GFXglyph {
        bitmap_offset: 13,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x24 '$'
    GFXglyph {
        bitmap_offset: 20,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x25 '%'
    GFXglyph {
        bitmap_offset: 27,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x26 '&'
    GFXglyph {
        bitmap_offset: 34,
        width: 2,
        height: 3,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x27 '''
    GFXglyph {
        bitmap_offset: 35,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x28 '('
    GFXglyph {
        bitmap_offset: 39,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x29 ')'
    GFXglyph {
        bitmap_offset: 43,
        width: 7,
        height: 5,
        x_advance: 8,
        x_offset: 0,
        y_offset: -5,
    }, // 0x2A '*'
    GFXglyph {
        bitmap_offset: 48,
        width: 6,
        height: 5,
        x_advance: 8,
        x_offset: 1,
        y_offset: -5,
    }, // 0x2B '+'
    GFXglyph {
        bitmap_offset: 52,
        width: 3,
        height: 3,
        x_advance: 8,
        x_offset: 1,
        y_offset: -1,
    }, // 0x2C ','
    GFXglyph {
        bitmap_offset: 54,
        width: 6,
        height: 2,
        x_advance: 8,
        x_offset: 1,
        y_offset: -3,
    }, // 0x2D '-'
    GFXglyph {
        bitmap_offset: 56,
        width: 2,
        height: 2,
        x_advance: 8,
        x_offset: 2,
        y_offset: -1,
    }, // 0x2E '.'
    GFXglyph {
        bitmap_offset: 57,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x2F '/'
    GFXglyph {
        bitmap_offset: 64,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x30 '0'
    GFXglyph {
        bitmap_offset: 71,
        width: 5,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x31 '1'
    GFXglyph {
        bitmap_offset: 76,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x32 '2'
    GFXglyph {
        bitmap_offset: 83,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x33 '3'
    GFXglyph {
        bitmap_offset: 90,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x34 '4'
    GFXglyph {
        bitmap_offset: 97,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x35 '5'
    GFXglyph {
        bitmap_offset: 104,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x36 '6'
    GFXglyph {
        bitmap_offset: 111,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x37 '7'
    GFXglyph {
        bitmap_offset: 118,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x38 '8'
    GFXglyph {
        bitmap_offset: 125,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x39 '9'
    GFXglyph {
        bitmap_offset: 132,
        width: 2,
        height: 5,
        x_advance: 8,
        x_offset: 2,
        y_offset: -5,
    }, // 0x3A ':'
    GFXglyph {
        bitmap_offset: 134,
        width: 3,
        height: 6,
        x_advance: 8,
        x_offset: 1,
        y_offset: -5,
    }, // 0x3B ';'
    GFXglyph {
        bitmap_offset: 137,
        width: 5,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x3C '<'
    GFXglyph {
        bitmap_offset: 142,
        width: 5,
        height: 4,
        x_advance: 8,
        x_offset: 1,
        y_offset: -4,
    }, // 0x3D '='
    GFXglyph {
        bitmap_offset: 145,
        width: 5,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x3E '>'
    GFXglyph {
        bitmap_offset: 150,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x3F '?'
    GFXglyph {
        bitmap_offset: 157,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x40 '@'
    GFXglyph {
        bitmap_offset: 164,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x41 'A'
    GFXglyph {
        bitmap_offset: 171,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x42 'B'
    GFXglyph {
        bitmap_offset: 178,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x43 'C'
    GFXglyph {
        bitmap_offset: 185,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x44 'D'
    GFXglyph {
        bitmap_offset: 192,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x45 'E'
    GFXglyph {
        bitmap_offset: 199,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x46 'F'
    GFXglyph {
        bitmap_offset: 206,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x47 'G'
    GFXglyph {
        bitmap_offset: 213,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x48 'H'
    GFXglyph {
        bitmap_offset: 220,
        width: 5,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x49 'I'
    GFXglyph {
        bitmap_offset: 225,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x4A 'J'
    GFXglyph {
        bitmap_offset: 232,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x4B 'K'
    GFXglyph {
        bitmap_offset: 239,
        width: 6,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x4C 'L'
    GFXglyph {
        bitmap_offset: 245,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x4D 'M'
    GFXglyph {
        bitmap_offset: 252,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x4E 'N'
    GFXglyph {
        bitmap_offset: 259,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x4F 'O'
    GFXglyph {
        bitmap_offset: 266,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x50 'P'
    GFXglyph {
        bitmap_offset: 273,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x51 'Q'
    GFXglyph {
        bitmap_offset: 280,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x52 'R'
    GFXglyph {
        bitmap_offset: 287,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x53 'S'
    GFXglyph {
        bitmap_offset: 294,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x54 'T'
    GFXglyph {
        bitmap_offset: 301,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x55 'U'
    GFXglyph {
        bitmap_offset: 308,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x56 'V'
    GFXglyph {
        bitmap_offset: 315,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x57 'W'
    GFXglyph {
        bitmap_offset: 322,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x58 'X'
    GFXglyph {
        bitmap_offset: 329,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x59 'Y'
    GFXglyph {
        bitmap_offset: 336,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x5A 'Z'
    GFXglyph {
        bitmap_offset: 343,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x5B '['
    GFXglyph {
        bitmap_offset: 347,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x5C '\'
    GFXglyph {
        bitmap_offset: 354,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x5D ']'
    GFXglyph {
        bitmap_offset: 358,
        width: 5,
        height: 2,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x5E '^'
    GFXglyph {
        bitmap_offset: 360,
        width: 7,
        height: 1,
        x_advance: 8,
        x_offset: 0,
        y_offset: 1,
    }, // 0x5F '_'
    GFXglyph {
        bitmap_offset: 361,
        width: 2,
        height: 2,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x60 '`'
    GFXglyph {
        bitmap_offset: 362,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x61 'a'
    GFXglyph {
        bitmap_offset: 369,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x62 'b'
    GFXglyph {
        bitmap_offset: 376,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x63 'c'
    GFXglyph {
        bitmap_offset: 383,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x64 'd'
    GFXglyph {
        bitmap_offset: 390,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x65 'e'
    GFXglyph {
        bitmap_offset: 397,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x66 'f'
    GFXglyph {
        bitmap_offset: 404,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x67 'g'
    GFXglyph {
        bitmap_offset: 411,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x68 'h'
    GFXglyph {
        bitmap_offset: 418,
        width: 5,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x69 'i'
    GFXglyph {
        bitmap_offset: 423,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x6A 'j'
    GFXglyph {
        bitmap_offset: 430,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x6B 'k'
    GFXglyph {
        bitmap_offset: 437,
        width: 6,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x6C 'l'
    GFXglyph {
        bitmap_offset: 443,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x6D 'm'
    GFXglyph {
        bitmap_offset: 450,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x6E 'n'
    GFXglyph {
        bitmap_offset: 457,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x6F 'o'
    GFXglyph {
        bitmap_offset: 464,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x70 'p'
    GFXglyph {
        bitmap_offset: 471,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x71 'q'
    GFXglyph {
        bitmap_offset: 478,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x72 'r'
    GFXglyph {
        bitmap_offset: 485,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x73 's'
    GFXglyph {
        bitmap_offset: 492,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x74 't'
    GFXglyph {
        bitmap_offset: 499,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x75 'u'
    GFXglyph {
        bitmap_offset: 506,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x76 'v'
    GFXglyph {
        bitmap_offset: 513,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x77 'w'
    GFXglyph {
        bitmap_offset: 520,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x78 'x'
    GFXglyph {
        bitmap_offset: 527,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x79 'y'
    GFXglyph {
        bitmap_offset: 534,
        width: 7,
        height: 7,
        x_advance: 8,
        x_offset: 0,
        y_offset: -6,
    }, // 0x7A 'z'
    GFXglyph {
        bitmap_offset: 541,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 2,
        y_offset: -6,
    }, // 0x7B '{'
    GFXglyph {
        bitmap_offset: 545,
        width: 2,
        height: 7,
        x_advance: 8,
        x_offset: 3,
        y_offset: -6,
    }, // 0x7C '|'
    GFXglyph {
        bitmap_offset: 547,
        width: 4,
        height: 7,
        x_advance: 8,
        x_offset: 1,
        y_offset: -6,
    }, // 0x7D '}'
    GFXglyph {
        bitmap_offset: 551,
        width: 7,
        height: 3,
        x_advance: 8,
        x_offset: 0,
        y_offset: -4,
    }, // 0x7E '~'
];

/// The complete font definition
pub const SUPER_MARIO_BROS_24PT: GFXfont = GFXfont {
    bitmap: SUPER_MARIO_BROS_24PT_BITMAPS,
    glyph: SUPER_MARIO_BROS_24PT_GLYPHS,
    first: 0x20, // First ASCII character (space)
    last: 0x7E,  // Last ASCII character (tilde)
};
