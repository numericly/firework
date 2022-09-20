use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    pub hardcore: bool,
    pub difficulty: u8,
}

impl GameSettings {
    pub fn from(path: String) -> GameSettings {
        match std::fs::read_to_string(&path) {
            Ok(file) => match serde_json::from_str::<GameSettings>(&file) {
                Ok(settings) => settings,
                Err(e) => panic!("Failed to parse settings file: {}", e),
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    std::fs::File::create(&path).unwrap();
                    let default_settings = GameSettings::default();
                    std::fs::write(path, serde_json::to_string(&default_settings).unwrap())
                        .unwrap();

                    default_settings
                }
                _ => panic!("Error opening file: {:?}", e),
            },
        }
    }
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            hardcore: false,
            difficulty: 0,
        }
    }
}
