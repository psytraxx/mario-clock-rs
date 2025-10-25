#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use clock::{Clock, ClockBuffs};
use core::{future::Future, sync::atomic::AtomicU32};
use display::{
    display_task::display_task,
    hub75_task::{hub75_task, Hub75Peripherals},
};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::{
    gpio::Pin,
    i2c::master::{Config, I2c},
    interrupt::{software::SoftwareInterruptControl, Priority},
    rng::Rng,
    time::Rate,
    timer::timg::TimerGroup,
    Blocking,
};
use esp_hub75::framebuffer::{compute_frame_count, compute_rows, plain::DmaFrameBuffer};
use esp_println::{logger::init_logger_from_env, println};
use esp_rtos::embassy::InterruptExecutor;
use log::info;
use wifi_task::{connect_to_wifi, shutdown_wifi};

mod clock;
mod display;
mod engine;
mod mario;
mod wifi_task;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const ROWS: usize = 64;
const COLS: usize = 64;
const BITS: u8 = 4; // 3-bit with double buffering = smooth, no flicker
const NROWS: usize = compute_rows(ROWS);
const FRAME_COUNT: usize = compute_frame_count(2); // Use double buffering

// Define a fixed-size buffer type for the display
type FBType = DmaFrameBuffer<ROWS, COLS, NROWS, BITS, FRAME_COUNT>;
type FrameBufferExchange = Signal<CriticalSectionRawMutex, &'static mut FBType>;
pub type I2CType = I2c<'static, Blocking>;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

static REFRESH_RATE: AtomicU32 = AtomicU32::new(0);

pub(crate) trait ClockfaceTrait {
    fn update(&mut self, fb: &mut FBType) -> impl Future<Output = ()> + Send;
}

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 73744);

    init_logger_from_env();

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let software_interrupt = sw_ints.software_interrupt2;

    // --- RTC Initialization Start ---
    println!("Initializing I2C for BM8563 RTC...");
    let config = Config::default().with_frequency(Rate::from_khz(100));

    // Initialize I2C with error handling
    let i2c = match I2c::new(peripherals.I2C0, config) {
        Ok(i2c) => i2c
            .with_scl(peripherals.GPIO42)
            .with_sda(peripherals.GPIO41),
        Err(e) => {
            println!("FATAL ERROR: Unable to create I2C instance: {:?}", e);
            println!("Cannot continue without I2C for RTC");
            loop {
                // Halt execution - RTC is critical for clock functionality
                Timer::after(Duration::from_secs(1)).await;
            }
        }
    };

    let mut clock_buffs = ClockBuffs::default();
    let mut clock = Clock::<I2CType>::new(i2c);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(timg0.timer0);

    println!("init framebuffer exchange");
    static TX: FrameBufferExchange = FrameBufferExchange::new();
    static RX: FrameBufferExchange = FrameBufferExchange::new();

    println!("init framebuffer 0");
    let fb0 = mk_static!(FBType, FBType::new());
    println!("Framebuffer 0 initialized");
    let fb1 = mk_static!(FBType, FBType::new());
    println!("Framebuffer 1 initialized");

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

    println!("Starting second core for display task");

    // run hub75 and display on second core
    let cpu1_fnctn = {
        move || {
            use esp_rtos::embassy::Executor;
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

    use esp_hal::system::Stack;
    const DISPLAY_STACK_SIZE: usize = 8192;
    let app_core_stack = mk_static!(Stack<DISPLAY_STACK_SIZE>, Stack::new());

    esp_rtos::start_second_core(
        peripherals.CPU_CTRL,
        sw_ints.software_interrupt0,
        sw_ints.software_interrupt1,
        app_core_stack,
        cpu1_fnctn,
    );

    let rng = Rng::new();
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    // WiFi is used only for initial NTP time synchronization
    // This is a one-shot operation to minimize power consumption
    {
        // Connect to WiFi with error handling
        let stack = match connect_to_wifi(peripherals.WIFI, seed, spawner).await {
            Ok(s) => s,
            Err(e) => {
                println!("ERROR: Failed to connect to WiFi: {:?}", e);
                println!("Continuing with RTC time only (may be inaccurate)");
                // Don't panic - clock can still function with RTC time
                // Skip NTP sync and continue
                println!("WiFi resources released. Running clock from RTC.");
                loop {
                    Timer::after(Duration::from_secs(60)).await;
                }
            }
        };

        if let Some(stack_config) = stack.config_v4() {
            println!("Client IP: {}", stack_config.address);
        } else {
            println!("Warning: Failed to get stack config");
        }

        // Sync NTP with error handling
        match clock.sync_ntp(stack, &mut clock_buffs).await {
            Ok(_) => {
                println!("NTP sync successful");
                let time = Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich);
                println!("Current time: {}", time);
            }
            Err(e) => {
                println!("Warning: Failed to sync NTP: {:?}", e);
                println!("Continuing with RTC time (may be inaccurate)");
                // Don't panic - clock can still work with RTC
            }
        }

        println!("Shutting down WiFi and network stack");

        // Properly shutdown WiFi
        shutdown_wifi().await;

        // Stack is dropped here, freeing network resources
        // RTC maintains time accuracy between reboots
    }

    println!("WiFi resources released. Running clock from RTC.");

    loop {
        info!("Starting clock update cycle");
        // Main task keeps running to prevent executor exit
        // Display continues on Core 1 independently
        Timer::after(Duration::from_secs(60)).await;
    }
}
