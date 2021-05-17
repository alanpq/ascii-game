use naia_derive::EventType;

use crate::{AuthEvent, KeyCommand};

#[derive(EventType, Clone)]
pub enum Events {
    KeyCommand(KeyCommand),
    AuthEvent(AuthEvent),
}