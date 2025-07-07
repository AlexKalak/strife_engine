use crate::core::{sf_events::MouseMoveEvent, sf_layers};

pub struct TestLayer {
    pub name: String,
    pub mouse_x: f64,
    pub mouse_y: f64,
}

impl sf_layers::Layer for TestLayer {
    fn get_name(&mut self) -> &String {
        &self.name
    }

    fn on_attach(&mut self) {
        todo!()
    }

    fn on_detach(&mut self) {
        todo!()
    }

    fn on_update(&mut self) {
        todo!()
    }
    fn on_event(&mut self, event: &dyn crate::core::sf_events::Event) {
        if let Some(e) = event.downcast_ref::<MouseMoveEvent>() {}
    }
}
