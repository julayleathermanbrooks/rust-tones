use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    for c in device.supported_output_configs().unwrap() {
        println!("{:?}", c);
    }
}
