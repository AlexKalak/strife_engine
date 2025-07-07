use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::EventLoopError,
    event::{self, DeviceId, ElementState, MouseButton},
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    keyboard::{KeyCode, PhysicalKey},
    window::{WindowBuilder, WindowId},
};

use crate::core::sf_events::{
    self, Event, KeyPressedEvent, KeyReleasedEvent, MouseButtonPressedEvent,
    MouseButtonReleasedEvent, MouseMoveEvent, TerminateWindowEvent, WindowCloseEvent,
    WindowResizeEvent,
};

pub enum WindowManagerCustomEvents {
    TerminateWindow,
}
pub struct WindowManager<F>
where
    F: Fn(&dyn Event),
{
    event_callback: F,
    pub event_loop_proxy: Option<EventLoopProxy<WindowManagerCustomEvents>>,
}

impl<F> WindowManager<F>
where
    F: Fn(&dyn Event),
{
    pub fn new(event_callback: F) -> WindowManager<F> {
        Self {
            event_callback,
            event_loop_proxy: None,
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoopBuilder::<WindowManagerCustomEvents>::with_user_event()
            .build()
            .unwrap();

        let event_loop_proxy = event_loop.create_proxy();
        self.event_loop_proxy = Some(event_loop_proxy);

        let window = WindowBuilder::new().build(&event_loop).unwrap();

        let _ = event_loop.run(move |event, elwt| match event {
            winit::event::Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                event::WindowEvent::CloseRequested { .. } => {
                    self.handle_window_close_event(window_id);
                    elwt.exit();
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
            },

            winit::event::Event::UserEvent(WindowManagerCustomEvents::TerminateWindow) => {
                elwt.exit()
            }
            _ => {}
        });
    }

    fn handle_window_close_event(&self, window_id: WindowId) {
        let event = WindowCloseEvent {
            name: String::from("WINDOW CLOSE EVENT"),
            is_handled: false,
            window_id,
        };

        self.call_event(&event);
    }

    fn handle_keyboard_input(&self, state: &ElementState, keycode: &KeyCode, repeat: bool) {
        let is_pressed = *state == ElementState::Pressed;

        match is_pressed {
            true => self.call_event(&KeyPressedEvent {
                name: String::from("KeyPressedEvent"),
                repeat,
                is_handled: false,
                keycode: *keycode,
            }),
            false => self.call_event(&KeyReleasedEvent {
                name: String::from("KeyRELEASED EVENT"),
                is_handled: false,
                keycode: *keycode,
            }),
        };
    }

    fn handle_window_resized_event(&self, window_id: WindowId, physical_size: &PhysicalSize<u32>) {
        let event = WindowResizeEvent {
            name: String::from("WINDOW RESIZE EVENT"),
            is_handled: false,
            window_id,
            width: physical_size.width,
            height: physical_size.height,
        };

        self.call_event(&event);
    }

    fn handle_mouse_input(&self, state: &ElementState, button: MouseButton, device_id: DeviceId) {
        let is_pressed = *state == ElementState::Pressed;

        match is_pressed {
            true => self.call_event(&MouseButtonPressedEvent {
                name: String::from("MousePressedEvent"),
                button,
                is_handled: false,
                device_id,
            }),
            false => self.call_event(&MouseButtonReleasedEvent {
                name: String::from("MouseReleasedEvent"),
                button,
                is_handled: false,
                device_id,
            }),
        };
    }

    fn handle_mouse_move_event(&self, device_id: DeviceId, position: &PhysicalPosition<f64>) {
        let event = MouseMoveEvent {
            name: String::from("WINDOW MOUSE MOVE EVENT"),
            is_handled: false,
            device_id,
            x: position.x,
            y: position.y,
        };

        self.call_event(&event);
    }

    fn call_event(&self, event: &dyn Event) {
        (self.event_callback)(event);
    }
}
