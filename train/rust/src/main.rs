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

const THIS_TRAIN: Train = Train::T2;
const SPEED_OFFSET:f32 = 0.75;

fn main() -> Result<()> {
    sys::link_patches();
    EspLogger::initialize_default();

    let hal = EspTrainHal::new()?;

    let mut last_position_red = None;
    loop {
        
        let position = hal.read_position()?;
        if let Some(position) = position{
            if Some(position) != last_position_red{
                hal.send_message_to_master(
                    MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position)
                )?;
                last_position_red = Some(position);
            }
        }

        let message = hal.get_message()?;
        if let Some(message) = message{
            match message {
                TrainMessage::SetSpeed(speed) => hal.set_speed(speed)?,
                TrainMessage::SetSpeedAndStopAt(speed,position) => {
                    hal.set_speed(speed)?;
                    stop_at_tag(&hal,  speed, position, last_position_red)?;
                    last_position_red = Some(position);
                    hal.send_message_to_master(MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position))?;
                }
                _ => {}
            }
        }
        //hal.sleep_for_ms(1);
        //info!("alive");
    }
}

fn stop_at_tag(hal: &EspTrainHal<'_>, speed:i8, position_to_stop: Position, last_position: Option<Position>) -> Result<()>{

    loop{
        if let Some(position) = hal.read_position()?{

            if Some(position) == last_position{
                continue;
            }

            if position == position_to_stop{
                break;
            }

            // tell to the master that i have reached an unexpected position
            hal.send_message_to_master(MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position))?;
            
            return Err(anyhow::anyhow!("Unexpected position!"));
        }
    }

    hal.set_speed(0)?;
    hal.sleep_for_ms(500);
    if speed>0{
        hal.set_speed(-25)?;
    }else{
        hal.set_speed(25)?;
    }
    
    loop{
        if let Some(position) = hal.read_position()?{

            if Some(position) == last_position{
                continue;
            }

            if position == position_to_stop{
                break;
            }

            // tell to the master that i have reached an unexpected position
            hal.send_message_to_master(MasterMessage::TrainHasReachedPosition(THIS_TRAIN, position))?;
            
            return Err(anyhow::anyhow!("Unexpected position!"));
        }
    }
    hal.set_speed(0)?;

    return Ok(());
}