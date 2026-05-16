#[derive(Debug, Clone)]
pub struct SemanticSymbol {
    pub value: String,
}

pub struct SemanticInterpreter;

impl SemanticInterpreter {
    pub fn interpret(
        input: &str,
    ) -> SemanticSymbol {
        println!(
            "semantic interpretation started"
        );

        SemanticSymbol {
            value: input.to_string(),
        }
    }
}