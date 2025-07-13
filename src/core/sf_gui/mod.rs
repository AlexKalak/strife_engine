use core::time;
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    rc::Rc,
    time::SystemTime,
};

use egui::CentralPanel;
use egui_winit::{EventResponse, State, winit::window::Window};
use winit::{event::WindowEvent, window::WindowId};

use crate::{info_core, warn_core};

use super::sf_layers::Layer;

pub struct SfGuiLayer {
    name: String,
    window: Rc<RefCell<Window>>,
    egui_context: egui::Context,
    egui_winit_state: egui_winit::State,
}

impl SfGuiLayer {
    pub fn new(name: String, window: Rc<RefCell<Window>>) -> SfGuiLayer {
        let egui_context = egui::Context::default();

        let egui_winit_state = State::new(
            egui_context.clone(),
            egui_context.viewport_id(),
            &*window.borrow(),
            None,
            None,
            None,
        );

        Self {
            name,
            window,
            egui_context,
            egui_winit_state,
        }
    }

    pub fn on_window_event(
        &mut self,
        event: &egui_winit::winit::event::WindowEvent,
    ) -> EventResponse {
        warn_core!("WINDOW EVENT IN GUI ON WINDOW EVENT: {:?}", event);
        self.egui_winit_state
            .on_window_event(&*self.window.borrow(), event)
    }
}

impl Layer for SfGuiLayer {
    fn get_name(&mut self) -> &String {
        &self.name
    }

    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn on_update(&mut self) {
        info_core!(
            "{}",
            format!("ON UPDATE IN SF_GUI: {:?}", SystemTime::now())
        );
        let raw_input = self
            .egui_winit_state
            .take_egui_input(&*self.window.borrow());
        let full_output = self.egui_context.run(raw_input, |ctx| {
            // This is where you define your egui UI
            CentralPanel::default().show(ctx, |ui| {
                ui.heading("Hello egui!");
                if ui.button("Click me!").clicked() {
                    println!("Button clicked!");
                }
            });
        });

        self.egui_winit_state
            .handle_platform_output(&*self.window.borrow(), full_output.platform_output);
    }

    fn on_event(&mut self, event: &dyn super::sf_events::Eventable) {
        if event.type_id() == TypeId::of::<WindowEvent>() {
            if let Some(e) = (event as &dyn Any).downcast_ref::<WindowEvent>() {
                self.on_window_event(e);
            }
        }
    }
}
