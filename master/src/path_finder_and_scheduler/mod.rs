use common_infrastructure::hals::MasterHal;
use crate::low_level_controller::LowLevelController;
use crate::map::views::map_controller_view::MapControllerView;
use crate::map::views::map_factory::MapFactory;
use crate::map::views::map_navigation_view::MapNavigationView;

pub struct PathFinderAndScheduler<'a, T: MasterHal> {
    hal: &'a T,
    factory: &'a MapFactory,
    map_controller: MapControllerView<'a, T>,
    low_level_controller: LowLevelController<'a, T>,
}

impl<'a, T: MasterHal> PathFinderAndScheduler<'a, T> {
    pub fn new(hal: &'a T, factory: &'a MapFactory) -> Self{
        Self{
            hal,
            factory,
            map_controller: factory.build_controller_view(hal),
            low_level_controller: LowLevelController::new(hal, factory.build_controller_view(hal)),
        }
    }
}