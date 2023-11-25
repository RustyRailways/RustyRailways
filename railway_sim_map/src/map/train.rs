use crate::map::nodes::{Node, NodeType, RoadNode, SwitchNode};
use super::Map;
struct Train{
    position: NodeType,
    speed: i8,
    // say if the direction is straight compared to the node is on.
    is_straight: bool,
}

impl Train{
    fn new(position: NodeType)->Self{
        Self{
            position,
            speed: 0,
            is_straight: true,
        }
    }
    
    fn do_move(&mut self, map: &Map){
        if self.speed == 0{
            return;
        }
        let next = match (self.is_straight,self.speed) {
            (true, 0..) | (false, ..=-1) => self.position.next(map),
            (true, ..=-1) | (false, 0..) => self.position.prev(map),
        };

        let next = match next{
            Some(n) => n,
            None => panic!("The train crashed because it went out of the map!")
        };

        self.position = next;
    }

    fn set_speed(&mut self, speed: i8){
        self.speed = speed;
    }
}