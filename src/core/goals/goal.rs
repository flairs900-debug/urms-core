#[derive(Debug, Clone)]
pub struct Goal {

    pub name: String,
    pub priority: f32,
    pub reward: f32,
}

impl Goal {

    pub fn new(
        name: &str,
        priority: f32,
        reward: f32,
    ) -> Self {

        Self {

            name: name.to_string(),
            priority,
            reward,
        }
    }
}