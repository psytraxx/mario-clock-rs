// Define color constants
pub const SKY_COLOR: u16 = 0x000E;
pub const BLACK: u16 = 0x0000;
pub const _MASK: u16 = SKY_COLOR;

pub const M_RED: u16 = 0xF801;
pub const M_SKIN: u16 = 0xfd28;
pub const M_SHOES: u16 = 0xC300;
pub const M_SHIRT: u16 = 0x7BCF;
pub const M_HAIR: u16 = 0x0000;

use esp_println::println;
use heapless::Vec;
use num_traits::float::FloatCore;
use rand::{rngs::SmallRng, Rng, RngCore, SeedableRng};

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

/// Generates cloud pixel data dynamically.
///
/// # Arguments
///
/// * `width`: The width of the cloud sprite.
/// * `height`: The height of the cloud sprite.
/// * `num_circles`: The number of circles to use for generating the cloud shape.
/// * `seed`: A seed for the random number generator for deterministic results.
///
/// # Returns
///
/// An `Option<Vec<u16, 512>>` containing the pixel data for the generated cloud.
/// The size of the Vec will be `width * height`.
pub fn generate_cloud(
    width: usize,
    height: usize,
    num_circles: u8,
    seed: u64,
) -> Option<Vec<u16, 512>> {
    let size = width * height;
    if size > 512 {
        return None; // Ensure we don't exceed the heapless Vec capacity
    }

    // Initialize SmallRng with the provided seed
    let mut rng = SmallRng::seed_from_u64(seed);

    // Create a heapless Vec with the required size
    let mut pixels: Vec<u16, 512> = Vec::new();
    pixels.resize(size, SKY_COLOR).ok()?; // Initialize with sky color

    let max_radius = (width.min(height) / 2) as f32;
    let min_radius = max_radius * 0.5;

    for _ in 0..num_circles {
        // Generate radius first
        let radius = rng.random_range(min_radius..max_radius);
        let radius_sq = radius * radius;

        // Ensure the center allows the circle to fit within bounds
        // The valid range for the center is from `radius` to `dimension - radius`
        let center_x = rng.random_range(radius..(width as f32 - radius));
        let center_y = rng.random_range(radius..(height as f32 - radius));

        // Determine the bounding box for the circle to optimize pixel checks
        let x_start = (center_x - radius).max(0.0).floor() as usize;
        let x_end = (center_x + radius).min(width as f32).ceil() as usize;
        let y_start = (center_y - radius).max(0.0).floor() as usize;
        let y_end = (center_y + radius).min(height as f32).ceil() as usize;

        // Iterate only over the bounding box pixels
        for y in y_start..y_end.min(height) {
            // Ensure y does not exceed height
            for x in x_start..x_end.min(width) {
                // Ensure x does not exceed width
                let dx = x as f32 + 0.5 - center_x; // Use pixel center for check
                let dy = y as f32 + 0.5 - center_y; // Use pixel center for check
                if dx * dx + dy * dy <= radius_sq {
                    // Use standard row-major indexing: y * width + x
                    let idx = y * width + x;
                    if let Some(pixel) = pixels.get_mut(idx) {
                        *pixel = 0xFFFF; // White color for cloud body
                    }
                }
            }
        }
    }

    // NEW: Identify boundary pixels (cloud pixel adjacent to sky)
    let original = pixels.clone();
    let mut boundary = [false; 512];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if original[idx] == 0xFFFF
                && ((x == 0 || original[y * width + (x - 1)] == SKY_COLOR)
                    || (x == width - 1 || original[y * width + (x + 1)] == SKY_COLOR)
                    || (y == 0 || original[(y - 1) * width + x] == SKY_COLOR)
                    || (y == height - 1 || original[(y + 1) * width + x] == SKY_COLOR))
            {
                boundary[idx] = true;
            }
        }
    }
    // NEW: Inset the edge: for interior cloud pixels next to boundary, set outline color
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let idx = y * width + x;
            if original[idx] == 0xFFFF
                && !boundary[idx]
                && (boundary[idx - 1]
                    || boundary[idx + 1]
                    || boundary[idx - width]
                    || boundary[idx + width])
            {
                if let Some(pixel) = pixels.get_mut(idx) {
                    *pixel = if rng.next_u32() % 4 == 0 {
                        0xFFFF // White sprinkle
                    } else {
                        0x3DFF // Bluish outline
                    };
                }
            }
        }
    }

    Some(pixels)
}

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
