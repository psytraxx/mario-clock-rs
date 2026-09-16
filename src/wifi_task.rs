use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::WIFI;
use esp_println::println;
use esp_radio::wifi::{
    self, AuthenticationMethodConfig, ControllerConfig, Interface, WifiController, WifiError,
    sta::StationConfig,
};

use static_cell::StaticCell;

static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

/// Delay before retrying a failed association attempt.
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

pub async fn connect_to_wifi(
    wifi: WIFI<'static>,
    seed: u64,
    spawner: Spawner,
) -> Result<Stack<'static>, WifiError> {
    let station_config = wifi::Config::Station(
        StationConfig::default()
            .with_ssid(env!("WIFI_SSID").try_into().expect("invalid WIFI_SSID"))
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(
                env!("WIFI_PSK").try_into().expect("invalid WIFI_PSK"),
            )),
    );

    let interfaces = esp_radio::wifi::Interface::station();
    let controller = wifi::WifiController::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .inspect_err(|e| println!("Failed to create WiFi controller: {:?}", e))?;

    let dhcp_config = DhcpConfig::default();
    let config = Config::dhcpv4(dhcp_config);

    println!("Initialize network stack");
    let stack_resources: &'static mut _ = STACK_RESOURCES.init(StackResources::new());
    let (stack, runner) = embassy_net::new(interfaces, config, stack_resources, seed);

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
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}

#[embassy_executor::task]
async fn connection(controller: WifiController<'static>) {
    if let Err(error) = connection_fallible(controller).await {
        println!("Cannot connect to WiFi: {:?}", error);
    }
}

/// Keeps the station associated for the lifetime of the device.
///
/// The link must stay up because periodic NTP re-sync reuses this stack: the
/// embassy tasks and `StackResources` are single-instance, so the stack cannot
/// be torn down and rebuilt later.
async fn connection_fallible(mut controller: WifiController<'static>) -> Result<(), WifiError> {
    println!("Start connection task");
    println!("About to connect to {}...", env!("WIFI_SSID"));
    loop {
        match controller.connect_async().await {
            Ok(info) => {
                println!("Connected to WiFi network: {:?}", info);
                // Resolves when the AP drops us; fall through and reconnect.
                match controller.wait_for_disconnect_async().await {
                    Ok(info) => println!("WiFi disconnected ({:?}) - reconnecting", info),
                    Err(e) => {
                        println!("Error waiting for disconnect: {:?} - retrying", e);
                        Timer::after(RECONNECT_DELAY).await;
                    }
                }
            }
            Err(error) => {
                println!("Failed to connect to WiFi network: {:?}", error);
                Timer::after(RECONNECT_DELAY).await;
            }
        }
    }
}
