use std::f32::consts::PI;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat,
    SampleRate,
    SupportedStreamConfig,
    SupportedStreamConfigRange,
};

const SAMPLE_RATE: u32 = 48_000;
const FREQUENCY: f32 = 1000.0;

fn into_output_config(r: SupportedStreamConfigRange) -> Option<SupportedStreamConfig> {
    if r.channels() != 1 {
        return None;
    }
    r.try_with_sample_rate(SampleRate(SAMPLE_RATE))
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device
        .supported_output_configs()
        .unwrap()
        .find_map(into_output_config)
        .unwrap()
        .into();

    let mut t = 0.0;
    let sine_samples = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for (i, v) in data.iter_mut().enumerate() {
            let ti = t + i as f32 / SAMPLE_RATE as f32;
            let w = 2.0 * PI * FREQUENCY;
            *v = 0.25 * f32::sin(w * ti);
        }
        t = (t + data.len() as f32 / SAMPLE_RATE as f32) % (2.0 * PI);
    };

    let stream = device.build_output_stream(
        &config,
        sine_samples,
        |e| panic!("{}", e),
        None,
    ).unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}
