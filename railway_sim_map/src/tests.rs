use crate::*;
use common_infrastructure::hals::{GenericHal, MasterHal};

#[test]
fn test_creation(){
    let _ = SimulatedMap::new().unwrap();
}

#[test]
fn test_move_t1(){
    let map = SimulatedMap::new().unwrap();

    map.sleep_for_ms(0);
    assert_eq!(map.get_message().unwrap(), None);
    map.sleep_for_ms(0);
    assert_eq!(map.get_message().unwrap(), None);
    map.sleep_for_ms(0);
    assert_eq!(map.get_message().unwrap(), None);
    map.sleep_for_ms(0);

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(10)).unwrap();
    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P17).into()
    );

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P3).into()
    );
}

#[test]
fn test_switch(){
    let map = SimulatedMap::new().unwrap();

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(10)).unwrap();
    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P17).into()
    );

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P3).into()
    );

    map.send_message_to_switch(Switch::S1, SwitchMessage::SetPositionDiverging).unwrap();

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P1).into()
    );

    map.send_message_to_switch(Switch::S1, SwitchMessage::SetPositionStraight).unwrap();

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(-10)).unwrap();

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P2).into()
    );

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P14).into()
    );

    map.send_message_to_switch(Switch::S2, SwitchMessage::SetPositionDiverging).unwrap();

    map.sleep_for_ms(0);
    assert_eq!(
        map.get_message().unwrap(),
        MasterMessage::TrainHasReachedPosition(TrainEnum::T1, Position::P15).into()
    );
}

#[test]
#[should_panic(expected = "The train T1 has crashed with the train T2 while in move")]
fn test_crash_1(){
    let map = SimulatedMap::new().unwrap();

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(-10)).unwrap();

    map.sleep_for_ms(0);
}

#[test]
#[should_panic(expected = "The train T2 has crashed with the train T1 while in move")]
fn test_crash_2(){
    let map = SimulatedMap::new().unwrap();

    map.send_message_to_train(TrainEnum::T2, TrainMessage::SetSpeed(10)).unwrap();

    map.sleep_for_ms(0);
}

#[test]
#[should_panic(expected = "The train T2 has crashed with the train T1 while in move")]
fn test_crash_3(){
    let map = SimulatedMap::new().unwrap();

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(10)).unwrap();
    map.send_message_to_train(TrainEnum::T2, TrainMessage::SetSpeed(10)).unwrap();

    map.sleep_for_ms(0);
}


#[test]
#[should_panic(expected = "The train T1 has crashed with the train T2 while in move")]
fn test_crash_4(){
    let map = SimulatedMap::new().unwrap();

    map.send_message_to_switch(Switch::S4,SwitchMessage::SetPositionDiverging).unwrap();

    map.send_message_to_train(TrainEnum::T1, TrainMessage::SetSpeed(-10)).unwrap();
    map.send_message_to_train(TrainEnum::T2, TrainMessage::SetSpeed(-10)).unwrap();

    map.sleep_for_ms(0);
}

