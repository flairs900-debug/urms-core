#[derive(Debug, Clone)]
pub struct CognitiveState {

    pub energy: f32,
    pub entropy: f32,
    pub focus: f32,
    pub stability: f32,
    pub recursion_depth: u32,
}

impl CognitiveState {

    pub fn new() -> Self {

        Self {

            energy: 1.0,
            entropy: 0.0,
            focus: 1.0,
            stability: 1.0,
            recursion_depth: 0,
        }
    }

    pub fn evolve(&mut self) {

        self.energy *= 0.99;

        self.entropy += 0.01;

        self.focus *= 0.995;

        self.recursion_depth += 1;

        if self.entropy > 1.0 {

            self.stability *= 0.95;
        }

        println!("COGNITIVE STATE");

        println!("energy -> {}", self.energy);

        println!("entropy -> {}", self.entropy);

        println!("focus -> {}", self.focus);

        println!("stability -> {}", self.stability);

        println!("recursion depth -> {}", self.recursion_depth);
    }
}