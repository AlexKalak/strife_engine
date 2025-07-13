use core::{borrow, time};
use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell},
    rc::Rc,
    time::SystemTime,
};

use egui::{CentralPanel, FullOutput};
use egui_wgpu::Renderer;
use egui_winit::{EventResponse, State, winit::window::Window};
use winit::{event::WindowEvent, window::WindowId};

use crate::{info_core, warn_core};

use super::{sf_graphics::wgpu_backend::WgpuGraphics, sf_layers::Layer};

pub struct SfGuiLayer {
    name: String,
    window: Rc<RefCell<Window>>,
    graphics: Rc<RefCell<WgpuGraphics>>,
    egui_context: egui::Context,
    egui_winit_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

impl SfGuiLayer {
    pub fn new(
        name: String,
        window: Rc<RefCell<Window>>,
        graphics: Rc<RefCell<WgpuGraphics>>,
    ) -> SfGuiLayer {
        let egui_context = egui::Context::default();

        let egui_winit_state = State::new(
            egui_context.clone(),
            egui_context.viewport_id(),
            &*window.borrow(),
            None,
            None,
            None,
        );

        let graphics_ref = graphics.borrow();

        let egui_renderer = Renderer::new(
            graphics_ref.device,
            graphics_ref.surface_config.format,
            None,
            1,
            false,
        );

        Self {
            name,
            window,
            graphics,
            egui_context,
            egui_winit_state,
            egui_renderer,
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
        let raw_input = self.egui_winit_state.take_egui_input(&self.window.borrow());

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

        let graphics_ref = self.graphics.borrow();
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            pixels_per_point: self.egui_context.pixels_per_point(),
            size_in_pixels: [
                graphics_ref.surface_config.width,
                graphics_ref.surface_config.height,
            ],
        };

        let current_texture = graphics_ref
            .surface
            .get_current_texture()
            .expect("egui renderer encoder");
        let view = current_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = graphics_ref
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Egui encoder"),
            });

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(
                &graphics_ref.device,
                &graphics_ref.queue,
                *id,
                image_delta,
            );
        }

        self.egui_renderer.update_buffers(
            &graphics_ref.device,
            &graphics_ref.queue,
            &mut encoder,
            &full_output.shapes,
            &screen_descriptor,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.egui_renderer
                .render(render_pass, &full_output.shapes, &screen_descriptor);
        }

        graphics_ref.queue.submit(std::iter::once(encoder.finish()));
        current_texture.present();

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }

    fn on_event(&mut self, event: &dyn super::sf_events::Eventable) {
        if event.type_id() == TypeId::of::<WindowEvent>() {
            if let Some(e) = (event as &dyn Any).downcast_ref::<WindowEvent>() {
                self.on_window_event(e);
            }
        }
    }
}
