
use anyhow::Result;

pub trait GenericHal where Self: Sized{
    // initialize the components of the embedded platform and return an object that can be used 
    // abstract the hardware
    fn new() -> Result<Self>;

    // put the controller to sleep for up to an amount of milliseconds.
    // the function sleep for less than that if an interrupt or a particular request arrives. 
    fn sleep_for_ms(ms: u32);
}