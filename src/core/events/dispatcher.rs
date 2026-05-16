use crate::core::events::event::RuntimeEvent;

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(event: RuntimeEvent) {
        match event {
            RuntimeEvent::SystemOverload => {
                println!("event -> SystemOverload");
            }

            RuntimeEvent::MemoryOverflow => {
                println!("event -> MemoryOverflow");
            }

            RuntimeEvent::ReflectionStarted => {
                println!("event -> ReflectionStarted");
            }

            RuntimeEvent::ReflectionFinished => {
                println!("event -> ReflectionFinished");
            }
        }
    }
}