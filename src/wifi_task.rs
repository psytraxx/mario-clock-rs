use embassy_executor::Spawner;
use embassy_net::{Config, DhcpConfig, Runner, Stack, StackResources};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::{self};
use esp_println::println;
use esp_radio::{
    wifi::{
        self, ClientConfig, ModeConfig, WifiController, WifiDevice, WifiError, WifiEvent,
        WifiStaState,
    },
    Controller,
};

use static_cell::StaticCell;

/// Static cell for network stack resources
static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

/// Signal to request to stop WiFi
pub(crate) static STOP_WIFI_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Signal to stop network task
static STOP_NET_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

/// Shutdown WiFi and clean up resources
pub async fn shutdown_wifi() {
    println!("Requesting WiFi shutdown...");

    // Signal both tasks to stop
    STOP_WIFI_SIGNAL.signal(());
    STOP_NET_SIGNAL.signal(());

    // Wait longer for proper cleanup
    Timer::after(Duration::from_millis(3000)).await;

    println!("WiFi shutdown complete");
}

pub async fn connect_to_wifi(
    wifi: peripherals::WIFI<'static>,
    seed: u64,
    spawner: Spawner,
) -> Result<Stack<'static>, WifiError> {
    static INIT: StaticCell<Controller<'static>> = StaticCell::new();

    // Initialize radio with error handling
    let init = match esp_radio::init() {
        Ok(radio) => INIT.init(radio),
        Err(e) => {
            println!("Failed to initialize radio: {:?}", e);
            return Err(WifiError::NotInitialized);
        }
    };

    // Create WiFi controller with error handling
    let (controller, interfaces) = match wifi::new(init, wifi, Default::default()) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to create WiFi controller: {:?}", e);
            return Err(e);
        }
    };

    let wifi_interface = interfaces.sta;

    // initialize network stack
    let dhcp_config = DhcpConfig::default();
    let config = Config::dhcpv4(dhcp_config);

    println!("Initialize network stack");
    let stack_resources: &'static mut _ = STACK_RESOURCES.init(StackResources::new());
    let (stack, runner) = embassy_net::new(wifi_interface, config, stack_resources, seed);

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    println!("Wait for network link (timeout: 30s)");
    let link_timeout = Duration::from_secs(30);
    let start = embassy_time::Instant::now();

    loop {
        if stack.is_link_up() {
            break;
        }
        if start.elapsed() > link_timeout {
            println!("ERROR: Timeout waiting for network link");
            return Err(WifiError::Disconnected);
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
            return Err(WifiError::Disconnected);
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    Ok(stack)
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    use embassy_futures::select::{select, Either};

    match select(runner.run(), STOP_NET_SIGNAL.wait()).await {
        Either::First(_) => {
            // Runner completed
        }
        Either::Second(_) => {
            // Stop signal received
            println!("Network task received stop signal");
        }
    }
    println!("Network task stopped");
}

/// Task for WiFi connection
///
/// This will wrap [`connection_fallible()`] and trap any error.
#[embassy_executor::task]
async fn connection(controller: WifiController<'static>) {
    if let Err(error) = connection_fallible(controller).await {
        println!("Cannot connect to WiFi: {:?}", error);
    }
}

async fn connection_fallible(mut controller: WifiController<'static>) -> Result<(), WifiError> {
    println!("Start connection task, device capabilities:");

    // Get capabilities with error handling
    match controller.capabilities() {
        Ok(caps) => {
            caps.iter().for_each(|o| {
                println!("{:?}", o);
            });
        }
        Err(e) => {
            println!("Warning: Could not get WiFi capabilities: {:?}", e);
            // Continue anyway - not critical
        }
    }

    loop {
        if wifi::sta_state() == WifiStaState::Connected {
            // wait until we're no longer connected
            controller.wait_for_event(WifiEvent::StaDisconnected).await;
            Timer::after(Duration::from_millis(5000)).await
        }

        if !matches!(controller.is_started(), Ok(true)) {
            // Convert SSID with error handling (use SSID env like example)
            let ssid = match env!("WIFI_SSID").try_into() {
                Ok(s) => s,
                Err(_) => {
                    println!("ERROR: WIFI_SSID is invalid or too long (max 32 chars)");
                    return Err(WifiError::InvalidArguments);
                }
            };

            // Convert password with error handling (use PASSWORD env like example)
            let password = match env!("WIFI_PSK").try_into() {
                Ok(p) => p,
                Err(_) => {
                    println!("ERROR: WIFI_PSK is invalid or too long (max 64 chars)");
                    return Err(WifiError::InvalidArguments);
                }
            };

            println!("Connecting to wifi with SSID: {}", ssid);
            let client_config = ModeConfig::Client(
                ClientConfig::default()
                    .with_ssid(ssid)
                    .with_password(password),
            );
            controller.set_config(&client_config)?;
            println!("Starting WiFi controller");
            controller.start_async().await?;
            println!("WiFi controller started");
        }

        println!("About to connect to {}...", env!("WIFI_SSID"));
        match controller.connect_async().await {
            Ok(()) => {
                println!("Connected to WiFi network");
                println!("Wait for request to stop wifi");
                STOP_WIFI_SIGNAL.wait().await;
                println!("Received signal to stop wifi");

                // Proper shutdown sequence
                if let Err(e) = controller.disconnect_async().await {
                    println!("Error disconnecting: {:?}", e);
                }
                if let Err(e) = controller.stop_async().await {
                    println!("Error stopping controller: {:?}", e);
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
