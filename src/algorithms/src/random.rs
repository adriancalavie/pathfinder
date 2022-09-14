use rand::{rngs::ThreadRng, Rng};

pub(crate) struct RandomEngine {
    pub(crate) generator: ThreadRng,
    pub(crate) range: std::ops::Range<f64>,
}

impl RandomEngine {
    pub(crate) fn new(generator: ThreadRng, range: std::ops::Range<f64>) -> Self {
        Self { generator, range }
    }

    pub(crate) fn get_random_value(&mut self) -> f64 {
        self.generator.gen_range(self.range.clone())
    }

    pub fn get_x(&mut self) -> f64 {
        self.get_random_value()
    }

    pub fn get_y(&mut self) -> f64 {
        self.get_random_value()
    }
}
