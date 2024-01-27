use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use anyhow::{anyhow, Result};
use map::views::{MapNavigationView, MapVisualizationView};
use crate::path_finder_and_scheduler::map_navigation_functions::path_finder::find_best_path;
use super::{find_path_to_intersection,find_path_to_move_out_of_the_way, find_path_to_position};

/// if the train can't reach is destination directly, and
/// the other train can't move out of the way, is necessary to find a switch point
/// this function returns the shortest path to the closest switch point
/// this function can be called only if the two conditions above have already been checked
pub fn find_path_to_switch_point(
    train: Train,
    destination: Position,
    vv: &MapVisualizationView,
    nv: &MapNavigationView,
) -> Result<Option<Vec<Position>>>{
    let path_to_destination = find_path_to_position(train, destination, vv, nv, true)?;
    let path_to_destination = match path_to_destination {
        None => return Err(anyhow!("Scheduler error inside find_path_to_switch_point")),
        Some(s) => s
    };

    let path_to_intersection = find_path_to_intersection(train, vv, nv)?;
    let path_to_intersection = match path_to_intersection {
        // if there is no intersection, is impossible to have a switch point
        None => return Ok(None),
        Some(s) => s
    };

    let intersection = *path_to_intersection.last().unwrap();

    let position_to_avoid = vec![
        path_to_destination.get(path_to_destination.len().wrapping_sub(2)),
        path_to_destination.get(0),
        path_to_destination.get(1)
    ];

    let position_to_avoid: Vec<Position> = position_to_avoid.iter().filter_map(|x| *x).cloned().collect();

    let mut switch_point = None;

    for n in nv.get_node(intersection).unwrap().neighbours_iter(){
        if position_to_avoid.contains(&n.get_position()){
            continue
        }
        switch_point = Some(n.get_position());
    }

    let switch_point = match switch_point {
        None => return Err(anyhow!("Scheduler error 2 inside find_path_to_switch_point")),
        Some(s) => s
    };

    let path = find_path_to_position(train, switch_point, vv, nv, false)?;
    if path.is_none(){
        return Err(anyhow!("Scheduler error 3 inside find_path_to_switch_point"))
    }
    return Ok(path);
}