use crate::{
    core::sf_events::{Event, KeyPressedEvent},
    debug_core, info_core, trace_core,
};

pub struct EventSystem {}

impl EventSystem {
    pub fn on_event(&self, event: &dyn Event) {
        let s = event.to_string();
        debug_core!(s);
    }
}
