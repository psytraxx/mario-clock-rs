use chrono::{DateTime, Datelike, Timelike, Utc};
use core::net::SocketAddr::V4;
use core::net::SocketAddrV4;
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{IpAddress, Ipv4Address, Stack, dns};
use embassy_time::Instant;
use embedded_hal::i2c::I2c;
use esp_println::println;
use pcf8563::PCF8563;
use sntpc::{NtpContext, NtpTimestampGenerator, sntp_process_response, sntp_send_request};
use sntpc_net_embassy::UdpSocketWrapper;

static TIME_OFFSET_SECONDS: AtomicU32 = AtomicU32::new(0);

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
    socket: Option<UdpSocketWrapper<'a>>,
}

impl<'a, I2C: I2c> Clock<'a, I2C> {
    pub fn new<T: I2c>(i2c: T) -> Self
    where
        I2C: From<T>,
    {
        let mut rtc = PCF8563::new(i2c.into());
        let datetime = rtc.get_datetime().ok();

        println!("RTC time: {:?}", datetime);

        if let Some(time) = datetime {
            let timestamp = Self::rtc_datetime_to_timestamp(time);
            let uptime_seconds = Instant::now().as_secs() as u32;
            let boot_time_offset = timestamp.saturating_sub(uptime_seconds);
            TIME_OFFSET_SECONDS.store(boot_time_offset, Ordering::Relaxed);
        } else {
            println!(
                "Failed to read RTC time - you should call sync_ntp() - otherwise we are unable to determine the time"
            );
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

        // Bind socket with error handling
        if let Err(e) = socket.bind(123) {
            println!("Failed to bind UDP socket to port 123: {:?}", e);
            return Err(dns::Error::Failed);
        }

        let socket = UdpSocketWrapper::from(socket);

        let addr: Ipv4Address = self.dns_query(&stack, "pool.ntp.org").await?;

        self.socket = Some(socket);

        let offset_seconds = TIME_OFFSET_SECONDS.load(Ordering::Relaxed);
        let context = NtpContext::new(TimeStampGen::new(offset_seconds as i64 * 1_000_000));

        println!("getting time from {}", addr);
        let addr = V4(SocketAddrV4::new(addr, 123));

        // Send NTP request with error handling
        let socket_ref = self.socket.as_ref().ok_or_else(|| {
            println!("ERROR: Socket not initialized");
            dns::Error::Failed
        })?;

        let req = match sntp_send_request(addr, socket_ref, context).await {
            Ok(r) => r,
            Err(e) => {
                println!("Failed to send NTP request: {:?}", e);
                return Err(dns::Error::Failed);
            }
        };

        // Process NTP response
        let socket_ref = self.socket.as_ref().ok_or_else(|| {
            println!("ERROR: Socket not initialized");
            dns::Error::Failed
        })?;

        if let Ok(response) = sntp_process_response(addr, socket_ref, context, req).await {
            println!("received NTP response: {:?}", response);
            let uptime_seconds = Instant::now().as_secs() as u32;
            let boot_time_offset = response.seconds.saturating_sub(uptime_seconds);
            TIME_OFFSET_SECONDS.store(boot_time_offset, Ordering::Relaxed);
            self.set_rtc();
        } else {
            println!("Failed to process NTP response");
            return Err(dns::Error::Failed);
        }

        Ok(())
    }

    pub fn get_time() -> DateTime<Utc> {
        let instant_seconds = Instant::now().as_secs();
        let offset_seconds: u64 = TIME_OFFSET_SECONDS.load(Ordering::Relaxed) as u64;
        let time_seconds = instant_seconds + offset_seconds;

        // Convert with fallback to epoch if conversion fails
        let time_i64 = match time_seconds.try_into() {
            Ok(t) => t,
            Err(_) => {
                println!("Warning: Time overflow, using epoch");
                0i64
            }
        };

        // Create timestamp with fallback to epoch
        match DateTime::from_timestamp(time_i64, 0) {
            Some(dt) => dt,
            None => {
                println!("Warning: Invalid timestamp, using epoch");
                DateTime::UNIX_EPOCH
            }
        }
    }

    pub fn get_time_in_zone(zone: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
        Self::get_time().with_timezone(&zone)
    }

    fn rtc_datetime_to_timestamp(datetime: pcf8563::DateTime) -> u32 {
        let year = datetime.year as i32 + 2000; // pcf8563 year is since 2000
        let month = datetime.month as u32;
        let day = datetime.day as u32;
        let hour = datetime.hours as u32;
        let minute = datetime.minutes as u32;
        let second = datetime.seconds as u32;

        // Use chrono to create a DateTime object with error handling
        let naive = match chrono::NaiveDate::from_ymd_opt(year, month, day)
            .and_then(|date| date.and_hms_opt(hour, minute, second))
        {
            Some(dt) => dt,
            None => {
                println!("Warning: Invalid RTC date/time, using epoch");
                return 0; // Return Unix epoch
            }
        };

        // Convert to DateTime<Utc>
        let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, chrono::Utc);
        // Get the Unix timestamp (seconds since epoch)
        datetime_utc.timestamp() as u32
    }

    fn set_rtc(&mut self) {
        let time_seconds = TIME_OFFSET_SECONDS.load(Ordering::Relaxed);

        // Convert timestamp with error handling
        let t = match DateTime::<Utc>::from_timestamp(time_seconds as i64, 0) {
            Some(dt) => dt,
            None => {
                println!("ERROR: Invalid timestamp, cannot set RTC");
                return;
            }
        };

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
                    .map(|item| match item {
                        IpAddress::Ipv4(v4) => *v4,
                    })
                    .next()
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
        // Convert with saturating behavior if overflow occurs
        let stamp: i64 = Instant::now().as_micros().try_into().unwrap_or_else(|_| {
            println!("Warning: Timestamp overflow in NTP generator");
            i64::MAX
        });
        self.val = self.val.saturating_add(stamp);
    }

    fn timestamp_sec(&self) -> u64 {
        // Saturate to max value if conversion fails
        (self.val.div_euclid(1_000_000))
            .try_into()
            .unwrap_or_else(|_| {
                println!("Warning: Timestamp seconds overflow");
                u64::MAX
            })
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        // Saturate to max value if conversion fails
        (self.val.rem_euclid(1_000_000))
            .try_into()
            .unwrap_or_else(|_| {
                println!("Warning: Timestamp microseconds overflow");
                u32::MAX
            })
    }
}
