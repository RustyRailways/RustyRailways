use common_infrastructure::devices::Switch;
use common_infrastructure::devices::Train;
use common_infrastructure::hals::GenericHal;
use common_infrastructure::Position;
use crate::high_level_controller::Request;
use crate::low_level_controller::LowLevelController;
use crate::map::views::{MapCreationView, MapFactory};
use crate::map::map_tests::fake_hal::Hal;
use crate::path_finder_and_scheduler::PathFinderAndScheduler;

pub fn get_test_map() -> MapFactory{

    let mut mcv = MapCreationView::new();

    mcv.add_nodes(&vec![
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
    ]).unwrap();

    mcv.add_switches(
        &vec![
            Switch::S1,
            Switch::S2,
            Switch::S3,
            Switch::S4
        ]
    ).unwrap();

    mcv.add_switch_station(Switch::S1, Position::P1, Position::P3, Position::P2).unwrap();
    mcv.add_switch_station(Switch::S2, Position::P14, Position::P15, Position::P13).unwrap();
    mcv.add_switch_station(Switch::S3, Position::P10, Position::P9, Position::P11).unwrap();
    mcv.add_switch_station(Switch::S4, Position::P5, Position::P7, Position::P6).unwrap();

    mcv.add_link(Position::P3, Position::P17,1,1).unwrap();
    mcv.add_link(Position::P17, Position::P4,1,1).unwrap();
    mcv.add_link(Position::P7, Position::P8,1,1).unwrap();
    mcv.add_link(Position::P5, Position::P9,1,1).unwrap();
    mcv.add_link(Position::P10, Position::P12,1,1).unwrap();
    mcv.add_link(Position::P11, Position::P13,1,1).unwrap();
    mcv.add_link(Position::P2, Position::P14,1,1).unwrap();
    mcv.add_link(Position::P15, Position::P16,1,1).unwrap();

    mcv.add_train(Train::T1,Position::P4,Some(Position::P17)).unwrap();
    mcv.add_train(Train::T2,Position::P6,Some(Position::P5)).unwrap();

    let mf: MapFactory = mcv.into();

    return mf;
}


#[test]
fn test_scheduler(){

    let hal = Hal::new().unwrap();

    let mf = get_test_map();

    let mvv = mf.build_visualization_view();

    let llc = LowLevelController::new(&hal, mf.build_controller_view(&hal));

    let mut pfes = PathFinderAndScheduler::new(&hal, &mf);

    pfes.execute_request(Request::new(Train::T1, Position::P16)).unwrap();

    println!("{:?}", mvv.get_train_position(Train::T1).unwrap());
    println!("{:?}", mvv.get_node_status(Position::P16).unwrap());

    pfes.execute_request(Request::new(Train::T2, Position::P16)).unwrap();

}