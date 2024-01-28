use std::ffi::c_char;
use std::ops::Not;
use map::map_creation_object::Position;
/// Return codes:
/// 0: Success
/// 1: Map not initialized
/// 2: buffer too small
/// 3: a general error in the map creation, see the error message for more details
/// 4: the Train/Node/Switch passed does not exist

use map::views::MapCreationView;
use num_traits::FromPrimitive;

static mut MAP: Option<MapCreationView> = None;

macro_rules! get_map {
    () => {
        unsafe{
            match &mut MAP{
                Some(map) => map,
                None => return 1,
            }
        }
    };
}

macro_rules! propage_error {
    ($result: expr) => {
        match $result{
            Ok(value) => value,
            Err(error) => {
                println!("Error in map creation: {:?}", error);
                return 3
            },
        }
    };
}

macro_rules! int_to_enum {
    ($pos: ident) => {
        match FromPrimitive::from_i32($pos){
            Some(pos) => pos,
            None => return 4,
        }
    };
    (*$pos: ident) => {
        match FromPrimitive::from_i32(*$pos){
            Some(pos) => pos,
            None => return 4,
        }
    };
}


pub extern "C" fn map_init() -> i32{
    unsafe{
        MAP = Some(MapCreationView::new());
    }
    return 0
}

pub extern "C" fn add_node(position: i32) -> i32{
    let map = get_map!();
    propage_error!(map.add_node(int_to_enum!(position)));
    return 0;
}

pub extern "C" fn add_switch(position: i32) -> i32{
    let map = get_map!();
    propage_error!(map.add_switch(int_to_enum!(position)));
    return 0;
}

pub extern "C" fn add_switch_station(switch: i32, position_center: i32, position_diverted: i32, position_straight: i32) -> i32{
    let map = get_map!();

    propage_error!(map.add_switch_station(
        int_to_enum!(switch),
        int_to_enum!(position_center),
        int_to_enum!(position_diverted),
        int_to_enum!(position_straight)
    ));
    return 0;
}

pub extern "C" fn add_link(position_1: i32, position_2: i32, max_speed_1_to_2: i8, max_speed_2_to_1: i8, length: u32) -> i32{
    let map = get_map!();

    propage_error!(map.add_link(
        int_to_enum!(position_1),
        int_to_enum!(position_2),
        max_speed_1_to_2,
        max_speed_2_to_1,
        length
    ));
    return 0;
}

/// pointing_to:
///  - if is a number representing a node, the train will be pointing to this node
///  - if is -1, it wull be interpreder as None, witch measn the train will be pointing
///    the dead end of the link (if ther is one)
pub extern "C" fn add_train(train: i32, position: i32, pointing_to: i32) -> i32{
    let map = get_map!();

    let pointing_to: Option<Position> = if pointing_to == -1{
        None
    }else{
        Some(int_to_enum!(pointing_to))
    };

    propage_error!(map.add_train(
        int_to_enum!(train),
        int_to_enum!(position),
        pointing_to
    ));
    return 0;
}

pub extern "C" fn get_map_json(buffer: *mut u8, buffer_len: u32) -> i32{
    let map = get_map!();
    let json: String = propage_error!(map.to_json());
    let json_len = json.len();
    if json_len+1 > buffer_len as usize{
        return 2;
    }
    unsafe{
        std::ptr::copy_nonoverlapping(json.as_ptr(), buffer, json_len);
        *buffer.offset(json_len as isize) = 0;
    }
    return 0;
}