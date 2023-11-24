use std::fmt::Display;
use serde::{Deserialize,Serialize};
use anyhow::{Result, Error};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
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
    P10
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