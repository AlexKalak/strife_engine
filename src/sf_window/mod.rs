use std::{
    any::Any,
    cell::RefCell,
    marker::PhantomData,
    ops::Deref,
    path::Component,
    rc::Rc,
    sync::{Arc, Mutex},
};

use egui::ViewportId;
use egui_winit::State;
use wgpu::rwh::{
    HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::EventLoopError,
    event::{self, DeviceId, ElementState, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, EventLoopBuilder, EventLoopProxy},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    core::sf_events::{
        self, Event, Eventable, KeyPressedEvent, KeyReleasedEvent, MouseButtonPressedEvent,
        MouseButtonReleasedEvent, MouseMoveEvent, WindowCloseEvent, WindowRedrawRequestedEvent,
        WindowResizeEvent,
    },
    info_core,
};

pub struct WindowWrapper<W> {
    reference: Arc<dyn Any + Send + Sync>,
    ty: PhantomData<W>,
}
impl<W: Send + Sync + 'static> WindowWrapper<W> {
    /// Creates a `WindowWrapper` from a window.
    pub fn new(window: Arc<W>) -> WindowWrapper<W> {
        WindowWrapper {
            reference: window.clone(),
            ty: PhantomData,
        }
    }
}
impl<W: 'static> Deref for WindowWrapper<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        self.reference.downcast_ref::<W>().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct RawWindowHandleWrapper {
    _window: Arc<dyn Any + Send + Sync>,
    window_handle: RawWindowHandle,
    display_handle: RawDisplayHandle,
}

impl RawWindowHandleWrapper {
    pub fn new<W: HasWindowHandle + HasDisplayHandle + 'static>(
        window: &WindowWrapper<W>,
    ) -> Result<RawWindowHandleWrapper, HandleError> {
        Ok(RawWindowHandleWrapper {
            _window: window.reference.clone(),
            window_handle: window.window_handle()?.as_raw(),
            display_handle: window.display_handle()?.as_raw(),
        })
    }

    pub fn get_window_handle(&self) -> RawWindowHandle {
        self.window_handle
    }
    pub fn get_displat_handle(&self) -> RawDisplayHandle {
        self.display_handle
    }

    /// Gets the stored window handle.
    pub fn get_handle(&self) -> ThreadLockedRawWindowHandleWrapper {
        ThreadLockedRawWindowHandleWrapper(self.clone())
    }
}

unsafe impl Send for RawWindowHandleWrapper {}
unsafe impl Sync for RawWindowHandleWrapper {}

pub struct ThreadLockedRawWindowHandleWrapper(RawWindowHandleWrapper);

impl HasWindowHandle for ThreadLockedRawWindowHandleWrapper {
    fn window_handle(&self) -> Result<wgpu::rwh::WindowHandle<'_>, HandleError> {
        Ok(unsafe { WindowHandle::borrow_raw(self.0.window_handle) })
    }
}

impl HasDisplayHandle for ThreadLockedRawWindowHandleWrapper {
    fn display_handle(&self) -> Result<wgpu::rwh::DisplayHandle<'_>, HandleError> {
        Ok(unsafe { wgpu::rwh::DisplayHandle::borrow_raw(self.0.display_handle) })
    }
}

#[derive(Debug, Clone)]
pub struct RawHandleWrapperHolder(pub Arc<Mutex<Option<RawWindowHandleWrapper>>>);

pub enum WindowManagerCustomEvent {
    TerminateWindow,
}

pub trait WindowEventListener {
    fn on_raw_window_event(&mut self, event: winit::event::WindowEvent);
    fn on_handled_event<T: Eventable>(&mut self, event: T);
}

pub struct WindowEventHandler<H>
where
    H: WindowEventListener,
{
    event_listener: Option<H>,
}

