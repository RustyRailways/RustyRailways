use common_infrastructure::Position;
use anyhow::Result;
use common_infrastructure::devices::Train;
use crate::map::views::{MapNavigationView, MapVisualizationView};
use super::path_finder::find_best_path;

/// return the shortest path from the current position to the destination
/// you can decide to include the trains that are locked in the path finding
pub fn find_path_to_position(
    from: Train,
    to: Position,
    vv: &MapVisualizationView,
    nv: &MapNavigationView,
    include_unlocked_trains: bool,
) -> Result<Option<Vec<Position>>>{
    return find_best_path(
        from,
        &|pos| pos == to,
        vv,
        nv,
        include_unlocked_trains
    );
}