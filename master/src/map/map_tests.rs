use std::collections::HashSet;
use super::*;
use std::ops::Deref;
use crate::map::initialization::{Initialize};
use serde_json;
use crate::map::nodes::Link;
#[allow(unused_imports)]
use crate::map::views::map_creation_view::*;

#[derive(Debug,Clone,Eq, PartialEq,Hash)]
struct LinkLite{
    length: u32,
    max_speed: i8,
    node: Position,
}
impl LinkLite{
    fn new(length: u32, max_speed: i8, node: Position) -> Self{
        LinkLite{
            length,
            max_speed,
            node
        }
    }
}

impl From<&Link<MapStateUninitialized>> for LinkLite{
    fn from(value: &Link<MapStateUninitialized>) -> Self {
        Self::new(value.length,value.max_speed,value.node.position)
    }
}
impl From<&Link<MapStateInitialized>> for LinkLite{
    fn from(value: &Link<MapStateInitialized>) -> Self {
        Self::new(value.length,value.max_speed,value.node.position)
    }
}


#[derive(Debug,Clone,Eq, PartialEq,Hash)]
struct LinkLiteWithDirection{
    length: u32,
    max_speed: i8,
    node: Position,
    direction: Direction,
    switch: Option<(Switch,SwitchPosition)>
}
impl LinkLiteWithDirection{
    fn new(length: u32, max_speed: i8, node: Position, direction: Direction,
           switch: Option<(Switch,SwitchPosition)>) -> Self{
        LinkLiteWithDirection{
            length,
            max_speed,
            node,
            direction,
            switch
        }
    }
}

impl From<&Link<MapStateUninitialized>> for LinkLiteWithDirection{
    fn from(value: &Link<MapStateUninitialized>) -> Self {
        let switch = match value.controller{
            SwitchControllerOption::NoSwitch => None,
            SwitchControllerOption::SwitchToSetDiverted(x) => Some((x.switch,SwitchPosition::Diverted)),
            SwitchControllerOption::SwitchToSetStraight(x) => Some((x.switch,SwitchPosition::Straight)),
        };
        Self::new(value.length,value.max_speed,value.node.position,value.direction,switch)
    }
}
impl From<&Link<MapStateInitialized>> for LinkLiteWithDirection{
    fn from(value: &Link<MapStateInitialized>) -> Self {
        let switch = match &value.controller{
            SwitchControllerOption::NoSwitch => None,
            SwitchControllerOption::SwitchToSetDiverted(x) => Some((x.deref().switch,SwitchPosition::Diverted)),
            SwitchControllerOption::SwitchToSetStraight(x) => Some((x.deref().switch,SwitchPosition::Straight)),
        };
        Self::new(value.length,value.max_speed,value.node.position,value.direction,switch)
    }
}

#[test]
fn test_creation(){
    let mut map = Map::new();

    map.add_node(Position::P1).unwrap();
    map.add_node(Position::P2).unwrap();
    map.add_node(Position::P3).unwrap();

    map.add_switch(Switch::S1).unwrap();

    map.add_link(Position::P1, Position::P2, Direction::Backward,
                 Direction::Backward,0,0,Some((Switch::S1,SwitchPosition::Diverted))).unwrap();
    map.add_link(Position::P2, Position::P3, Direction::Forward,
                 Direction::Backward,0,0,Some((Switch::S1,SwitchPosition::Straight))).unwrap();

    map.add_train(Train::T1, Direction::Forward,Position::P1).unwrap();


    println!("{}",serde_json::to_string_pretty(&map).unwrap());

    let map = map.initialize();

    assert_eq!(
        map.get_train(Train::T1).unwrap().get_current_node().deref() as *const Node<MapStateInitialized>,
        map.get_node(Position::P1).unwrap() as *const Node<MapStateInitialized>
    );

    assert_eq!(
        map.get_node(Position::P2).unwrap().adjacent_nodes.borrow().get_adjacent_nodes()[0].node.deref() as *const Node<MapStateInitialized>,
        map.get_node(Position::P1).unwrap() as *const Node<MapStateInitialized>
    );
}

