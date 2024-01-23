use common_infrastructure::hals::{GenericHal, TrainHal};
use common_infrastructure::messages::{MasterMessage, TrainMessage};
use common_infrastructure::devices::Train;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::sys;
use anyhow::Result;
use common_infrastructure::Position;
use log::info;

mod train_hal;
use train_hal::EspTrainHal;

const THIS_TRAIN: Train = Train::T1;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    let hal = EspTrainHal::new()?;
    let mut stop_position = None;
    let mut last_position_red = None;
    loop {
        
        let position = hal.read_position()?;
        if let Some(position) = position{

            if Some(position) != last_position_red{
                hal.send_message_to_master(
                    MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position)
                )?
            }
            
            last_position_red = Some(position);
        }

        if (stop_position == position && stop_position != None) {
            hal.set_speed(0)?;
            stop_position = None;
        }
        

        let message = hal.get_message()?;
        if let Some(message) = message{
            match message {
                TrainMessage::SetSpeed(speed) => hal.set_speed(speed)?,
                TrainMessage::SetSpeedAndStopAt(speed,position) => {
                    hal.set_speed(speed)?;
                    stop_position = Some(position);
                }
                _ => {}
            }
        }
        hal.sleep_for_ms(1);
        //info!("alive");
    }
}