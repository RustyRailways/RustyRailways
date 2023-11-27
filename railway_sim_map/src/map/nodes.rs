use std::cell::RefCell;
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;
use super::Map;

pub trait SwitchNodeTrait{
    fn set_straight(&self)->Result<(),()>;
    fn set_diverted(&self)->Result<(),()>;
}

pub trait NodeConnectionsTrait{
    fn next<'a>(&self, map: &'a Map)->Option<&'a GenericNode>;
    fn prev<'a>(&self, map: &'a Map)->Option<&'a GenericNode>;
}

pub trait NodeTrait: NodeConnectionsTrait + SwitchNodeTrait{
    fn get_position(&self)->Position;
}

// Node that can be both a simple road node, or a switch.
#[derive(PartialEq, Eq, Debug)]
pub struct GenericNode{
    position: Position,
    node: NodeType,
}

#[derive(PartialEq, Eq, Debug)]
pub enum NodeType{
    Switch(SwitchNode),
    Road(RoadNode)
}


impl NodeConnectionsTrait for GenericNode {
    fn next<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        match &self.node {
            NodeType::Road(r) => r.next(map),
            NodeType::Switch(s) => s.next(map)
        }
    }
    fn prev<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        match &self.node {
            NodeType::Road(r) => r.prev(map),
            NodeType::Switch(s) => s.prev(map)
        }
    }
}

impl SwitchNodeTrait for GenericNode {
    fn set_straight(&self) -> Result<(), ()> {
        match &self.node {
            NodeType::Road(r) => Err(()),
            NodeType::Switch(s) => s.set_straight()
        }
    }
    fn set_diverted(&self) -> Result<(), ()> {
        match &self.node {
            NodeType::Road(r) => Err(()),
            NodeType::Switch(s) => s.set_diverted()
        }
    }
}

impl NodeTrait for GenericNode {
    fn get_position(&self) -> Position {
        return self.position;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RoadNode{
    next: Option<Position>,
    prev: Option<Position>
}

impl NodeConnectionsTrait for RoadNode{
    fn next<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        return Some(map.get_node_at(self.next?));
    }
    fn prev<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        return Some(map.get_node_at(self.prev?));
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwitchNode{
    next_straight: Position,
    next_diverted: Position,
    prev: Option<Position>,
    is_straight: RefCell<bool>,
}

impl SwitchNodeTrait for SwitchNode {
    fn set_straight(&self) -> Result<(), ()> {
        *self.is_straight.borrow_mut() = true;
        Ok(())
    }
    fn set_diverted(&self) -> Result<(), ()> {
        *self.is_straight.borrow_mut() = false;
        Ok(())
    }
}

impl NodeConnectionsTrait for SwitchNode {
    fn next<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        if *self.is_straight.borrow(){
            return Some(map.get_node_at(self.next_straight))
        }else{
            return Some(map.get_node_at(self.next_diverted))
        }
    }
    fn prev<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        return Some(map.get_node_at(self.prev?));
    }
}