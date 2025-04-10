#![no_std]
#![no_main]

use core::{future::Future, sync::atomic::AtomicU32};
use display::{
    display_task::display_task,
    hub75_task::{hub75_task, Hub75Peripherals},
};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_alloc::{heap_allocator, psram_allocator};
use esp_backtrace as _;
use esp_hal::{
    gpio::Pin,
    i2c::master::{Config, I2c},
    interrupt::{software::SoftwareInterruptControl, Priority},
    system::{CpuControl, Stack},
    time::Rate,
    timer::timg::TimerGroup,
};
use esp_hal_embassy::{main, InterruptExecutor};
use esp_hub75::framebuffer::DmaFrameBuffer;
use esp_hub75::framebuffer::{compute_frame_count, compute_rows};
use esp_println::println;
use pcf8563::Pcf8563;
use wifi_task::connect_to_wifi;

mod display;
mod engine;
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
    fn update(&mut self, fb: &mut FBType) -> impl Future<Output = ()> + Send;
    fn setup(&mut self, fb: &mut FBType);
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // --- RTC Initialization Start ---
    println!("Initializing I2C for BM8563 RTC...");
    let config = Config::default().with_frequency(Rate::from_khz(100));
    let i2c = I2c::new(peripherals.I2C0, config)
        .expect("Unable to create I2C instance")
        .with_scl(peripherals.GPIO42)
        .with_sda(peripherals.GPIO41);

    // Create the RTC driver instance
    let mut rtc = Pcf8563::new(i2c);
    let current_time = rtc.datetime().expect("Failed to read RTC time");
    println!("Current RTC time: {:?}", current_time);

    heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let timg1 = TimerGroup::new(peripherals.TIMG1);

    esp_hal_embassy::init([timg0.timer0, timg0.timer1]);

    // Initialize the PSRAM allocator for extra memory requirements
    psram_allocator!(peripherals.PSRAM, esp_hal::psram);

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

    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let software_interrupt = sw_ints.software_interrupt2;

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
        red1: peripherals.GPIO2.degrade(),
        grn1: peripherals.GPIO6.degrade(),
        blu1: peripherals.GPIO10.degrade(),
        red2: peripherals.GPIO3.degrade(),
        grn2: peripherals.GPIO7.degrade(),
        blu2: peripherals.GPIO11.degrade(),
        addr0: peripherals.GPIO39.degrade(),
        addr1: peripherals.GPIO38.degrade(),
        addr2: peripherals.GPIO37.degrade(),
        addr3: peripherals.GPIO36.degrade(),
        addr4: peripherals.GPIO21.degrade(),
        blank: peripherals.GPIO35.degrade(),
        clock: peripherals.GPIO34.degrade(),
        latch: peripherals.GPIO33.degrade(),
    };

    // run hub75 and display on second core
    let cpu1_fnctn = {
        move || {
            use esp_hal_embassy::Executor;
            let hp_executor = mk_static!(
                InterruptExecutor<2>,
                InterruptExecutor::new(software_interrupt)
            );
            let high_pri_spawner = hp_executor.start(Priority::Priority3);

            // hub75 runs as high priority task
            high_pri_spawner
                .spawn(hub75_task(hub75_peripherals, &RX, &TX, fb1))
                .ok();

            let lp_executor = mk_static!(Executor, Executor::new());
            // display task runs as low priority task
            lp_executor.run(|spawner| {
                spawner.spawn(display_task(&TX, &RX, fb0)).ok();
            });
        }
    };

    let mut _cpu_control = CpuControl::new(peripherals.CPU_CTRL);
    const DISPLAY_STACK_SIZE: usize = 8192;
    let app_core_stack = mk_static!(Stack<DISPLAY_STACK_SIZE>, Stack::new());

    #[allow(static_mut_refs)]
    let _guard = _cpu_control
        .start_app_core(app_core_stack, cpu1_fnctn)
        .unwrap();

    loop {
        // The main task keeps running so the executor doesn't exit
        Timer::after(Duration::from_secs(1)).await;
    }
}
