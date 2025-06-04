use core::sync::atomic::Ordering;

use embassy_executor::task;
use embassy_time::{Duration, Instant};
use esp_hal::{
    gpio::Pin,
    peripherals::{
        DMA_CH0, GPIO10, GPIO11, GPIO2, GPIO21, GPIO3, GPIO33, GPIO34, GPIO35, GPIO36, GPIO37,
        GPIO38, GPIO39, GPIO6, GPIO7, LCD_CAM,
    },
    system::Cpu,
    time::Rate,
};
use esp_hub75::{Hub75, Hub75Pins16};
use esp_println::println;

use crate::{FBType, FrameBufferExchange, REFRESH_RATE};

type Hub75Type = Hub75<'static, esp_hal::Async>;

pub(crate) struct Hub75Peripherals {
    pub lcd_cam: LCD_CAM<'static>,
    pub dma_channel: DMA_CH0<'static>,
    pub red1: GPIO2<'static>,
    pub grn1: GPIO6<'static>,
    pub blu1: GPIO10<'static>,
    pub red2: GPIO3<'static>,
    pub grn2: GPIO7<'static>,
    pub blu2: GPIO11<'static>,
    pub addr0: GPIO39<'static>,
    pub addr1: GPIO38<'static>,
    pub addr2: GPIO37<'static>,
    pub addr3: GPIO36<'static>,
    pub addr4: GPIO21<'static>,
    pub blank: GPIO35<'static>,
    pub clock: GPIO34<'static>,
    pub latch: GPIO33<'static>,
}

#[task]
pub(crate) async fn hub75_task(
    peripherals: Hub75Peripherals,
    rx: &'static FrameBufferExchange,
    tx: &'static FrameBufferExchange,
    fb: &'static mut FBType,
) {
    println!("Starting hub75_task() on core {}", Cpu::current() as usize);
    let channel = peripherals.dma_channel;
    let (_, tx_descriptors) = esp_hal::dma_descriptors!(0, size_of::<FBType>());

    let pins = Hub75Pins16 {
        red1: peripherals.red1.degrade(),
        grn1: peripherals.grn1.degrade(),
        blu1: peripherals.blu1.degrade(),
        red2: peripherals.red2.degrade(),
        grn2: peripherals.grn2.degrade(),
        blu2: peripherals.blu2.degrade(),
        addr0: peripherals.addr0.degrade(),
        addr1: peripherals.addr1.degrade(),
        addr2: peripherals.addr2.degrade(),
        addr3: peripherals.addr3.degrade(),
        addr4: peripherals.addr4.degrade(),
        blank: peripherals.blank.degrade(),
        clock: peripherals.clock.degrade(),
        latch: peripherals.latch.degrade(),
    };

    let mut hub75 = Hub75Type::new_async(
        peripherals.lcd_cam,
        pins,
        channel,
        tx_descriptors,
        Rate::from_mhz(10),
    )
    .expect("failed to create Hub75!");

    let mut count = 0u32;
    let mut start = Instant::now();

    // keep the frame buffer in an option so we can swap it
    let mut fb = Some(fb);

    loop {
        // if there is a new buffer available, swap it and send the old one
        if rx.signaled() {
            let new_fb = rx.wait().await;
            let old_fb = fb.replace(new_fb).unwrap();
            tx.signal(old_fb);
        }
        if let Some(ref mut fb) = fb {
            let mut xfer = hub75
                .render(fb)
                .map_err(|(e, _hub75)| e)
                .expect("failed to start render!");
            xfer.wait_for_done()
                .await
                .expect("render DMA transfer failed");
            let (result, new_hub75) = xfer.wait();
            hub75 = new_hub75;
            result.expect("transfer failed");
        }

        count += 1;
        const FPS_INTERVAL: Duration = Duration::from_secs(1);
        if start.elapsed() > FPS_INTERVAL {
            REFRESH_RATE.store(count, Ordering::Relaxed);
            count = 0;
            start = Instant::now();
        }
    }
}
