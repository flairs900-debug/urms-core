use super::event::ObservationEvent;

pub struct ObservationEngine;

impl ObservationEngine {
    pub fn observe(text: &str) -> ObservationEvent {
        ObservationEvent::new(
            text.to_string(),
            "runtime".to_string(),
            0.1,
        )
    }
}