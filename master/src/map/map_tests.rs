use super::*;
use crate::map::initialization::{Initialize};
use super::map_creation_object::*;
use serde_json;
#[test]
fn test_creation(){
    let mut map = Map::new();

    map.add_node(Position::P1).unwrap();
    map.add_node(Position::P2).unwrap();
    map.add_node(Position::P3).unwrap();

    map.add_link(Position::P1, Position::P2, Direction::Backward,Direction::Backward,0,0,None).unwrap();
    map.add_link(Position::P2, Position::P3, Direction::Forward,Direction::Backward,0,0,None).unwrap();

    map.add_train(Train::T1, Direction::Forward,Position::P1).unwrap();


    println!("{}",serde_json::to_string_pretty(&map).unwrap());

    let map = map.initialize();

    assert_eq!(
        map.get_train(Train::T1).get_current_node().deref() as *const Node<MapStateInitialized>,
        map.get_node(Position::P1) as *const Node<MapStateInitialized>
    );

    assert_eq!(
        map.get_node(Position::P2).adjacent_nodes.borrow().get_adjacent_nodes()[0].node.deref() as *const Node<MapStateInitialized>,
        map.get_node(Position::P1) as *const Node<MapStateInitialized>
    );



}