
use serde::{Serialize, Deserialize};
use crate::references::{IntiNodeRef, IntiSwitchRef, IntiTrainRef};

impl Serialize for IntiNodeRef {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        unsafe{(*self.node).position}.serialize(serializer)
    }
}

impl Serialize for IntiTrainRef {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        unsafe{(*self.train).train}.serialize(serializer)
    }
}

impl Serialize for IntiSwitchRef {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        unsafe{(*self.switch).switch}.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IntiNodeRef {
    fn deserialize<D>(_: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error> where
        D: serde::Deserializer<'de> {
        unimplemented!("Initialized node references can't be deserialized, use the uninitialized version instead")
    }
}

impl<'de> Deserialize<'de> for IntiTrainRef {
    fn deserialize<D>(_: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error> where
        D: serde::Deserializer<'de> {
        unimplemented!("Initialized train references can't be deserialized, use the uninitialized version instead")
    }
}

impl<'de> Deserialize<'de> for IntiSwitchRef {
    fn deserialize<D>(_: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error> where
        D: serde::Deserializer<'de> {
        unimplemented!("Initialized switch references can't be deserialized, use the uninitialized version instead")
    }
}
