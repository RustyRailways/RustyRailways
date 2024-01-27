use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use common_infrastructure::devices::{Switch, Train as TrainEnum};
use common_infrastructure::hals::{GenericHal, MasterHal};
use common_infrastructure::Position;
use common_infrastructure::messages::{MasterMessage, SwitchMessage, TrainMessage};
use crate::map::Map;
pub mod map;
mod train;
use train::Train;
#[cfg(test)]
mod tests;


pub struct SimulatedMap{
    message_queue: RefCell<VecDeque<MasterMessage>>,
    map: RefCell<Map>,
    trains: RefCell<HashMap<TrainEnum,Train>>
}


impl GenericHal for SimulatedMap{
    fn new() -> anyhow::Result<Self> {
        let mut trains = HashMap::new();
        trains.insert(TrainEnum::T1, Train::new(TrainEnum::T1, Position::P4));
        trains.insert(TrainEnum::T2, Train::new(TrainEnum::T2, Position::P6));

        Ok(Self{
            message_queue: RefCell::new(VecDeque::new()),
            map: Map::new().into(),
            trains: trains.into(),
        })
    }
    fn sleep_for_ms(&self, _: u32) {
        for (train_id, train) in self.trains.borrow().iter(){
            let nex_pos = train.get_next_position(self.map.borrow().deref());
            for (train_id2, train2) in self.trains.borrow().iter(){
                if train_id == train_id2{
                    continue
                }
                assert_ne!(nex_pos, train2.position, "The train {:?} has crashed with the train {:?} while in move",train_id, train_id2);
            }
        }
        for (_, train) in self.trains.borrow_mut().iter_mut(){
            let message = train.do_move(self.map.borrow().deref());
            if let Some(m) = message{
                self.message_queue.borrow_mut().push_back(m);
            }
        }

        for (train_id, train) in self.trains.borrow().iter(){
            for (train_id2, train2) in self.trains.borrow().iter(){
                if train_id == train_id2{
                    continue
                }
                assert_ne!(train.position, train2.position, "The train {:?} has crashed with the train {:?} while in move",train_id, train_id2);
            }
        }
    }
}

impl MasterHal for SimulatedMap {
    fn get_message(&self) -> anyhow::Result<Option<MasterMessage>> {
        Ok(self.message_queue.borrow_mut().pop_front())
    }
    fn send_message_to_train(&self, train: TrainEnum, message: TrainMessage) -> anyhow::Result<()> {
        if let TrainMessage::SetSpeed(speed) = message{
            self.trains.borrow_mut().get_mut(&train).unwrap().set_speed(speed);
        }
        Ok(())
    }
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> anyhow::Result<()> {
        match message {
            SwitchMessage::SetPositionDiverging => self.map.borrow_mut().set_diverted(switch),
            SwitchMessage::SetPositionStraight => self.map.borrow_mut().set_straight(switch)
        };
        Ok(())
    }
}
