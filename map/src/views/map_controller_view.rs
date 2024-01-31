use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::thread::AccessError;
use common_infrastructure::hals::MasterHal;
use crate::Map;
use crate::states::MapStateInitialized;
use anyhow::Result;
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::messages::{SwitchMessage, TrainMessage};
use common_infrastructure::Position;
use crate::devices::{SwitchControllerOption, SwitchPosition};
use crate::map_creation_object::{Direction, TrainStatus};
use crate::nodes::NodeStatus;
use crate::references::{IntiNodeRef, IntiTrainRef};

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
    /// ## Errors
    /// Returns an error if the connection to the train is lost
    /// Returns an error if the train does not exist
    pub fn set_train_speed(&self, train: Train, speed: i8) -> Result<()>{
        self.hal.send_message_to_train(train, TrainMessage::SetSpeed(speed))?;
        let mut map = self.map.borrow_mut();
        map.get_train_mut(train)?.current_speed = speed;
        if let Some(c) = &mut map.comunciator{
            c.set_train_speed(train, speed);
        }
        Ok(())
    }

    /// # Stops a train by setting the speed to 0
    /// ## Errors
    /// Returns an error if the connection to the train is lost
    /// Returns an error if the train does not exist
    pub fn stop_train(&self, train: Train) -> Result<()>{
        self.set_train_speed(train, 0)
    }

    /// # Sets the position of a switch
    /// ## Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch(&self, switch: Switch, position: SwitchPosition) -> Result<()>{
        match position {
            SwitchPosition::Straight => self.set_switch_straight(switch),
            SwitchPosition::Diverted => self.set_switch_diverted(switch)
        }
    }

    /// # Sets the position of a switch to straight
    /// ## Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch_straight(&self, switch: Switch) -> Result<()>{
        self.hal.send_message_to_switch(switch, SwitchMessage::SetPositionStraight)?;
        let mut map = self.map.borrow_mut();
        map.get_switch_mut(switch)?.position = SwitchPosition::Straight.into();
        if let Some(c) = &mut map.comunciator{
            c.set_switch(switch, SwitchPosition::Straight);
        }
        Ok(())
    }

    /// # Sets the position of a switch to diverted
    /// ## Errors
    /// Returns an error if the connection to the switch is lost
    /// Returns an error if the switch does not exist
    pub fn set_switch_diverted(&self, switch: Switch) -> Result<()>{
        self.hal.send_message_to_switch(switch, SwitchMessage::SetPositionDiverted)?;
        let mut map = self.map.borrow_mut();
        map.get_switch_mut(switch)?.position = SwitchPosition::Diverted.into();
        if let Some(c) = &mut map.comunciator{
            c.set_switch(switch, SwitchPosition::Diverted);
        }
        Ok(())
    }

    /// # Sets the switch between two nodes in a way that the train can pass between them
    /// ## args:
    ///  - p1: Position of the first node
    ///  - p2: Position of the second node
    /// ## Errors
    ///  - Returns an error if the nodes do not exist
    ///  - Returns an error if there is no link between the two nodes
    ///  - Returns an error if the connection to the switch is lost
    /// ## Nodes
    ///  if there is no switch between the two nodes, this function does nothing
    pub fn set_switch_between(&self, p1: Position, p2: Position) -> Result<()>{
        let map = self.map.borrow();
        let node1 = map.get_node(p1)?;
        let adjacent_nodes = node1.adjacent_nodes.borrow();
        let link = adjacent_nodes.get_link_to(p2);
        let link = match link{
            None => return Err(anyhow::anyhow!("No link between {:?} and {:?}",p1,p2)),
            Some(link) => link
        };
        let (action,switch) = match &link.controller{
            SwitchControllerOption::NoSwitch => return Ok(()),
            SwitchControllerOption::SwitchToSetStraight(switch) => {
                (SwitchPosition::Straight,switch.deref().switch)
            },
            SwitchControllerOption::SwitchToSetDiverted(switch) => {
                (SwitchPosition::Diverted,switch.deref().switch)
            }
        };

        drop(link);
        drop(adjacent_nodes);
        drop(node1);
        drop(map);

        match action {
            SwitchPosition::Diverted => self.set_switch_diverted(switch)?,
            SwitchPosition::Straight => self.set_switch_straight(switch)?,
        }

        Ok(())
    }

    /// # Move a train to a position
    /// ## Errors
    /// - Returns an error if the train does not exist
    /// - Returns an error if the node does not exist
    /// - Returns an error if the node is occupied or locked by another train
    /// - Returns an error if the position is not adjacent to the current position of the train
    /// ## notes
    /// - if the train is already in the position, this function does nothing
    /// - this function dose not move the train "in the real world", only in the map
    /// - the function will unlock the node that the train was in before
    pub fn move_train(&self, train: Train, position: Position) -> Result<()>{
        let mut map = self.map.borrow_mut();
        let this_train = map.get_train(train)?;
        let direction_link_forward = this_train.current_position.borrow().adjacent_nodes.borrow().get_link_to(position).ok_or(
            anyhow::anyhow!("No link between {:?} and {:?}",this_train.current_position.borrow().position,position)
        )?.direction;
        let this_train = IntiTrainRef{
            train: this_train
        };
        let original_position = map.get_train(train)?.get_position();
        if original_position == position {
            return Ok(());
        }
        let next_node = map.get_node(position)?;
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
        let current_node = map.get_node(original_position)?;
        *current_node.status.borrow_mut() = NodeStatus::Unlocked;

        let this_train = map.get_train_mut(train)?;
        this_train.current_position = next_node.into();

        let direction_link_backward = this_train.current_position.borrow().adjacent_nodes.borrow().get_link_to(original_position).ok_or(
            anyhow::anyhow!("No link between {:?} and {:?}",this_train.current_position.borrow().position,original_position)
        )?.direction;

        // if the direction is the same, it means that i need to
        // flip the relative orientation of the train
        if direction_link_forward == direction_link_backward{
            this_train.direction = match this_train.direction {
                Direction::Forward => Direction::Backward,
                Direction::Backward => Direction::Forward
            };
        }
        if let Some(c) = &mut map.comunciator{
            c.set_train_position(train, position);
        }
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
            _ => {}//return Err(anyhow::anyhow!("Node {:?} is not locked", position))
        };
        Ok(())
    }

    /// # Gets the speed that a train needs to reach a node
    /// the function keeps track of the direction of the train, as well as the max
    /// speed of the link between the two nodes
    /// ## Errors
    /// - Returns an error if the train does not exist
    /// - Returns an error if the node does not exist
    /// - Returns an error if the node is not adjacent to the current position of the train
    pub fn get_speed_to_reach(&self, train: Train, node: Position) -> Result<i8>{
        let map = self.map.borrow();
        let train = map.get_train(train)?;
        let current_node = train.current_position.borrow();
        let adjacent_nodes = current_node.adjacent_nodes.borrow();
        let link = adjacent_nodes.get_link_to(node).ok_or(
            anyhow::anyhow!("No link between {:?} and {:?}",current_node.position,node)
        )?;
        if train.direction == link.direction {
            Ok(link.max_speed)
        }else{
            Ok(-link.max_speed)
        }
    }

    /// # Sets the status of a train
    /// ## Errors
    /// - Returns an error if the train does not exist
    pub fn set_train_status(&mut self, train: Train, status: TrainStatus) -> Result<()>{
        let mut map = self.map.borrow_mut();
        let train_controller = map.get_train_mut(train)?;
        train_controller.status = status;
        if let Some(c) = &mut map.comunciator{
            c.set_train_status(train, status);
        }
        Ok(())
    }


    /// move the train to a position
    /// this function is unsafe because it does not check if the position can be reached
    /// it also don't update the direction of the train, to this function must be always
    /// reverted after been used
    pub unsafe fn move_train_unchecked(&mut self, train: Train, position: Position) -> Result<()>{
        let mut map = self.map.borrow_mut();

        // update destination
        let train_ref: IntiTrainRef = map.get_train(train)?.into();
        let mut destination = map.get_node_mut(position)?.status.borrow_mut();
        if destination.deref() != &NodeStatus::Unlocked && destination.deref() != &NodeStatus::LockedByTrain(train_ref.clone()) {
            return Err(anyhow::anyhow!("Node {:?} is not unlocked", position));
        }
        *destination= NodeStatus::OccupiedByTrain(train_ref);
        drop(destination);

        // update start position and train position
        let node_ref = map.get_node(position)?.into();
        let start_position = map.get_train(train)?.get_position();
        let this_train = map.get_train_mut(train)?;
        *this_train.current_position.borrow_mut() = node_ref;
        *map.get_node_mut(start_position)?.status.borrow_mut() = NodeStatus::Unlocked;

        Ok(())
    }
}