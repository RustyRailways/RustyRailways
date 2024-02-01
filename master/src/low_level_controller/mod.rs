use common_infrastructure::devices::Train;
use common_infrastructure::hals::MasterHal;
use common_infrastructure::Position;
use map::views::map_controller_view::MapControllerView;
use anyhow::Result;
use common_infrastructure::messages::{MasterMessage, TrainMessage};
use map::map_creation_object::TrainStatus;

pub struct LowLevelController<'a, T: MasterHal> {
    hal: &'a T,
    map_controller: MapControllerView<'a, T>,
}

impl<'a,T:MasterHal> LowLevelController<'a,T> {
    pub fn new(hal: &'a T, map_controller: MapControllerView<'a,T>) -> Self{
        Self{
            hal,
            map_controller
        }
    }

    pub fn move_train(&mut self, train: Train, stations: &[Position]) -> Result<()>{
        match self.move_train_wrapper(train, stations) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.map_controller.stop_train(train)?;
                for position in stations {
                    self.map_controller.unlock_node(*position)?;
                }
                Err(error)
            }
        }
    }

    fn move_train_wrapper(&mut self, train: Train, stations: &[Position]) -> Result<()>{
        println!("moving train {:?} to {:?} path: {:?}", train, stations.last().unwrap(),stations);
        self.map_controller.set_train_status(train, TrainStatus::Moving)?;

        let final_destination = match stations.last() {
            Some(v) => v,
            None => return Err(anyhow::anyhow!("You passed an empty slice"))
        };

        let mut prev_position = &stations[0];
        for i in 1..stations.len() {

            let position = &stations[i];

            //self.map_controller.lock_node(*prev_position, train)?;
            self.map_controller.lock_node(*position, train)?;
            self.map_controller.set_switch_between(*prev_position, *position)?;

            let wanted_train_speed = self.map_controller.get_speed_to_reach(train, *position)?;
            self.map_controller.move_train(train, *position)?;
            let message;
            if position == final_destination {
                message = TrainMessage::SetSpeedAndStopAt(wanted_train_speed, *position);
            } else {
                let next_position = &stations[i+1];
                let next_wanted_train_speed = self.map_controller.get_speed_to_reach(train, *next_position)?;
                if next_wanted_train_speed.is_negative() && wanted_train_speed.is_positive() ||
                    next_wanted_train_speed.is_positive() && wanted_train_speed.is_negative()
                {
                    message = TrainMessage::SetSpeedAndStopAt(wanted_train_speed, *position);
                }else{
                    message = TrainMessage::SetSpeed(wanted_train_speed);
                }
            }
            println!("Sending message: {:?}",message);
            self.hal.send_message_to_train(train, message)?;

            let mut option_recv_message = self.hal.get_message()?;
            while option_recv_message == None {
                option_recv_message = self.hal.get_message()?;
            }
            if let Some(MasterMessage::TrainHasReachedPosition(moving_train, reached_position)) = option_recv_message{
                if moving_train == train && reached_position == *position {
                    self.map_controller.unlock_node(*prev_position)?;
                } else {
                    self.map_controller.stop_train(train)?;
                    self.map_controller.stop_train(moving_train)?;
                    return Err(anyhow::anyhow!("expected train and tag: {:?}{:?}, got: {:?}{:?}",train,position,moving_train,reached_position));
                }
            } else {
                todo!("HeartBeat management");
            }

            prev_position = position;
        }
        Ok(())
    }
}