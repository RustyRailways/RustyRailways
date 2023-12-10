use std::cell::RefCell;
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
    pub(crate) position: Position,
    node: NodeType,
}

#[derive(PartialEq, Eq, Debug)]
pub enum NodeType{
    Switch(SwitchNode),
    Road(RoadNode)
}


impl GenericNode{
    pub fn new_road(position: Position, prev: Option<Position>, next: Option<Position>)->Self{
        Self{
            position,
            node: NodeType::new_road(prev,next)
        }
    }
    pub fn new_switch(position: Position, prev: Option<Position>, next_straight: Position, next_diverted: Position)->Self{
        Self{
            position,
            node: NodeType::new_switch(prev,next_straight,next_diverted)
        }
    }
}

impl NodeType {
    pub fn new_road(prev: Option<Position>, next: Option<Position>) ->Self{
        Self::Road(RoadNode::new(prev,next))
    }
    pub fn new_switch(prev: Option<Position>, next_straight: Position, next_diverted: Position)->Self{
        Self::Switch(SwitchNode::new(prev,next_straight,next_diverted))
    }
}

impl SwitchNode {
    pub fn new(prev: Option<Position>, next_straight: Position, next_diverted: Position)->Self{
        Self{
            next_diverted,
            next_straight,
            prev,
            is_straight: RefCell::new(true),
        }
    }
}

impl RoadNode {
    pub fn new(prev: Option<Position>, next: Option<Position>)->Self{
        Self{
            prev,
            next,
        }
    }
}

impl NodeConnectionsTrait for GenericNode {
    fn next<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {
        match &self.node {
            NodeType::Road(r) => r.next(map),
            NodeType::Switch(s) => s.next(map)
        }
    }
    fn prev<'a>(&self, map: &'a Map) -> Option<&'a GenericNode> {

        // check that if i am going to a switch,
        // the switch is set in the correct position, and not in the opposite one.
        // to avoid the train derailing.
        if let NodeType::Switch(s) = &self.node{
            let prev = s.prev(map);
            if let Some(prev) = prev{

                let option_1 = prev.next(map).map(|x|x.position);
                let option_2 = prev.prev(map).map(|x|x.position);

                if option_1 != Some(self.position) && option_2 != Some(self.position){
                    panic!(
                        "The switch at position {:?} was not set in the correct position,\
                        wen the train was going to it from {:?}"
                        ,prev.position,
                        self.position
                    );
                }
            }
        }


        match &self.node {
            NodeType::Road(r) => r.prev(map),
            NodeType::Switch(s) => s.prev(map)
        }
    }
}

impl SwitchNodeTrait for GenericNode {
    fn set_straight(&self) -> Result<(), ()> {
        match &self.node {
            NodeType::Road(_) => panic!("call straight on a road node!"),
            NodeType::Switch(s) => s.set_straight()
        }
    }
    fn set_diverted(&self) -> Result<(), ()> {
        match &self.node {
            NodeType::Road(_) => panic!("call diverted on a road node!"),
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