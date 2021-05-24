use crate::Events;
use naia_derive::Event;
use naia_shared::{Event, Property};

#[derive(Event, Clone)]
#[type_name = "Events"]
pub struct ChatEvent {
    pub body: Property<String>,
}

impl ChatEvent {
    fn is_guaranteed() -> bool {
        true
    }

    pub fn new(body: &str) -> ChatEvent {
        return ChatEvent::new_complete(body.to_string());
    }
}