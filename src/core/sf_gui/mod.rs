pub mod gui_widgets;

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    rc::Rc,
    sync::Arc,
    time::{Duration, SystemTime},
};

use egui::{CentralPanel, FullOutput, RawInput};
use egui_wgpu::{Renderer, ScreenDescriptor};
use egui_winit::{EventResponse, State, winit::window::Window};
use gui_widgets::WorldRenderWidget;
use winit::event::WindowEvent;

use super::{
    sf_events::{EventDispatcher, EventListener, WindowResizeEvent},
    sf_graphics::wgpu_backend::WgpuGraphics,
    sf_layers::Layer,
    world::World,
};

pub struct SfGuiLayerWrapper<'a> {
    name: String,
    sf_gui_layer_rc: Rc<RefCell<SfGuiLayer>>,
    event_dispatcher: EventDispatcher<'a>,
}

impl<'a> SfGuiLayerWrapper<'a> {
    pub fn new(name: String, window: Arc<Window>, graphics: Rc<RefCell<WgpuGraphics>>) -> Self {
        let sf_gui_layer_rc = Rc::new(RefCell::new(SfGuiLayer::new(window, graphics)));
        let sf_gui_layer_rc2 = sf_gui_layer_rc.clone();
        let mut event_dispatcher = EventDispatcher::new();

        let window_resize_listener = SfGuiWindowResizeListener {
            callback: Box::new(move |event| {
                (*sf_gui_layer_rc2.borrow_mut()).on_resized(event);
                false
            }),
        };
        event_dispatcher.add_listener(window_resize_listener);

        Self {
            name,
            sf_gui_layer_rc: sf_gui_layer_rc.clone(),
            event_dispatcher,
        }
    }
}

impl<'a> Layer for SfGuiLayerWrapper<'a> {
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
        self.sf_gui_layer_rc.borrow_mut().on_update();
    }

    fn on_event(&mut self, event: &dyn super::sf_events::Eventable) {
        if event.type_id() == TypeId::of::<WindowEvent>() {
            if let Some(e) = (event as &dyn Any).downcast_ref::<WindowEvent>() {
                self.sf_gui_layer_rc.borrow_mut().on_window_event(e);
            }
        }
        self.event_dispatcher.dispatch_dynamic(event);
    }
}

struct SfGuiWindowResizeListener<'a> {
    callback: Box<dyn Fn(&WindowResizeEvent) -> bool + 'a>,
}
impl<'a> EventListener for SfGuiWindowResizeListener<'a> {
    type EventableConcreteType = WindowResizeEvent;

    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool {
        (self.callback)(event)
    }
}

struct SfGuiLayer {
    graphics: Rc<RefCell<WgpuGraphics>>,
    window: Arc<Window>,
    egui_context: egui::Context,
    egui_winit_state: egui_winit::State,
    egui_renderer: Rc<RefCell<egui_wgpu::Renderer>>,
    //tmp
    last_fps_check_time: SystemTime,
    last_frame_time: SystemTime,
    frames_count: u64,
    fps_count: u64,
    max_frame_time_for_second: Duration,
    //tmp
    world: World,
    world_renderer_widget: WorldRenderWidget,
}

impl SfGuiLayer {
    pub fn new(window: Arc<Window>, graphics: Rc<RefCell<WgpuGraphics>>) -> Self {
        let egui_context = egui::Context::default();

        let egui_winit_state = State::new(
            egui_context.clone(),
            egui_context.viewport_id(),
            &window,
            None,
            None,
            None,
        );

        let graphics_ref = graphics.borrow();

        let egui_renderer = Rc::new(RefCell::new(Renderer::new(
            &graphics_ref.device,
            graphics_ref.surface_config.format,
            None,
            1,
            false,
        )));

        let world_renderer_widget = WorldRenderWidget::new((100, 100), graphics.clone());

        Self {
            window,
            graphics: graphics.clone(),
            egui_context,
            egui_winit_state,
            egui_renderer,
            last_frame_time: SystemTime::now(),
            last_fps_check_time: SystemTime::now(),
            frames_count: 0,
            fps_count: 0,
            max_frame_time_for_second: Duration::new(0, 0),
            world: World::new(graphics.clone()),
            world_renderer_widget,
        }
    }

    pub fn on_window_event(
        &mut self,
        event: &egui_winit::winit::event::WindowEvent,
    ) -> EventResponse {
        self.egui_winit_state.on_window_event(&self.window, event)
    }

