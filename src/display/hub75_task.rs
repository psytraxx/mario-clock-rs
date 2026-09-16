use core::sync::atomic::Ordering;

use embassy_executor::task;
use embassy_time::{Duration, Instant};
use esp_hal::{
    gpio::{AnyPin, Pin},
    peripherals::{DMA_CH0, LCD_CAM},
    system::Cpu,
};
use esp_hub75::{Hub75, Hub75Config, Hub75Pins16};
use esp_println::println;

use crate::{FBType, FrameBufferExchange, REFRESH_RATE};

type Hub75Type = Hub75<esp_hal::Async, FBType>;

pub(crate) struct Hub75Peripherals<'a> {
    pub lcd_cam: LCD_CAM<'a>,
    pub dma_channel: DMA_CH0<'a>,
    pub red1: AnyPin<'a>,
    pub grn1: AnyPin<'a>,
    pub blu1: AnyPin<'a>,
    pub red2: AnyPin<'a>,
    pub grn2: AnyPin<'a>,
    pub blu2: AnyPin<'a>,
    pub addr0: AnyPin<'a>,
    pub addr1: AnyPin<'a>,
    pub addr2: AnyPin<'a>,
    pub addr3: AnyPin<'a>,
    pub addr4: AnyPin<'a>,
    pub blank: AnyPin<'a>,
    pub clock: AnyPin<'a>,
    pub latch: AnyPin<'a>,
}

#[task]
pub(crate) async fn hub75_task(
    peripherals: Hub75Peripherals<'static>,
    rx: &'static FrameBufferExchange,
    tx: &'static FrameBufferExchange,
    fb: &'static mut FBType,
) {
    println!("Starting hub75_task() on core {}", Cpu::current() as usize);
    let channel = peripherals.dma_channel;

    let tx_descriptors = esp_hub75::hub75_dma_descriptors!(FBType);

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

    let hub75 = Hub75Type::new_async(
        peripherals.lcd_cam,
        pins,
        channel,
        tx_descriptors,
        Hub75Config::new().with_frequency(crate::PIXEL_CLOCK),
        fb,
    )
    .expect("failed to create Hub75!");

    let mut count = 0u32;
    let mut start = Instant::now();

    loop {
        // wait for a new buffer to render, then swap it in
        let new_fb = rx.wait().await;
        let mut xfer = hub75.swap(new_fb).expect("failed to start swap!");
        xfer.wait_for_done().await;
        let old_fb = xfer.wait().expect("swap DMA transfer failed");
        tx.signal(old_fb);

        count += 1;
        const FPS_INTERVAL: Duration = Duration::from_secs(1);
        if start.elapsed() > FPS_INTERVAL {
            REFRESH_RATE.store(count, Ordering::Relaxed);
            count = 0;
            start = Instant::now();
        }
    }
}
