use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

use event_system::EventSystem;
use layers::TestLayer;
use winit::{
    dpi::PhysicalSize,
    event_loop::{EventLoop, EventLoopProxy},
    window::{Window, WindowAttributes},
};

use crate::{
    core::{
        sf_events::{EventListener, Eventable, MouseMoveEvent},
        sf_graphics,
        sf_gui::{self, SfGuiLayerWrapper},
        sf_layers::Layer,
    },
    sf_window::{
        self, RawWindowHandleWrapper, WindowManager, WindowManagerCustomEvent, WindowWrapper,
    },
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

pub async fn run() {
    let mut event_system = EventSystem::new();

    let event_loop = EventLoop::<WindowManagerCustomEvent>::with_user_event()
        .build()
        .unwrap();

    let event_loop_proxy = event_loop.create_proxy();

    let window = event_loop.create_window(WindowAttributes::new()).unwrap();
    let window_wrapper = WindowWrapper::new(window);
    let raw_window_handle_wrapper = RawWindowHandleWrapper::new(&window_wrapper).unwrap();

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let surface = instance
        .create_surface(raw_window_handle_wrapper.get_window_handle())
        .unwrap();

    //let mut window_manager = sf_window::WindowManager::<EventListenerForWindow>::new(
    //    None,
    //    event_loop,
    //    event_loop_proxy,
    //    window.clone(),
    //);

    event_system
        .non_layer_event_dispatcher
        .add_listener(MouseMoveListener);

    let inner_size = window.inner_size();
    let graphics = Rc::new(RefCell::new(
        sf_graphics::wgpu_backend::WgpuGraphics::new(instance, surface, inner_size).await,
    ));

    let mut sf_gui_layer = Box::new(SfGuiLayerWrapper::new(
        "hello".to_string(),
        &window,
        graphics.clone(),
    ));

    sf_gui_layer.get_name();
    event_system.layer_stack.push_layer(sf_gui_layer);
}
