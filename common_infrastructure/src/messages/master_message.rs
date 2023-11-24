use serde::{Deserialize,Serialize};
use crate::{devices::*, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum MasterMessage {
    // todo: add general hart beat
    HartBeatFromTrain(Train),
    HartBeatFromSwitch(Switch),
    TrainHasReachedPosition(Train,Position),
    SendTrainTo(Train,Position)
}
