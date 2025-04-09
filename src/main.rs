#![no_std]
#![no_main]

use core::{future::Future, ptr::addr_of_mut, sync::atomic::AtomicU32};
use display_task::display_task;
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use embedded_graphics::pixelcolor::Rgb888;
use engine::display::Display;
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::{
    gpio::Pin,
    interrupt::{software::SoftwareInterruptControl, Priority},
    system::{CpuControl, Stack},
    timer::timg::TimerGroup,
};
use esp_hal_embassy::{main, InterruptExecutor};
use esp_hub75::framebuffer::DmaFrameBuffer;
use esp_hub75::framebuffer::{compute_frame_count, compute_rows};
use esp_println::println;
use hub75_task::{hub75_task, Hub75Peripherals};
use wifi_task::connect_to_wifi;

mod display_task;
mod engine;
mod hub75_task;
mod mario;
mod wifi_task;

extern crate alloc;

const GRID_SIZE: usize = 64;
const ROWS: usize = 64;
const COLS: usize = 64;
const BITS: u8 = 4;
const NROWS: usize = compute_rows(ROWS);
const FRAME_COUNT: usize = compute_frame_count(BITS);

// Define the channel type for passing display data
// Define a fixed-size buffer type for the display
type DisplayBuffer = [Rgb888; GRID_SIZE * GRID_SIZE];
type FBType = DmaFrameBuffer<ROWS, COLS, NROWS, BITS, FRAME_COUNT>;
type FrameBufferExchange = Signal<CriticalSectionRawMutex, &'static mut FBType>;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

static REFRESH_RATE: AtomicU32 = AtomicU32::new(0);

pub trait ClockfaceTrait {
    fn update(&mut self, display: &mut Display) -> impl Future<Output = ()> + Send;
    fn setup(&mut self, display: &mut Display);
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let software_interrupt = sw_ints.software_interrupt2;

    heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let timg1 = TimerGroup::new(peripherals.TIMG1);

    let stack = connect_to_wifi(
        peripherals.WIFI,
        timg1.timer0,
        peripherals.RADIO_CLK,
        peripherals.RNG,
        spawner,
    )
    .await
    .expect("Failed to connect to WiFi");

    if let Some(stack_config) = stack.config_v4() {
        println!("Client IP: {}", stack_config.address);
    } else {
        println!("Failed to get stack config");
    }

    esp_hal_embassy::init([timg0.timer0, timg0.timer1]);

    let mut cpu_control = CpuControl::new(peripherals.CPU_CTRL);

    static mut APP_CORE_STACK: Stack<8192> = Stack::new();

    println!("init framebuffer exchange");
    static TX: FrameBufferExchange = FrameBufferExchange::new();
    static RX: FrameBufferExchange = FrameBufferExchange::new();

    println!("init framebuffers");
    let fb0 = mk_static!(FBType, FBType::new());
    let fb1 = mk_static!(FBType, FBType::new());
    fb0.clear();
    fb1.clear();

    let hub75_peripherals = Hub75Peripherals {
        lcd_cam: peripherals.LCD_CAM,
        dma_channel: peripherals.DMA_CH0,
        red1: peripherals.GPIO38.degrade(),
        grn1: peripherals.GPIO42.degrade(),
        blu1: peripherals.GPIO48.degrade(),
        red2: peripherals.GPIO47.degrade(),
        grn2: peripherals.GPIO2.degrade(),
        blu2: peripherals.GPIO21.degrade(),
        addr0: peripherals.GPIO14.degrade(),
        addr1: peripherals.GPIO46.degrade(),
        addr2: peripherals.GPIO13.degrade(),
        addr3: peripherals.GPIO9.degrade(),
        addr4: peripherals.GPIO3.degrade(),
        blank: peripherals.GPIO11.degrade(),
        clock: peripherals.GPIO12.degrade(),
        latch: peripherals.GPIO10.degrade(),
    };

    spawner.spawn(display_task(&TX, &RX, fb0)).ok();

    let _guard = cpu_control
        .start_app_core(unsafe { &mut *addr_of_mut!(APP_CORE_STACK) }, move || {
            let executor = mk_static!(
                InterruptExecutor<2>,
                InterruptExecutor::new(software_interrupt)
            );
            let high_pri_spawner = executor.start(Priority::Priority3);
            high_pri_spawner
                .spawn(hub75_task(hub75_peripherals, &RX, &TX, fb1))
                .ok();
        })
        .unwrap();

    loop {
        Timer::after(Duration::from_millis(100)).await;
    }
}
