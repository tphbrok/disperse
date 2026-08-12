use disperse::{
    backends,
    generators::{generator::Generator, sawtooth::Sawtooth},
};

/**
 * Plays a clean sawtooth for 1 second, then an LPF-filtered sawtooth for 1 second.
 * The filter is extremely basic, so the audible difference is minimal.
 */
fn main() {
    let mut sawtooth = Sawtooth::new(110.0, 44100);

    let samples = vec![0.0; 88200]
        .iter()
        .enumerate()
        .scan(0.0f32, |current_sample, (index, _)| {
            let next_sample = sawtooth.get_next_value();

            if index < 44100 {
                return Some(next_sample);
            }

            let next_sample = 0.5 * next_sample + 0.5 * *current_sample;

            if index > 44100 {
                *current_sample = next_sample;
            }

            Some(next_sample)
        })
        .collect();

    backends::cpal::play_samples(samples);
}
