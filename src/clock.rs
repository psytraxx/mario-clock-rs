use chrono::{DateTime, Datelike, Timelike, Utc};
use core::net::SocketAddr::V4;
use core::net::SocketAddrV4;
use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{dns, IpAddress, Ipv4Address, Stack};
use embassy_time::Instant;
use embedded_hal::i2c::I2c;
use esp_println::println;
use pcf8563::PCF8563;
use sntpc::{sntp_process_response, sntp_send_request, NtpContext, NtpTimestampGenerator};

static TIME_OFFSET_US: AtomicUsize = AtomicUsize::new(0);

pub struct ClockBuffs {
    rx_meta: [PacketMetadata; 16],
    rx_buffer: [u8; 1024],
    tx_meta: [PacketMetadata; 16],
    tx_buffer: [u8; 1024],
}

impl Default for ClockBuffs {
    fn default() -> Self {
        Self {
            rx_meta: [PacketMetadata::EMPTY; 16],
            rx_buffer: [0; 1024],
            tx_meta: [PacketMetadata::EMPTY; 16],
            tx_buffer: [0; 1024],
        }
    }
}

pub struct Clock<'a, I2C: I2c> {
    rtc: PCF8563<I2C>,
    socket: Option<UdpSocket<'a>>,
}

impl<'a, I2C: I2c> Clock<'a, I2C> {
    pub fn new<T: I2c>(i2c: T) -> Self
    where
        I2C: From<T>,
    {
        let mut rtc = PCF8563::new(i2c.into());
        let datetime = rtc.get_datetime().ok();

        if let Some(time) = datetime {
            let timestamp = Self::rtc_datetime_to_timestamp(time);
            TIME_OFFSET_US.store(
                timestamp.try_into().expect("Unable to convert to usize"),
                Ordering::Relaxed,
            );
        } else {
            println!("Failed to read RTC time - you should call sync_ntp() - otherwise we are unable to determine the time");
        };

        Clock { rtc, socket: None }
    }

    pub async fn sync_ntp(
        &mut self,
        stack: Stack<'a>,
        buffs: &'a mut ClockBuffs,
    ) -> Result<(), dns::Error> {
        let mut socket = UdpSocket::new(
            stack,
            &mut buffs.rx_meta,
            &mut buffs.rx_buffer,
            &mut buffs.tx_meta,
            &mut buffs.tx_buffer,
        );
        socket.bind(123).unwrap();

        let addr: Ipv4Address = self.dns_query(&stack, "pool.ntp.org").await?;

        self.socket = Some(socket);

        let offset_microseconds = TIME_OFFSET_US.load(Ordering::Relaxed);
        let context = NtpContext::new(TimeStampGen::new(
            offset_microseconds
                .try_into()
                .expect("Unable to convert to usize"),
        ));

        println!("getting time from {}", addr);
        let addr = V4(SocketAddrV4::new(addr, 123));

        let req = sntp_send_request(addr, self.socket.as_ref().unwrap(), context)
            .await
            .unwrap();

        let response = sntp_process_response(addr, self.socket.as_ref().unwrap(), context, req)
            .await
            .expect("Failed to process NTP response");

        println!("received NTP response: {:?}", response);
        TIME_OFFSET_US.fetch_add(
            response
                .offset
                .try_into()
                .expect("Unable to convert to usize"),
            Ordering::Relaxed,
        );

        self.set_rtc();

        Ok(())
    }

    pub fn get_time() -> DateTime<Utc> {
        let instant_us: usize = Instant::now().as_micros().try_into().unwrap();
        let offset_us = TIME_OFFSET_US.load(Ordering::Relaxed);
        let time_us = instant_us + offset_us;
        DateTime::from_timestamp_micros(time_us.try_into().expect("Unable to convert to i64"))
            .unwrap()
    }

    pub fn get_time_in_zone(zone: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
        Self::get_time().with_timezone(&zone)
    }

    fn rtc_datetime_to_timestamp(datetime: pcf8563::DateTime) -> i64 {
        let year = datetime.year as i32 + 2000; // pcf8563 year is since 2000
        let month = datetime.month as u32;
        let day = datetime.day as u32;
        let hour = datetime.hours as u32;
        let minute = datetime.minutes as u32;
        let second = datetime.seconds as u32;

        // Use chrono to create a DateTime object
        let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
            .expect("Failed to create NaiveDateTime");

        // Convert to DateTime<Utc>
        let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, chrono::Utc);
        // Get the Unix timestamp (microseconds since epoch)
        datetime_utc.timestamp_micros()
    }

    fn set_rtc(&mut self) {
        let time_us = TIME_OFFSET_US.load(Ordering::Relaxed);
        let t = DateTime::<Utc>::from_timestamp_micros(
            time_us.try_into().expect("Unable to convert to usize"),
        )
        .unwrap();
        // Set RTC time
        if let Err(e) = self.rtc.set_datetime(&pcf8563::DateTime {
            hours: t.hour() as u8,
            minutes: t.minute() as u8,
            seconds: t.second() as u8,
            year: (t.year() - 2000) as u8,
            month: t.month() as u8,
            day: t.day() as u8,
            weekday: t.weekday() as u8,
        }) {
            println!("Failed to set RTC time: {:?}", e);
        }
    }

    async fn dns_query(&self, stack: &Stack<'_>, domain: &str) -> Result<Ipv4Address, dns::Error> {
        stack
            .dns_query(domain, embassy_net::dns::DnsQueryType::A)
            .await
            .and_then(|addrs| {
                addrs
                    .iter()
                    .find_map(|item| match item {
                        IpAddress::Ipv4(v4) => Some(*v4),
                        _ => None,
                    })
                    .ok_or(dns::Error::Failed)
            })
    }
}

#[derive(Copy, Clone)]
struct TimeStampGen {
    val: i64, // Boot offset in microseconds
}
impl TimeStampGen {
    fn new(boot_offset: i64) -> Self {
        TimeStampGen { val: boot_offset }
    }
}

impl NtpTimestampGenerator for TimeStampGen {
    fn init(&mut self) {
        let stamp: i64 = Instant::now().as_micros().try_into().unwrap();
        self.val += stamp;
    }

    fn timestamp_sec(&self) -> u64 {
        (self.val.div_euclid(1_000_000)).try_into().unwrap()
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        (self.val.rem_euclid(1_000_000)).try_into().unwrap()
    }
}
