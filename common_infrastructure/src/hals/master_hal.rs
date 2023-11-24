use crate::{messages::{MasterMessage, TrainMessage, SwitchMessage}, devices::{Train, Switch}};

use super::generic_hal::GenericHal;
use anyhow::Result;

pub trait MasterHal: GenericHal {
    // returns the next message in the queue of messages, the function can fail if the queue has overflown.
    // the function can return None if the queue is empty;
    fn get_message(&self) -> Result<Option<MasterMessage>>;

    // send a message to a train, the code can fail if some error with the wifi are encountered.
    fn send_message_to_train(&self, train: Train, message: TrainMessage) -> Result<()>;

    // send a message to a switch, the code can fail if some error with the wifi are encountered.
    fn send_message_to_switch(&self, switch: Switch, message: SwitchMessage) -> Result<()>;
}