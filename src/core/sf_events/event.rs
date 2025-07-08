use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::Debug,
};

use crate::info_core;

#[derive(Debug)]
pub enum EventCategory {
    NoCategory = 0,
    ApplicationCategory = 1,
    InputCategory = 1 << 1,
    KeyboardCategory = 1 << 2,
    MouseCategory = 1 << 3,
    MouseButtonCategory = 1 << 4,
    UserCategory = 1 << 5,
}
pub trait EventWrapper {
    fn get_inner_payload(&self) -> &dyn Eventable;
}

pub trait Eventable: Any + Send + Sync + 'static {
    fn get_name(&self) -> &str;

    fn to_string(&self) -> String {
        String::from(self.get_name())
    }
    fn get_type_id(&self) -> TypeId {
        self.type_id()
    }

    fn is_handled(&self) -> bool;
}

pub struct Event<'a, T> {
    pub event_type: T,
    pub event_payload: &'a dyn Eventable,
}

impl<'a, T: 'static> EventWrapper for Event<'a, T> {
    fn get_inner_payload(&self) -> &dyn Eventable {
        self.event_payload
    }
}

pub trait EventListener {
    type EventableConcreteType: Eventable;
    fn handle(&mut self, event: &Self::EventableConcreteType) -> bool;
}

pub trait AnyListener: Send + Sync {
    fn handle_erased(&mut self, payload: &dyn Eventable) -> bool;
}

impl<L> AnyListener for L
where
    L: EventListener + 'static + Send + Sync,
{
    fn handle_erased(&mut self, payload: &dyn Eventable) -> bool {
        if let Some(concrete_payload) =
            (payload as &dyn Any).downcast_ref::<L::EventableConcreteType>()
        {
            self.handle(concrete_payload)
        } else {
            false
        }
    }
}

pub struct EventDispatcher {
    event_listeners: HashMap<TypeId, Vec<Box<dyn AnyListener>>>,
}

impl EventDispatcher {
    pub fn new() -> EventDispatcher {
        Self {
            event_listeners: HashMap::new(),
        }
    }
    pub fn add_listener<L>(&mut self, listener: L)
    where
        L: EventListener + 'static + Send + Sync,
    {
        let type_id = std::any::TypeId::of::<L::EventableConcreteType>();
        self.event_listeners
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(listener))
    }

    pub fn log_listeners(&self) {
        self.event_listeners
            .iter()
            .for_each(|(key, value)| info_core!("{}", format!("{:?}: {:?}", key, value.len())));
    }

    pub fn dispatch<T>(&mut self, event: T)
    where
        T: Eventable,
    {
        let type_id = std::any::TypeId::of::<T>();
        if let Some(listeners) = self.event_listeners.get_mut(&type_id) {
            for listener in listeners.iter_mut() {
                let handled = listener.handle_erased(&event);
                if handled {
                    break;
                }
            }
        }
    }
}
