use std::any::Any;

use crate::{
    core::{
        sf_events::{Eventable, WindowResizeEvent},
        sf_layers,
    },
    info_client, info_core,
};

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
    fn on_event(&mut self, event: &dyn Eventable) {
        let any = event as &dyn Any;
        if let Some(e) = any.downcast_ref::<WindowResizeEvent>() {
            info_client!("HELLO {}", e.to_string());
        }
    }
}
