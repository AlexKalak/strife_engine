use super::Eventable;

//Key Pressed
#[derive(Debug)]
pub struct KeyPressedEvent {
    pub name: String,
    pub keycode: winit::keyboard::KeyCode,
    pub repeat: bool,
    pub is_handled: bool,
}

impl Eventable for KeyPressedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn to_string(&self) -> String {
        format!("Event {}, keycode: {:?}", &self.name, &self.keycode)
    }

    fn is_handled(&self) -> bool {
        self.is_handled
    }
}

impl KeyPressedEvent {
    pub fn get_keycode(&self) -> winit::keyboard::KeyCode {
        self.keycode
    }
    pub fn get_repeat(&self) -> bool {
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

impl Eventable for KeyReleasedEvent {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn is_handled(&self) -> bool {
        self.is_handled
    }
    fn to_string(&self) -> String {
        format!("Event {}, keycode: {:?}", &self.name, &self.keycode)
    }
}

impl KeyReleasedEvent {
    pub fn get_keycode(&self) -> winit::keyboard::KeyCode {
        self.keycode
    }
}
