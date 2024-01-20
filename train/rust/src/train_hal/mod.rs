use common_infrastructure::hals::TrainHal;
use anyhow::Result;


mod message_sender;
mod message_receiver;
mod fnc_reader;
mod speed_setter;
mod wifi_configuration;


struct EspTrainHal{

}

impl TrainHal for EspTrainHal {
    fn get_message(&self) -> Result<Option<common_infrastructure::messages::TrainMessage>> {
        todo!()
    }
    fn read_position(&self) -> Result<Option<common_infrastructure::Position>> {
        todo!()
    }
    fn send_message_to_master(&self, message: common_infrastructure::messages::MasterMessage) -> Result<()> {
        todo!()
    }
    fn set_speed(&self, speed: i8) {
        todo!()
    }
}