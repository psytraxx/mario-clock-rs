#![no_std]
#![no_main]

use core::future::Future;
use embassy_executor::{task, Spawner};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Sender},
};
use embassy_time::{Delay, Duration, Timer};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{Dimensions, Point, RgbColor, Size},
    primitives::Rectangle,
};
use engine::display::Display;
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output, OutputConfig},
    timer::timg::TimerGroup,
};
use esp_hal_embassy::main;
use esp_println::println;
use hub75::Hub75;
use mario::clockface::Clockface;
use static_cell::StaticCell;

const GRID_SIZE: usize = 64;

mod engine;
mod hub75;
mod mario;

extern crate alloc;

// Define the channel type for passing display data
// Define a fixed-size buffer type for the display
type DisplayBuffer = [Rgb888; GRID_SIZE * GRID_SIZE];

// Define the channel type for passing display data
static DISPLAY_CHANNEL: StaticCell<Channel<CriticalSectionRawMutex, DisplayBuffer, 1>> =
    StaticCell::new();

pub trait ClockfaceTrait {
    fn update(&mut self, display: &mut Display) -> impl Future<Output = ()> + Send;
    fn setup(&mut self, display: &mut Display);
}

#[task]
async fn clockface_task(sender: Sender<'static, CriticalSectionRawMutex, DisplayBuffer, 1>) {
    // Initialize clockface
    let mut cf = Clockface::new();

    // Initial setup
    let mut display = Display::new();
    cf.setup(&mut display);

    loop {
        // Update clock logic
        cf.update(&mut display).await;

        // Send updated display buffer through channel
        sender.send(*display.get_buffer()).await;

        // Clock update can have a slower cadence
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _timg1 = TimerGroup::new(peripherals.TIMG1);

    esp_hal_embassy::init(timg0.timer0);

    // Initialize the channel
    let channel = DISPLAY_CHANNEL.init(Channel::new());
    let sender = channel.sender();
    let receiver = channel.receiver();

    /*

         E,
        R1: OutputPin<Error = E>,
        G1: OutputPin<Error = E>,
        B1: OutputPin<Error = E>,
        R2: OutputPin<Error = E>,
        G2: OutputPin<Error = E>,
        B2: OutputPin<Error = E>,
        A: OutputPin<Error = E>,
        B: OutputPin<Error = E>,
        C: OutputPin<Error = E>,
        D: OutputPin<Error = E>,
        F: OutputPin<Error = E>,
        CLK: OutputPin<Error = E>,
        LAT: OutputPin<Error = E>,
        OE: OutputPin<Error = E>,
    > Outputs for (R1, G1, B1, R2, G2, B2, A, B, C, D, F, CLK, LAT, OE) */

    let r1 = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let r2 = Output::new(peripherals.GPIO3, Level::Low, OutputConfig::default());
    let g1 = Output::new(peripherals.GPIO6, Level::Low, OutputConfig::default());
    let g2 = Output::new(peripherals.GPIO7, Level::Low, OutputConfig::default());
    let b1 = Output::new(peripherals.GPIO11, Level::Low, OutputConfig::default());
    let b2 = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());

    let a = Output::new(peripherals.GPIO39, Level::Low, OutputConfig::default());
    let b = Output::new(peripherals.GPIO38, Level::Low, OutputConfig::default());
    let c = Output::new(peripherals.GPIO37, Level::Low, OutputConfig::default());
    let d = Output::new(peripherals.GPIO36, Level::Low, OutputConfig::default());
    let f = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());

    let clk = Output::new(peripherals.GPIO34, Level::Low, OutputConfig::default());
    let oe = Output::new(peripherals.GPIO35, Level::Low, OutputConfig::default());
    let lat = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    // Spawn the two tasks with their respective channel ends
    spawner.spawn(clockface_task(sender)).unwrap();

    let mut matrix = Hub75::new((r1, g1, b1, r2, g2, b2, a, b, c, d, f, clk, lat, oe), 4);
    matrix.clear();
    let _ = matrix.fill_solid(
        &Rectangle::new(Point::zero(), Size::new(64, 64)),
        RgbColor::BLUE,
    );
    println!("Matrix initialized");

    // Create an initial buffer
    let mut current_buffer = [Rgb888::new(0, 0, 0); GRID_SIZE * GRID_SIZE];

    loop {
        // Try to get a new buffer (non-blocking)
        if let Ok(new_buffer) = receiver.try_receive() {
            // Update our current buffer when new data is available
            current_buffer = new_buffer;
        }

        // Always render the current buffer
        matrix
            .fill_contiguous(&matrix.bounding_box(), current_buffer)
            .expect("Failed to fill matrix");
        matrix.output(&mut Delay).expect("Failed to output buffer");
    }
}
