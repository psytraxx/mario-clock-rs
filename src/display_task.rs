use core::sync::atomic::Ordering;
use embassy_executor::task;
use embassy_time::{Duration, Timer};
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::{
    prelude::{Point, RgbColor, Size},
    primitives::Rectangle,
};
use esp_hal::system::Cpu;
use esp_println::println;

use crate::engine::display::Display;
use crate::mario::clockface::Clockface;
use crate::{ClockfaceTrait, FBType, FrameBufferExchange, REFRESH_RATE};

#[task]
pub async fn display_task(
    rx: &'static FrameBufferExchange,
    tx: &'static FrameBufferExchange,
    mut fb: &'static mut FBType,
) {
    println!(
        "Starting display_task() on core {}",
        Cpu::current() as usize
    );

    // Initialize clockface
    let mut cf = Clockface::new();

    // Initial setup
    let mut display = Display::new();
    cf.setup(&mut display);

    fb.clear();
    let _ = fb.fill_solid(
        &Rectangle::new(Point::zero(), Size::new(64, 64)),
        RgbColor::BLUE,
    );

    loop {
        fb.clear();

        // Update clock logic
        cf.update(&mut display).await;

        fb.fill_contiguous(&fb.bounding_box(), *display.get_buffer())
            .expect("Failed to fill matrix");

        println!("Refresh: {:4}", REFRESH_RATE.load(Ordering::Relaxed));

        // send the frame buffer to be rendered
        tx.signal(fb);

        // get the next frame buffer
        fb = rx.wait().await;

        // Clock update can have a slower cadence
        Timer::after(Duration::from_millis(100)).await;
    }
}
