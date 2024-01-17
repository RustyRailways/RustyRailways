use std::{thread::sleep, time::Duration, sync::Arc};

use anyhow::Ok;
use embedded_svc::{
    http::{client::Client as HttpClient, Method, server::Handler},
    io::Write,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

use esp_idf_svc::hal::{peripherals::Peripherals, units::Hertz};
use esp_idf_svc::hal;
use esp_idf_svc::http::server::{EspHttpServer,Configuration as ServerConfigurations, HandlerResult};
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use enumset::enum_set;
use hal::prelude::*;
use hal::spi::*;
use hal::task::*;

use mfrc522::comm::eh02::spi::SpiInterface;
use mfrc522::Mfrc522;

use log::{error, info};


// set up for the 
const SSID: &str = "Rete Pra Alto";
const PASSWORD: &str = "teonilla";
const URL: &str = "http://192.168.1.118:9000";




fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    // Setup Wifi
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;
    connect_wifi(&mut wifi)?;

    let mut server = EspHttpServer::new(&ServerConfigurations::default())?;

    server.fn_handler("/message",Method::Post,|mut x| {
        let mut buff = [0 as u8;100];
        let n: usize = x.read(&mut buff)?;
        let (data,_) = buff.as_slice().split_at(n);
        println!("{:?}",std::str::from_utf8(data));
        HandlerResult::Ok(())
    })?;

    loop {
        info!("alive");
        sleep(Duration::from_millis(1000))
    }

    Ok(())
}



 
fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    info!("Wifi started");

    wifi.connect()?;
    info!("Wifi connected");

    wifi.wait_netif_up()?;
    info!("Wifi netif up");

    Ok(())
}
