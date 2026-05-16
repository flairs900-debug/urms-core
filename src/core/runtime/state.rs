#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub running: bool,
    pub tick: u64,
}

impl RuntimeState {
    pub fn new() -> Self {
        Self {
            running: true,
            tick: 0,
        }
    }

    pub fn next_tick(&mut self) {
        self.tick += 1;
    }
}