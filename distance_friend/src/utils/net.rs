use cyw43::Control;
use defmt::{debug, error, info};
use dotenvy_macro::dotenv;
use embassy_net::Stack;
use embassy_time::{Duration, Timer};
use embassy_usb::class::cdc_ncm::embassy_net::Device;
use heapless::Vec;

const MAX_RETRIES: i8 = 10;
const KNOWN_NETWORKS_NUM: usize = 4;

#[derive(Debug)]
pub struct NetConnectError;

pub async fn connect_to_network(
    control: &mut Control<'_>,
    stack: &Stack<'_, Device<'static, 1514>>,
) -> Result<(), NetConnectError> {
    let known_networks: Vec<&str, KNOWN_NETWORKS_NUM> =
        dotenv!("WIFI_NETWORK").split(',').collect();
    let passwords: Vec<&str, KNOWN_NETWORKS_NUM> = dotenv!("WIFI_PASSWORD").split(',').collect();
    let mut connect_index: Option<usize> = None;

    info!("Scanning for networks!");

    let mut all_networks = control.scan().await;

    while let Some(network) = all_networks.next().await {
        if let Ok(name) = core::str::from_utf8(&network.ssid) {
            let ssid = name.trim_matches(char::from(0));
            debug!("Found network: {:?}", name);
            if let Some(index) = known_networks.iter().position(|n| n.eq(&ssid)) {
                connect_index = Some(index);
                info!("Network {:?} is known", ssid);
                break;
            }
        }
    }

    drop(all_networks);

    if let Some(network_index) = connect_index {
        connect(
            control,
            stack,
            known_networks.get(network_index).expect(""),
            passwords.get(network_index).expect("msg"),
        )
        .await
    } else {
        error!("No network found");
        Err(NetConnectError)
    }
}

async fn connect(
    control: &mut Control<'_>,
    stack: &Stack<'_, Device<'static, 1514>>,
    ssid: &str,
    password: &str,
) -> Result<(), NetConnectError> {
    let mut delay = 2;
    let mut connected = false;

    for _ in 0..MAX_RETRIES {
        let connect_result = control.join_wpa2(ssid, password).await;

        match connect_result {
            Ok(_) => {
                info!("Connected to network!!");
                connected = true;
                break;
            }
            Err(e) => {
                info!(
                    "Failed with err: {:?} Waiting {:?} to reconnect",
                    e.status, delay
                );
                Timer::after(Duration::from_secs(delay)).await;
                delay *= 2
            }
        }
    }

    if connected {
        info!("Waiting for DHCP up...");
        while stack.config_v4().is_none() {
            Timer::after(Duration::from_secs(1)).await;
            debug!("DHCP is down, retrying");
        }
        info!("DHCP is up!");
        Ok(())
    } else {
        Err(NetConnectError)
    }
}
