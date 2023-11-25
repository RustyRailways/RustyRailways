use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::hals::{GenericHal, MasterHal};
use common_infrastructure::messages::{MasterMessage, SwitchMessage, TrainMessage};

mod map;



pub struct SimulatedMap{

}

impl GenericHal for SimulatedMap{
    fn new() -> anyhow::Result<Self> {
        todo!()
    }
    fn sleep_for_ms(ms: u32) {
        todo!()
    }
}

impl MasterHal for SimulatedMap {
    fn get_message(&self) -> anyhow::Result<Option<MasterMessage>> {
        todo!()
    }
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> anyhow::Result<()> {
        todo!()
    }
    fn send_message_to_train(&self, train: Train, message: TrainMessage) -> anyhow::Result<()> {
        todo!()
    }
}