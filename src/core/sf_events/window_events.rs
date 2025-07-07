use winit::window::WindowId;

use super::Event;

//Window Events
pub trait EventWindow: Event {
    fn get_window_id(&self) -> WindowId;
}

pub struct TerminateWindowEvent {
    pub name: String,
    pub window_id: WindowId,
}
impl Event for TerminateWindowEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
}
impl EventWindow for TerminateWindowEvent {
    fn get_window_id(&self) -> WindowId {
        self.window_id
    }
}

//Window Close
#[derive(Debug)]
pub struct WindowCloseEvent {
    pub name: String,
    pub window_id: WindowId,
    pub is_handled: bool,
}

impl Event for WindowCloseEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl EventWindow for WindowCloseEvent {
    fn get_window_id(&self) -> WindowId {
        self.window_id
    }
}

//Window Resize Event
pub trait EventWindowResize: Event {
    fn get_window_width_height(&self) -> (u32, u32);
}

#[derive(Debug)]
pub struct WindowResizeEvent {
    pub name: String,
    pub window_id: WindowId,
    pub width: u32,
    pub height: u32,
    pub is_handled: bool,
}

impl Event for WindowResizeEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!(
            "{}: width - {}, height - {}",
            self.name, self.width, self.height
        )
    }
}

impl EventWindow for WindowResizeEvent {
    fn get_window_id(&self) -> WindowId {
        self.window_id
    }
}

impl EventWindowResize for WindowResizeEvent {
    fn get_window_width_height(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
