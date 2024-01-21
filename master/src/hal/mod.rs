use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::hals::{MasterHal, GenericHal};
use common_infrastructure::messages::{MasterMessage, SwitchMessage, TrainMessage};
use anyhow::Result;
use common_infrastructure::HasIpAddress;

mod message_getter;

pub struct MasterHalRaspberryPi{
    client: reqwest::blocking::Client
}

impl MasterHalRaspberryPi {
    fn send_message_to_ip(&self, ip: &str, message: String) -> Result<()> {
        self.client.post(format!("http://{}:{}/", ip, common_infrastructure::COMMON_PORT))
            .body(message)
            .send()?;
        return Ok(());
    }
}


impl GenericHal for MasterHalRaspberryPi{
    fn new() -> Result<Self> {
        return Ok(
            Self{
                client: reqwest::blocking::Client::new()
            }
        );
    }
    fn sleep_for_ms(&self, ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
}

impl MasterHal for MasterHalRaspberryPi {
    fn get_message(&self) -> Result<Option<MasterMessage>> {
        todo!()
    }
    fn send_message_to_train(&self, train: Train, message: TrainMessage) -> Result<()> {
        let ip = train.get_ip_address();
        let message_json = serde_json::to_string(&message)?;
        self.send_message_to_ip(ip, message_json)
    }
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> Result<()> {
        let ip = switch.get_ip_address();
        let message_json = serde_json::to_string(&message)?;
        self.send_message_to_ip(ip, message_json)
    }
}