// Define color constants
pub const SKY_COLOR: u16 = 0x000E;
pub const BLACK: u16 = 0x0000;
pub const _MASK: u16 = SKY_COLOR;

pub const M_RED: u16 = 0xF801;
pub const M_SKIN: u16 = 0xfd28;
pub const M_SHOES: u16 = 0xC300;
pub const M_SHIRT: u16 = 0x7BCF;
pub const M_HAIR: u16 = 0x0000;

use micromath::F32Ext;
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
/// An `Option<[u16; 512]>` containing the pixel data for the generated cloud
/// within a fixed-size buffer. The relevant part corresponds to `width * height`.
pub fn generate_cloud(
    width: usize,
    height: usize,
    num_circles: u8,
    seed: u64,
) -> Option<[u16; 512]> {
    let size = width * height;
    if size == 0 || size > 512 {
        return None; // Ensure size is valid and doesn't exceed capacity
    }

    // Initialize RNG and pixel buffer (stack-allocated array)
    let mut rng = SmallRng::seed_from_u64(seed);
    let mut pixels = [SKY_COLOR; 512]; // Use fixed-size array

    // --- Generate Circles ---
    let max_radius = (width.min(height) as f32 / 2.0).max(1.0); // Ensure radius is at least 1
    let min_radius = (max_radius * 0.3).max(1.0); // More variation in circle sizes

    for _ in 0..num_circles {
        let radius = rng.random_range(min_radius..max_radius);
        let radius_sq = radius * radius;

        // Calculate valid center range, ensuring max >= min
        let center_x_min = radius;
        let center_x_max = (width as f32 - radius).max(center_x_min);
        let center_x = if center_x_min >= center_x_max {
            center_x_min
        } else {
            rng.random_range(center_x_min..center_x_max)
        };

        let center_y_min = radius;
        let center_y_max = (height as f32 - radius).max(center_y_min);
        let center_y = if center_y_min >= center_y_max {
            center_y_min
        } else {
            let t: f32 = rng.random();
            let t_biased = t.sqrt(); // Bias towards bottom
            center_y_min + t_biased * (center_y_max - center_y_min)
        };

        // Calculate bounding box, clamping to image dimensions
        let x_start = (center_x - radius).max(0.0).floor() as usize;
        let x_end = ((center_x + radius).ceil() as usize).min(width);
        let y_start = (center_y - radius).max(0.0).floor() as usize;
        let y_end = ((center_y + radius).ceil() as usize).min(height);

        // Draw circle within bounding box
        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = x as f32 + 0.5 - center_x;
                let dy = y as f32 + 0.5 - center_y;
                if dx * dx + dy * dy <= radius_sq {
                    let idx = y * width + x;
                    // Direct array access (ensure idx is always < size <= 512)
                    pixels[idx] = 0xFFFF; // White
                }
            }
        }
    }

    // --- Apply Outline ---
    // No need to clone pixels; we read from it to determine boundary, then modify it.
    let mut boundary = [false; 512]; // Buffer for boundary flags

    // 1. Identify boundary pixels (reading from the current state of pixels)
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if pixels[idx] == 0xFFFF {
                // Is it a cloud pixel?
                let neighbors = [
                    (x.wrapping_sub(1), y), // Left
                    (x + 1, y),             // Right
                    (x, y.wrapping_sub(1)), // Top
                    (x, y + 1),             // Bottom
                ];

                let mut is_boundary = false;
                for (nx, ny) in neighbors {
                    // Check if neighbor is outside bounds OR is sky color
                    if nx >= width || ny >= height || pixels[ny * width + nx] == SKY_COLOR {
                        is_boundary = true;
                        break;
                    }
                }
                // Ensure index is within bounds for the boundary array
                if idx < 512 {
                    boundary[idx] = is_boundary;
                }
            }
        }
    }

    // 2. Add irregularity to boundary pixels for more natural look
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if idx < 512 && boundary[idx] {
                // Randomly remove some boundary pixels to create irregular edges
                if rng.next_u32() % 5 == 0 {
                    pixels[idx] = SKY_COLOR;
                    boundary[idx] = false; // Update boundary flag
                }
            }
        }
    }

    // 3. Inset the edge (apply outline color, modifying pixels array)
    // Need to read original state for this pass. Let's re-introduce the clone here,
    // or perform the check differently. A clone is simpler for now.
    let original_pixels = pixels; // Clone the state after circle generation

    for y in 1..(height.saturating_sub(1)) {
        for x in 1..(width.saturating_sub(1)) {
            let idx = y * width + x;

            // Check if it's an interior cloud pixel (originally white and not marked as boundary)
            // Read from original_pixels for the state check
            if idx < 512 && original_pixels[idx] == 0xFFFF && !boundary[idx] {
                // Check if any orthogonal neighbor IS a boundary pixel
                let is_near_boundary = boundary[idx.wrapping_sub(1)]    // Left
                                    || boundary[idx + 1]                // Right
                                    || boundary[idx.wrapping_sub(width)] // Top
                                    || boundary[idx + width]; // Bottom

                if is_near_boundary {
                    // Apply randomized outline color directly to pixels array
                    pixels[idx] = if rng.next_u32() % 4 == 0 {
                        0xFFFF
                    } else {
                        0x3DFF
                    };
                }
            }
        }
    }

    Some(pixels) // Return the modified array
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
