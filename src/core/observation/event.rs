#[derive(Debug, Clone)]
pub struct ObservationEvent {
    pub text: String,
    pub source: String,
    pub uncertainty: f32,
}