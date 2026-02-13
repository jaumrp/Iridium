use std::any::{Any, TypeId};

use ahash::AHashMap;
use parking_lot::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Monitor,
    Lowest,
    Low,
    Normal,
    High,
    Highest,
}

#[derive(thiserror::Error, Debug)]
pub enum EventError {
    #[error("Unexpected error: {0}")]
    UnexpectedType(String),
}

pub trait Cancelable {
    fn is_canceled(&self) -> bool;
    fn set_canceled(&mut self, canceled: bool);
}

pub trait Event: Any + Send + Sync {
    fn name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

struct RegisteredHandler {
    priority: EventPriority,
    handler: Box<dyn Fn(&mut dyn Any) -> Result<(), EventError> + Send + Sync>,
}

pub struct EventBus {
    listeners: RwLock<AHashMap<TypeId, Vec<RegisteredHandler>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: RwLock::new(AHashMap::new()),
        }
    }

    pub fn emit<EventContext>(&self, event: &mut EventContext) -> Result<(), EventError>
    where
        EventContext: Event + Cancelable + 'static,
    {
        let listeners = self.listeners.read();
        let type_id = TypeId::of::<EventContext>();

        if let Some(handlers) = listeners.get(&type_id) {
            for wrapper in handlers {
                if event.is_canceled() && wrapper.priority != EventPriority::Monitor {
                    continue;
                }
                (wrapper.handler)(event.as_any_mut())?;
            }
        }

        Ok(())
    }

    pub fn subscribe<EventContext, Handler>(&self, handler: Handler)
    where
        EventContext: Event + 'static,
        Handler: Fn(&mut EventContext) -> Result<(), EventError> + Send + Sync + 'static,
    {
        self.subscribe_with_priority(EventPriority::Normal, handler);
    }

    pub fn subscribe_with_priority<EventContext, Handler>(
        &self,
        priority: EventPriority,
        handler: Handler,
    ) where
        EventContext: Event + 'static,
        Handler: Fn(&mut EventContext) -> Result<(), EventError> + Send + Sync + 'static,
    {
        let mut listeners = self.listeners.write();
        let type_id = TypeId::of::<EventContext>();

        let boxed_handler = Box::new(move |event_any: &mut dyn Any| {
            if let Some(event) = event_any.downcast_mut::<EventContext>() {
                handler(event)
            } else {
                let name = event_any
                    .downcast_ref::<EventContext>()
                    .unwrap()
                    .name()
                    .to_string();

                return Err(EventError::UnexpectedType(name));
            }
        });
        let entry = listeners.entry(type_id).or_default();
        entry.push(RegisteredHandler {
            priority,
            handler: boxed_handler,
        });

        entry.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}
