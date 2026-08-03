use std::f32::consts::PI;

use crate::generators::generator::Generator;

pub struct Sine {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl Sine {
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Sine {
            frequency,
            phase: 0.0,
            sample_rate: sample_rate as f32,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency
    }
}

impl Generator for Sine {
    fn get_next_value(&mut self) -> f32 {
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        let new_phase = (self.phase + phase_increment) % (2.0 * PI);

        let next_value = self.phase.sin();
        self.phase = new_phase;

        next_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_at_zero_with_no_phase_offset() {
        let mut sine = Sine::new(440.0, 44100);

        let result = sine.get_next_value();

        assert_eq!(result, 0.0);
    }

    #[test]
    fn it_generates_correct_values_for_6_digit_precision() {
        let mut sine = Sine::new(44100.0 / 8.0, 44100);

        // Skip first value, this is already tested
        sine.get_next_value();

        // Testing it this way because f32::sin is non-deterministic, unfortunately
        assert!((0.7071067 - sine.get_next_value()).abs() < 1e-6);
        assert!((1.0 - sine.get_next_value()).abs() < 1e-6);
        assert!((0.7071067 - sine.get_next_value()).abs() < 1e-6);
        assert!((0.0 - sine.get_next_value()).abs() < 1e-6);
        assert!((-0.7071067 - sine.get_next_value()).abs() < 1e-6);
        assert!((-1.0 - sine.get_next_value()).abs() < 1e-6);
        assert!((-0.7071067 - sine.get_next_value()).abs() < 1e-6);
        assert!((0.0 - sine.get_next_value()).abs() < 1e-6);
    }
}
