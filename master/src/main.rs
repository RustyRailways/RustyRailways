#[allow(unused_imports)]
use railway_sim_map::SimulatedMap;
#[allow(unused_imports)]
use common_infrastructure::hals::{GenericHal, MasterHal};
use crate::map::views::map_factory::MapFactory;
#[macro_use]
extern crate rouille;

mod map;
mod constants;
mod low_level_controller;
mod high_level_controller;
mod path_finder_and_scheduler;

mod hal;
use hal::MasterHalRaspberryPi;

type Hal = MasterHalRaspberryPi;
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
        let request = hlc.get_request()?;
        pfas.add_request(request)?;
        hal.sleep_for_ms(1000);
    }
}

fn get_map()-> anyhow::Result<MapFactory>{
    let map = map::views::MapCreationView::new();
    // todo: create the acutal map
    let factory: MapFactory = map.into();
    Ok(factory)
}


#[test]
fn receiver_test(){
    let hal = Hal::new().unwrap();
    loop {
        while let Some(m) = hal.get_message().unwrap() {
            println!("Got message: {m:?}")
        }
    }
}

#[test]
fn test_send_message(){
    let hal = Hal::new().unwrap();
    loop {
        hal.send_message_to_train(common_infrastructure::devices::Train::T1, common_infrastructure::messages::TrainMessage::SetSpeed(30)).unwrap();
        hal.sleep_for_ms(2000);
        hal.send_message_to_train(common_infrastructure::devices::Train::T1, common_infrastructure::messages::TrainMessage::SetSpeed(80)).unwrap();
        hal.sleep_for_ms(2000);
        hal.send_message_to_train(common_infrastructure::devices::Train::T1, common_infrastructure::messages::TrainMessage::SetSpeed(100)).unwrap();
        hal.sleep_for_ms(2000);
    }
}