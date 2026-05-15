#[derive(Debug, Clone)]
pub struct RuntimeEvent {
    pub name: String,
}

pub struct EventQueue {
    pub events: Vec<RuntimeEvent>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        name: String,
    ) {
        self.events.push(RuntimeEvent {
            name,
        });
    }

    pub fn pop(
        &mut self,
    ) -> Option<RuntimeEvent> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }
}