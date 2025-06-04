use core::str::FromStr;
use embassy_executor::Spawner;
use embassy_net::{Runner, Stack, StackResources};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};
use esp_hal::{
    peripherals::{self, RNG},
    rng::Rng,
};
use esp_println::println;
use esp_wifi::wifi::{
    ClientConfiguration, Configuration, WifiController, WifiDevice, WifiError, WifiEvent, WifiState,
};
use heapless::String;
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
    timer: esp_hal::timer::timg::Timer<'static>,
    radio_clocks: peripherals::RADIO_CLK<'static>,
    rng: RNG<'static>,
    spawner: Spawner,
) -> Result<Stack<'static>, WifiError> {
    let mut rng = Rng::new(rng);

    static INIT: StaticCell<esp_wifi::EspWifiController<'static>> = StaticCell::new();
    let init = INIT.init(esp_wifi::init(timer, rng, radio_clocks).unwrap());

    let (controller, interfaces) = esp_wifi::wifi::new(init, wifi).unwrap();

    let wifi_interface = interfaces.sta;

    // initialize network stack
    let mut dhcp_config = embassy_net::DhcpConfig::default();
    dhcp_config.hostname = Some(String::<32>::from_str("mario-clock-rs").unwrap());

    let seed = rng.random();
    let config = embassy_net::Config::dhcpv4(dhcp_config);

    println!("Initialize network stack");
    let stack_resources: &'static mut _ = STACK_RESOURCES.init(StackResources::new());
    let (stack, runner) = embassy_net::new(wifi_interface, config, stack_resources, seed.into());

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    println!("Wait for network link");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Wait for IP address");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Connected to WiFi with IP address {}", config.address);
            break;
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
    let caps = controller.capabilities().unwrap();
    caps.iter().for_each(|o| {
        println!("{:?}", o);
    });

    loop {
        if esp_wifi::wifi::wifi_state() == WifiState::StaConnected {
            // wait until we're no longer connected
            controller.wait_for_event(WifiEvent::StaDisconnected).await;
            Timer::after(Duration::from_millis(5000)).await
        }

        if !matches!(controller.is_started(), Ok(true)) {
            let ssid = env!("WIFI_SSID").try_into().unwrap();
            let password = env!("WIFI_PSK").try_into().unwrap();
            println!("Connecting to wifi with SSID: {}", ssid);
            let client_config = Configuration::Client(ClientConfiguration {
                ssid,
                password,
                ..Default::default()
            });
            controller.set_configuration(&client_config)?;
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
