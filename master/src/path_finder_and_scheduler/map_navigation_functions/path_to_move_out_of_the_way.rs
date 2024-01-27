use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use anyhow::Result;
use map::views::{MapNavigationView, MapVisualizationView};
use super::path_finder::find_best_path;

/// This function moves a train away from a vector of positions
/// if the function finds a way, it returns a list of positions that the train will move to
/// if the function does not find a way, it returns None
pub fn find_path_to_move_out_of_the_way(
        train: Train,
        locked_positions: &Vec<Position>,
        vv: &MapVisualizationView,
        nv: &MapNavigationView
    ) -> Result<Option<Vec<Position>>>{
    return find_best_path(
        train,
        &|pos| !locked_positions.contains(&pos),
        vv,
        nv,
        false
    );
}