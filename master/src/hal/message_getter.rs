
#![allow(unreachable_code)]

use std::marker::PhantomData;
use std::sync::Mutex;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use serde::Deserialize;
use serde_json;
use anyhow::Result;
use common_infrastructure::URL_MASTER;
pub struct MessageReceiver<T: for<'a> Deserialize<'a>>{
    _pd: PhantomData<T>,
    rx: Receiver<String>
}
impl<T: for<'a> Deserialize<'a>> MessageReceiver<T> {
    pub fn new() -> Self{

        let (tx,rx) =  mpsc::channel::<String>();
        // required by the server library since more that one handler could try to use tx at the same time
        let tx = Mutex::new(tx);
        thread::spawn(move ||{
            // slice here is to remove http:// /master_message handler the router 
            rouille::start_server(&URL_MASTER[7..25], move|request| {
                router!(request,
                    (POST) (/master_message) => {
                        let body: String = try_or_400!(rouille::input::plain_text_body(&request));
                        tx.lock().unwrap().send(body).unwrap();
                        rouille::Response::text("ok")
                    },
                    _ => rouille::Response::empty_404()
                )
            });
        });

        return Self{
            rx,
            _pd: PhantomData::default() 
        }
    }

    pub fn try_get_message(&self) -> Result<Option<T>>{
        match self.rx.try_recv() {
            Ok(message) => return  Ok(Some(
                serde_json::from_str(&message)?
            )),
            Err(TryRecvError::Disconnected) => return Err(anyhow::anyhow!("Receiver thread has crashed")),
            Err(TryRecvError::Empty) => return Ok(None)
        }
    }
}
