use chrono::{DateTime, Utc};
use mario::clockface::Clockface;
use minifb::{Key, Window, WindowOptions};
const GRID_SIZE: usize = 64;
const SCALE: usize = 4;

mod engine;
mod mario;

#[derive(Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    fn white() -> Self {
        Pixel {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    fn black() -> Self {
        Pixel { r: 0, g: 0, b: 0 }
    }
}

pub trait ClockfaceTrait {
    fn update(&mut self);
    fn setup(&mut self, date_time: DateTime<Utc>);
}

fn main() {
    let mut pixel_buffer = vec![Pixel::black(); GRID_SIZE * GRID_SIZE];
    // Set every second pixel to white
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if (x + y) % 2 == 0 {
                pixel_buffer[y * GRID_SIZE + x] = Pixel::white();
            }
        }
    }

    let mut cf = Clockface::new();

    let now = Utc::now();
    cf.setup(now);
    cf.update();

    let mut window = Window::new(
        "Grid - ESC to exit",
        GRID_SIZE * SCALE,
        GRID_SIZE * SCALE,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer: Vec<u32> = pixel_buffer.iter().map(|p| p.to_u32()).collect();

        // Display the buffer
        window
            .update_with_buffer(&buffer, GRID_SIZE, GRID_SIZE)
            .unwrap();
    }
}
