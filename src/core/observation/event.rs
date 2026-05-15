use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationEvent {
    pub text: String,
    pub source: String,
    pub uncertainty: f32,
}

impl ObservationEvent {
    pub fn new(text: String, source: String, uncertainty: f32) -> Self {
        Self {
            text,
            source,
            uncertainty,
        }
    }
}