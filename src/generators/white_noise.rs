use rand::{RngExt, rng, rngs::ThreadRng};

use crate::generators::generator::Generator;

pub struct WhiteNoise {
    rng: ThreadRng,
}

impl WhiteNoise {
    pub fn new() -> Self {
        WhiteNoise { rng: rng() }
    }
}

impl Generator for WhiteNoise {
    fn get_next_value(&mut self) -> f32 {
        self.rng.random()
    }
}
