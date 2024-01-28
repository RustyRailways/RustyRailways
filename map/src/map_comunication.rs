/// a module used for other components to communicate with the map
/// in particular the visualizer need to know the position of the trains,
/// and this module is necessart because the visuallizer is on a separate thread

use std::sync::mpsc::{Sender, Receiver};
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::Position;
use crate::devices::SwitchPosition;
use std::collections::HashMap;
use crate::map_creation_object::TrainStatus;

pub enum Message{
    SetSwitch(Switch, SwitchPosition),
    SetTrainPosition(Train, Position),
    SetTrainSpeed(Train, i8),
    SetTrainStatus(Train, TrainStatus),
}


pub trait Comunicator{
    fn get_comunicators() -> (MapComunicationMaster, MapComunicationSlave){
        let (tx, rx) = std::sync::mpsc::channel();
        (MapComunicationMaster::new(tx), MapComunicationSlave::new(rx))
    }
}

#[derive(Debug)]
pub struct MapComunicationMaster{
    tx: Sender<Message>
}

impl MapComunicationMaster {
    fn new(tx: Sender<Message>) -> Self{
        MapComunicationMaster{
            tx
        }
    }
    pub fn set_switch(&mut self, switch: Switch, position: SwitchPosition){
        self.tx.send(Message::SetSwitch(switch, position)).unwrap();
    }
    pub fn set_train_position(&mut self, train: Train, position: Position){
        self.tx.send(Message::SetTrainPosition(train, position)).unwrap();
    }
    pub fn set_train_speed(&mut self, train: Train, speed: i8){
        self.tx.send(Message::SetTrainSpeed(train, speed)).unwrap();
    }
    pub fn set_train_status(&mut self, train: Train, status: TrainStatus){
        self.tx.send(Message::SetTrainStatus(train, status)).unwrap();
    }
}

impl Comunicator for MapComunicationMaster{}

#[derive(Debug)]
pub struct MapComunicationSlave{
    rx: Receiver<Message>,
    speeds: HashMap<Train, i8>,
    positions: HashMap<Train, Position>,
    status: HashMap<Train, TrainStatus>,
    switches: HashMap<Switch, SwitchPosition>,
}

impl MapComunicationSlave {

    fn new(rx: Receiver<Message>) -> Self{
        MapComunicationSlave{
            rx,
            speeds: HashMap::new(),
            positions: HashMap::new(),
            status: HashMap::new(),
            switches: HashMap::new(),
        }
    }

    fn process_messages(&mut self){
        let _ = self.rx.try_recv().map(|message|{
            match message{
                Message::SetSwitch(switch, position) => {
                    self.switches.insert(switch, position);
                },
                Message::SetTrainPosition(train, position) => {
                    self.positions.insert(train, position);
                },
                Message::SetTrainSpeed(train, speed) => {
                    self.speeds.insert(train, speed);
                },
                Message::SetTrainStatus(train, status) => {
                    self.status.insert(train, status);
                },
            }
        });
    }
    pub fn get_switch_position(&mut self, switch: Switch) -> Option<SwitchPosition>{
        self.process_messages();
        self.switches.get(&switch).cloned()
    }
    pub fn get_train_position(&mut self, train: Train) -> Option<Position>{
        self.process_messages();
        self.positions.get(&train).cloned()
    }
    pub fn get_train_speed(&mut self, train: Train) -> Option<i8>{
        self.process_messages();
        self.speeds.get(&train).cloned()
    }
    pub fn get_train_status(&mut self, train: Train) -> Option<TrainStatus>{
        self.process_messages();
        self.status.get(&train).cloned()
    }
}
impl Comunicator for MapComunicationSlave{}