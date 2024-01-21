use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::hals::{MasterHal, GenericHal};
use common_infrastructure::messages::{MasterMessage, SwitchMessage, TrainMessage};
use anyhow::Result;
use common_infrastructure::HasUrl;

mod message_getter;
use message_getter::MessageReceiver;

pub struct MasterHalRaspberryPi{
    client: reqwest::blocking::Client,
    message_receiver: MessageReceiver<MasterMessage> 
}

impl MasterHalRaspberryPi {
    fn send_message_to_url(&self, url: &str, message: String) -> Result<()> {
        self.client.post(url)
            .body(message)
            .send()?;
        return Ok(());
    }
}


impl GenericHal for MasterHalRaspberryPi{
    fn new() -> Result<Self> {
        return Ok(
            Self{
                client: reqwest::blocking::Client::new(),
                message_receiver: MessageReceiver::new()
            }
        );
    }
    fn sleep_for_ms(&self, ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
}

impl MasterHal for MasterHalRaspberryPi {
    fn get_message(&self) -> Result<Option<MasterMessage>> {
        self.message_receiver.try_get_message()
    }
    fn send_message_to_train(&self, train: Train, message: TrainMessage) -> Result<()> {
        let ip = train.get_url();
        let message_json = serde_json::to_string(&message)?;
        self.send_message_to_url(ip, message_json)
    }
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> Result<()> {
        let ip = switch.get_url();
        let message_json = serde_json::to_string(&message)?;
        self.send_message_to_url(ip, message_json)
    }
}