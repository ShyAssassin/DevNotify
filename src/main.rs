mod config;

use ears::{Sound, AudioController};
use udev::{Event, EventType, MonitorBuilder};
use std::{error::Error, ffi::OsStr, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let config_path = std::env::var("DEVNOTIFY_CONFIG").unwrap_or_else(|_|
        "~/.config/devnotify/devnotify.ron".to_string()
    );

    let home = std::env::var("HOME").unwrap();
    println!("Config loading from: {}", config_path);
    let config_path = config_path.replace("~", &home);
    let config = config::Config::load(config_path.as_str());
    let connect_sound_path = config.connect_sound.replace("~", &home);
    let disconnect_sound_path = config.disconnect_sound.replace("~", &home);
    std::env::set_current_dir(Path::new(&config_path).parent().unwrap()).expect("Failed to set current directory");

    let mut connect_sound = Sound::new(&connect_sound_path)?;
    connect_sound.set_volume(config.volume as f32 / 100.0);
    let mut disconnect_sound = Sound::new(&disconnect_sound_path)?;
    disconnect_sound.set_volume(config.volume as f32 / 100.0);

    libnotify::init("USB Monitor")?;
    let monitor = MonitorBuilder::new()?
        .match_subsystem("usb")?
    .listen()?;

    loop {
        for event in monitor.iter() {
            if !event.is_initialized() {continue}
            if event.devnode().is_none() {continue}
            if event.event_type() != EventType::Add && event.event_type() != EventType::Remove {continue}

            print_info(&event);
            let devnode = event.devnode().unwrap_or(Path::new("Unknown"));
            let device_type = event.devtype().unwrap_or(OsStr::new("Unknown"));
            let product = event.property_value("ID_MODEL_FROM_DATABASE").unwrap_or(OsStr::new("Unknown"));
            let manufacturer = event.property_value("ID_VENDOR_FROM_DATABASE").unwrap_or(OsStr::new("Unknown"));

            let notification = config.notify_message.clone();
            let notification = notification.replace("${product}", &product.to_string_lossy());
            let notification = notification.replace("${devnode}", &devnode.to_string_lossy());
            let notification = notification.replace("${device_type}", &device_type.to_string_lossy());
            let notification = notification.replace("${manufacturer}", &manufacturer.to_string_lossy());

            if event.event_type() == EventType::Add {
                println!("Device added: {:?}", event.devnode());
                if config.notification {
                    libnotify::Notification::new("USB Device Added", Some(notification.as_str()), None)
                        .show()
                    .expect("Failed to show notification");
                }
                connect_sound.play();
            } else if event.event_type() == EventType::Remove {
                println!("Device removed: {:?}", event.devnode());
                if config.notification {
                    libnotify::Notification::new("USB Device Removed", Some(notification.as_str()), None)
                        .show()
                    .expect("Failed to show notification");
                }
                disconnect_sound.play();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn print_info(event: &Event) {
    println!("===============================");

    if let Some(devnode) = event.devnode() {
        println!("Device Node: {:?}", devnode);
    }
    if let Some(subsystem) = event.subsystem() {
        println!("Subsystem: {:?}", subsystem);
    }
    if let Some(devtype) = event.devtype() {
        println!("Device Type: {:?}", devtype);
    }
    if let Some(action) = event.property_value("ACTION") {
        println!("Action: {:?}", action);
    }
    if let Some(vendor) = event.property_value("ID_VENDOR_ID") {
        println!("Vendor ID: {}", vendor.to_string_lossy());
    }
    if let Some(product) = event.property_value("ID_MODEL_ID") {
        println!("Product ID: {}", product.to_string_lossy());
    }
    if let Some(manufacturer) = event.property_value("ID_VENDOR_FROM_DATABASE") {
        println!("Manufacturer: {}", manufacturer.to_string_lossy());
    }
    if let Some(product_name) = event.property_value("ID_MODEL_FROM_DATABASE") {
        println!("Product Name: {}", product_name.to_string_lossy());
    }
    if let Some(serial) = event.property_value("ID_SERIAL_SHORT") {
        println!("Serial Number: {}", serial.to_string_lossy());
    }
    println!("===============================\n");
}
