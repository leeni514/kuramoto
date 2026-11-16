use crate::models::{AppSettings, VisualNovel};
use std::path::{Path, PathBuf};
use std::{fs, io};
///returns app data dir
pub fn kuramoto_data_dir() -> Result<PathBuf, io::Error> {
    let data_dir = match dirs::data_dir() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not find an app data directory",
            ));
        }
    };
    Ok(data_dir.join("kuramoto"))
}
/// Returns the path where Kuramoto stores the JSON file
pub fn library_file_path() -> Result<PathBuf, io::Error> {
    Ok(kuramoto_data_dir()?.join("library_json"))
}
///Loads the library from disk, returns empty if the files don't exist yet
pub fn load_library() -> Result<Vec<VisualNovel>, io::Error> {
    let library_file = library_file_path()?;
    let json_text = match fs::read_to_string(&library_file) {
        Ok(text) => text,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                return Ok(Vec::new());
            }
            return Err(error);
        }
    };
    let library = match serde_json::from_str::<Vec<VisualNovel>>(&json_text) {
        Ok(vns) => vns,
        Err(error) => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, error));
        }
    };
    Ok(library)
}
///saves the library to disk as a json
pub fn save_library(library: Vec<VisualNovel>) -> Result<(), io::Error> {
    let library_file = library_file_path()?;
    let library_dir = match library_file.parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "library file path has no parent directory",
            ));
        }
    };
    fs::create_dir_all(library_dir)?;
    let json_text = serde_json::to_string_pretty(&library)?;
    fs::write(library_file, json_text)?;
    Ok(())
}
///returns the path where settings are stored
pub fn settings_file_path() -> Result<PathBuf, io::Error> {
    Ok(kuramoto_data_dir()?.join("settings.json"))
}
///loads the settings
pub fn load_settings() -> Result<AppSettings, io::Error> {
    let settings_file = settings_file_path()?;
    let json_text = match fs::read_to_string(&settings_file) {
        Ok(text) => text,
        Err(error) => {
            if error.kind() == io::ErrorKind::NotFound {
                return Ok(AppSettings::default());
            }
            return Err(error);
        }
    };
    let settings = match serde_json::from_str::<AppSettings>(&json_text) {
        Ok(settings) => settings,
        Err(error) => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, error));
        }
    };
    Ok(settings)
}
///saves settings
pub fn save_settings(settings: AppSettings) -> Result<(), io::Error> {
    let settings_file = settings_file_path()?;
    let settings_dir = match settings_file.parent() {
        Some(path) => path,
        None => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Settings file path has no parent directory",
            ));
        }
    };
    fs::create_dir_all(settings_dir)?;
    let json_text = serde_json::to_string_pretty(&settings)?;
    fs::write(settings_file, json_text)?;
    Ok(())
}
///what the first-launch import brought across from kakera
pub struct KakeraImport {
    pub vn_count: usize,
}
///the folders kakera may have kept its data in
fn kakera_data_dirs() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(data_dir) = dirs::data_dir() {
        candidates.push(data_dir.join("kakera"));
    }
    //inside kuramoto's flatpak, data_dir is kuramoto's own sandbox, so also look where native and flatpak kakera keep theirs
    #[cfg(target_os = "linux")]
    if let Some(home_dir) = dirs::home_dir() {
        candidates.push(home_dir.join(".local/share/kakera"));
        candidates.push(home_dir.join(".var/app/xyz.majou.Kakera/data/kakera"));
    }
    candidates.dedup();
    candidates
}
///copies kakera's library (with its playtime), settings and cached covers the first time kuramoto runs,
///leaving kakera's own files untouched
pub fn import_kakera_data_if_new() -> Result<Option<KakeraImport>, io::Error> {
    let data_dir = kuramoto_data_dir()?;
    if data_dir.join("library_json").exists() {
        return Ok(None);
    }
    let Some(kakera_dir) = kakera_data_dirs()
        .into_iter()
        .find(|dir| dir.join("library_json").is_file())
    else {
        return Ok(None);
    };
    let vn_count = import_kakera_data(&kakera_dir, &data_dir)?;
    Ok(Some(KakeraImport { vn_count }))
}
///copies kakera's data folder into kuramoto's, returning how many vns came across
fn import_kakera_data(kakera_dir: &Path, data_dir: &Path) -> Result<usize, io::Error> {
    let json_text = fs::read_to_string(kakera_dir.join("library_json"))?;
    let mut library = serde_json::from_str::<Vec<VisualNovel>>(&json_text)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::create_dir_all(data_dir)?;
    //cached covers are saved as full paths into kakera's folder, so copy them over and point at the copies
    let kakera_covers_dir = kakera_dir.join("covers");
    let covers_dir = data_dir.join("covers");
    for vn in library.iter_mut() {
        let Some(cover_path) = vn.cover_path.clone().map(PathBuf::from) else {
            continue;
        };
        let Some(file_name) = cover_path.file_name() else {
            continue;
        };
        //covers the user picked from elsewhere stay where they are
        if !cover_path.starts_with(&kakera_covers_dir) {
            continue;
        }
        fs::create_dir_all(&covers_dir)?;
        let new_cover_path = covers_dir.join(file_name);
        if fs::copy(&cover_path, &new_cover_path).is_ok() {
            vn.cover_path = Some(new_cover_path.to_string_lossy().to_string());
        }
    }
    if let Ok(settings_text) = fs::read_to_string(kakera_dir.join("settings.json"))
        && let Ok(mut settings) = serde_json::from_str::<AppSettings>(&settings_text)
    {
        if settings.discord_idle_name == "Kakera" {
            settings.discord_idle_name = "Kuramoto".to_string();
        }
        fs::write(data_dir.join("settings.json"), serde_json::to_string_pretty(&settings)?)?;
    }
    //the library goes last, so an import that fails partway is tried again on the next launch
    fs::write(data_dir.join("library_json"), serde_json::to_string_pretty(&library)?)?;
    Ok(library.len())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PlaySession, VisualNovel};
    #[test]
    fn imports_kakera_library_with_playtime_and_covers() {
        let root = std::env::temp_dir().join(format!("kuramoto-import-test-{}", std::process::id()));
        let kakera_dir = root.join("kakera");
        let data_dir = root.join("kuramoto");
        fs::create_dir_all(kakera_dir.join("covers")).unwrap();
        let cached_cover = kakera_dir.join("covers").join("1.jpg");
        fs::write(&cached_cover, b"cover").unwrap();
        let picked_cover = root.join("my-cover.png");
        let mut cached = VisualNovel::new(1, "Cached cover".to_string());
        cached.cover_path = Some(cached_cover.to_string_lossy().to_string());
        cached.previous_playtime_seconds = 600;
        cached.play_sessions.push(PlaySession {
            vn_id: 1,
            started_at: "2026-11-01T20:00:00+00:00".to_string(),
            duration_seconds: 3600,
            notes: None,
        });
        let mut picked = VisualNovel::new(2, "Picked cover".to_string());
        picked.cover_path = Some(picked_cover.to_string_lossy().to_string());
        fs::write(kakera_dir.join("library_json"), serde_json::to_string(&vec![cached, picked]).unwrap()).unwrap();
        let mut settings = AppSettings::default();
        settings.discord_idle_name = "Kakera".to_string();
        fs::write(kakera_dir.join("settings.json"), serde_json::to_string(&settings).unwrap()).unwrap();

        assert_eq!(import_kakera_data(&kakera_dir, &data_dir).unwrap(), 2);

        let imported: Vec<VisualNovel> =
            serde_json::from_str(&fs::read_to_string(data_dir.join("library_json")).unwrap()).unwrap();
        assert_eq!(imported[0].total_playtime_seconds(), 4200);
        let new_cover = data_dir.join("covers").join("1.jpg");
        assert_eq!(imported[0].cover_path, Some(new_cover.to_string_lossy().to_string()));
        assert_eq!(fs::read(&new_cover).unwrap(), b"cover");
        assert_eq!(imported[1].cover_path, Some(picked_cover.to_string_lossy().to_string()));
        let imported_settings: AppSettings =
            serde_json::from_str(&fs::read_to_string(data_dir.join("settings.json")).unwrap()).unwrap();
        assert_eq!(imported_settings.discord_idle_name, "Kuramoto");
        //kakera's own files are left as they were
        assert!(cached_cover.exists());
        assert!(kakera_dir.join("library_json").exists());
        fs::remove_dir_all(&root).unwrap();
    }
}
