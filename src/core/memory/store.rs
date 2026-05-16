use crate::core::memory::trace::MemoryTrace;

pub struct MemoryStore {

    pub traces: Vec<MemoryTrace>,
}

impl MemoryStore {

    pub fn new() -> Self {

        Self {

            traces: Vec::new(),
        }
    }

    pub fn store(
        &mut self,
        trace: MemoryTrace,
    ) {

        println!(
            "STORE MEMORY => {}",
            trace.content
        );

        self.traces.push(trace);
    }

    pub fn recall(&self) {

        println!("MEMORY RECALL:");

        for trace in &self.traces {

            println!(
                "[{}] {}",
                trace.id,
                trace.content
            );
        }
    }
}