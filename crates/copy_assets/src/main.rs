use path_clean::PathClean;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use tar::Builder;

use settings::Settings;

/// Loads settings from a JSON file.
fn load_settings<P: AsRef<Path>>(path: P) -> Result<Settings, io::Error> {
    let file = File::open(path)?;
    serde_json::from_reader(file).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Checks if the given EU4 folder exists and returns its normalized path.
fn validate_eu4_folder<P: AsRef<Path>>(eu4_folder: P) -> Result<PathBuf, io::Error> {
    let path = eu4_folder.as_ref();
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("❗ EU4 folder '{}' not found.", path.display()),
        ));
    }
    Ok(path.to_path_buf())
}

/// Archives the specified directories into a single TAR file.
fn create_tar_archive<P: AsRef<Path>>(
    archive_path: P,
    base_folder: P,
    subdirs: &[&str],
) -> Result<(), io::Error> {
    let file = File::create(archive_path)?;
    let mut archive = Builder::new(file);

    // TAR format requires paths to be separated by '/' (POSIX standard).
    // Each subdirectory is added relative to the archive root.
    let base = base_folder.as_ref();

    let total_dirs = subdirs.len();
    println!("📦 Total directories to process: {}", total_dirs);

    for (index, subdir) in subdirs.iter().enumerate() {
        let current_step = index + 1;
        let remaining = total_dirs - current_step + 1;
        let full_path = base.join(subdir).clean();

        if full_path.exists() && full_path.is_dir() {
            println!(
                "[{}/{}] Adding to archive: {} (Folders left: {})",
                current_step,
                total_dirs,
                full_path.display(),
                remaining
            );

            // append_dir_all recursively adds the folder.
            // Explicitly force forward slashes for the archive internal path format.
            let archive_name = subdir.replace('\\', "/");
            archive.append_dir_all(archive_name, &full_path)?;
        } else {
            println!(
                "⚠️ [{}/{}] Warning: Path '{}' does not exist or is not a directory (Folders left: {})",
                current_step,
                total_dirs,
                full_path.display(),
                remaining
            );
        }
    }

    // Finalize the archive write operation
    archive.finish()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_settings("./settings.json")?;
    let eu4_path = validate_eu4_folder(&settings.eu4_folder)?;

    let eu4_assets_subdirs = vec!["gfx/cursors", "gfx/loadingscreens"];

    // Create the assets.tar archive in the current directory
    let output_tar = "assets.tar";
    println!("🚀 Starting archiving process to {}...", output_tar);

    create_tar_archive(output_tar, &eu4_path.to_str().unwrap(), &eu4_assets_subdirs)?;

    println!("✨ Archiving completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tar::Archive;

    // Helper function to create an isolated temporary directory for a test
    fn create_temp_dir(test_name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("eu4_tar_tests_{}", test_name));
        if path.exists() {
            fs::remove_dir_all(&path).unwrap();
        }
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn test_load_settings_valid_json() {
        let temp_dir = create_temp_dir("valid_json");
        let json_path = temp_dir.join("settings.json");

        let mut file = File::create(&json_path).unwrap();
        file.write_all(b"{\"eu4_folder\": \"/mock/path\"}").unwrap();

        let settings = load_settings(&json_path);
        assert!(settings.is_ok());
        assert_eq!(settings.unwrap().eu4_folder, "/mock/path");
    }

    #[test]
    fn test_load_settings_missing_file() {
        let res = load_settings("this_file_does_not_exist.json");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_validate_eu4_folder_exists() {
        let temp_dir = create_temp_dir("folder_exists");
        let res = validate_eu4_folder(&temp_dir);
        assert!(res.is_ok());

        // canonicalize resolves to strict absolute paths
        assert_eq!(res.unwrap(), temp_dir.to_path_buf());
    }

    #[test]
    fn test_validate_eu4_folder_missing() {
        let temp_dir = create_temp_dir("folder_missing");
        let fake_path = temp_dir.join("non_existent_folder");

        let res = validate_eu4_folder(&fake_path);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn test_create_tar_archive_success_and_internal_paths() {
        let temp_dir = create_temp_dir("tar_success");

        // Mocking the EU4 folder structure
        let mock_eu4 = temp_dir.join("EU4");
        let cursor_dir = mock_eu4.join("gfx/cursors");
        let loading_dir = mock_eu4.join("gfx/loadingscreens");

        fs::create_dir_all(&cursor_dir).unwrap();
        fs::create_dir_all(&loading_dir).unwrap();

        // Put a dummy file inside one of the directories
        let dummy_file_path = cursor_dir.join("pointer.png");
        let mut file = File::create(&dummy_file_path).unwrap();
        file.write_all(b"fake_png_data").unwrap();

        let output_tar = temp_dir.join("test_output.tar");
        let subdirs = vec!["gfx/cursors", "gfx/loadingscreens"];

        // Run archiving
        let result = create_tar_archive(&output_tar, &mock_eu4, &subdirs);
        assert!(result.is_ok());
        assert!(output_tar.exists());

        // Read the produced TAR file back to verify internal paths are POSIX standard
        let tar_file = File::open(&output_tar).unwrap();
        let mut archive = Archive::new(tar_file);

        let mut found_file = false;

        for entry in archive.entries().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().unwrap();

            // The tar crate cross-platform path API yields standard paths.
            // Converting to a string representation via components ensures we check POSIX rules.
            let path_str: Vec<String> = path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().into_owned())
                .collect();
            let joined_path = path_str.join("/");

            if joined_path.contains("gfx/cursors/pointer.png") {
                found_file = true;
                // Verify no explicit OS-specific backslashes leaked into the path string
                assert!(!joined_path.contains('\\'));
            }
        }

        assert!(
            found_file,
            "The file 'gfx/cursors/pointer.png' was not found inside the archive!"
        );
    }
}
