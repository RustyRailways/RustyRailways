use crate::messages::{MasterMessage, TrainMessage,HeartBeatManagerMessage};
use crate::devices::{Train};
use crate::hals::generic_hal::GenericHal;
use anyhow::Result;

pub trait HeartBeatMenagerHal: GenericHal where Self: Sized{
    // returns the next message in the queue of messages, the function can fail if the queue has overflown.
    // the function can return None if the queue is empty;
    fn get_message(&self) -> Result<Option<HeartBeatManagerMessage>>;

    // send a message to a train, the code can fail if some error with the wifi are encountered.
    fn send_message_to_train(&self, train: Train, message: TrainMessage) -> Result<()>;

    // send a message to a switch, the code can fail if some error with the wifi are encountered.
    fn send_message_to_master(&self, message: MasterMessage) -> Result<()>;

    // read the kill switch
    fn read_kill_switch(&self) -> bool;
}