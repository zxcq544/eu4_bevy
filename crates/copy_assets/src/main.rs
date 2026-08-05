use settings::Settings;
fn main() {
    // read settings.json file to settings struct
    let settings_filename = "./settings.json";
    if !std::path::Path::new(settings_filename).exists() {
        panic!(
            "❗  Settings file \'{0}\' doesn't exist, please create it",
            settings_filename
        );
    }
    // load settings from file
    let settings_file = std::fs::File::open(settings_filename).unwrap();
    let settings: Settings = serde_json::from_reader(settings_file).unwrap();
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
    let eu4_path_for_tar = settings.eu4_folder.replace('\\', "/");

    let eu4_assets_dirs = vec!["gfx/cursors", "gfx/loadingscreens"];
    let mut dirs_for_tar = Vec::new();
    for dir in eu4_assets_dirs {
        dirs_for_tar.push(format!("{}/{}", eu4_path_for_tar, dir));
    }
    println!("eu4_assets_dirs: {:?}", dirs_for_tar);

    println!("Hello, world!");
}
