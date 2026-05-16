#[derive(Debug)]
pub struct EvolutionHistory {
    pub records: Vec<String>,
}

impl EvolutionHistory {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn add(&mut self, event: &str) {
        self.records.push(event.to_string());

        println!("HISTORY ADD -> {}", event);
    }

    pub fn show(&self) {
        println!("===== EVOLUTION HISTORY =====");

        for item in &self.records {
            println!("{}", item);
        }
    }
}