use std::iter::Map;
use common_infrastructure::devices::Train;
use common_infrastructure::hals::MasterHal;
use common_infrastructure::Position;
use crate::map::views::map_controller_view::MapControllerView;

pub struct LowLevelController<'a, T: MasterHal> {
    hal: &'a T,
    map_controller: MapControllerView<'a, T>,
}

impl<'a,T:MasterHal> LowLevelController<T> {
    pub fn new(hal: &'a T, map_controller: MapControllerView<'a,T>) -> Self{
        Self{
            hal,
            map_controller
        }
    }

    pub fn move_train(&mut self, train: Train, stations: &[Position]){
        todo!()
    }
}