use std::{marker::PhantomData, collections::VecDeque};
use anyhow::Result;
use embedded_svc::utils::mutex::{Mutex,StdRawMutex};
use serde::Deserialize;
use esp_idf_svc::http::server::{EspHttpServer,Configuration as Configurations, HandlerResult};
use embedded_svc::http::Method;


static MESSAGE_QUEUE: Mutex<StdRawMutex,VecDeque<(Box<[u8;100]>,usize)>> = Mutex::new(VecDeque::new());


fn push_message(message: Box<[u8;100]>, len: usize){
    let mut queue = MESSAGE_QUEUE.lock();
    queue.push_back((message,len))
}
fn pop_message() -> Option<Result<String>>{
    let mut queue = MESSAGE_QUEUE.lock();
    let (message,len) = queue.pop_front()?;
    let (bites,_) = message.split_at(len);
    let string = std::str::from_utf8(bites);
    let string = match string {
        Ok(v) => v,
        Err(_) => return Some(Err(anyhow::anyhow!("message is not in valid UTF-8")))
    };
    return Some(Ok(string.to_owned()));
}



pub struct MessageReceiver<'a, T: for<'b> Deserialize<'b>>{
    _pd: PhantomData<T>,
    server: EspHttpServer<'a>
}

impl<T: for<'b> Deserialize<'b> + Clone> MessageReceiver<'_,T> {
    pub fn new(handler: &str) -> Result<Self>{
        
        let mut server = EspHttpServer::new(&Configurations::default())?;
        
        server.fn_handler(handler,Method::Post,|mut x| {
            let mut buff = Box::new([0 as u8;100]);
            let n: usize = x.read(&mut *buff)?;
            push_message( buff,n);
            HandlerResult::Ok(())
        })?;

        Ok(Self{
            _pd: PhantomData::default(),
            server
        })
    }


    pub fn get_message(&self) -> Option<Result<T>>{
        let string = pop_message()?;
        let string = match string {
            Ok(v) => v,
            Err(e) => return Some(Err(e.into()))
        };
        let value: T = match serde_json::from_str(&string){
            Ok(v) => v,
            Err(e) => return Some(Err(e.into()))
        };
        return Some(Ok(value));
    }

}

