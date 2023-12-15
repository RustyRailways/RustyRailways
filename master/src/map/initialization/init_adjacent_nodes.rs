use super::CompleteInitializationMut;
use crate::map::nodes::AdjacentNodes;
use crate::map::Map;
use crate::map::references::IntiNodeRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitializationMut for AdjacentNodes<MapStateInitialized>{
    type InitFromType = AdjacentNodes<MapStateUninitialized>;
    fn complete_initialization(&mut self, init_from: Self::InitFromType, map: & Map<MapStateInitialized>) {
        macro_rules! init_node {
            ($node:ident) => {
                IntiNodeRef{
                    node: map.get_node($node.position)
                }
            }
        }

        *self = match init_from {
            AdjacentNodes::None => {
                AdjacentNodes::<MapStateInitialized>::None
            }
            AdjacentNodes::One([n1]) => {
                AdjacentNodes::<MapStateInitialized>::One([
                    init_node!(n1)
                ])
            }
            AdjacentNodes::Two([n1,n2]) => {
                AdjacentNodes::<MapStateInitialized>::Two([
                    init_node!(n1),
                    init_node!(n2)
                ])
            }
            AdjacentNodes::Tree([n1,n2,n3]) => {
                AdjacentNodes::<MapStateInitialized>::Tree([
                    init_node!(n1),
                    init_node!(n2),
                    init_node!(n3)
                ])
            }
        }
    }
}