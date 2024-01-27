use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use anyhow::Result;
use map::views::{MapNavigationView, MapVisualizationView};
use super::path_finder::find_best_path;

/// return the shortest path from the current position to the closest intersection
pub fn find_path_to_intersection(
    train: Train,
    vv: &MapVisualizationView,
    nv: &MapNavigationView
) -> Result<Option<Vec<Position>>>{
    return find_best_path(
        train,
        &|pos| {
            nv.get_node(pos).unwrap().neighbours_iter().count() == 3
        },
        vv,
        nv,
        false
    );
}