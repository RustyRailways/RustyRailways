use crate::Map;
use crate::states::MapStateInitialized;

/// Complete the initialization of an object (Train, Switch, Node,...) after it has been
/// created from an uninitialized version of the same object.
///
/// The Initialization must be done in two stages because objects need to reference other objects
/// In the first stage the object is created, and it get a fixed position in memory.
/// In the second stage the references to other objects are set.
///
/// This two traits are used for the second steps.
///
/// There is a mutable and an immutable version of the trait.
/// the first for normal objects, the second for objects that support interior mutability.

pub trait CompleteInitialization{
    type InitFromType;
    fn complete_initialization(&self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>);
}

pub trait CompleteInitializationMut{
    type InitFromType;
    fn complete_initialization(&mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>);
}

pub trait Initialize{
    type InitializedType;
    fn initialize(self) -> Self::InitializedType;
}

pub trait UnInitialize{
    type UninitializedType;
    fn un_initialize(self) -> Self::UninitializedType;
}

mod init_switch_controller_option;
mod init_link;
mod init_adjacent_nodes;
mod init_node_status;
mod init_node;
mod init_map;
mod init_train_controller;