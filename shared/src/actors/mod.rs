use std::{cell::RefCell, rc::Rc};

use naia_derive::ActorType;

use crate::{PointActor, WorldActor};

pub mod point_actor;
pub mod world_actor;

#[derive(ActorType, Clone)]
pub enum Actors {
    PointActor(Rc<RefCell<PointActor>>),
    WorldActor(Rc<RefCell<WorldActor>>)
}
