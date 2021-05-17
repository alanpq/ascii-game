use nanoserde::{DeBin, SerBin};

use naia_derive::Actor;
use naia_shared::{Actor, Property};

use crate::Actors;

#[derive(Actor)]
#[type_name = "Actors"]
pub struct WorldActor {
    pub seed: Property<u64>,
}

impl WorldActor {
    pub fn new(seed: u64) -> WorldActor {
        WorldActor::new_complete(seed)
    }
}