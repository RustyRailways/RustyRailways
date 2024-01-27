use std::fmt::Display;
use serde::{Deserialize,Serialize};
use anyhow::{Result, Error};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Deserialize, Serialize,FromPrimitive,ToPrimitive)]
pub enum Position{
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
    P16,
    P17,
    P18,
    P19,
    P20,
    P21,
    P22,
    P23,
    P24,
    P25,
    P26,
    P27,
    P28,
    P29,
    P30,
}

impl Position {
    pub fn from_id(id: [u8;4]) -> Result<Position>{
        match id {
            ID_P1 => Ok(Position::P1),
            ID_P2 => Ok(Position::P2),
            ID_P3 => Ok(Position::P3),
            ID_P4 => Ok(Position::P4),
            ID_P5 => Ok(Position::P5),
            ID_P6 => Ok(Position::P6),
            ID_P7 => Ok(Position::P7),
            ID_P8 => Ok(Position::P8),
            ID_P9 => Ok(Position::P9),
            ID_P10 => Ok(Position::P10),
            ID_P11 => Ok(Position::P11),
            ID_P12 => Ok(Position::P12),
            ID_P13 => Ok(Position::P13),
            ID_P14 => Ok(Position::P14),
            ID_P15 => Ok(Position::P15),
            ID_P16 => Ok(Position::P16),
            ID_P17 => Ok(Position::P17),
            ID_P18 => Ok(Position::P18),
            ID_P19 => Ok(Position::P19),
            ID_P20 => Ok(Position::P20),
            ID_P21 => Ok(Position::P21),
            ID_P22 => Ok(Position::P22),
            ID_P23 => Ok(Position::P23),
            ID_P24 => Ok(Position::P24),
            ID_P25 => Ok(Position::P25),
            ID_P26 => Ok(Position::P26),
            ID_P27 => Ok(Position::P27),
            ID_P28 => Ok(Position::P28),
            ID_P29 => Ok(Position::P29),
            ID_P30 => Ok(Position::P30),
            _ => Err(Error::new(PositionCreationError))
        }
    }

    pub fn to_id(&self)->[u8;4]{
        match self {
            Position::P1 => ID_P1,
            Position::P2 => ID_P2,
            Position::P3 => ID_P3,
            Position::P4 => ID_P4,
            Position::P5 => ID_P5,
            Position::P6 => ID_P6,
            Position::P7 => ID_P7,
            Position::P8 => ID_P8,
            Position::P9 => ID_P9,
            Position::P10 => ID_P10,
            Position::P11 => ID_P11,
            Position::P12 => ID_P12,
            Position::P13 => ID_P13,
            Position::P14 => ID_P14,
            Position::P15 => ID_P15,
            Position::P16 => ID_P16,
            Position::P17 => ID_P17,
            Position::P18 => ID_P18,
            Position::P19 => ID_P19,
            Position::P20 => ID_P20,
            Position::P21 => ID_P21,
            Position::P22 => ID_P22,
            Position::P23 => ID_P23,
            Position::P24 => ID_P24,
            Position::P25 => ID_P25,
            Position::P26 => ID_P26,
            Position::P27 => ID_P27,
            Position::P28 => ID_P28,
            Position::P29 => ID_P29,
            Position::P30 => ID_P30,
        }
    }
}



#[derive(Debug,Clone)]
struct PositionCreationError;
impl Display for PositionCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Creation of position failed because of invalid Tag ID")
    }
}

impl std::error::Error for PositionCreationError {
    fn description(&self) -> &str {
        "Creation of position failed because of invalid Tag ID"
    }
}


const ID_P1: [u8;4] = [209, 205, 207, 49];
const ID_P2: [u8;4] = [81, 38, 223, 49];
const ID_P3: [u8;4] = [177, 10, 108, 49];
const ID_P4: [u8;4] = [1, 161, 57, 64];
const ID_P5: [u8;4] = [241, 246, 86, 64];
const ID_P6: [u8;4] = [49, 148, 43, 64];
const ID_P7: [u8;4] = [81, 116, 106, 49];
const ID_P8: [u8;4] = [225, 97, 106, 49];
const ID_P9: [u8;4] = [129, 71, 136, 49];
const ID_P10: [u8;4] = [33, 184, 159, 49];
const ID_P11: [u8;4] = [97, 254, 188, 49];
const ID_P12: [u8;4] = [209, 188, 219, 49];
const ID_P13: [u8;4] = [17, 86, 131, 49];
const ID_P14: [u8;4] = [193, 25, 203, 49];
const ID_P15: [u8;4] = [225, 213, 154, 49];
const ID_P16: [u8;4] = [193, 67, 218, 49];
const ID_P17: [u8;4] = [225, 8, 133, 49];
const ID_P18: [u8;4] = [113, 246, 229, 47];
const ID_P19: [u8;4] = [81, 223, 207, 49];
const ID_P20: [u8;4] = [49, 44, 222, 47];
const ID_P21: [u8;4] = [113, 151, 238, 49];
const ID_P22: [u8;4] = [129, 213, 178, 49];
const ID_P23: [u8;4] = [209, 11, 131, 64];
const ID_P24: [u8;4] = [81, 235, 189, 49];
const ID_P25: [u8;4] = [241, 89, 21, 64];
const ID_P26: [u8;4] = [177, 227, 216, 49];
const ID_P27: [u8;4] = [0,0,0,1];
const ID_P28: [u8;4] = [0,0,0,2];
const ID_P29: [u8;4] = [0,0,0,3];
const ID_P30: [u8;4] = [0,0,0,4];