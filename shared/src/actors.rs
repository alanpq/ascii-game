use std::{cell::RefCell, rc::Rc};

use naia_derive::ActorType;

use crate::PointActor;

#[derive(ActorType, Clone)]
pub enum Actors {
    PointActor(Rc<RefCell<PointActor>>),
}