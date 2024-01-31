use common_infrastructure::{devices::Switch, devices::Train, messages::SwitchMessage, Position};
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

    let mut map = map::views::MapCreationView::new();

    //// nodes ////
    map.add_nodes(&[
        Position::P1,
        Position::P2,
        Position::P3,
        Position::P4,
        Position::P5,
        Position::P6,
        Position::P7,
        Position::P8,
        Position::P9,
        Position::P10,
        Position::P11,
        Position::P12,
        Position::P13,
        Position::P14,
        Position::P15,
        Position::P16,
        Position::P17,
        Position::P18,
        Position::P19,
        Position::P20,
        Position::P21,
        Position::P22,
        Position::P23,
        Position::P24,
        Position::P25,
        Position::P26
    ])?;
    
    
    map.add_switches(&vec![Switch::S1,Switch::S2,Switch::S3,Switch::S4,Switch::S5,Switch::S6])?;

    //// switch stations ////
    map.add_switch_station(Switch::S1, Position::P21, Position::P2, Position::P5)?;
    map.add_switch_station(Switch::S2, Position::P24, Position::P1, Position::P12)?;
    map.add_switch_station(Switch::S3, Position::P19, Position::P20, Position::P11)?;
    map.add_switch_station(Switch::S4, Position::P23, Position::P22, Position::P24)?;
    map.add_switch_station(Switch::S5, Position::P6, Position::P16, Position::P7)?;
    map.add_switch_station(Switch::S6, Position::P4, Position::P3, Position::P21)?;


    //// S1 to S4 ////
    map.add_link(Position::P23, Position::P2, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;

    //// S2 to S3 ////
    map.add_link(Position::P1, Position::P19, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;

    //// S3 to S5 ////
    map.add_link(Position::P11, Position::P7, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;

    //// S5 to S6 ////
    map.add_link(Position::P6, Position::P4, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;

    //// S3 to S6 ////
    map.add_link(Position::P3, Position::P17, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_link(Position::P17, Position::P8, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_link(Position::P8, Position::P20, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;

    //// S2 to S5 | bridge ////
    map.add_link(Position::P12, Position::P13, DEFAULT_UPHILL_SPEED,0, 50)?;
    map.add_link(Position::P13, Position::P9, 25,25, 50)?;
    map.add_link(Position::P9, Position::P14, 25,25, 50)?;
    map.add_link(Position::P14, Position::P15, 15,DEFAULT_UPHILL_SPEED, 50)?;
    map.add_link(Position::P15, Position::P16, 15,DEFAULT_UPHILL_SPEED, 50)?;

    //// dead track S1 ////
    map.add_link(Position::P5, Position::P10, DEFAULT_UPHILL_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_link(Position::P10, Position::P25, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    map.add_link(Position::P25, Position::P18, DEFAULT_STRAIGHT_SPEED,DEFAULT_STRAIGHT_SPEED, 50)?;
    

    //// trains ////
    map.add_train(Train::T2, Position::P18, None)?;
    map.add_train(Train::T1, Position::P22, None)?;
    
    
    //// creation ////
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
    hal.send_message_to_switch(Switch::S1, SwitchMessage::SetPositionStraight).unwrap();
    return;
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