#[test]
fn test_map_creation_view_nodes(){

    let mut mcv = MapCreationView::new();

    mcv.add_node(Position::P1).unwrap();
    mcv.add_nodes(&[Position::P2,Position::P3]).unwrap();

    let s = mcv.add_node(Position::P2).unwrap_err().to_string();

    assert_eq!(s,"Node already exists");

    let s = mcv.add_nodes(&[Position::P4,Position::P1]).unwrap_err().to_string();

    assert_eq!(s,"Node already exists");

    let map = mcv.to_map().initialize();

    let keys: HashSet<_> = map.nodes.keys().collect();

    assert_eq!(keys.len(),4);

    assert_eq!(keys,[Position::P1,Position::P2,Position::P3,Position::P4].iter().collect());
}

#[test]
fn test_map_creation_view_nodes_switches(){

    let mut mcv = MapCreationView::new();

    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3,Position::P4]).unwrap();

    mcv.add_switch(Switch::S1).unwrap();

    let s = mcv.add_switch(Switch::S1).unwrap_err().to_string();
    assert_eq!(s,"Switch already exists");

    mcv.add_switches(&[Switch::S2,Switch::S3]).unwrap();

    let s = mcv.add_switches(&[Switch::S2,Switch::S4]).unwrap_err().to_string();
    assert_eq!(s,"Switch already exists");

    let map = mcv.to_map().initialize();

    let keys: HashSet<_> = map.switches.keys().collect();

    assert_eq!(keys.len(),3);

    assert_eq!(keys,[Switch::S1,Switch::S2,Switch::S3].iter().collect());
}

#[test]
fn test_map_creation_view_links(){

    let mut mcv = MapCreationView::new();

    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3,Position::P4]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 1,10).unwrap();

    let s = mcv.add_link(Position::P1, Position::P2, 1,10).unwrap_err().to_string();
    assert_eq!(s,"Impossible to add a link to a node that is already linked");

    mcv.add_link(Position::P2, Position::P3, 2,20).unwrap();
    mcv.add_link(Position::P3, Position::P4, 3,30).unwrap();

    let s = mcv.add_link(Position::P1, Position::P3, 1,10).unwrap_err().to_string();
    assert_eq!(s,"The node already has two links");

    let s = mcv.add_link(Position::P3, Position::P1, 1,10).unwrap_err().to_string();
    assert_eq!(s,"The node already has two links");

    mcv.add_link(Position::P4, Position::P1, 4,40).unwrap();

    let s = mcv.add_link(Position::P1, Position::P3, 1,10).unwrap_err().to_string();
    assert_eq!(s,"The node already has two links");

    let map = mcv.to_map().initialize();

    for node in map.nodes.values(){
        let adjacent_nodes = node.adjacent_nodes.borrow();
        let adjacent_nodes_slice = adjacent_nodes.get_adjacent_nodes();
        assert_eq!(adjacent_nodes_slice.len(),2);

        assert_ne!(adjacent_nodes_slice[0].direction, adjacent_nodes_slice[1].direction);

        let adjacent_nodes_set: HashSet<_> = adjacent_nodes_slice.iter().map(|x| LinkLite{
            length: x.length,
            max_speed: x.max_speed,
            node: x.node.position
        }).collect();

        match node.position {
            Position::P1 => {
                assert_eq!(adjacent_nodes_set,
                           [
                               LinkLite::new(10,1,Position::P2),
                               LinkLite::new(40,4,Position::P4)
                           ].into_iter().collect()
                );
            },
            Position::P2 => {
                assert_eq!(adjacent_nodes_set,
                           [
                               LinkLite::new(10,1,Position::P1),
                               LinkLite::new(20,2,Position::P3)
                           ].into_iter().collect()
                );
            },
            Position::P3 => {
                assert_eq!(adjacent_nodes_set,
                           [
                               LinkLite::new(20,2,Position::P2),
                               LinkLite::new(30,3,Position::P4)
                           ].into_iter().collect()
                );
            },
            Position::P4 => {
                assert_eq!(adjacent_nodes_set,
                           [
                               LinkLite::new(30,3,Position::P3),
                               LinkLite::new(40,4,Position::P1)
                           ].into_iter().collect()
                );
            },
            _ => panic!("Impossible")
        }
    }
}


