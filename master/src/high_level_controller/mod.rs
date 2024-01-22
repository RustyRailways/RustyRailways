use common_infrastructure::devices::Train;
use common_infrastructure::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Request{
    pub train_id: Train,
    pub destination: Position,
}

impl Request{
    pub fn new(train_id: Train, destination: Position) -> Self{
        Self{
            train_id,
            destination,
        }
    }
}

pub struct HighLevelController{

}

impl HighLevelController{
    pub fn new() -> Self{
        Self{

        }
    }

    /// get_request is blocking
    pub fn get_request(&self)->Request{
        todo!()
    }
}