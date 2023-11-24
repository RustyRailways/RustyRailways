use crate::ip_addresses;
use crate::HasIpAddress;
use serde::{Deserialize,Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum Train{
    T1,
    T2
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Copy,PartialEq, Eq,Debug,Deserialize,Serialize)]
pub enum Device {
    Master,
    Train(Train),
    Switch(Switch)
}

impl HasIpAddress for Train {
    fn get_ip_address(&self)->&'static str {
        match self {
            Train::T1 => ip_addresses::IP_T1,
            Train::T2 => ip_addresses::IP_T2,
        }
    }
}

impl HasIpAddress for Switch {
    fn get_ip_address(&self)->&'static str {
        match self {
            Switch::S1 => ip_addresses::IP_S1,
            Switch::S2 => ip_addresses::IP_S2,
            Switch::S3 => ip_addresses::IP_S3,
            Switch::S4 => ip_addresses::IP_S4,
            Switch::S5 => ip_addresses::IP_S5,
            Switch::S6 => ip_addresses::IP_S6,
            Switch::S7 => ip_addresses::IP_S7,
            Switch::S8 => ip_addresses::IP_S8,
            Switch::S9 => ip_addresses::IP_S9,
            Switch::S10 => ip_addresses::IP_S10
        }
    }
}

impl HasIpAddress for Device {
    fn get_ip_address(&self)->&'static str {
        match self {
            Device::Master => ip_addresses::IP_MASTER,
            Device::Switch(s) => s.get_ip_address(),
            Device::Train(t) => t.get_ip_address()
        }
    }
}