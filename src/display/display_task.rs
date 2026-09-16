use core::sync::atomic::Ordering;
use embassy_executor::task;
use esp_hal::system::Cpu;
use esp_println::println;
use static_cell::StaticCell;

use crate::mario::clockface::Clockface;
use crate::{FBType, FrameBufferExchange, REFRESH_RATE};

#[task]
pub(crate) async fn display_task(
    rx: &'static FrameBufferExchange,
    tx: &'static FrameBufferExchange,
    mut fb: &'static mut FBType,
) {
    println!(
        "Starting display_task() on core {}",
        Cpu::current() as usize
    );

    // Initialize clockface in static storage: it holds several KB of sprite
    // buffers, more than this task's stack can hold.
    static CLOCKFACE: StaticCell<Clockface> = StaticCell::new();
    let cf = Clockface::init(&CLOCKFACE);

    loop {
        // Update clock logic
        cf.update(fb);

        let _rate = REFRESH_RATE.load(Ordering::Relaxed);

        // send the frame buffer to be rendered
        tx.signal(fb);

        // get the next frame buffer
        fb = rx.wait().await;
    }
}
