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
pub static STOP_WIFI_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

pub async fn connect_to_wifi(
    wifi: peripherals::WIFI,
    timer: esp_hal::timer::timg::Timer,
    radio_clocks: peripherals::RADIO_CLK,
    rng: RNG,
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
    runner.run().await
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
                controller.stop_async().await?;
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
