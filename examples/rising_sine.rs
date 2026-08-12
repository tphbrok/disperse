use disperse::{
    backends,
    generators::{generator::Generator, sine::Sine},
};

/**
 * Plays a sine for 1 second, while linearly increasing its frequency from 440 to 540
 */
fn main() {
    let mut frequency = 440.0;
    let mut sine = Sine::new(frequency, 44100);

    let samples = vec![0.0; 44100]
        .iter()
        .map(|_| {
            sine.set_frequency(frequency);

            // Make it rise 100 Hz over the span of 44100 samples
            frequency = frequency + 100.0 / 44100.0;

            sine.get_next_value()
        })
        .collect();

    backends::cpal::play_samples(samples);
}
