use common_infrastructure::{Position, devices::Train};
#[allow(unused_imports)]
use railway_sim_map::SimulatedMap;
#[allow(unused_imports)]
use common_infrastructure::hals::{GenericHal, MasterHal};
use map::views::map_factory::MapFactory;
#[macro_use]
extern crate rouille;
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

    let com = factory.add_comunicator();
    let hlc = high_level_controller::HighLevelController::new(com);

    let mut pfas = path_finder_and_scheduler::PathFinderAndScheduler::new(&hal, &factory);

    loop{
        let request = hlc.get_request()?;
        pfas.add_request(request)?;
        hal.sleep_for_ms(1000);
    }
}

fn get_map()-> anyhow::Result<MapFactory>{

    const DEFAULT_STRAIGHT_SPEED: i8 = 35;
    const DEFAULT_UPHILL_SPEED: i8 = 50;
    const DEFAULT_DOWNHILL_SPEED: i8 = 5;

    let mut map = map::views::MapCreationView::new();

    map.add_nodes(&[Position::P11,Position::P12,Position::P13,Position::P14,Position::P15,Position::P16])?;
    //map.add_switch(Switch::S1)?;
    //map.add_switch_station(Switch::S1, Position::P1, Position::P3, Position::P2)?;
    map.add_link(Position::P11, Position::P12, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_link(Position::P12, Position::P13, DEFAULT_UPHILL_SPEED,0, 50)?;
    map.add_link(Position::P13, Position::P14, 25,25, 50)?;
    map.add_link(Position::P14, Position::P15, 15,DEFAULT_UPHILL_SPEED, 50)?;
    map.add_link(Position::P15, Position::P16, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_train(Train::T2, Position::P16, Some(Position::P15))?;
    let factory: MapFactory = map.into();
    Ok(factory)
}


#[test]fn receiver_test(){
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
        hal.send_message_to_train(common_infrastructure::devices::Train::T2, common_infrastructure::messages::TrainMessage::SetSpeed(0)).unwrap();
        hal.sleep_for_ms(2000);
        return;
        hal.send_message_to_train(common_infrastructure::devices::Train::T1, common_infrastructure::messages::TrainMessage::SetSpeed(80)).unwrap();
        hal.sleep_for_ms(2000);
        hal.send_message_to_train(common_infrastructure::devices::Train::T1, common_infrastructure::messages::TrainMessage::SetSpeed(100)).unwrap();
        hal.sleep_for_ms(2000);
    }
}