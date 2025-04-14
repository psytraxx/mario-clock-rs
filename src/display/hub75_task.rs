use core::sync::atomic::Ordering;

use embassy_executor::task;
use embassy_time::{Duration, Instant};
use esp_hal::{gpio::AnyPin, peripherals::LCD_CAM, system::Cpu, time::Rate};
use esp_hub75::{lcd_cam::Hub75, Hub75Pins};
use esp_println::println;

use crate::{FBType, FrameBufferExchange, REFRESH_RATE};

type Hub75Type = Hub75<'static, esp_hal::Async>;

pub(crate) struct Hub75Peripherals {
    pub lcd_cam: LCD_CAM,
    pub dma_channel: esp_hal::dma::DmaChannel0,
    pub red1: AnyPin,
    pub grn1: AnyPin,
    pub blu1: AnyPin,
    pub red2: AnyPin,
    pub grn2: AnyPin,
    pub blu2: AnyPin,
    pub addr0: AnyPin,
    pub addr1: AnyPin,
    pub addr2: AnyPin,
    pub addr3: AnyPin,
    pub addr4: AnyPin,
    pub blank: AnyPin,
    pub clock: AnyPin,
    pub latch: AnyPin,
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

    let pins = Hub75Pins {
        red1: peripherals.red1,
        grn1: peripherals.grn1,
        blu1: peripherals.blu1,
        red2: peripherals.red2,
        grn2: peripherals.grn2,
        blu2: peripherals.blu2,
        addr0: peripherals.addr0,
        addr1: peripherals.addr1,
        addr2: peripherals.addr2,
        addr3: peripherals.addr3,
        addr4: peripherals.addr4,
        blank: peripherals.blank,
        clock: peripherals.clock,
        latch: peripherals.latch,
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
