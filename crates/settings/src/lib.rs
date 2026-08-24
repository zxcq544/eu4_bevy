use bevy::ecs::resource::Resource;
use fonts::Fonts;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Resource)]
pub struct Settings {
    pub eu4_folder: String,
    pub monitor_index: usize,
    pub volume: f32,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub window_decorations: bool,
    pub initial_bootscreen_show_time: u64,
    pub main_loading_screen_show_time: u64,
    pub fonts: Fonts,
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
            monitor_index: 0,
            volume: 1.0,
            resolution_width: 1920,
            resolution_height: 1080,
            window_decorations: true,
            initial_bootscreen_show_time: 1,
            main_loading_screen_show_time: 1,
            fonts: Fonts {
                loading_screen_tooltip_font: "fonts/FiraSans-Bold.ttf".to_string(),
                loading_screen_loading_text_font: "fonts/FiraSans-Bold.ttf".to_string(),
                main_font: "fonts/FiraSans-Bold.ttf".to_string(),
                button_font: "fonts/FiraSans-Bold.ttf".to_string(),
            },
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
