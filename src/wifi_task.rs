use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, Runner, Stack, StackResources};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::WIFI;
use esp_println::println;
use esp_radio::wifi::{
    self, ControllerConfig, Interface, WifiController, WifiError, sta::StationConfig,
};

use static_cell::StaticCell;

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

pub(crate) static STOP_WIFI_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

static STOP_NET_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub async fn shutdown_wifi() {
    println!("Requesting WiFi shutdown...");
    STOP_WIFI_SIGNAL.signal(());
    STOP_NET_SIGNAL.signal(());
    Timer::after(Duration::from_millis(3000)).await;
    println!("WiFi shutdown complete");
}

pub async fn connect_to_wifi(
    wifi: WIFI<'static>,
    seed: u64,
    spawner: Spawner,
) -> Result<Stack<'static>, WifiError> {
    let station_config = wifi::Config::Station(
        StationConfig::default()
            .with_ssid(env!("WIFI_SSID"))
            .with_password(env!("WIFI_PSK").into()),
    );

    let (controller, interfaces) = wifi::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .inspect_err(|e| println!("Failed to create WiFi controller: {:?}", e))?;

    let wifi_interface = interfaces.station;

    let dhcp_config = DhcpConfig::default();
    let config = Config::dhcpv4(dhcp_config);

    println!("Initialize network stack");
    let stack_resources: &'static mut _ = STACK_RESOURCES.init(StackResources::new());
    let (stack, runner) = embassy_net::new(wifi_interface, config, stack_resources, seed);

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());

    println!("Wait for network link (timeout: 30s)");
    let link_timeout = Duration::from_secs(30);
    let start = embassy_time::Instant::now();

    loop {
        if stack.is_link_up() {
            break;
        }
        if start.elapsed() > link_timeout {
            println!("ERROR: Timeout waiting for network link");
            return Err(WifiError::NotConnected);
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Wait for IP address (timeout: 30s)");
    let ip_start = embassy_time::Instant::now();

    loop {
        if let Some(config) = stack.config_v4() {
            println!("Connected to WiFi with IP address {}", config.address);
            break;
        }
        if ip_start.elapsed() > link_timeout {
            println!("ERROR: Timeout waiting for IP address");
            return Err(WifiError::NotConnected);
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    Ok(stack)
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface<'static>>) {
    use embassy_futures::select::{Either, select};

    match select(runner.run(), STOP_NET_SIGNAL.wait()).await {
        Either::First(_) => {}
        Either::Second(_) => {
            println!("Network task received stop signal");
        }
    }
    println!("Network task stopped");
}

#[embassy_executor::task]
async fn connection(controller: WifiController<'static>) {
    if let Err(error) = connection_fallible(controller).await {
        println!("Cannot connect to WiFi: {:?}", error);
    }
}

async fn connection_fallible(mut controller: WifiController<'static>) -> Result<(), WifiError> {
    println!("Start connection task");
    println!("About to connect to {}...", env!("WIFI_SSID"));
    loop {
        match controller.connect_async().await {
            Ok(info) => {
                println!("Connected to WiFi network: {:?}", info);
                STOP_WIFI_SIGNAL.wait().await;
                println!("Received signal to stop wifi");
                if let Err(e) = controller.disconnect_async().await {
                    println!("Error disconnecting: {:?}", e);
                }
                break;
            }
            Err(error) => {
                println!("Failed to connect to WiFi network: {:?}", error);
                Timer::after(Duration::from_millis(5000)).await;
            }
        }
    }
    println!("Leave connection task");
    Ok(())
}
