#[derive(Clone, Debug)]
pub struct Edge {

    pub from: usize,

    pub to: usize,

    pub weight: f32,
}

impl Edge {

    pub fn new(
        from: usize,
        to: usize,
    ) -> Self {

        Self {

            from,

            to,

            weight: 1.0,
        }
    }

    pub fn reinforce(
        &mut self,
        amount: f32,
    ) {

        self.weight += amount;
    }

    pub fn decay(
        &mut self,
    ) {

        self.weight *= 0.98;

        if self.weight < 0.01 {

            self.weight = 0.01;
        }
    }
}