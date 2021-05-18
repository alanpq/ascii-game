use naia_shared::Manifest;

use crate::{events::{AuthEvent, KeyCommand}, Actors, Events, actors::{PointActor, WorldActor}};

pub fn manifest_load() -> Manifest<Events, Actors> {
    let mut manifest = Manifest::<Events, Actors>::new();

    manifest.register_event(AuthEvent::get_builder());
    manifest.register_pawn(PointActor::get_builder(), KeyCommand::get_builder());
    manifest.register_actor(WorldActor::get_builder());

    manifest
}