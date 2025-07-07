use event_system::EventSystem;

use crate::sf_window;

pub mod event_system;
pub mod layers;

pub struct Application {
    pub event_handler: EventSystem,
}

impl Application {
    pub fn new() -> Application {
        let event_handler = EventSystem {};

        Self { event_handler }
    }

    pub fn run(&self) {
        let ev_sys = EventSystem {};
        let mut window_manager = sf_window::WindowManager::new(|event| {
            ev_sys.on_event(event);
        });

        window_manager.run();
    }
}
