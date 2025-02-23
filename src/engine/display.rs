use crate::GRID_SIZE;

use super::GFXfont;

#[derive(Debug)]
pub struct Display {
    pixel_buffer: [u16; GRID_SIZE * GRID_SIZE],
    font: Option<GFXfont<'static>>,
}

impl Default for Display {
    fn default() -> Self {
        Self::new()
    }
}

impl Display {
    pub fn new() -> Self {
        let pixel_buffer = [0; GRID_SIZE * GRID_SIZE];
        Display {
            pixel_buffer,
            font: None,
        }
    }

    pub fn get_buffer(&self) -> &[u16] {
        &self.pixel_buffer
    }

    pub fn draw_rgb_bitmap(
        &mut self,
        x: i32,
        y: i32,
        image: &'static [u16],
        width: i32,
        height: i32,
    ) {
        if x >= GRID_SIZE as i32 || y >= GRID_SIZE as i32 || x + width <= 0 || y + height <= 0 {
            return;
        }

        let start_row = if y < 0 { -y } else { 0 };
        let end_row = if y + height > GRID_SIZE as i32 {
            GRID_SIZE as i32 - y
        } else {
            height
        };

        let start_col = if x < 0 { -x } else { 0 };
        let end_col = if x + width > GRID_SIZE as i32 {
            GRID_SIZE as i32 - x
        } else {
            width
        };

        for row in start_row..end_row {
            let dest_y = y + row;

            for col in start_col..end_col {
                let dest_x = x + col;

                let src_index = (row * width + col) as usize;
                if src_index >= image.len() {
                    continue;
                }

                let dest_index = dest_y as usize * GRID_SIZE + dest_x as usize;
                self.pixel_buffer[dest_index] = image[src_index];
            }
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u16) {
        if x >= GRID_SIZE as i32 || y >= GRID_SIZE as i32 || x + width <= 0 || y + height <= 0 {
            return;
        }

        let start_row = if y < 0 { -y } else { 0 };
        let end_row = if y + height > GRID_SIZE as i32 {
            GRID_SIZE as i32 - y
        } else {
            height
        };

        let start_col = if x < 0 { -x } else { 0 };
        let end_col = if x + width > GRID_SIZE as i32 {
            GRID_SIZE as i32 - x
        } else {
            width
        };

        for row in start_row..end_row {
            let dest_y = y + row;
            let dest_index_base = dest_y as usize * GRID_SIZE;

            for col in start_col..end_col {
                let dest_x = x + col;
                let dest_index = dest_index_base + dest_x as usize;
                self.pixel_buffer[dest_index] = color;
            }
        }
    }

    pub fn print(&mut self, text: &str, x: i32, y: i32, color: u16) {
        if let Some(font) = &self.font {
            let mut cursor_x = x;
            let cursor_y = y;

            for c in text.chars() {
                if c < font.first as char || c > font.last as char {
                    continue; // Skip characters not in the font
                }

                let glyph_index = c as usize - font.first as usize;
                let glyph = &font.glyph[glyph_index];
                let bitmap = &font.bitmap[glyph.bitmap_offset as usize..];

                for row in 0..glyph.height {
                    let dest_y = cursor_y + glyph.y_offset as i32 + row as i32;
                    if dest_y < 0 || dest_y >= GRID_SIZE as i32 {
                        continue;
                    }

                    let dest_index_base = dest_y as usize * GRID_SIZE;
                    let bitmap_row_start = (row * glyph.width) as usize;

                    for col in 0..glyph.width {
                        let dest_x = cursor_x + glyph.x_offset as i32 + col as i32;
                        if dest_x < 0 || dest_x >= GRID_SIZE as i32 {
                            continue;
                        }

                        let bit_index = bitmap_row_start + col as usize;
                        if bitmap[bit_index / 8] & (0x80 >> (bit_index % 8)) != 0 {
                            let dest_index = dest_index_base + dest_x as usize;
                            self.pixel_buffer[dest_index] = color;
                        }
                    }
                }

                cursor_x += glyph.x_advance as i32;
            }
        }
    }

    pub fn set_font(&mut self, font: GFXfont<'static>) {
        self.font = Some(font);
    }
}
