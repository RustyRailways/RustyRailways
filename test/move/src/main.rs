use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

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
const SSID: &str = "Rete abc";
const PASSWORD: &str = "ciaociaocattai";

enum speed_level{
    slow,
    mid_slow,
    moderate,
    mid_fast,
    fast,
}

enum directions{
    stop,
    forward,
    backward,
}

struct Motor{
    pub pwm_driver:LedcDriver,
    pub timer:LedcTimerDriver,
    pub motor_pin1:pin,
    pub motor_pin2:pin,
}

fn set_speed(speed:speed_level, motor:Motor){
    if speed==speed_level::slow{
        motor::pwm_driver.set_duty(map(50, 0, 250)).unwrap();
    }else if speed==speed_level::mid_slow {
        motor::pwm_driver.set_duty(map(100, 0, 250)).unwrap();
    }else if  speed==speed_level::moderate{
        motor::pwm_driver.set_duty(map(150, 0, 250)).unwrap();
    }else if  speed==speed_level::mid_fast{
        motor::pwm_driver.set_duty(map(200, 0, 250)).unwrap();
    }else if speed==speed_level::fast {
        motor::pwm_driver.set_duty(map(250, 0, 250)).unwrap();
    }else{
        motor::pwm_driver.set_duty(map(150, 0, 250)).unwrap();
    }
}

fn set_direction(direction:directions, motor:Motor){
    if direction==directions::stop{
        motor::motor_pin1.set_low().unwrap();
        motor::motor_pin2.set_low().unwrap();
    }else if direction==directions::forward{
        motor::motor_pin1.set_high().unwrap();
        motor::motor_pin2.set_low().unwrap();
    }else if direction==directions::backward{
        motor::motor_pin1.set_low().unwrap();
        motor::motor_pin2.set_high().unwrap();
    }else{
        motor::motor_pin1.set_low().unwrap();
        motor::motor_pin2.set_low().unwrap();
    }
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

fn create_server() -> anyhow::Result<EspHttpServer<'static>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    let wifi_configuration = wifi::Configuration::AccessPoint(AccessPointConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;
    wifi.wait_netif_up()?;

    info!(
        "Created Wi-Fi with WIFI_SSID `{}` and WIFI_PASS `{}`",
        SSID, PASSWORD
    );

    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    // Keep wifi running beyond when this function returns (forever)
    // Do not call this if you ever want to stop or access it later.
    // Otherwise it should be returned from this function and kept somewhere
    // so it does not go out of scope.
    // https://doc.rust-lang.org/stable/core/mem/fn.forget.html
    core::mem::forget(wifi);

    Ok(EspHttpServer::new(&server_configuration)?)
}

fn main() -> ! {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    // Take Peripherals
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    //disable watchdog timer
    let config = hal::task::watchdog::TWDTConfig {
        duration: Duration::from_secs(1_000_000_000),
        panic_on_trigger: false,
        subscribed_idle_tasks: enum_set!(hal::cpu::Core::Core0)
    };
    let _ = hal::task::watchdog::TWDTDriver::new(
        peripherals.twdt,
        &config,
    )?;
    
    // set motor
    let mut motor:Motor;
    // pin motor
    motor::motor_pin1 = PinDriver::output(peripherals.pins.gpio18).unwrap();
    motor::motor_pin2 = PinDriver::output(peripherals.pins.gpio19).unwrap();
    // pwm motor
    motor::timer = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default()
            .frequency(50.Hz())
            .resolution(Resolution::Bits14),
    )
    .unwrap();
    motor::pwm_driver = LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio21,
    )
    .unwrap();
    
    // starting default value
    motor::pwm_driver.set_duty(map(0, 0, 250)).unwrap();
    set_direction(directions::stop, motor);

    // setting NFC reader
    let spi = peripherals.spi2;
    let sclk = peripherals.pins.gpio15;
    let serial_in = peripherals.pins.gpio16; // SDI or MISO
    let serial_out = peripherals.pins.gpio17; // SDO or MOSI
    let cs_1 = peripherals.pins.gpio18; // NSS

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


    //server and wifi
    let mut server = create_server()?;

    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?.write("train".as_bytes())?;
        Ok(())
    })?;
    server
        .ws_handler("/", move |ws| {
            if ws.is_new(){
                info!("new session");
                return Ok(());
            }else if ws.is_closed(){
                info!("end session");
                set_direction(directions::stop, motor);
                return Ok(());
            }
            let (_frame_type, len) = match ws.recv(&mut []) {
                Ok(frame) => frame,
                Err(e) => return Err(e),
            };
            if len > MAX_LEN {
                ws.send(FrameType::Text(false), "Request too big".as_bytes())?;
                ws.send(FrameType::Close, &[])?;
                return Err(EspError::from_infallible::<ESP_ERR_INVALID_SIZE>());
            }
        })
        /*
        .ws_handler("/setSpeed", move |ws| {
            set_speed(); 
        }
         */

    .unwrap();

    // reading tag
    loop {
        if let Some(v) = try_get_tag(&mut mfrc522){
            let tag: &[u8] = v.as_bytes();
            info!("Tag: {:?}",tag);
        }
    }
    
    Ok(())

}