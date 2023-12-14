use super::CompleteInitializationMut;
use crate::map::nodes::AdjacentNodes;
use crate::map::Map;
use crate::map::references::IntiNodeRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl<'a> CompleteInitializationMut<'a> for AdjacentNodes<'a,MapStateInitialized>{
    type InitFromType = AdjacentNodes<'a,MapStateUninitialized>;
    fn complete_initialization(&'a mut self, init_from: Self::InitFromType, map: &'a Map<'a, MapStateInitialized>) {
        macro_rules! init_node {
            ($node:ident) => {
                IntiNodeRef{
                    node: map.get_node($node.position)
                }
            }
        }

        *self = match init_from {
            AdjacentNodes::None => {
                AdjacentNodes::<'a,MapStateInitialized>::None
            }
            AdjacentNodes::One([n1]) => {
                AdjacentNodes::<'a,MapStateInitialized>::One([
                    init_node!(n1)
                ])
            }
            AdjacentNodes::Two([n1,n2]) => {
                AdjacentNodes::<'a,MapStateInitialized>::Two([
                    init_node!(n1),
                    init_node!(n2)
                ])
            }
            AdjacentNodes::Tree([n1,n2,n3]) => {
                AdjacentNodes::<'a,MapStateInitialized>::Tree([
                    init_node!(n1),
                    init_node!(n2),
                    init_node!(n3)
                ])
            }
        }
    }
}