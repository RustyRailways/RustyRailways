use crate::urls;
use crate::urls::HasUrl;
use serde::{Deserialize,Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Deserialize, Serialize)]
pub enum Train{
    T1,
    T2
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Deserialize, Serialize)]
pub enum Switch {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10
}

#[derive(Clone, Copy,PartialEq, Eq,Debug, Hash,Deserialize,Serialize)]
pub enum Device {
    Master,
    Train(Train),
    Switch(Switch)
}

impl HasUrl for Train {
    fn get_url(&self)->&'static str {
        match self {
            Train::T1 => urls::URL_T1,
            Train::T2 => urls::URL_T2,
        }
    }
}

impl HasUrl for Switch {
    fn get_url(&self)->&'static str {
        match self {
            Switch::S1 => urls::URL_S1,
            Switch::S2 => urls::URL_S2,
            Switch::S3 => urls::URL_S3,
            Switch::S4 => urls::URL_S4,
            Switch::S5 => urls::URL_S5,
            Switch::S6 => urls::URL_S6,
            Switch::S7 => urls::URL_S7,
            Switch::S8 => urls::URL_S8,
            Switch::S9 => urls::URL_S9,
            Switch::S10 => urls::URL_S10
        }
    }
}

impl HasUrl for Device {
    fn get_url(&self)->&'static str {
        match self {
            Device::Master => urls::URL_MASTER,
            Device::Switch(s) => s.get_url(),
            Device::Train(t) => t.get_url()
        }
    }
}