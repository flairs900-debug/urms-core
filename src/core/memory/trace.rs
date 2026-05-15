#[derive(Debug, Clone)]

pub struct MemoryTrace {

    pub id: usize,

    pub content: String,
}

impl MemoryTrace {

    pub fn new(
        id: usize,
        content: &str,
    ) -> Self {

        Self {

            id,

            content: content.to_string(),
        }
    }
}