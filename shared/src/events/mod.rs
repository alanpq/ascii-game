pub use {
    auth_event::AuthEvent,
    key_command::KeyCommand,
    chat_event::ChatEvent
};
use naia_derive::EventType;

mod auth_event;
mod key_command;
mod chat_event;

#[derive(EventType, Clone)]
pub enum Events {
    KeyCommand(KeyCommand),
    AuthEvent(AuthEvent),
    ChatEvent(ChatEvent),
}
