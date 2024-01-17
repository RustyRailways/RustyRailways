use std::{thread::sleep, time::Duration, sync::Arc};

use anyhow::Ok;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    io::Write,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

use esp_idf_svc::hal::{peripherals::Peripherals, units::Hertz};
use esp_idf_svc::hal;
use esp_idf_svc::http::client::{EspHttpConnection, Configuration as HttpConfiguration};
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

    // Create HTTP(S) client
    let mut config = HttpConfiguration::default();
    config.timeout = Some(core::time::Duration::from_millis(10_000));
    // POST 
    for _ in 0..100{
        let mut client = HttpClient::wrap(EspHttpConnection::new(&config)?);
        post_request(&mut client,"Hello from ESP!")?;
        sleep(Duration::from_millis(1000))
    }


    Ok(())
}
