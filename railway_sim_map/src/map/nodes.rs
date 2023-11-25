use std::cell::RefCell;
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;
use super::Map;
pub trait Node{
    fn next(&self, map: &Map)->Option<NodeType>;
    fn prev(&self, map: &Map)->Option<NodeType>;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType{
    Switch(Switch),
    Road(RoadNode)
}

impl Node for NodeType{
    fn next(&self, map: &Map)->Option<NodeType>{
        match self{
            NodeType::Switch(s) => map.switches.get(s).unwrap().next(map),
            NodeType::Road(r) => r.next(map)
        }
    }
    fn prev(&self, map: &Map)->Option<NodeType>{
        match self{
            NodeType::Switch(s) => map.switches.get(s).unwrap().prev(map),
            NodeType::Road(r) => r.prev(map)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RoadNode{
    next: Option<Position>,
    prev: Option<Position>
}

impl Node for RoadNode{
    fn next(&self, map: &Map)->Option<NodeType>{
        // node, node are always present, if not, the map is broken.
        // and i rather unwrap, so i know the problem is there.
        Some(
            *map.node.get(&self.next?).unwrap()
        )
    }
    fn prev(&self, map: &Map)->Option<NodeType>{
        // node, node are always present, if not, the map is broken.
        // and i rather unwrap, so i know the problem is there.
        Some(
            *map.node.get(&self.prev?).unwrap()
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchNode{
    next_straight: Position,
    next_diverted: Position,
    prev: Option<Position>,
    is_straight: RefCell<bool>,
}

impl Node for SwitchNode{
    fn next(&self, map: &Map)->Option<NodeType>{

        let p = match *self.is_straight.borrow(){
            true => self.next_straight,
            false => self.next_diverted
        };

        // node, node are always present, if not, the map is broken.
        // and i rather unwrap, so i know the problem is there.
        Some(
            *map.node.get(&p).unwrap()
        )
    }
    fn prev(&self, map: &Map)->Option<NodeType>{
        // node, node are always present, if not, the map is broken.
        // and i rather unwrap, so i know the problem is there.
        Some(
            *map.node.get(&self.prev?).unwrap()
        )
    }
}