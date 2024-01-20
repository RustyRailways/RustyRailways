use common_infrastructure::hals::{GenericHal, TrainHal};
use common_infrastructure::messages::{MasterMessage, TrainMessage};
use common_infrastructure::devices::Train;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys;
use anyhow::Result;
use log::info;

mod train_hal;
use train_hal::EspTrainHal;

const THIS_TRAIN: Train = Train::T1;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    let hal = EspTrainHal::new()?;

    loop {
        
        let position = hal.read_position()?;
        if let Some(position) = position{
            hal.send_message_to_master(
                MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position)
            )?
        }
        

        let message = hal.get_message()?;
        if let Some(message) = message{
            match message {
                TrainMessage::SetSpeed(seed) => hal.set_speed(seed)?,
                _ => {}
            }
        }
        hal.sleep_for_ms(1);
        //info!("alive");
    }
}