use crate::core::memory::record::MemoryRecord;

pub struct MemoryStore {

    pub records: Vec<MemoryRecord>,
}

impl MemoryStore {

    pub fn new() -> Self {

        Self {
            records: Vec::new(),
        }
    }

    pub fn write(
        &mut self,
        event: String,
        weight: f32,
        tag: String,
    ) {

        println!(
            "MEMORY WRITE => {} [{}]",
            event,
            tag
        );

        self.records.push(
            MemoryRecord {
                event,
                weight,
                tag,
            }
        );
    }

    pub fn read_all(&self) {

        println!("MEMORY READ:");

        for item in &self.records {

            println!(
                "-> {} | weight={} | tag={}",
                item.event,
                item.weight,
                item.tag
            );
        }
    }

    pub fn replay(&self) {

        println!("MEMORY REPLAY START");

        for (index, item) in self.records.iter().enumerate() {

            println!(
                "[{}] {} [{}]",
                index,
                item.event,
                item.tag
            );
        }

        println!("MEMORY REPLAY END");
    }

    pub fn important(&self) {

        println!("IMPORTANT MEMORIES");

        for item in &self.records {

            if item.weight > 0.7 {

                println!(
                    "IMPORTANT => {}",
                    item.event
                );
            }
        }
    }
}