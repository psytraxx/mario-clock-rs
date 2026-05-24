use core::sync::atomic::Ordering;

use embassy_executor::task;
use embassy_time::{Duration, Instant};
use esp_hal::{
    gpio::{AnyPin, Pin},
    peripherals::{DMA_CH0, LCD_CAM},
    system::Cpu,
    time::Rate,
};
use esp_hub75::{Hub75, Hub75Pins16};
use esp_println::println;

use crate::{FBType, FrameBufferExchange, REFRESH_RATE};

type Hub75Type = Hub75<'static, esp_hal::Async>;

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
