use super::symbol::SemanticSymbol;
use crate::core::observation::event::ObservationEvent;

pub struct InterpretationEngine;

impl InterpretationEngine {
    pub fn interpret(observation: &ObservationEvent) -> SemanticSymbol {
        SemanticSymbol {
            value: observation.text.clone(),
        }
    }
}