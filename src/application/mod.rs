use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

use event_system::EventSystem;
use layers::TestLayer;

use crate::{
    core::{
        sf_events::{EventListener, Eventable, MouseMoveEvent},
        sf_gui::{self, SfGuiLayer},
        sf_layers::Layer,
    },
    sf_window::{self},
};

pub mod event_system;
pub mod layers;

struct EventListenerForWindow<'a> {
    event_sys: &'a mut EventSystem<'a>,
}

impl<'a> sf_window::WindowEventListener for EventListenerForWindow<'a> {
    fn on_handled_event<T: Eventable>(&mut self, event: T) {
        self.event_sys.on_event(event);
    }

    fn on_raw_window_event(&mut self, event: winit::event::WindowEvent) {
        self.event_sys.on_event(event);
    }
}

struct MouseMoveListener;
impl EventListener for MouseMoveListener {
    type EventableConcreteType = MouseMoveEvent;

    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool {
        false
    }
}
pub struct Application<'a> {
    pub event_system: EventSystem<'a>,
    pub sf_gui_layer: Option<Box<SfGuiLayer>>,
    layers: Vec<Box<dyn Layer>>,
}

impl<'a> Application<'a> {
    pub fn new() -> Application<'a> {
        let event_system = EventSystem::<'a>::new();

        Self {
            event_system,
            sf_gui_layer: None,
            layers: Vec::new(),
        }
    }

    pub fn run(&'a mut self) {
        self.event_system
            .non_layer_event_dispatcher
            .add_listener(MouseMoveListener);

        let mut window_manager = sf_window::WindowManager::<EventListenerForWindow>::new(None);
        let window = window_manager.get_window_shared();

        self.sf_gui_layer = Some(Box::new(SfGuiLayer::new("hello".to_string(), window)));

        self.event_system
            .layer_stack
            .push_overlay(&mut **self.sf_gui_layer.as_mut().unwrap());

        let window_event_listener = EventListenerForWindow::<'a> {
            event_sys: &mut self.event_system,
        };

        window_manager.set_event_listener(Some(window_event_listener));
        window_manager.run();
    }
}
