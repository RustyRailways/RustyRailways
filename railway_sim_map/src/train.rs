use common_infrastructure::messages::MasterMessage;
use common_infrastructure::devices::Train as TrainEnum;
use common_infrastructure::Position;

use crate::map::nodes::{NodeTrait,NodeConnectionsTrait};


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

        let position_prev = self.position;

        self.position = self.get_next_position(map);

        // if the track has changed the primary orientation,
        // i need to change the train's direction
        // to keep the actual direction of the train coherent
        if self.get_option_next_position(map) == Some(position_prev){
            self.is_straight = !self.is_straight;
        }

        return Some(MasterMessage::TrainHasReachedPosition(self.train,self.position));
    }

    pub fn set_speed(&mut self, speed: i8){
        self.speed = speed;
    }

    fn get_option_next_position(&self, map: &Map) -> Option<Position>{
        if self.speed == 0{
            return Some(self.position);
        }

        let current_node = map.get_node_at(self.position);

        match (self.is_straight,self.speed) {
            (true, 0..) | (false, ..=-1) => current_node.next(map),
            (true, ..=-1) | (false, 0..) => current_node.prev(map),
        }.map(|x| x.get_position())
    }

    pub fn get_next_position(&self, map: &Map) -> Position{
        match self.get_option_next_position(map){
            Some(n) => n,
            None => panic!("The train crashed because it went out of the map!")
        }
    }
}