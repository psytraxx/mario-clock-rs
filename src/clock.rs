use chrono::{DateTime, Utc};
use core::net::SocketAddrV4;
use core::sync::atomic::{AtomicUsize, Ordering};
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{dns, IpAddress, Ipv4Address, Stack};
use embassy_time::Instant;
use esp_println::println;
use sntpc::{sntp_process_response, sntp_send_request, NtpContext, NtpTimestampGenerator};

// Atomic variable to store the offset between embassy time and the "real" time
static TIME_OFFSET: AtomicUsize = AtomicUsize::new(0);

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

pub struct NtpClock<'a> {
    boot_offset: i64, // Boot offset in microseconds
    server_addr: Ipv4Address,
    socket: UdpSocket<'a>,
}

impl NtpClock<'_> {
    pub async fn sync<'a>(
        stack: Stack<'a>,
        buffs: &'a mut ClockBuffs,
    ) -> Result<NtpClock<'a>, dns::Error> {
        let mut socket = UdpSocket::new(
            stack,
            &mut buffs.rx_meta,
            &mut buffs.rx_buffer,
            &mut buffs.tx_meta,
            &mut buffs.tx_buffer,
        );
        socket.bind(123).unwrap();

        let addr = NtpClock::dns_query(&stack, "pool.ntp.org").await?;
        // let addr = Ipv4Address::new(192, 168, 178, 101);

        let mut clock = NtpClock {
            boot_offset: 0,
            server_addr: addr,
            socket,
        };

        clock.sync_time().await.unwrap();

        Ok(clock)
    }

    async fn sync_time(&mut self) -> Result<(), sntpc::Error> {
        let stamper = TimeStampGen::new(self.boot_offset);
        let context = NtpContext::new(stamper);

        println!("getting time");
        let addr = core::net::SocketAddr::V4(SocketAddrV4::new(self.server_addr, 123));

        let req = sntp_send_request(addr, &self.socket, context)
            .await
            .unwrap();

        println!("got time request");
        let response = sntp_process_response(addr, &self.socket, context, req)
            .await
            .unwrap();

        // let response = get_time(
        //     core::net::SocketAddr::V4(SocketAddrV4::new(self.server_addr, 123)),
        //     &mut self.socket,
        //     context,
        // )
        // .await?;

        println!("got time");
        self.boot_offset += response.offset;

        Ok(())
    }

    pub fn get_time(&self) -> DateTime<Utc> {
        let time_since_boot: i64 = Instant::now().as_micros().try_into().unwrap();
        let time_us = time_since_boot + self.boot_offset;
        DateTime::from_timestamp_micros(time_us).unwrap()
    }

    pub fn get_time_in_zone(&self, zone: chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
        self.get_time().with_timezone(&zone)
    }

    async fn dns_query(stack: &Stack<'_>, domain: &str) -> Result<Ipv4Address, dns::Error> {
        stack
            .dns_query(domain, embassy_net::dns::DnsQueryType::A)
            .await
            .map(|addrs| {
                *addrs
                    .iter()
                    .find_map(|item| match item {
                        IpAddress::Ipv4(v4) => Some(v4),
                        _ => None,
                    })
                    .unwrap()
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

fn microseconds_to_seconds(micros: i64) -> i64 {
    micros.div_euclid(1_000_000)
}

fn microseconds_to_micros_frac(micros: i64) -> i64 {
    micros.rem_euclid(1_000_000)
}

impl NtpTimestampGenerator for TimeStampGen {
    fn init(&mut self) {
        let stamp: i64 = Instant::now().as_micros().try_into().unwrap();
        self.val += stamp;
    }

    fn timestamp_sec(&self) -> u64 {
        microseconds_to_seconds(self.val).try_into().unwrap()
    }

    fn timestamp_subsec_micros(&self) -> u32 {
        microseconds_to_micros_frac(self.val).try_into().unwrap()
    }
}

pub fn store_offset(datetime: pcf8563::DateTime) {
    let year = datetime.year as i32 + 2000; // pcf8563 year is since 2000
    let month = datetime.month as u32;
    let day = datetime.day as u32;
    let hour = datetime.hours as u32;
    let minute = datetime.minutes as u32;
    let second = datetime.seconds as u32;

    // Use chrono to create a DateTime object
    let naive = chrono::NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(hour, minute, second))
        .unwrap();

    // Convert to DateTime<Utc>
    let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, chrono::Utc);
    // Get the Unix timestamp (seconds since epoch)
    TIME_OFFSET.store(datetime_utc.timestamp() as usize, Ordering::Relaxed);
}

pub fn get_now() -> u64 {
    let now = embassy_time::Instant::now().as_secs();

    now.checked_add(TIME_OFFSET.load(Ordering::Relaxed) as u64)
        .expect("Time offset overflow")
}
