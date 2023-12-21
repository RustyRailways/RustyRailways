use super::*;
use crate::map::initialization::{Initialize};
use super::map_creation_object::*;
use serde_json;
#[test]
fn test_creation(){
    let mut map = Map::new();

    map.add_node(Position::P1).unwrap();
    map.add_node(Position::P2).unwrap();

    map.add_link(Position::P1, Position::P2, Direction::Backward,Direction::Backward,0,0,None).unwrap();

    map.add_train(Train::T1, Direction::Forward,Position::P1).unwrap();

    println!("{}",serde_json::to_string_pretty(&map).unwrap());

    let map = map.initialize();
}