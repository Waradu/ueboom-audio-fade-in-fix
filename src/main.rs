#![windows_subsystem = "windows"] // Entfernt das Terminal-Fenster

use cpal::traits::{DeviceTrait, HostTrait};
use rodio::{source::SineWave, OutputStream, Sink, Source};
use tray_icon::Icon;
use std::process::{exit, Command};
use std::sync::mpsc::channel;
use std::thread;
use tray_icon::{
    menu::{Menu, MenuItem},
    TrayIconBuilder,
};

fn main() {
    let (tx, rx) = channel();

    let menu = Menu::new();
    let restart_item = MenuItem::new("Restart", true, None);
    let exit_item = MenuItem::new("Exit", true, None);

    menu.append(&restart_item);
    menu.append(&exit_item);

    let icon = Icon::from_rgba(vec![100, 100, 100], 32, 32).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()
        .unwrap();

    // Doppelklick-Handling entfernt, da `set_on_double_click` in dieser Version nicht existiert

    // Starte den Audio-Code in einem Thread
    thread::spawn(|| {
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
    });

    // Event-Loop für Tray-Aktionen
    for message in rx {
        match message {
            "restart" => {
                Command::new(std::env::current_exe().unwrap())
                    .spawn()
                    .unwrap();
                exit(0); // Beendet das aktuelle Programm
            }
            "exit" => {
                exit(0); // Beendet das Programm
            }
            _ => (),
        }
    }
}
