use nanoserde::{DeBin, SerBin};

use naia_derive::Actor;
use naia_shared::{Actor, Property};

use crate::Actors;

#[derive(Clone, PartialEq)]
#[derive(DeBin, SerBin)]
pub struct Message {
    pub name: String,
    pub body: String,
}

#[derive(Actor)]
#[type_name = "Actors"]
pub struct ChatActor {
    pub messages: Property<Vec<Message>>,
}

impl ChatActor {
    pub fn new() -> ChatActor {
        ChatActor::new_complete(Vec::new())
    }
}