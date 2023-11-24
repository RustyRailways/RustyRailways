use crate::{messages::{TrainMessage, MasterMessage}, Position};
use anyhow::Result;
use super::GenericHal;

pub trait TrainHal: GenericHal {
    // returns the next message in the queue of messages, the function can fail if the queue has overflown.
    // the function can return None if the queue is empty;
    fn get_message(&self) -> Result<Option<TrainMessage>>;

    // send a message to the master, the code can fail if some error with the wifi are encountered.
    fn send_message_to_master(&self, message: MasterMessage) -> Result<()>;

    // read a tag nfc. the function can fail if an error occurs in the serial connection, and can also return none if no tag is detected
    fn read_position(&self) -> Result<Option<Position>>;

    // set the speed of the train
    fn set_speed(&self, speed: i8);
}