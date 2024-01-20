use anyhow::Result;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;


const DEFAULT_SSID: &str = "PosteMobile-70228269";
const DEFAULT_PASSWORD: &str = "Pfut3fDRxEehGdhbFEH3bQZD";
const DEFAULT_BSSID: Option<[u8; 6]> = None;
const DEFAULT_AUTH_METHOD: AuthMethod = AuthMethod::WPA2Personal;
const DEFAULT_CHANNEL: Option<u8> = None;

pub struct WiFiManager{
    wifi: BlockingWifi<EspWifi<'static>>
}

impl WiFiManager {

    pub fn new(wifi: BlockingWifi<EspWifi<'static>>) -> Self{
        Self{
            wifi
        }
    }

    pub fn connect_wifi(
        &mut self,
        ssid: &str,
        password: &str,
        bssid:  Option<[u8; 6]>,
        auth_method: AuthMethod,
        channel: Option<u8>
    ) -> Result<()>{
        let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            bssid,
            auth_method,
            channel
        });
    
        self.wifi.set_configuration(&wifi_configuration)?;
    
        self.wifi.start()?;
        info!("Wifi started");
    
        self.wifi.connect()?;
        info!("Wifi connected");
    
        self.wifi.wait_netif_up()?;
        info!("Wifi netif up");
        Ok(())
    }

    pub fn connect_wifi_default(&mut self) -> Result<()>{
        self.connect_wifi(DEFAULT_SSID, DEFAULT_PASSWORD, DEFAULT_BSSID, DEFAULT_AUTH_METHOD, DEFAULT_CHANNEL)
    }

}