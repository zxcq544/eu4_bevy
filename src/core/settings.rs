use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Settings {
    pub eu4_folder: String,
}

pub fn get_eu4_settings() -> Settings {
    let settings_filename = "./settings.json";
    let steam_default_eu4_folder: &str =
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Europa Universalis IV";
    let settings: Settings = if std::path::Path::new(settings_filename).exists() {
        // Load settings from file
        let settings_file = std::fs::File::open(settings_filename).unwrap();
        serde_json::from_reader(settings_file).unwrap()
    } else {
        // Create settings file with default values
        println!(
            "ℹ️ Settings file \'{0}\' doesn't exist, creating default settings",
            settings_filename
        );
        let settings_file = std::fs::File::create(settings_filename).unwrap();
        let default_settings = Settings {
            eu4_folder: steam_default_eu4_folder.to_string(),
        };
        serde_json::to_writer_pretty(settings_file, &default_settings).unwrap();
        return default_settings;
    };
    // if eu4 folder from settings.json doesn't exist on disk - panic
    if !std::path::Path::new(&settings.eu4_folder).exists() {
        panic!(
            "❗  Europa Universalis 4 folder \'{0}\' doesn't exist on disk\nPlease set the correct path in settings.json",
            settings.eu4_folder
        );
    }
    println!(
        "✅ Europa Universalis 4 folder \'{0}\' exists on disk",
        settings.eu4_folder
    );
    settings
}
