use super::Event;

pub trait EventKey: Event {
    fn get_keycode(&self) -> winit::keyboard::KeyCode;
}
pub trait EventKeyPressed: EventKey {
    fn get_repeat(&self) -> bool;
}

//Key Pressed
#[derive(Debug)]
pub struct KeyPressedEvent {
    pub name: String,
    pub keycode: winit::keyboard::KeyCode,
    pub repeat: bool,
    pub is_handled: bool,
}

impl Event for KeyPressedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl EventKey for KeyPressedEvent {
    fn get_keycode(&self) -> winit::keyboard::KeyCode {
        self.keycode
    }
}

impl EventKeyPressed for KeyPressedEvent {
    fn get_repeat(&self) -> bool {
        self.repeat
    }
}

//Key Released
#[derive(Debug)]
pub struct KeyReleasedEvent {
    pub name: String,
    pub keycode: winit::keyboard::KeyCode,
    pub is_handled: bool,
}

impl Event for KeyReleasedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl EventKey for KeyReleasedEvent {
    fn get_keycode(&self) -> winit::keyboard::KeyCode {
        self.keycode
    }
}
