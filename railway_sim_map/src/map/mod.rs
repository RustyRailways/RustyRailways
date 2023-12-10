use std::collections::HashMap;
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;
pub mod nodes;
use nodes::GenericNode;
use crate::map::nodes::SwitchNodeTrait;


#[derive(Debug)]
pub struct Map{
    nodes: HashMap<Position, GenericNode>,
    switch_to_position: HashMap<Switch,Position>,
}



impl Map {
    pub fn get_node_at(&self, position: Position) -> &GenericNode{
        return self.nodes.get(&position).unwrap()
    }

    pub fn set_straight(&mut self, switch: Switch){
        let p = self.switch_to_position[&switch];
        self.nodes.get_mut(&p).unwrap().set_straight().unwrap();
    }

    pub fn set_diverted(&mut self, switch: Switch){
        let p = self.switch_to_position[&switch];
        self.nodes.get_mut(&p).unwrap().set_diverted().unwrap();
    }

    pub fn new() -> Self{
        let p1 = GenericNode::new_switch(
            Position::P1,
            None,
            Position::P2,
            Position::P3,
        );
        let p2 = GenericNode::new_road(
            Position::P2,
            Some(Position::P1),
            Some(Position::P14)
        );
        let p3 = GenericNode::new_road(
            Position::P3,
            Some(Position::P1),
            Some(Position::P17)
        );
        let p4 = GenericNode::new_road(
            Position::P4,
            Some(Position::P6),
            Some(Position::P17)
        );
        let p5 = GenericNode::new_switch(
            Position::P5,
            Some(Position::P9),
            Position::P6,
            Position::P7
        );
        let p6 = GenericNode::new_road(
            Position::P6,
            Some(Position::P5),
            Some(Position::P4)
        );
        let p7 = GenericNode::new_road(
            Position::P7,
            Some(Position::P5),
            Some(Position::P8)
        );
        let p8 = GenericNode::new_road(
            Position::P8,
            None,
            Some(Position::P7)
        );
        let p9 = GenericNode::new_road(
            Position::P9,
            Some(Position::P10),
            Some(Position::P5)
        );
        let p10 = GenericNode::new_switch(
            Position::P10,
            Some(Position::P12),
            Position::P11,
            Position::P9
        );
        let p11 = GenericNode::new_road(
            Position::P11,
            Some(Position::P10),
            Some(Position::P13)
        );
        let p12 = GenericNode::new_road(
            Position::P12,
            Some(Position::P10),
            None,
        );
        let p13 = GenericNode::new_road(
            Position::P13,
            Some(Position::P14),
            Some(Position::P11)
        );
        let p14 = GenericNode::new_switch(
            Position::P14,
            Some(Position::P2),
            Position::P13,
            Position::P15
        );
        let p15 = GenericNode::new_road(
            Position::P15,
            Some(Position::P14),
            Some(Position::P16)
        );
        let p16 = GenericNode::new_road(
            Position::P16,
            None,
            Some(Position::P15)
        );
        let p17 = GenericNode::new_road(
            Position::P17,
            Some(Position::P3),
            Some(Position::P4)
        );

        let mut nodes = HashMap::new();
        nodes.insert(Position::P1, p1);
        nodes.insert(Position::P2, p2);
        nodes.insert(Position::P3, p3);
        nodes.insert(Position::P4, p4);
        nodes.insert(Position::P5, p5);
        nodes.insert(Position::P6, p6);
        nodes.insert(Position::P7, p7);
        nodes.insert(Position::P8, p8);
        nodes.insert(Position::P9, p9);
        nodes.insert(Position::P10, p10);
        nodes.insert(Position::P11, p11);
        nodes.insert(Position::P12, p12);
        nodes.insert(Position::P13, p13);
        nodes.insert(Position::P14, p14);
        nodes.insert(Position::P15, p15);
        nodes.insert(Position::P16, p16);
        nodes.insert(Position::P17, p17);


        let mut switch_to_position = HashMap::new();
        switch_to_position.insert(Switch::S1, Position::P1);
        switch_to_position.insert(Switch::S2, Position::P14);
        switch_to_position.insert(Switch::S3, Position::P10);
        switch_to_position.insert(Switch::S4, Position::P5);

        return Self{
            nodes,
            switch_to_position
        }
    }

}
