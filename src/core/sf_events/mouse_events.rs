use winit::event::{DeviceId, MouseButton};

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

//Mouse press events
fn mouse_button_to_string(button: &MouseButton) -> String {
    match button {
        MouseButton::Left => "left".to_string(),
        MouseButton::Right => "right".to_string(),
        MouseButton::Forward => "forward".to_string(),
        MouseButton::Back => "back".to_string(),
        MouseButton::Middle => "middle".to_string(),
        MouseButton::Other(val) => format!("button {}", val),
    }
}
pub trait EventMouseButtonPressed: EventMouse {
    fn get_button(&self) -> &MouseButton;
}

#[derive(Debug)]
pub struct MouseButtonPressedEvent {
    pub name: String,
    pub is_handled: bool,
    pub device_id: DeviceId,
    pub button: MouseButton,
}

impl Event for MouseButtonPressedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!(
            "{}: button - {}, ",
            self.name,
            mouse_button_to_string(&self.button)
        )
    }
}
impl EventMouse for MouseButtonPressedEvent {
    fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
}
impl EventMouseButtonPressed for MouseButtonPressedEvent {
    fn get_button(&self) -> &MouseButton {
        &self.button
    }
}

//Mouse Released Event

pub trait EventMouseButtonReleased: EventMouse {
    fn get_button(&self) -> &MouseButton;
}

#[derive(Debug)]
pub struct MouseButtonReleasedEvent {
    pub name: String,
    pub is_handled: bool,
    pub device_id: DeviceId,
    pub button: MouseButton,
}

impl Event for MouseButtonReleasedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!(
            "{}: button - {}, ",
            self.name,
            mouse_button_to_string(&self.button)
        )
    }
}
impl EventMouse for MouseButtonReleasedEvent {
    fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
}
impl EventMouseButtonReleased for MouseButtonReleasedEvent {
    fn get_button(&self) -> &MouseButton {
        &self.button
    }
}
