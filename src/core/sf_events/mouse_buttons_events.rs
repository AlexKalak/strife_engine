use winit::event::DeviceId;

use super::Event;

pub trait EventMouse: Event {
    fn get_device_id(&self) -> &DeviceId;
}

pub trait EventMouseMove: EventMouse {
    fn get_pos(&self) -> (f64, f64);
}

//Key Pressed
#[derive(Debug)]
pub struct MouseMoveEvent {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub is_handled: bool,
    pub device_id: DeviceId,
}

impl Event for MouseMoveEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!("{}: x- {}, y- {}", self.name, self.x, self.y)
    }
}

impl EventMouse for MouseMoveEvent {
    fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
}
impl EventMouseMove for MouseMoveEvent {
    fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
