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
    use crate::map::views::map_creation_view::*;

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
    use crate::map::views::map_creation_view::*;

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
