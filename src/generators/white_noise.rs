use rand::{RngExt, rng, rngs::ThreadRng};

pub struct WhiteNoise {
    rng: ThreadRng,
}

impl WhiteNoise {
    pub fn new() -> Self {
        WhiteNoise { rng: rng() }
    }

    pub fn get_values(&mut self, count: usize) -> Vec<f32> {
        let mut result = vec![0.0; count];

        result.fill_with(|| self.rng.random());

        result
    }
}
