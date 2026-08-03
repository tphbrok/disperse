use std::f32::consts::PI;

use crate::generators::generator::Generator;

pub struct Triangle {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
}

impl Triangle {
    pub fn new(frequency: f32, sample_rate: u32) -> Self {
        Triangle {
            frequency,
            phase: 0.0,
            sample_rate: sample_rate as f32,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency
    }
}

impl Generator for Triangle {
    fn get_next_value(&mut self) -> f32 {
        let phase_increment = 2.0 * PI * self.frequency / self.sample_rate;
        let new_phase = (self.phase + phase_increment) % (2.0 * PI);

        let normalized_phase = self.phase / PI;

        let next_value = if normalized_phase >= 0.0 && normalized_phase < 0.5 {
            normalized_phase
        } else if normalized_phase >= 1.5 && normalized_phase < 2.0 {
            normalized_phase - 2.0
        } else {
            1.0 - normalized_phase
        } * 2.0;
        self.phase = new_phase;

        dbg!(next_value);

        next_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_at_zero_with_no_phase_offset() {
        let mut triangle = Triangle::new(440.0, 44100);

        let result = triangle.get_next_value();

        assert_eq!(result, 0.0);
    }

    #[test]
    fn it_generates_correct_values_for_6_digit_precision() {
        let mut triangle = Triangle::new(44100.0 / 8.0, 44100);

        // Skip first value, this is already tested
        triangle.get_next_value();

        assert!((0.5 - triangle.get_next_value()).abs() < 1e-6);
        assert!((1.0 - triangle.get_next_value()).abs() < 1e-6);
        assert!((0.5 - triangle.get_next_value()).abs() < 1e-6);
        assert!((0.0 - triangle.get_next_value()).abs() < 1e-6);
        assert!((-0.5 - triangle.get_next_value()).abs() < 1e-6);
        assert!((-1.0 - triangle.get_next_value()).abs() < 1e-6);
        assert!((-0.5 - triangle.get_next_value()).abs() < 1e-6);
        assert!((0.0 - triangle.get_next_value()).abs() < 1e-6);
    }
}
