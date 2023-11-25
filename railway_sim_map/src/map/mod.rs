use std::cell::RefCell;
use std::collections::HashMap;
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;

mod nodes;
use nodes::{Node, NodeType, RoadNode, SwitchNode};

mod train;

#[derive(Clone, Debug)]
pub struct Map{

    node: HashMap<Position, NodeType>,
    switches: HashMap<Switch,SwitchNode>,

}
