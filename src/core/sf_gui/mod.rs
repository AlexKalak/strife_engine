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

use super::{
    sf_events::{EventDispatcher, EventListener, WindowResizeEvent},
    sf_graphics::wgpu_backend::WgpuGraphics,
    sf_layers::Layer,
};

pub struct SfGuiLayer<'a> {
    sf_gui_layer_core: Rc<RefCell<SfGuiLayerCore<'a>>>,
    sf_gui_window_resize_listener: SfGuiWindowResizeListener<'a>,
}

impl<'a> SfGuiLayer<'a> {
    pub fn new(name: String, window: &'a Window, graphics: &'a WgpuGraphics<'a>) -> Self {
        let sf_gui_layer_core_rc =
            Rc::new(RefCell::new(SfGuiLayerCore::new(name, window, graphics)));

        let sf_gui_window_resize_listener = SfGuiWindowResizeListener {
            sf_gui_layer_core: sf_gui_layer_core_rc.clone(),
        };

        Self {
            sf_gui_layer_core: sf_gui_layer_core_rc.clone(),
            sf_gui_window_resize_listener,
        }
    }

    pub fn get_layer(&mut self) -> Rc<RefCell<SfGuiLayerCore<'a>>> {
        self.sf_gui_layer_core.clone()
    }
}

struct SfGuiWindowResizeListener<'a> {
    sf_gui_layer_core: Rc<RefCell<SfGuiLayerCore<'a>>>,
}
impl<'a> EventListener for SfGuiWindowResizeListener<'a> {
    type EventableConcreteType = WindowResizeEvent;

    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool {
        (*self.sf_gui_layer_core.borrow())
            .egui_context
            .set_pixels_per_point((*self.sf_gui_layer_core.borrow()).window.scale_factor() as f32);
        false
    }
}

struct SfGuiLayerCore<'a> {
    name: String,
    graphics: &'a WgpuGraphics<'a>,
    window: &'a Window,
    egui_context: egui::Context,
    egui_winit_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    event_dispatcher: EventDispatcher,
}

impl<'a> SfGuiLayerCore<'a> {
    pub fn new(name: String, window: &'a Window, graphics: &'a WgpuGraphics<'a>) -> Self {
        let egui_context = egui::Context::default();

        let egui_winit_state = State::new(
            egui_context.clone(),
            egui_context.viewport_id(),
            window,
            None,
            None,
            None,
        );

        let egui_renderer = Renderer::new(
            &graphics.device,
            graphics.surface_config.format,
            None,
            1,
            false,
        );

        let event_dispatcher = EventDispatcher::new();

        Self {
            name,
            window,
            graphics,
            egui_context,
            egui_winit_state,
            egui_renderer,
            event_dispatcher,
        }
    }

    pub fn on_window_event(
        &mut self,
        event: &egui_winit::winit::event::WindowEvent,
    ) -> EventResponse {
        self.egui_winit_state.on_window_event(self.window, event)
    }

    fn on_resized(&mut self, event: &WindowResizeEvent) {}

    pub fn add_listeners() {}
}

impl<'a> Layer for SfGuiLayerCore<'a> {
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
        let raw_input = self.egui_winit_state.take_egui_input(self.window);

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
            .handle_platform_output(self.window, full_output.platform_output);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            pixels_per_point: self.egui_context.pixels_per_point(),
            size_in_pixels: [
                self.graphics.surface_config.width,
                self.graphics.surface_config.height,
            ],
        };

        let current_texture = self
            .graphics
            .surface
            .get_current_texture()
            .expect("egui renderer encoder");
        let view = current_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Egui encoder"),
                });

        for (id, image_delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(
                &self.graphics.device,
                &self.graphics.queue,
                *id,
                image_delta,
            );
        }

        let clipped_primitives: Vec<egui::ClippedPrimitive> = self
            .egui_context
            .tessellate(full_output.shapes, self.egui_context.pixels_per_point());

        self.egui_renderer.update_buffers(
            &self.graphics.device,
            &self.graphics.queue,
            &mut encoder,
            &clipped_primitives,
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
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let mut static_render_pass = render_pass.forget_lifetime();

            self.egui_renderer.render(
                &mut static_render_pass,
                &clipped_primitives,
                &screen_descriptor,
            );
        }

        self.graphics
            .queue
            .submit(std::iter::once(encoder.finish()));
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

        self.event_dispatcher.dispatch_dynamic(event);
    }
}
