use std::fmt::Display;
use serde::{Deserialize,Serialize};
use anyhow::{Result, Error};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Deserialize, Serialize)]
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
    pub fn from_id(id: [i32;4]) -> Result<Position>{
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


const ID_P1: [i32;4] = [0,0,0,0];
const ID_P2: [i32;4] = [0,0,0,1];
const ID_P3: [i32;4] = [0,0,0,2];
const ID_P4: [i32;4] = [0,0,0,3];
const ID_P5: [i32;4] = [0,0,0,4];
const ID_P6: [i32;4] = [0,0,0,5];
const ID_P7: [i32;4] = [0,0,0,6];
const ID_P8: [i32;4] = [0,0,0,7];
const ID_P9: [i32;4] = [0,0,0,8];
const ID_P10: [i32;4] = [0,0,0,9];
const ID_P11: [i32;4] = [0,0,0,10];
const ID_P12: [i32;4] = [0,0,0,11];
const ID_P13: [i32;4] = [0,0,0,12];
const ID_P14: [i32;4] = [0,0,0,13];
const ID_P15: [i32;4] = [0,0,0,14];
const ID_P16: [i32;4] = [0,0,0,15];
const ID_P17: [i32;4] = [0,0,0,16];
const ID_P18: [i32;4] = [0,0,0,17];
const ID_P19: [i32;4] = [0,0,0,18];
const ID_P20: [i32;4] = [0,0,0,19];
const ID_P21: [i32;4] = [0,0,0,20];
const ID_P22: [i32;4] = [0,0,0,21];
const ID_P23: [i32;4] = [0,0,0,22];
const ID_P24: [i32;4] = [0,0,0,23];
const ID_P25: [i32;4] = [0,0,0,24];
const ID_P26: [i32;4] = [0,0,0,25];
const ID_P27: [i32;4] = [0,0,0,26];
const ID_P28: [i32;4] = [0,0,0,27];
const ID_P29: [i32;4] = [0,0,0,28];
const ID_P30: [i32;4] = [0,0,0,29];