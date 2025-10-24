#![no_std]
#![no_main]

use clock::{Clock, ClockBuffs};
use core::{future::Future, sync::atomic::AtomicU32};
use display::{
    display_task::display_task,
    hub75_task::{hub75_task, Hub75Peripherals},
};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
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
use esp_println::{logger::init_logger, println};
use esp_rtos::embassy::InterruptExecutor;
use log::info;
use wifi_task::{connect_to_wifi, shutdown_wifi};

mod clock;
mod display;
mod engine;
mod mario;
mod wifi_task;

extern crate alloc;

const ROWS: usize = 64;
const COLS: usize = 64;
const BITS: u8 = 4;
const NROWS: usize = compute_rows(ROWS);
const FRAME_COUNT: usize = compute_frame_count(BITS);

// Define the channel type for passing display data
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

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    init_logger(log::LevelFilter::Info);

    let peripherals = esp_hal::init(esp_hal::Config::default());
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let software_interrupt = sw_ints.software_interrupt2;

    // --- RTC Initialization Start ---
    println!("Initializing I2C for BM8563 RTC...");
    let config = Config::default().with_frequency(Rate::from_khz(100));
    let i2c = I2c::new(peripherals.I2C0, config)
        .expect("Unable to create I2C instance")
        .with_scl(peripherals.GPIO42)
        .with_sda(peripherals.GPIO41);

    let mut clock_buffs = ClockBuffs::default();
    let mut clock = Clock::<I2CType>::new(i2c);

    let timg0 = TimerGroup::new(peripherals.TIMG0);

    esp_rtos::start(timg0.timer0);

    println!("init framebuffer exchange");
    static TX: FrameBufferExchange = FrameBufferExchange::new();
    static RX: FrameBufferExchange = FrameBufferExchange::new();

    println!("init framebuffers");
    let fb0 = mk_static!(FBType, FBType::new());
    let fb1 = mk_static!(FBType, FBType::new());

    info!("fb0: {:?}", fb0);
    info!("fb1: {:?}", fb1);

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

    let stack = connect_to_wifi(peripherals.WIFI, seed, spawner)
        .await
        .expect("Failed to connect to WiFi");

    if let Some(stack_config) = stack.config_v4() {
        println!("Client IP: {}", stack_config.address);
    } else {
        println!("Failed to get stack config");
    }
    clock
        .sync_ntp(stack, &mut clock_buffs)
        .await
        .expect("Failed to sync NTP");

    let time = Clock::<I2CType>::get_time_in_zone(chrono_tz::Europe::Zurich);
    println!("Current time: {}", time);

    println!("Shutting down WiFi and network stack");

    // Properly shutdown WiFi
    shutdown_wifi().await;

    loop {
        // The main task keeps running so the executor doesn't exit
        Timer::after(Duration::from_secs(1)).await;
    }
}
