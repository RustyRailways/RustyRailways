use crate::map::devices::SwitchControllerOption;
use crate::map::initialization::UnInitialize;
use super::CompleteInitializationMut;
use crate::map::nodes::{AdjacentNodes, Link};
use crate::map::Map;
use crate::map::references::{IntiNodeRef, UnIntiNodeRef};
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitializationMut for AdjacentNodes<MapStateInitialized>{
    type InitFromType = AdjacentNodes<MapStateUninitialized>;
    fn complete_initialization(&mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        macro_rules! init_node {
            ($link:ident) => {
                Link::<MapStateInitialized>{
                    node: IntiNodeRef{
                        node: map.get_node($link.node.position)
                    },
                    length: $link.length,
                    max_speed: $link.max_speed,
                    controller: {
                        let mut controller = SwitchControllerOption::<MapStateInitialized>::NoSwitch;
                        controller.complete_initialization(&$link.controller, map);
                        controller
                    },
                    direction: $link.direction,
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
/*
impl UnInitialize for AdjacentNodes<MapStateInitialized> {
    type UninitializedType = AdjacentNodes<MapStateUninitialized>;
    fn un_initialize(self) -> Self::UninitializedType {
        macro_rules! uninit {
            ($node: ident) => {
                UnIntiNodeRef{
                    position: $node.position
                }
            };
        }

        match self {
            AdjacentNodes::None => {
                AdjacentNodes::<MapStateUninitialized>::None
            }
            AdjacentNodes::One([n1]) => {
                AdjacentNodes::<MapStateUninitialized>::One([
                    uninit!(n1)
                ])
            }
            AdjacentNodes::Two([n1,n2]) => {
                AdjacentNodes::<MapStateUninitialized>::Two([
                    uninit!(n1),
                    uninit!(n2)
                ])
            }
            AdjacentNodes::Tree([n1,n2,n3]) => {
                AdjacentNodes::<MapStateUninitialized>::Tree([
                    uninit!(n1),
                    uninit!(n2),
                    uninit!(n3)
                ])
            }
        }
    }
}*/