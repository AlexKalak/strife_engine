use std::any::Any;

pub enum EventCategory {
    NoCategory = 0,
    ApplicationCategory = 1,
    InputCategory = 1 << 1,
    KeyboardCategory = 1 << 2,
    MouseCategory = 1 << 3,
    MouseButtonCategory = 1 << 4,
}

pub trait Event: Any {
    fn get_name(&self) -> &str;

    fn to_string(&self) -> String {
        Self::get_name(&self).to_string()
    }

    //to stop propagation
    fn is_handled(&self) -> bool {
        false
    }
}