impl<H> WindowEventHandler<H>
where
    H: WindowEventListener,
{
    fn new(event_listener: Option<H>) -> WindowEventHandler<H> {
        Self { event_listener }
    }
    fn handle_window_event(
        &mut self,
        event: &winit::event::Event<WindowManagerCustomEvent>,
        elwt: &ActiveEventLoop,
        window: &Window,
    ) {
        match *event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match &mut self.event_listener {
                    Some(e) => e.on_raw_window_event(event.clone()),
                    None => {}
                }

                match event {
                    event::WindowEvent::CloseRequested { .. } => {
                        self.handle_window_close_event(window_id);
                        elwt.exit();
                    }
                    event::WindowEvent::RedrawRequested {} => {
                        window.request_redraw();
                        self.handle_redraw_requested_event(window_id);
                    }
                    event::WindowEvent::KeyboardInput {
                        event:
                            event::KeyEvent {
                                state,
                                physical_key: PhysicalKey::Code(keycode),
                                repeat,
                                ..
                            },
                        ..
                    } => {
                        self.handle_keyboard_input(state, keycode, *repeat);
                    }
                    event::WindowEvent::Resized(physical_size) => {
                        self.handle_window_resized_event(window_id, physical_size);
                    }
                    event::WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                    } => {
                        self.handle_mouse_input(state, *button, *device_id);
                    }
                    event::WindowEvent::CursorMoved {
                        device_id,
                        position,
                    } => {
                        self.handle_mouse_move_event(*device_id, position);
                    }

                    _ => {}
                }
            }

            winit::event::Event::UserEvent(WindowManagerCustomEvent::TerminateWindow) => {
                elwt.exit()
            }

            _ => {}
        };
    }

    fn handle_window_close_event(&mut self, window_id: WindowId) {
        let event = WindowCloseEvent {
            name: String::from("WINDOW CLOSE EVENT"),
            is_handled: false,
            window_id,
        };

        self.on_handled_event(event);
    }
    fn handle_redraw_requested_event(&mut self, window_id: WindowId) {
        let event = WindowRedrawRequestedEvent {
            name: String::from("WINDOW REDRAW REQUESTED EVENT"),
            is_handled: false,
            window_id,
        };

        self.on_handled_event(event);
    }

    fn handle_keyboard_input(&mut self, state: &ElementState, keycode: &KeyCode, repeat: bool) {
        let is_pressed = *state == ElementState::Pressed;

        match is_pressed {
            true => self.on_handled_event(KeyPressedEvent {
                name: String::from("KeyPressedEvent"),
                repeat,
                is_handled: false,
                keycode: *keycode,
            }),
            false => self.on_handled_event(KeyReleasedEvent {
                name: String::from("KeyRELEASED EVENT"),
                keycode: *keycode,
                is_handled: false,
            }),
        };
    }

    fn handle_window_resized_event(
        &mut self,
        window_id: WindowId,
        physical_size: &PhysicalSize<u32>,
    ) {
        let event = WindowResizeEvent {
            name: String::from("WINDOW RESIZE EVENT"),
            is_handled: false,
            window_id,
            width: physical_size.width,
            height: physical_size.height,
        };

        self.on_handled_event(event);
    }

    fn handle_mouse_input(
        &mut self,
        state: &ElementState,
        button: MouseButton,
        device_id: DeviceId,
    ) {
        let is_pressed = *state == ElementState::Pressed;

        match is_pressed {
            true => self.on_handled_event(MouseButtonPressedEvent {
                name: String::from("MousePressedEvent"),
                button,
                is_handled: false,
                device_id,
            }),
            false => self.on_handled_event(MouseButtonReleasedEvent {
                name: String::from("MouseReleasedEvent"),
                button,
                is_handled: false,
                device_id,
            }),
        };
    }

    fn handle_mouse_move_event(&mut self, device_id: DeviceId, position: &PhysicalPosition<f64>) {
        let event = MouseMoveEvent {
            name: String::from("WINDOW MOUSE MOVE EVENT"),
            is_handled: false,
            device_id,
            x: position.x,
            y: position.y,
        };

        self.on_handled_event(event);
    }

    fn on_handled_event<T: Eventable>(&mut self, event: T) {
        if let Some(e) = &mut self.event_listener {
            e.on_handled_event(event);
        }
    }
}

pub struct WindowManager<H>
where
    H: WindowEventListener,
{
    pub event_loop_proxy: EventLoopProxy<WindowManagerCustomEvent>,
    event_loop: winit::event_loop::EventLoop<WindowManagerCustomEvent>,
    event_handler: WindowEventHandler<H>,
    window: Arc<Window>,
}

impl<H> WindowManager<H>
where
    H: WindowEventListener,
{
    pub fn new(
        event_listener: Option<H>,
        event_loop: EventLoop<WindowManagerCustomEvent>,
        event_loop_proxy: EventLoopProxy<WindowManagerCustomEvent>,
        window: Arc<Window>,
    ) -> Self {
        let event_handler = WindowEventHandler::new(event_listener);

        Self {
            window,
            event_loop,
            event_handler,
            event_loop_proxy,
        }
    }

    pub fn set_event_listener(&mut self, event_listener: Option<H>) {
        self.event_handler.event_listener = event_listener;
    }

    pub fn run(mut self) {
        let _ = self.event_loop.run(move |event, elwt| {
            self.event_handler
                .handle_window_event(&event, elwt, &self.window)
        });
    }
}
