use std::{cell::RefCell, thread::sleep, time::Duration};

use common_infrastructure::hals::{TrainHal,GenericHal};
use common_infrastructure::messages::{TrainMessage,MasterMessage};
use common_infrastructure::Position;
use anyhow::Result;
use esp_idf_svc::hal;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use hal::peripherals::Peripherals;


mod message_sender;
use message_sender::MessageSender;
mod message_receiver;
use message_receiver::MessageReceiver;
mod nfc_reader;
use nfc_reader::NfcReader;
mod speed_setter;
use speed_setter::MotorDriver;
mod wifi_configuration;
use wifi_configuration::WiFiManager;


struct EspController<'a>{
    nfc_reader: NfcReader<'a>,
    motor_driver: MotorDriver<'a>,
    message_sender: MessageSender,
    message_receiver: MessageReceiver<'a,TrainMessage>,
}

impl EspController<'_> {    
    fn new() -> Result<Self> {    
        let peripherals = Peripherals::take()?;

        let spi = peripherals.spi2;
        let sclk = peripherals.pins.gpio15;
        let serial_in = peripherals.pins.gpio16;
        let serial_out = peripherals.pins.gpio17;
        let cs_1 = peripherals.pins.gpio18;
        let nfc_reader = NfcReader::new(spi, sclk, serial_in, serial_out, cs_1)?;

        let timer = peripherals.ledc.timer0;
        let pwm_pin = peripherals.pins.gpio25;
        let forward_pin = peripherals.pins.gpio26;
        let backward_pin = peripherals.pins.gpio27;
        let channel = peripherals.ledc.channel0;
        let motor_driver = MotorDriver::new(timer, pwm_pin, forward_pin, backward_pin, channel)?;

        let message_sender = MessageSender::new();

        let message_receiver = MessageReceiver::<TrainMessage>::new("/train_message")?;

        let sys_loop = EspSystemEventLoop::take()?;
        let nvs = EspDefaultNvsPartition::take()?;
    
        let wifi = BlockingWifi::wrap(
            EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
            sys_loop,
        )?;
        let mut wifi_manager = WiFiManager::new(wifi);

        wifi_manager.connect_wifi_default()?;

        return Ok(Self{motor_driver,message_receiver,message_sender,nfc_reader});
    }

}

pub struct EspTrainHal<'a>{
    controller: RefCell<EspController<'a>>
}

impl GenericHal for EspTrainHal<'_> {
    fn new() -> Result<Self> {
        Ok(Self{
          controller: EspController::new()?.into()
        })
    }
    fn sleep_for_ms(&self, ms: u32) {
        sleep(Duration::from_millis(ms as u64))
    }
}

impl TrainHal for EspTrainHal<'_> {
    fn get_message(&self) -> Result<Option<TrainMessage>> {
        self.controller.borrow_mut().message_receiver.get_message()
    }
    fn read_position(&self) -> Result<Option<common_infrastructure::Position>> {
        let tag = self.controller.borrow_mut().nfc_reader.try_get_tag();
        let tag = match tag{
            Some(t) => t,
            None => return Ok(None)
        };
        let tag = Position::from_id(tag)?;
        
        return Ok(Some(tag))
    }
    fn send_message_to_master(&self, message: MasterMessage) -> Result<()> {
        self.controller.borrow().message_sender.send_message(message)
    }
    fn set_speed(&self, speed: i8) -> Result<()> {
        self.controller.borrow_mut().motor_driver.set_speed(speed)
    }
}