use common_infrastructure::messages::MasterMessage;
use common_infrastructure::devices::Train as TrainEnum;
use common_infrastructure::Position;

use crate::map::nodes::{NodeTrait,SwitchNodeTrait,NodeConnectionsTrait, NodeType, RoadNode, SwitchNode};


use super::Map;
pub struct Train{
    train: TrainEnum,
    pub position: Position,
    speed: i8,
    // say if the direction is straight compared to the node is on.
    is_straight: bool,
}

impl Train{
    pub fn new(train: TrainEnum, position: Position)->Self{
        Self{
            train,
            position,
            speed: 0,
            is_straight: true,
        }
    }
    
    pub fn do_move(&mut self, map: &Map) -> Option<MasterMessage>{
        if self.speed == 0{
            return None;
        }
        let current_node = map.get_node_at(self.position);

        let next = match (self.is_straight,self.speed) {
            (true, 0..) | (false, ..=-1) => current_node.next(map),
            (true, ..=-1) | (false, 0..) => current_node.prev(map),
        };

        let next = match next{
            Some(n) => n,
            None => panic!("The train crashed because it went out of the map!")
        };

        self.position = next.get_position();
        
        return Some(MasterMessage::TrainHasReachedPosition(self.train,self.position));
    }

    pub fn set_speed(&mut self, speed: i8){
        self.speed = speed;
    }
}