#[test]
fn test_map_creation_view_switch_station(){

    use crate::constants::{DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED};

    let mut mcv = MapCreationView::new();

    mcv.add_nodes(
        &[Position::P1,Position::P2,Position::P3,Position::P4,Position::P5,Position::P6]
    ).unwrap();

    mcv.add_switches(&[Switch::S1,Switch::S2]).unwrap();

    let s = mcv.add_switch_station(
        Switch::S3,Position::P1,Position::P2,Position::P3
    ).unwrap_err().to_string();
    assert_eq!(s,"Switch does not exist");

    mcv.add_switch_station(
        Switch::S1,Position::P1,Position::P2,Position::P3
    ).unwrap();

    let s = mcv.add_switch_station(
        Switch::S1,Position::P4,Position::P5,Position::P6
    ).unwrap_err().to_string();

    assert_eq!(s,"Switch already used");

    mcv.add_switch_station(
        Switch::S2,Position::P4,Position::P5,Position::P6
    ).unwrap();


    mcv.add_link(Position::P1, Position::P4, 1,10).unwrap();
    mcv.add_link(Position::P2, Position::P5, 2,20).unwrap();
    mcv.add_link(Position::P3, Position::P6, 3,30).unwrap();


    let map = mcv.to_map().initialize();

    let get_links = |position: Position| -> HashSet<LinkLiteWithDirection>{
        map.get_node(position).unwrap()
            .adjacent_nodes.borrow()
            .get_adjacent_nodes()
            .iter().map(
            |x| LinkLiteWithDirection::from(x)
        ).collect()
    };

    let links_p1 = get_links(Position::P1);

    assert_eq!(
        links_p1,
        [
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P2,Direction::Forward,Some((Switch::S1,SwitchPosition::Diverted))),
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P3,Direction::Forward,Some((Switch::S1,SwitchPosition::Straight))),
            LinkLiteWithDirection::new(10,1,Position::P4,Direction::Backward,None),
        ].iter().map(|x|x.clone()).collect()
    );

    let links_p2 = get_links(Position::P2);

    assert_eq!(
        links_p2,
        [
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P1,Direction::Backward,Some((Switch::S1,SwitchPosition::Diverted))),
            LinkLiteWithDirection::new(20,2,Position::P5,Direction::Forward,None),
        ].iter().map(|x|x.clone()).collect()
    );

    let links_p3 = get_links(Position::P3);

    assert_eq!(
        links_p3,
        [
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P1,Direction::Backward,Some((Switch::S1,SwitchPosition::Straight))),
            LinkLiteWithDirection::new(30,3,Position::P6,Direction::Forward,None),
        ].iter().map(|x|x.clone()).collect()
    );

    let links_p4 = get_links(Position::P4);

    assert_eq!(
        links_p4,
        [
            LinkLiteWithDirection::new(10,1,Position::P1,Direction::Backward,None),
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P5,Direction::Forward,Some((Switch::S2,SwitchPosition::Diverted))),
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P6,Direction::Forward,Some((Switch::S2,SwitchPosition::Straight))),
        ].iter().map(|x|x.clone()).collect()
    );

    let links_p5 = get_links(Position::P5);

    assert_eq!(
        links_p5,
        [
            LinkLiteWithDirection::new(20,2,Position::P2,Direction::Forward,None),
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P4,Direction::Backward,Some((Switch::S2,SwitchPosition::Diverted))),
        ].iter().map(|x|x.clone()).collect()
    );

    let links_p6 = get_links(Position::P6);

    assert_eq!(
        links_p6,
        [
            LinkLiteWithDirection::new(30,3,Position::P3,Direction::Forward,None),
            LinkLiteWithDirection::new(DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED,Position::P4,Direction::Backward,Some((Switch::S2,SwitchPosition::Straight))),
        ].iter().map(|x|x.clone()).collect()
    );
}

#[test]
#[should_panic(expected = "max_speed must be positive")]
fn test_negative_link_speed(){

    let mut mcv = MapCreationView::new();

    mcv.add_nodes(&[Position::P1,Position::P2]).unwrap();
    mcv.add_link(Position::P1, Position::P2, -1,1).unwrap_err().to_string();
}

