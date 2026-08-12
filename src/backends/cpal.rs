use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn play_samples(samples: Vec<f32>) {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    let sample_count = samples.len();
    let channels = config.channels() as usize;

    let mut sample_clock = 0;
    let mut next_value = move || {
        sample_clock = sample_clock + 1;

        *samples.get(sample_clock).unwrap()
    };

    dbg!("a");

    let stream = device
        .build_output_stream(
            config.into(),
            move |data: &mut [f32], _| {
                dbg!("b");
                for frame in data.chunks_mut(channels) {
                    let value = next_value();

                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            |err| eprintln!("{err}"),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(
        (1000 * sample_count / 44100) as u64,
    ));
}
