use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub eu4_folder: String,
}

pub fn get_eu4_settings() -> Settings {
    let settings_file = "./settings.json";
    let settings: Settings = if std::path::Path::new(settings_file).exists() {
        // Load settings from file
        let settings_file = std::fs::File::open(settings_file).unwrap();
        serde_json::from_reader(settings_file).unwrap()
    } else {
        // Create settings file
        let settings_file = std::fs::File::create(settings_file).unwrap();
        serde_json::to_writer_pretty(
            settings_file,
            &Settings {
                eu4_folder:
                    "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Europa Universalis IV"
                        .to_string(),
            },
        )
        .unwrap();
        Settings {
            eu4_folder: "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Europa Universalis IV"
                .to_string(),
        }
    };
    // if eu4 folder from settings.json doesn't exist on disk - panic
    if !std::path::Path::new(&settings.eu4_folder).exists() {
        panic!(
            "Europa Universalis 4 folder \'{0}\' doesn't exist on disk\nPlease set the correct path in settings.json",
            settings.eu4_folder
        );
    }
    println!(
        "Found Europa Universalis 4 folder at {}",
        settings.eu4_folder
    );
    settings
}
