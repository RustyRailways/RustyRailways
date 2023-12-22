use std::collections::HashSet;
use super::*;
use std::ops::Deref;
use crate::map::initialization::{Initialize};
#[allow(unused_imports)]
use super::map_creation_object::*;
use serde_json;
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
    use crate::map::views::map_creation_view::*;

    let mut mcw = MapCreationView::new();

    mcw.add_node(Position::P1).unwrap();
    mcw.add_nodes(&[Position::P2,Position::P3]).unwrap();

    let s = mcw.add_node(Position::P2).unwrap_err().to_string();

    assert_eq!(s,"Node already exists");

    let s = mcw.add_nodes(&[Position::P4,Position::P1]).unwrap_err().to_string();

    assert_eq!(s,"Node already exists");

    let map = mcw.to_map().initialize();

    let keys: HashSet<_> = map.nodes.keys().collect();

    assert_eq!(keys.len(),4);

    assert_eq!(keys,[Position::P1,Position::P2,Position::P3,Position::P4].iter().collect());
}

#[test]
fn test_map_creation_view_nodes_switches(){
    use crate::map::views::map_creation_view::*;

    let mut mcw = MapCreationView::new();

    mcw.add_nodes(&[Position::P1,Position::P2,Position::P3,Position::P4]).unwrap();

    mcw.add_switch(Switch::S1).unwrap();

    let s = mcw.add_switch(Switch::S1).unwrap_err().to_string();
    assert_eq!(s,"Switch already exists");

    mcw.add_switches(&[Switch::S2,Switch::S3]).unwrap();

    let s = mcw.add_switches(&[Switch::S2,Switch::S4]).unwrap_err().to_string();
    assert_eq!(s,"Switch already exists");

    let map = mcw.to_map().initialize();

    let keys: HashSet<_> = map.switches.keys().collect();

    assert_eq!(keys.len(),3);

    assert_eq!(keys,[Switch::S1,Switch::S2,Switch::S3].iter().collect());
}