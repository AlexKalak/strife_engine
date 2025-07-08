use winit::event::{DeviceId, MouseButton};

use super::Eventable;

//Key Pressed
#[derive(Debug)]
pub struct MouseMoveEvent {
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub is_handled: bool,
    pub device_id: DeviceId,
}

impl Eventable for MouseMoveEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn to_string(&self) -> String {
        format!("{}: x- {}, y- {}", self.name, self.x, self.y)
    }
    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

impl MouseMoveEvent {
    pub fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
    pub fn get_pos(&self) -> (f64, f64) {
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

#[derive(Debug)]
pub struct MouseButtonPressedEvent {
    pub name: String,
    pub is_handled: bool,
    pub device_id: DeviceId,
    pub button: MouseButton,
}

impl Eventable for MouseButtonPressedEvent {
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
    fn is_handled(&self) -> bool {
        self.is_handled
    }
}
impl MouseButtonPressedEvent {
    pub fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
    fn get_button(&self) -> &MouseButton {
        &self.button
    }
}
//Mouse Released Event

#[derive(Debug)]
pub struct MouseButtonReleasedEvent {
    pub name: String,
    pub is_handled: bool,
    pub device_id: DeviceId,
    pub button: MouseButton,
}

impl Eventable for MouseButtonReleasedEvent {
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
    fn is_handled(&self) -> bool {
        self.is_handled
    }
}
impl MouseButtonReleasedEvent {
    pub fn get_device_id(&self) -> &DeviceId {
        &self.device_id
    }
    pub fn get_button(&self) -> &MouseButton {
        &self.button
    }
}