#[test]
fn test_add_switch_after_link(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3,Position::P4,Position::P5]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 1,10).unwrap();

    mcv.add_switch(Switch::S1).unwrap();

    let s = mcv.add_switch_station(Switch::S1,Position::P2,Position::P3,Position::P4).unwrap_err().to_string();

    assert_eq!(s,"all switch_stations must be added before any link is added");
}

#[test]
fn test_add_link_after_train(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 1,10).unwrap();

    mcv.add_train(Train::T1,Position::P2,Some(Position::P1)).unwrap();

    let s = mcv.add_link(Position::P2, Position::P3, 1,10).unwrap_err().to_string();

    assert_eq!(s,"all links must be added before any train is added");
}

#[test]
fn test_add_train_1(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    let s = mcv.add_train(Train::T1,Position::P2,None).unwrap_err().to_string();
    assert_eq!(s,"The pointing_to has not be found");
    let s = mcv.add_train(Train::T1,Position::P2,Some(Position::P2)).unwrap_err().to_string();
    assert_eq!(s,"The pointing_to has not be found");

    mcv.add_train(Train::T1,Position::P2,Some(Position::P1)).unwrap();


    let s = mcv.add_train(Train::T2,Position::P2,Some(Position::P1)).unwrap_err().to_string();
    assert_eq!(s,"Impossible to set train on a node that is not unlocked");

    let s = mcv.add_train(Train::T1,Position::P1,None).unwrap_err().to_string();
    assert_eq!(s,"Train already exists");
}

#[test]
fn test_add_train_2(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P1,None).unwrap();

    let map = mcv.to_map().initialize();


    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P1);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P2).unwrap();

    assert_ne!(link.direction,train.direction);
}

#[test]
fn test_add_train_3(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P3,None).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P3);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P2).unwrap();

    assert_ne!(link.direction,train.direction);
}

#[test]
fn test_add_train_4(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P1,Some(Position::P2)).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P1);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P2).unwrap();

    assert_eq!(link.direction,train.direction);
}

#[test]
fn test_add_train_5(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P3,Some(Position::P2)).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P3);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P2).unwrap();

    assert_eq!(link.direction,train.direction);
}

#[test]
fn test_add_train_6(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P2,Some(Position::P1)).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P2);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P1).unwrap();

    assert_eq!(link.direction,train.direction);
}

#[test]
fn test_add_train_7(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_link(Position::P1, Position::P2, 0,0).unwrap();
    mcv.add_link(Position::P2, Position::P3, 0,0).unwrap();

    mcv.add_train(Train::T1,Position::P2,Some(Position::P3)).unwrap();

    let map = mcv.to_map().initialize();


    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P2);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P3).unwrap();

    assert_eq!(link.direction,train.direction);
}

#[test]
fn test_add_train_8(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_switch(Switch::S1).unwrap();

    mcv.add_switch_station(Switch::S1,Position::P1,Position::P2,Position::P3).unwrap();

    mcv.add_train(Train::T1,Position::P1,Some(Position::P3)).unwrap();
    mcv.add_train(Train::T2,Position::P2,Some(Position::P1)).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P1);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P3).unwrap();

    assert_eq!(link.direction,train.direction);

    let train = map.trains.get(&Train::T2).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P2);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P1).unwrap();

    assert_eq!(link.direction,train.direction);
}

#[test]
fn test_add_train_9(){
    let mut mcv = MapCreationView::new();
    mcv.add_nodes(&[Position::P1,Position::P2,Position::P3]).unwrap();

    mcv.add_switch(Switch::S1).unwrap();

    mcv.add_switch_station(Switch::S1,Position::P1,Position::P2,Position::P3).unwrap();

    mcv.add_train(Train::T1,Position::P1,None).unwrap();
    mcv.add_train(Train::T2,Position::P2,None).unwrap();

    let map = mcv.to_map().initialize();

    let train = map.trains.get(&Train::T1).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P1);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P3).unwrap();

    assert_ne!(link.direction,train.direction);

    let train = map.trains.get(&Train::T2).unwrap();

    let node_ref = train.current_position.borrow();
    let node = node_ref.deref().deref();

    let pos = node.position;
    assert_eq!(pos,Position::P2);

    let adjacent_nodes = node.adjacent_nodes.borrow();
    let link = adjacent_nodes.get_link_to(Position::P1).unwrap();

    assert_ne!(link.direction,train.direction);
}