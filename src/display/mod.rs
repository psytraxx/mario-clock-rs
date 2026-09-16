pub mod display_task;
pub mod hub75_task;

use crate::{COLS, FBType};
use crate::{ROWS, mario::gfx::font::SUPER_MARIO_BROS_24PT};

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

/// Converts an RGB565 value to the framebuffer's RGB888 color, dimming it.
///
/// The `/ 4` on each channel is a deliberate brightness cut for the panel. It
/// is applied to the 8-bit output rather than to the 5/6-bit input so the
/// result keeps its gradation instead of collapsing to 3/4/3 bits.
pub(crate) const fn to_rgb888(color: u16) -> Rgb888 {
    let r5 = ((color >> 11) & 0x1F) as u8;
    let g6 = ((color >> 5) & 0x3F) as u8;
    let b5 = (color & 0x1F) as u8;

    // Expand to full 8-bit range, then dim.
    let r8 = (r5 << 3) | (r5 >> 2);
    let g8 = (g6 << 2) | (g6 >> 4);
    let b8 = (b5 << 3) | (b5 >> 2);

    Rgb888::new(r8 / 4, g8 / 4, b8 / 4)
}

/// The transparency key for sprite data.
///
/// NOTE: pure black is the sentinel, so black is *not* usable as a sprite
/// color -- a black pixel in any sprite is treated as see-through. Use a very
/// dark non-zero value if you need near-black in artwork.
pub(crate) const TRANSPARENT: u16 = 0x0000;

/// Draws an RGB565 sprite, skipping transparent pixels.
///
/// Writes through `set_pixel`, which is an inlined direct write, rather than
/// going through `Drawable` per pixel.
pub(crate) fn draw_rgb_bitmap(
    fb: &mut FBType,
    x: i32,
    y: i32,
    image: &[u16],
    width: i32,
    height: i32,
) {
    for row in 0..height {
        let dest_y = y + row;
        if dest_y < 0 || dest_y >= COLS as i32 {
            continue;
        }
        let row_start = (row * width) as usize;

        for col in 0..width {
            let dest_x = x + col;
            if dest_x < 0 || dest_x >= ROWS as i32 {
                continue;
            }

            let Some(&rgb565) = image.get(row_start + col as usize) else {
                continue;
            };
            if rgb565 == TRANSPARENT {
                continue;
            }

            fb.set_pixel(Point::new(dest_x, dest_y), to_rgb888(rgb565));
        }
    }
}

pub(crate) fn fill_rect(fb: &mut FBType, x: i32, y: i32, width: u32, height: u32, color565: u16) {
    let color = to_rgb888(color565);
    for row in 0..height as i32 {
        let dest_y = y + row;
        if dest_y < 0 || dest_y >= COLS as i32 {
            continue;
        }
        for col in 0..width as i32 {
            let dest_x = x + col;
            if dest_x < 0 || dest_x >= ROWS as i32 {
                continue;
            }
            fb.set_pixel(Point::new(dest_x, dest_y), color);
        }
    }
}

pub(crate) fn print_text(fb: &mut FBType, text: &str, x: i32, y: i32, color565: u16) {
    let font = SUPER_MARIO_BROS_24PT;
    let mut cursor_x = x;
    let cursor_y = y;
    let color = to_rgb888(color565);

    for c in text.chars() {
        if c < font.first as char || c > font.last as char {
            continue; // Skip characters not in the font
        }

        let glyph_index = c as usize - font.first as usize;
        let Some(glyph) = font.glyph.get(glyph_index) else {
            continue;
        };
        let bitmap = &font.bitmap[glyph.bitmap_offset as usize..];

        for row in 0..glyph.height {
            let dest_y = cursor_y + glyph.y_offset as i32 + row as i32;
            if dest_y < 0 || dest_y >= COLS as i32 {
                continue;
            }

            let bitmap_row_start = (row * glyph.width) as usize;

            for col in 0..glyph.width {
                let dest_x = cursor_x + glyph.x_offset as i32 + col as i32;
                if dest_x < 0 || dest_x >= ROWS as i32 {
                    continue;
                }

                let bit_index = bitmap_row_start + col as usize;
                // Checked read: a truncated/malformed glyph must not panic on a
                // device that runs unattended. Missing bytes read as blank.
                let byte = bitmap.get(bit_index / 8).copied().unwrap_or(0);
                if byte & (0x80 >> (bit_index % 8)) != 0 {
                    fb.set_pixel(Point::new(dest_x, dest_y), color);
                }
            }
        }

        cursor_x += glyph.x_advance as i32;
    }
}
