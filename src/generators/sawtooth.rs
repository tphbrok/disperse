use crate::generators::generator::Generator;

pub struct Sawtooth {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl Sawtooth {
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Sawtooth {
            frequency,
            phase: 0.0,
            sample_rate: sample_rate as f32,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency
    }
}

impl Generator for Sawtooth {
    fn get_next_value(&mut self) -> f32 {
        let phase_increment = self.frequency / self.sample_rate;
        let new_phase = (self.phase + phase_increment) % 1.0;

        let next_value = if self.phase >= 0.0 && self.phase < 0.5 {
            self.phase
        } else {
            self.phase - 1.0
        } * 2.0;
        self.phase = new_phase;

        next_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_at_zero_with_no_phase_offset() {
        let mut sawtooth = Sawtooth::new(440.0, 44100);

        let result = sawtooth.get_next_value();

        assert_eq!(result, 0.0);
    }

    #[test]
    fn it_generates_correct_values() {
        let mut sawtooth = Sawtooth::new(44100.0 / 8.0, 44100);

        // Skip first value, this is already tested
        sawtooth.get_next_value();

        assert_eq!(0.25, sawtooth.get_next_value());
        assert_eq!(0.5, sawtooth.get_next_value());
        assert_eq!(0.75, sawtooth.get_next_value());
        assert_eq!(-1.0, sawtooth.get_next_value());
        assert_eq!(-0.75, sawtooth.get_next_value());
        assert_eq!(-0.5, sawtooth.get_next_value());
        assert_eq!(-0.25, sawtooth.get_next_value());
        assert_eq!(0.0, sawtooth.get_next_value());
    }
}
