use event_system::EventSystem;
use winit::window;

use crate::{
    core::sf_events::{EventListener, Eventable, MouseMoveEvent},
    info_core,
    sf_window::{self, WindowEventHandler},
};

pub mod event_system;
pub mod layers;

pub struct Application {
    pub event_handler: EventSystem,
}

struct EventHandlerForWindow<'a> {
    event_sys: &'a mut EventSystem,
}

impl<'a> sf_window::WindowEventHandler for EventHandlerForWindow<'a> {
    fn handle_event<T: Eventable>(&mut self, event: T) {
        self.event_sys.on_event(event);
    }
}

struct MouseMoveListener;
impl EventListener for MouseMoveListener {
    type EventableConcreteType = MouseMoveEvent;

    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool {
        info_core!("HELLO FROM EVENT: {}", event.to_string());
        false
    }
}

impl Application {
    pub fn new() -> Application {
        let event_handler = EventSystem::new();

        Self { event_handler }
    }

    pub fn run(&self) {
        let mut ev_sys = EventSystem::new();
        ev_sys.event_dispatcher.add_listener(MouseMoveListener);

        let window_event_handler = EventHandlerForWindow {
            event_sys: &mut ev_sys,
        };

        let mut window_manager = sf_window::WindowManager::new(window_event_handler);

        window_manager.run();
    }
}
