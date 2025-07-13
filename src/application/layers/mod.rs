use crate::{
    core::{
        sf_events::{EventDispatcher, EventListener, Eventable, MouseMoveEvent},
        sf_layers,
    },
    info_client, info_core,
};

pub struct TestLayer {
    pub name: String,
    pub event_dispatcher: EventDispatcher,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

struct TestMouseMoveListenerWithLayerName {
    pub layer_name: String,
}
impl EventListener for TestMouseMoveListenerWithLayerName {
    type EventableConcreteType = MouseMoveEvent;

    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool {
        info_client!(
            "{}",
            format!(
                "MOUSE MOVE FROM {} : x- {}, y -{}",
                self.layer_name, event.x, event.y
            )
        );
        true
    }
}

impl TestLayer {
    pub fn new(name: &String) -> TestLayer {
        let mut event_dispatcher = EventDispatcher::new();
        event_dispatcher.add_listener(TestMouseMoveListenerWithLayerName {
            layer_name: name.clone(),
        });

        Self {
            name: name.clone(),
            event_dispatcher,
            mouse_y: -1.0,
            mouse_x: -1.0,
        }
    }
}

impl sf_layers::Layer for TestLayer {
    fn get_name(&mut self) -> &String {
        &self.name
    }

    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn on_update(&mut self) {
        info_core!("TEST LAYER ON UPDATE");
    }
    fn on_event(&mut self, event: &dyn Eventable) {
        self.event_dispatcher.dispatch_dynamic(event);
    }
}
