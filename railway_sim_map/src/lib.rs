use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use common_infrastructure::devices::{Switch, Train as TrainEnum};
use common_infrastructure::hals::{GenericHal, MasterHal};
use common_infrastructure::messages::{MasterMessage, SwitchMessage, TrainMessage};
use crate::map::Map;

pub mod map;
mod train;
use train::Train;


pub struct SimulatedMap{
    message_queue: RefCell<VecDeque<MasterMessage>>,
    map: RefCell<Map>,
    trains: RefCell<HashMap<TrainEnum,Train>>
}

impl GenericHal for SimulatedMap{
    fn new() -> anyhow::Result<Self> {
        todo!()
    }
    fn sleep_for_ms(ms: u32) {
        return;
    }
}

impl MasterHal for SimulatedMap {
    fn get_message(&self) -> anyhow::Result<Option<MasterMessage>> {
        Ok(self.message_queue.borrow_mut().pop_front())
    }
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> anyhow::Result<()> {
        let switch = self.map.borrow_mut().
    }
    fn send_message_to_train(&self, train: TrainEnum, message: TrainMessage) -> anyhow::Result<()> {
        if let TrainMessage::SetSpeed(speed) = message{
            self.trains.borrow_mut().get_mut(&train).unwrap().set_speed(speed);
        }
        Ok(())
    }
}
