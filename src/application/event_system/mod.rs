use std::any::{Any, TypeId};

use winit::event::WindowEvent;

use crate::{
    core::{
        sf_events::{EventDispatcher, Eventable, WindowRedrawRequestedEvent},
        sf_layers::LayerStack,
    },
    info_core,
};

pub struct EventSystem<'a> {
    pub layer_stack: LayerStack<'a>,
    pub non_layer_event_dispatcher: EventDispatcher,
}

impl<'a> EventSystem<'a> {
    pub fn new() -> Self {
        Self {
            layer_stack: LayerStack::new(),
            non_layer_event_dispatcher: EventDispatcher::new(),
        }
    }

    pub fn on_event<E: Eventable>(&mut self, event: E) {
        if event.type_id() == TypeId::of::<WindowRedrawRequestedEvent>() {
            for layer in self.layer_stack.layers.iter_mut().rev() {
                layer.on_update();
            }
            return;
        }

        self.non_layer_event_dispatcher.dispatch(&event);
        for layer in self.layer_stack.layers.iter_mut().rev() {
            layer.on_event(&event);
        }
    }
}
