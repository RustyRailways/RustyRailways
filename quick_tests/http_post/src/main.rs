use std::{thread::sleep, time::Duration, collections::VecDeque};
use anyhow::Result;

use embedded_svc::{
    http::Method,
    wifi::{AuthMethod, ClientConfiguration, Configuration}
};
use embedded_svc::utils::mutex::{Mutex,StdRawMutex};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::http::server::{EspHttpServer,Configuration as ServerConfigurations, HandlerResult};
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::{eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};
use log::{error, info};


// set up for the 
const SSID: &str = "Rete Pra Alto";
const PASSWORD: &str = "teonilla";
const URL: &str = "http://192.168.1.118:9000";

static MESSAGE_QUEUE: Mutex<StdRawMutex,VecDeque<(Box<[u8;100]>,usize)>> = Mutex::new(VecDeque::new());


fn push_message(message: Box<[u8;100]>, len: usize){
    let mut queue = MESSAGE_QUEUE.lock();
    queue.push_back((message,len))
}
fn pop_message() -> Option<Result<String>>{
    let mut queue = MESSAGE_QUEUE.lock();
    let (message,len) = queue.pop_front()?;
    let (bites,_) = message.split_at(len);
    let string = std::str::from_utf8(bites);
    let string = match string {
        Ok(v) => v,
        Err(_) => return Some(Err(anyhow::anyhow!("message is not in valid UTF-8")))
    };
    return Some(Ok(string.to_owned()));
}




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
        let mut buff = Box::new([0 as u8;100]);
        let n: usize = x.read(&mut *buff)?;
        push_message(buff,n);
        HandlerResult::Ok(())
    })?;

    loop {
        
        sleep(Duration::from_millis(1000));
        while let Some(m) = pop_message() {
            info!("Message: {}",m?)
        }
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
