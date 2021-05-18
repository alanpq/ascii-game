pub use {
    auth_event::AuthEvent,
    key_command::KeyCommand
};
use naia_derive::EventType;

mod auth_event;
mod key_command;

#[derive(EventType, Clone)]
pub enum Events {
    KeyCommand(KeyCommand),
    AuthEvent(AuthEvent),
}
