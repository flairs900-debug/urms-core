use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

#[derive(Clone, Debug)]
pub struct Node {

    pub id: usize,

    pub name: String,

    pub activation: f32,

    pub weight: f32,

    pub created_at: u64,

    pub updated_at: u64,
}

impl Node {

    pub fn new(
        id: usize,
        name: String,
    ) -> Self {

        let now =
            SystemTime::now()
                .duration_since(
                    UNIX_EPOCH
                )
                .unwrap()
                .as_secs();

        Self {

            id,

            name,

            activation: 1.0,

            weight: 1.0,

            created_at: now,

            updated_at: now,
        }
    }

    pub fn reinforce(
        &mut self,
        amount: f32,
    ) {

        self.weight += amount;

        self.activation += amount * 0.5;

        self.updated_at =
            current_time();
    }

    pub fn decay(
        &mut self,
    ) {

        self.activation *= 0.95;

        if self.activation < 0.01 {

            self.activation = 0.01;
        }

        self.updated_at =
            current_time();
    }

    pub fn touch(
        &mut self,
    ) {

        self.activation += 0.2;

        self.updated_at =
            current_time();
    }
}

fn current_time() -> u64 {

    SystemTime::now()
        .duration_since(
            UNIX_EPOCH
        )
        .unwrap()
        .as_secs()
}