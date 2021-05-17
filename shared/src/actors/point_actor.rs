use nanoserde::{DeBin, SerBin};

use naia_derive::Actor;
use naia_shared::{Actor, Property};

use crate::Actors;

#[derive(Clone, PartialEq, DeBin, SerBin)]
pub enum PointActorColor {
    Red,
    Blue,
    Yellow,
}

impl Default for PointActorColor {
    fn default() -> Self {
        PointActorColor::Red
    }
}

#[derive(Actor)]
#[type_name = "Actors"]
pub struct PointActor {
    #[predict]
    pub x: Property<i32>,
    #[predict]
    pub y: Property<i32>,
    pub color: Property<PointActorColor>,
}

impl PointActor {
    pub fn new(x: i32, y: i32, color: PointActorColor) -> PointActor {
        PointActor::new_complete(x, y, color)
    }
}