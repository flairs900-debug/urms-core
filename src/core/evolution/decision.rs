pub struct EvolutionDecision;

impl EvolutionDecision {

    pub fn decide(
        signal: &str,
    ) -> String {

        if signal == "system overloaded" {

            return
                "ParallelExecutionEngine"
                .to_string();
        }

        if signal == "memory overflow" {

            return
                "CompressedMemoryEngine"
                .to_string();
        }

        "AdaptiveExecutionEngine"
            .to_string()
    }
}