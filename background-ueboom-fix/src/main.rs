#![windows_subsystem = "windows"]

use cpal::traits::{DeviceTrait, HostTrait};
use rodio::{source::SineWave, OutputStream, Sink, Source};

fn main() {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    let target_device_name = "Headphones (WaraBoom)";

    let device = devices
        .filter(|d| d.name().unwrap() == target_device_name)
        .next();

    if let Some(device) = device {
        println!("Gerät gefunden: {}", device.name().unwrap());

        let (_stream, stream_handle) = OutputStream::try_from_device(&device).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = SineWave::new(20.0).amplify(0.00001);
        sink.append(source);

        sink.sleep_until_end();
    } else {
        println!("Gerät nicht gefunden: {}", target_device_name);
    }
}
