use std::{cell::RefCell, rc::Rc};

use naia_derive::ActorType;

pub use {
    point_actor::{PointActor, PointActorColor},
    world_actor::WorldActor,
    chat::ChatActor
};

mod point_actor;
mod world_actor;
mod chat;

#[derive(ActorType, Clone)]
pub enum Actors {
    PointActor(Rc<RefCell<PointActor>>),
    WorldActor(Rc<RefCell<WorldActor>>),
    ChatActor(Rc<RefCell<ChatActor>>),
}
