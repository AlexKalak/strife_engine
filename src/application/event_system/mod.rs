use std::any::Any;

use crate::{
    core::sf_events::{EventDispatcher, EventWrapper, Eventable, WindowResizeEvent},
    debug_core, info_client,
};

pub struct EventSystem {
    pub event_dispatcher: EventDispatcher,
}

impl EventSystem {
    pub fn new() -> EventSystem {
        Self {
            event_dispatcher: EventDispatcher::new(),
        }
    }

    pub fn on_event<E: Eventable>(&mut self, event: E) {
        let s = event.to_string();
        debug_core!(s);

        self.event_dispatcher.dispatch(event);
    }
}
