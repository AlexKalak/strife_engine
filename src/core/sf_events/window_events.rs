use winit::{event::WindowEvent, window::WindowId};

impl Eventable for WindowEvent {
    fn get_name(&self) -> &str {
        "Window Event"
    }

    fn is_handled(&self) -> bool {
        false
    }
}

use super::Eventable;

pub struct WindowCloseEvent {
    pub name: String,
    pub window_id: WindowId,
    pub is_handled: bool,
}

impl WindowCloseEvent {
    pub fn get_window_id(&self) -> WindowId {
        self.window_id
    }
}

impl Eventable for WindowCloseEvent {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

#[derive(Debug)]
pub struct WindowResizeEvent {
    pub name: String,
    pub window_id: WindowId,
    pub width: u32,
    pub height: u32,
    pub is_handled: bool,
}

impl Eventable for WindowResizeEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!(
            "{}: width - {}, height - {}",
            self.name, self.width, self.height
        )
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

impl WindowResizeEvent {
    pub fn get_width_and_height(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

#[derive(Debug)]
pub struct WindowRedrawRequestedEvent {
    pub name: String,
    pub window_id: WindowId,
    pub is_handled: bool,
}

impl Eventable for WindowRedrawRequestedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!("{}", self.name)
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

impl WindowRedrawRequestedEvent {
    pub fn get_window_id(&self) -> &WindowId {
        &self.window_id
    }
}
