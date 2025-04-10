pub mod display_task;
pub mod hub75_task;

use crate::{mario::gfx::font::SUPER_MARIO_BROS_24PT, ROWS};
use crate::{FBType, COLS};

use embedded_graphics::{
    pixelcolor::Rgb565,
    pixelcolor::Rgb888,
    prelude::{Point, Primitive, Size},
    primitives::{PrimitiveStyleBuilder, Rectangle},
    Drawable, Pixel,
};

// Helper function to convert RGB565 u16 value  to RGB888
fn to_rgb888(color: u16) -> Rgb888 {
    // Extract 5-bit red, 6-bit green, and 5-bit blue.
    let r5 = ((color >> 11) & 0x1F) as u8;
    let g6 = ((color >> 5) & 0x3F) as u8;
    let b5 = (color & 0x1F) as u8;
    let rgb565: Rgb565 = Rgb565::new(r5, g6, b5);

    rgb565.into()
}

pub(crate) fn draw_rgb_bitmap(
    fb: &mut FBType,
    x: i32,
    y: i32,
    image: &'static [u16],
    width: i32,
    height: i32,
) {
    let image_width = width as usize;
    let image_height = height as usize;
    let start_point = Point::new(x, y);

    // Iterate over the pixels of the image
    for row in 0..image_height {
        for col in 0..image_width {
            let pixel_index = row * image_width + col;
            // Basic bounds check for the source image data
            if pixel_index >= image.len() {
                continue;
            }

            let rgb565_color = image[pixel_index];
            // Simple transparency: skip black pixels (adjust if needed, 0x0000 is black in RGB565)
            if rgb565_color == 0 {
                continue;
            }

            // Use the helper from this module
            let rgb888_color = to_rgb888(rgb565_color);

            // Calculate the target point on the framebuffer
            let target_point = start_point + Point::new(col as i32, row as i32);

            // Draw the single pixel if it's within the framebuffer bounds
            if target_point.x >= 0
                && target_point.x < ROWS as i32
                && target_point.y >= 0
                && target_point.y < COLS as i32
            {
                Pixel(target_point, rgb888_color).draw(fb).ok(); // Ignore errors
            }
        }
    }
}

pub(crate) fn fill_rect(fb: &mut FBType, x: i32, y: i32, width: u32, height: u32, color565: u16) {
    let start_point = Point::new(x, y);
    let size = Size::new(width, height);
    let style = PrimitiveStyleBuilder::new()
        .fill_color(to_rgb888(color565))
        .build();
    Rectangle::new(start_point, size)
        .into_styled(style)
        .draw(fb)
        .expect("Failed to draw rectangle");
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
        let glyph = &font.glyph[glyph_index];
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
                if bitmap[bit_index / 8] & (0x80 >> (bit_index % 8)) != 0 {
                    let target_point = Point::new(dest_x, dest_y);
                    Pixel(target_point, color).draw(fb).ok(); // Draw pixel using fb
                }
            }
        }

        cursor_x += glyph.x_advance as i32;
    }
}
