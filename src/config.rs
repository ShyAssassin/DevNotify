use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub volume: u32,
    pub notification: bool,
    pub connect_sound: String,
    pub notify_message: String,
    pub disconnect_sound: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            volume: 100,
            notification: true,
            connect_sound: "connect.wav".to_string(),
            disconnect_sound: "disconnect.wav".to_string(),
            notify_message: "Device: ${devnode}".to_string(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Self {
        let home = std::env::var("HOME").unwrap();
        let path = path.replace("~", &home);
        let path = Path::new(&path);

        let config: Config = match std::fs::read_to_string(&path) {
            Ok(content) => {
                match ron::de::from_str(&content) {
                    Ok(config) => config,
                    Err(_) => {
                        println!("Invalid config file, using default config");
                        Config::default()
                    }
                }
            }
            Err(_) => {
                println!("Config file not found, using default config");
                std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
                Config::default()
            }
        };

        let config_str = ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::new()).unwrap();
        std::fs::write(path, config_str).unwrap();
        return config
    }
}
