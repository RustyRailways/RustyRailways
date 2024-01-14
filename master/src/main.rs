use railway_sim_map::SimulatedMap;
use common_infrastructure::hals::GenericHal;
use crate::map::views::map_factory::MapFactory;

mod map;
mod constants;
mod low_level_controller;
mod high_level_controller;
mod path_finder_and_scheduler;

mod hal;

type Hal = SimulatedMap;
fn main(){
    let r = run();
    if let Err(e) = r{
        println!("Error: {:?}", e);
    }
}


fn run()-> anyhow::Result<()>{
    let hal = Hal::new()?;
    let factory = get_map()?;

    let mut pfas = path_finder_and_scheduler::PathFinderAndScheduler::new(&hal, &factory);
    let hlc = high_level_controller::HighLevelController::new();

    loop{
        let request = hlc.get_request();
        pfas.add_request(request);
        hal.sleep_for_ms(1000);
    }
}

fn get_map()-> anyhow::Result<MapFactory>{
    let map = map::views::MapCreationView::new();
    // todo: create the acutal map
    let factory: MapFactory = map.into();
    Ok(factory)
}