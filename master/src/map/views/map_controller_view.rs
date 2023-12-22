use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use common_infrastructure::hals::MasterHal;
use crate::map::Map;
use crate::map::states::MapStateInitialized;
use anyhow::Result;
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::messages::{SwitchMessage, TrainMessage};
use common_infrastructure::Position;
use crate::map::devices::SwitchPosition;
use crate::map::nodes::NodeStatus;
use crate::map::references::{IntiNodeRef, IntiTrainRef};

/// # allows you to control the map
pub struct MapControllerView<'a,T:MasterHal>{
    hal: &'a T,
    map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl<'a,T:MasterHal> MapControllerView<'a,T>{
    /// # Creates a new MapControllerView
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>, hal: &'a T) -> Self{
        MapControllerView{ map, hal }
    }

    /// # Sets the speed of a train
    /// # Errors
    /// Returns an error if the connection to the train is lost
    /// Returns an error if the train does not exist
    pub fn set_train_speed(&self, train: Train, speed: i8) -> Result<()>{
        self.hal.send_message_to_train(train, TrainMessage::SetSpeed(speed))?;
        self.map.borrow_mut().get_train_mut(train)?.current_speed = speed;
        Ok(())
    }

    /// # Stops a train by setting the speed to 0
    /// # Errors
    /// Returns an error if the connection to the train is lost
    /// Returns an error if the train does not exist
    pub fn stop_train(&self, train: Train) -> Result<()>{
        self.set_train_speed(train, 0)
    }

    /// # Sets the position of a switch
    /// # Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch(&self, switch: Switch, position: SwitchPosition) -> Result<()>{
        match position {
            SwitchPosition::Straight => self.set_switch_straight(switch),
            SwitchPosition::Diverted => self.set_switch_diverted(switch)
        }
    }

    /// # Sets the position of a switch to straight
    /// # Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch_straight(&self, switch: Switch) -> Result<()>{
        self.hal.send_message_to_switch(switch, SwitchMessage::SetPositionStraight)?;
        self.map.borrow_mut().get_switch_mut(switch)?.position = SwitchPosition::Straight.into();
        Ok(())
    }

    /// # Sets the position of a switch to diverted
    /// # Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch_diverted(&self, switch: Switch) -> Result<()>{
        self.hal.send_message_to_switch(switch, SwitchMessage::SetPositionDiverging)?;
        self.map.borrow_mut().get_switch_mut(switch)?.position = SwitchPosition::Diverted.into();
        Ok(())
    }

    /// # Sets the switch between two nodes in a way that the train can pass between them
    /// # args:
    ///  - p1: Position of the first node
    ///  - p2: Position of the second node
    /// # Errors
    ///  - Returns an error if the nodes do not exist
    ///  - Returns an error if there is no link between the two nodes
    ///  - Returns an error if the connection to the switch is lost
    /// # Nodes
    ///  if there is no switch between the two nodes, this function does nothing
    pub fn set_switch_between(&self, p1: Position, p2: Position) -> Result<()>{
        let map = self.map.borrow_mut();
        let node1 = map.get_node(p1)?;
        let adjacent_nodes = node1.adjacent_nodes.borrow();
        let link = adjacent_nodes.get_link_to(p2);
        let link = match link{
            None => return Err(anyhow::anyhow!("No link between {:?} and {:?}",p1,p2)),
            Some(link) => link
        };
        link.controller.set(self.hal)?;
        Ok(())
    }

    /// # Move a train to a position
    /// ## Errors
    /// - Returns an error if the train does not exist
    /// - Returns an error if the node does not exist
    /// - Returns an error if the node is occupied or locked by another train
    /// ## notes
    /// - if the train is already in the position, this function does nothing
    /// - this function dose not move the train "in the real world", only in the map
    /// - the function will unlock the node that the train was in before
    pub fn move_train(&self, train: Train, position: Position) -> Result<()>{
        let mut map = self.map.borrow_mut();
        let this_train = map.get_train(train)?;
        let this_train = IntiTrainRef{
            train: this_train
        };
        let current_position = map.get_train(train)?.get_position();
        if current_position == position {
            return Ok(());
        }
        let next_node = map.get_node_mut(position)?;
        match next_node.status.borrow().deref(){
            NodeStatus::Unlocked => {},
            NodeStatus::OccupiedByTrain(_) => {
                return Err(anyhow::anyhow!("Node {:?} is occupied", position));
            },
            NodeStatus::LockedByTrain(t) => {
                let occupying_train = t.deref().train;
                if occupying_train != train {
                    return Err(anyhow::anyhow!("Node {:?} is locked by train {:?}", position, occupying_train));
                }
            }
        }
        *next_node.status.borrow_mut() = NodeStatus::OccupiedByTrain(this_train);
        let next_node = IntiNodeRef{
            node: next_node
        };
        let current_node = map.get_node_mut(current_position)?;
        *current_node.status.borrow_mut() = NodeStatus::Unlocked;

        map.get_train_mut(train)?.current_position = next_node.into();

        Ok(())
    }

    /// # Locks a node for a train
    /// ## Errors
    /// - Returns an error if the node does not exist
    /// - Returns an error if the train does not exist
    /// - Returns an error if the node is already locked/occupied by another train
    pub fn lock_node(&self, position: Position, train: Train) -> Result<()>{
        let map = self.map.borrow();
        let node = map.get_node(position)?;
        let train = map.get_train(train)?;
        let mut status = node.status.borrow_mut();
        match status.deref(){
            NodeStatus::Unlocked => {
                *status = NodeStatus::LockedByTrain(IntiTrainRef{train});
            },
            _ => return Err(anyhow::anyhow!("Node {:?} is not unlocked", position))
        };
        Ok(())
    }
    /// # Unlocks a node
    /// ## Errors
    /// - Returns an error if the node does not exist
    /// - Returns an error if the node is not locked by any train
    /// - Returns an error if the node is occupied by a train
    pub fn unlock_node(&self, position: Position) -> Result<()>{
        let map = self.map.borrow();
        let node = map.get_node(position)?;
        let mut status = node.status.borrow_mut();
        match status.deref(){
            NodeStatus::LockedByTrain(_) => {
                *status = NodeStatus::Unlocked;
            },
            _ => return Err(anyhow::anyhow!("Node {:?} is not locked", position))
        };
        Ok(())
    }
}