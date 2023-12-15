
use serde::{Serialize, Deserialize};
use crate::map::references::{IntiNodeRef, IntiSwitchRef, IntiTrainRef};

impl Serialize for IntiNodeRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        self.node.position.serialize(serializer)
    }
}

impl Serialize for IntiTrainRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        self.train.train.serialize(serializer)
    }
}

impl Serialize for IntiSwitchRef<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        self.switch.switch.serialize(serializer)
    }
}

