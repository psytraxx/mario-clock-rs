use core::sync::atomic::Ordering;
use embassy_executor::task;
use esp_hal::system::Cpu;
use esp_println::println;

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
    cf.setup(fb);

    loop {
        // Update clock logic
        cf.update(fb).await;

        let _rate = REFRESH_RATE.load(Ordering::Relaxed);

        //println!("Refresh: {:4}", rate);

        // send the frame buffer to be rendered
        tx.signal(fb);

        // get the next frame buffer
        fb = rx.wait().await;
    }
}
