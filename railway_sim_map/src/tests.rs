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