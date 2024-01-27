use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;
use map::views::{MapNavigationView, MapVisualizationView};
use std::collections::binary_heap::BinaryHeap;
use map::map_creation_object::TrainStatus;
use super::OrderedPosition;
/// find the best path (in therms of total distance) for a train to a position
/// that satisfies a condition
pub fn find_best_path(
        train: Train,
        condition: &dyn Fn(Position) -> bool,
        vv: &MapVisualizationView,
        nv: &MapNavigationView,
        include_unlocked_trains: bool
    ) -> Result<Option<Vec<Position>>>{


    let mut distances = HashMap::<Position,u32>::new();
    let mut queue = BinaryHeap::<OrderedPosition>::new();

    let start = vv.get_train_position(train)?;
    let _ = queue.push(OrderedPosition::new(0, start));

    let mut final_position = None;

    ///////////// dijkstra's algorithm /////////////


    while !queue.is_empty() {

        let current = queue.pop().unwrap();

        // i already found a shorter path to this node
        if distances.contains_key(&current.position) {
            continue;
        }

        distances.insert(current.position, current.priority);

        // i found the destination
        if condition(current.position){
            final_position = Some(current.position);
            break
        }

        let current_node = nv.get_node(current.position)?;

        for neighbor in current_node.neighbours_iter() {

            if distances.contains_key(&neighbor.get_position()){
                continue
            }

            let occupying_train = neighbor.get_node().get_occupying_train();

            if let Some(t) = occupying_train {
                let status = vv.get_train_status(t)?;
                // can't move a locked train under any circumstances
                if status == TrainStatus::Locked{
                    continue
                }
                // if this flag is enabled, i can't move a train regardless of its status
                if !include_unlocked_trains{
                    continue
                }
            }

            // if all controls are passed, i can add the new node to the priority queue
            let _ = queue.push(OrderedPosition::new(current.priority + neighbor.get_length(), neighbor.get_position()));
        }
    }

    let final_position = match final_position {
        None => return Ok(None),
        Some(p) => p
    };

    /////////////// reconstruct the path back ///////////////

    let mut path = VecDeque::<Position>::new();

    let mut current = final_position;
    let _ = path.push_front(current);

    'while_loop:
    while current!=start{
        let current_node = nv.get_node(current)?;

        for neighbor in current_node.neighbours_iter() {

            let neighbor_distance = distances
                .get(&neighbor.get_position())
                .map(|x| *x)
                .unwrap_or(u32::MAX);

            // this unwrap should never fail
            let current_distance = distances.get(&current).map(|x| *x).unwrap();

            if neighbor_distance == current_distance - neighbor.get_length(){
                current = neighbor.get_position();
                let _ = path.push_front(current);
                continue 'while_loop
            }
        }
        // should never happen
        panic!("Fatal error in path finding, couldn't reconstruct the path")
    }

    //return the path
    Ok(Some(path.into()))
}