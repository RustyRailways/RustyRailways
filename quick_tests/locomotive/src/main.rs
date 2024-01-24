use std::{thread::sleep, time::Duration, sync::Arc};

use anyhow::Ok;
use embedded_svc::{
    http::{client::Client as HttpClient, Method},
    io::Write,
    utils::io,
    wifi::{AuthMethod, ClientConfiguration, Configuration},
};

use esp_idf_svc::hal::{gpio::PinDriver, peripherals::Peripherals, units::Hertz};
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
use mfrc522::register::Register;

use log::{error, info};


// set up for the 
const SSID: &str = "Rete Pra Alto";
const PASSWORD: &str = "teonilla";
const URL: &str = "http://192.168.1.118:9000";


fn main() -> anyhow::Result<()> {
    //////////////////// configure basic stuff ////////////////////////
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    ////////////////////////// take periferal  ////////////////////////
    let peripherals = Peripherals::take()?;


    ////////////////////////// disable wd timer  ////////////////////////
    let config = hal::task::watchdog::TWDTConfig {
        duration: Duration::from_secs(1_000_000_000),
        panic_on_trigger: false,
        subscribed_idle_tasks: enum_set!(hal::cpu::Core::Core0)
    };

    let _ = hal::task::watchdog::TWDTDriver::new(
        peripherals.twdt,
        &config,
    )?;

    ////////////////////////// configure RFID reader  ////////////////////////
    
    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio15;
    let serial_in = peripherals.pins.gpio16; // SDI or MISO
    let serial_out = peripherals.pins.gpio17; // SDO or MOSI
    let cs_1 = peripherals.pins.gpio18; // NSS

    let interupt_pin = peripherals.pins.gpio22;
    let interupt_pin = PinDriver::input(interupt_pin)?;


    

    println!("Starting SPI loopback test");

    let driver = hal::spi::SpiDriver::new::<hal::spi::SPI2>(
        spi,
        sclk,
        serial_out,
        Some(serial_in),
        &hal::spi::SpiDriverConfig::new(),
    )?;

    let mfrc522 = hal::spi::config::Config::new().baudrate(Hertz::from(10_000));
    let mfrc522 = hal::spi::SpiDeviceDriver::new(&driver, Some(cs_1), &mfrc522)?;
    let mfrc522: SpiInterface<SpiDeviceDriver<'_, &SpiDriver<'_>>, mfrc522::comm::eh02::spi::DummyNSS, mfrc522::comm::eh02::spi::DummyDelay> = SpiInterface::new(mfrc522);
    let mut mfrc522 = Mfrc522::new(mfrc522).init().unwrap();


    //let mut r = mfrc522.read(Register::ComlEnReg).unwrap();
    //r |= 1 << 5;
    //mfrc522.write(Register::ComlEnReg, r).unwrap();

    mfrc522.write(Register::ComIrqReg, 0x00).unwrap(); //Clear interrupts
    //mfrc522.write(Register::ComlEnReg, 0x20).unwrap(); //Enable all interrupts
    mfrc522.write(Register::ComlEnReg, 0xA0); // enabler reader interupt
    mfrc522.write(Register::DivlEnReg, 0x14 | 0x80).unwrap();



    mfrc522.write(Register::ComIrqReg, 0x00).unwrap(); //Clear interrupts
    /////////////////////////////////////////// Loop reading tags ////////////////////////////////////
    loop {
        
        /*
        let r = mfrc522.read(Register::ComIrqReg).unwrap();

        let interrupt = r & 1<<6 != 0;

        println !("interrupt: {}", interrupt);

        println !("{} - {:b}", interupt_pin.is_high(),r);

        if interrupt{
            mfrc522.write(Register::ComIrqReg, 0x00).unwrap(); //Clear interrupts
            sleep(Duration::from_millis(1000));
        }
        */


        //while interupt_pin.is_high() {
        //    mfrc522.write(Register::ComIrqReg, 0x00).unwrap(); //Clear interrupts
        //}
        //info!("pass");
    
        if let Some(v) = try_get_tag(&mut mfrc522){
            let tag: &[u8] = v.as_bytes();
            info!("Tag: {:?}",tag);
        }
    }
    //return anyhow::Ok(())
}


fn try_get_tag<T: mfrc522::comm::Interface>(mfrc522: &mut Mfrc522<T,mfrc522::Initialized>) -> Option<mfrc522::Uid>{
    let atqa = match mfrc522.reqa(){
        Result::Ok(v) => v,
        Err(_) => return None
    };
    match mfrc522.select(&atqa) {
        Result::Ok(v) => Some(v),
        Err(_) => return None
    }
}

/*
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



/// Send an HTTP POST request.
fn post_request(client: &mut HttpClient<EspHttpConnection>, message: &str) -> anyhow::Result<()> {
    // Prepare payload
    let payload = message.as_bytes();

    // Prepare headers and URL
    let content_length_header = format!("{}", payload.len());
    let headers = [
        ("content-type", "text/plain"),
        ("content-length", &*content_length_header),
    ];

    // Send request
    let mut request = client.post(URL, &headers)?;
    request.write_all(payload)?;
    request.flush()?;
    info!("-> POST {}", URL);
    let mut response = request.submit()?;

    // Process response
    let status = response.status();
    info!("<- {}", status);
    let mut buf = [0u8; 1024];
    let bytes_read = io::try_read_full(&mut response, &mut buf).map_err(|e| e.0)?;
    info!("Read {} bytes", bytes_read);
    match std::str::from_utf8(&buf[0..bytes_read]) {
        Result::Ok(body_string) => info!(
            "Response body (truncated to {} bytes): {:?}",
            buf.len(),
            body_string
        ),
        Err(e) => error!("Error decoding response body: {}", e),
    };

    // Drain the remaining response bytes
    while response.read(&mut buf)? > 0 {}

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
*/