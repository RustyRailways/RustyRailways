use common_infrastructure::devices::Train;
use common_infrastructure::hals::MasterHal;
use common_infrastructure::Position;
use crate::map::views::{MapControllerView, MapVisualizationView};
use anyhow::Result;

pub struct MoveRecorder{
    stack: Vec<Move>,
}

impl MoveRecorder {
    pub fn new()-> Self{
        Self{
            stack: Vec::new(),
        }
    }
    /// is unsafe because is up to the caller to make sure that position can be reached
    /// can return an error if the train, or the position is not found
    pub unsafe fn add_move<T: MasterHal>(
        &mut self, map_controller: &mut MapControllerView<T>,
        map_visualization: & MapVisualizationView, train: Train, destination: Position
    ) -> Result<()>{
        let start = map_visualization.get_train_position(train)?;
        map_controller.move_train_unchecked(train, destination)?;
        self.stack.push(Move::new(train, start, destination));
        Ok(())
    }

    pub fn undo<T: MasterHal>(
        &mut self, map_controller: &mut MapControllerView<T>
    ) -> Result<()>{
        if let Some(last_move) = self.stack.pop(){
            // this is safe is the move was safe wen it was added
            unsafe{
                map_controller.move_train_unchecked(last_move.train, last_move.start)?
            };
            Ok(())
        }else{
            Err(anyhow::anyhow!("No moves to undo"))
        }
    }
}

struct Move{
    pub train: Train,
    pub start: Position,
    pub end: Position,
}

impl Move{
    pub fn new(train: Train, start: Position, end: Position)-> Self{
        Self{
            train,
            start,
            end,
        }
    }
}