    fn on_resized(&mut self, event: &WindowResizeEvent) {
        self.egui_context
            .set_pixels_per_point(self.window.scale_factor() as f32);

        let (width, height) = event.get_width_and_height();
        self.graphics
            .borrow_mut()
            .resize(winit::dpi::PhysicalSize { width, height });
        let graphics = self.graphics.borrow();
        self.world_renderer_widget.update_size(
            &graphics,
            ((width as f32 * 0.5) as u32, (height as f32 * 0.5) as u32),
        );
    }

    fn on_attach(&mut self) {}

    fn on_detach(&mut self) {}

    fn on_update(&mut self) {
        self.update_fps();
        self.update_gui();
    }

    fn update_fps(&mut self) {
        self.frames_count += 1;

        let now = SystemTime::now();
        let duration_from_last_frame = now.duration_since(self.last_frame_time).unwrap();
        let duration_from_last_fps_check = now.duration_since(self.last_fps_check_time).unwrap();

        if duration_from_last_frame > self.max_frame_time_for_second {
            self.max_frame_time_for_second = duration_from_last_frame
        }
        if duration_from_last_fps_check > Duration::from_secs(1) {
            self.fps_count = self.frames_count;
            self.frames_count = 0;
            self.max_frame_time_for_second = Duration::new(0, 0);
            self.last_fps_check_time = now;
        }
        self.last_frame_time = now;
    }

    fn update_gui(&mut self) {
        let raw_input = self.egui_winit_state.take_egui_input(&self.window);
        let graphics = self.graphics.borrow();

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            pixels_per_point: self.egui_context.pixels_per_point(),
            size_in_pixels: [
                graphics.surface_config.width,
                graphics.surface_config.height,
            ],
        };

        let current_texture = graphics
            .surface
            .get_current_texture()
            .expect("egui renderer encoder");

        let view = current_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let (surface_width, surface_height) = (
            graphics.surface_config.width,
            graphics.surface_config.height,
        );

        let texture_width = surface_width as f32 * 0.5;
        let texture_height = surface_height;

        drop(graphics);

        self.present_gui_output(screen_descriptor, raw_input, &view);
        current_texture.present();
    }

    fn get_frame_output(&self, raw_input: RawInput) -> FullOutput {
        let fps = self.fps_count;
        let max_frame_time = self.max_frame_time_for_second;

        let full_output = self.egui_context.run(raw_input, |ctx| {
            // This is where you define your egui UI
            CentralPanel::default().show(ctx, move |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(format!("FPS: {}", fps));
                        ui.heading(format!("MAX: {:?}", max_frame_time));
                        if ui.button("Click me!").clicked() {
                            println!("Button clicked!");
                        }
                    });

                    ui.vertical(|ui| {
                        ui.heading("render");
                        self.world_renderer_widget
                            .ui(ui, &self.world, self.egui_renderer.clone())
                    });

                    ui.vertical(|ui| {
                        ui.heading("Hello second vertical");
                        if ui.button("Click me!").clicked() {
                            println!("Button clicked!");
                        }
                    });
                });
            });
        });
        full_output
    }

    fn present_gui_output(
        &mut self,
        screen_descriptor: ScreenDescriptor,
        raw_input: RawInput,
        surface_view: &wgpu::TextureView,
    ) {
        let graphics = self.graphics.borrow();
        let full_output = self.get_frame_output(raw_input);
        let mut egui_renderer = self.egui_renderer.borrow_mut();

        let mut encoder = graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Egui encoder"),
            });

        self.egui_winit_state
            .handle_platform_output(&self.window, full_output.platform_output);

        for (id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(&graphics.device, &graphics.queue, *id, image_delta);
        }

        let clipped_primitives: Vec<egui::ClippedPrimitive> = self
            .egui_context
            .tessellate(full_output.shapes, self.egui_context.pixels_per_point());

        egui_renderer.update_buffers(
            &graphics.device,
            &graphics.queue,
            &mut encoder,
            &clipped_primitives,
            &screen_descriptor,
        );

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let mut static_render_pass = render_pass.forget_lifetime();

            egui_renderer.render(
                &mut static_render_pass,
                &clipped_primitives,
                &screen_descriptor,
            );
        }
        graphics.queue.submit(std::iter::once(encoder.finish()));

        // self.egui_renderer.free_texture(&egui_texture_id);
        for id in &full_output.textures_delta.free {
            egui_renderer.free_texture(id);
        }
    }
}
