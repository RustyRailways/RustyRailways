use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use rouille::websocket::Message;
use std::io;
use anyhow::Result;
use serde_json;
use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug,Deserialize)]
pub struct Request{
    pub train_id: Train,
    pub destination: Position,
}

impl Request{
    pub fn new(train_id: Train, destination: Position) -> Self{
        Self{
            train_id,
            destination,
        }
    }
}

pub struct HighLevelController{

}

impl HighLevelController{
    pub fn new() -> Self{
        Self{

        }
    }

    /// get_request is blocking
    pub fn get_request(&self)->Result<Request>{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let message = serde_json::from_str::<Request>(&buffer)?;
        Ok(message)
    }
}