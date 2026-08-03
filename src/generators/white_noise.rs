use rand::{RngExt, rng, rngs::ThreadRng};

pub struct WhiteNoise {
    rng: ThreadRng,
}

impl WhiteNoise {
    pub fn new() -> Self {
        WhiteNoise { rng: rng() }
    }

    pub fn get_next_value(&mut self) -> f32 {
        self.rng.random()
    }
}
