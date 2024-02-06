use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use std::io;
use anyhow::Result;
use serde_json;
use serde::Deserialize;
use map::map_comunication::MapComunicationSlave;
use std::sync::mpsc::{Receiver,channel};
use std::sync::Mutex;
use std::thread;
use num_traits::FromPrimitive;
use common_infrastructure::{IP_MASTER, MASTER_PORT_VISUALIZER};
#[derive(Clone, Copy, PartialEq, Eq, Debug,Deserialize)]
pub enum Request{
    MoveTrain{
        train_id: Train,
        destination: Position,
    },
    LockTrain{
        train_id: Train,
    },
    UnlockTrain{
        train_id: Train,
    }
}

pub struct HighLevelController{
    incoming_requests: Receiver<Request>
}

macro_rules! unwrap_or_return {
    ($expr:expr) => {
        match $expr{
            Ok(value) => value,
            Err(err) => {
                println!("{:?}",err);
                return rouille::Response::text(format!("error: {}", err)).with_status_code(500)
            }
        }
    };
    () => {};
}

macro_rules! convert_id_or_return {
    ($expr:expr) => {
        match FromPrimitive::from_i32($expr-1){
            Some(value) => value,
            None => return rouille::Response::text(format!("error: impossible to convert: {}", $expr)).with_status_code(500)
        }
    };
    () => {};
}

impl HighLevelController{
    pub fn new(com: MapComunicationSlave) -> Self{

        let (sender,receiver) = channel();

        let com = Mutex::new(com);
        let sender = Mutex::new(sender);

        thread::spawn(move ||{
            // slice here is to remove http:// /master_message handler the router
            rouille::start_server(&format!("{IP_MASTER}:{MASTER_PORT_VISUALIZER}"), move|request| {
                router!(request,
                    (POST) (/add_request) => {
                        let body = rouille::input::plain_text_body(&request);
                        let mut body: String = try_or_400!(body);
                        body = body.replace("\\","");
                        println!("{:?}",body);
                        let message = serde_json::from_str::<Request>(&body);
                        let message= unwrap_or_return!(message);
                        println!("{:?}",message);
                        // is ok if this thread panics because the main one has been closed
                        sender.lock().unwrap().send(message).unwrap();
                        rouille::Response::text("ok")
                    },
                    (GET) (/train_position/{id: i32}) => {
                        let train = convert_id_or_return!(id);
                        let pos = com.lock().unwrap().get_train_position(train);
                        serde_json::to_string(&pos).unwrap();
                        rouille::Response::text(&serde_json::to_string(&pos).unwrap())
                    },
                    (GET) (/train_status/{id: i32}) => {
                        let train = convert_id_or_return!(id);
                        let pos = com.lock().unwrap().get_train_status(train);
                        serde_json::to_string(&pos).unwrap();
                        rouille::Response::text(&serde_json::to_string(&pos).unwrap())
                    },
                    (GET) (/train_speed/{id: i32}) => {
                        let train = convert_id_or_return!(id);
                        let pos = com.lock().unwrap().get_train_speed(train);
                        serde_json::to_string(&pos).unwrap();
                        rouille::Response::text(&serde_json::to_string(&pos).unwrap())
                    },
                    (GET) (/switch_position/{id: i32}) => {
                        let switch = convert_id_or_return!(id);
                        let pos = com.lock().unwrap().get_switch_position(switch);
                        serde_json::to_string(&pos).unwrap();
                        rouille::Response::text(&serde_json::to_string(&pos).unwrap())
                    },
                    _ => {
                        println!("not found");
                        rouille::Response::empty_404()
                    }
                )
            });
        });


        Self{
            incoming_requests: receiver
        }
    }

    /// get_request is blocking
    pub fn get_request(&self)->Result<Request>{
        self.incoming_requests.recv().map_err(|e| e.into())
    }
}