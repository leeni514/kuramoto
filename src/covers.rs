use crate::storage::kuramoto_data_dir;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
///folders kuramoto keeps cached covers in, below the user's home folder
const COVER_CACHE_SUFFIXES: [&str; 3] = [
    "/appdata/roaming/kuramoto/covers",
    "/.local/share/kuramoto/covers",
    "/.var/app/io.github.leeni514.kuramoto/data/kuramoto/covers",
];
///returns the folder where cover cache is stored
pub fn cover_cache_dir() -> Result<PathBuf, io::Error> {
    let cache_dir = kuramoto_data_dir()?.join("covers");
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}
///cached covers are saved as just a file name, so a synced library works whatever the username or os is.
///turns full paths into any kuramoto's cover cache into the file name and leaves other paths alone
pub fn portable_cover_path(cover_path: &str) -> String {
    let cache_dir = kuramoto_data_dir().ok().map(|dir| dir.join("covers"));
    portable_cover_path_in(cover_path, cache_dir.as_deref())
}
fn portable_cover_path_in(cover_path: &str, cache_dir: Option<&Path>) -> String {
    //the path may come from another os, so both kinds of separator count
    let Some((folder, file_name)) = cover_path.rsplit_once(['/', '\\']) else {
        return cover_path.to_string();
    };
    let is_in_own_cache = cache_dir.is_some() && Path::new(cover_path).parent() == cache_dir;
    let folder = folder.replace('\\', "/").to_lowercase();
    let is_in_other_cache = COVER_CACHE_SUFFIXES
        .iter()
        .any(|suffix| folder.ends_with(suffix));
    if !file_name.is_empty() && (is_in_own_cache || is_in_other_cache) {
        file_name.to_string()
    } else {
        cover_path.to_string()
    }
}
///where a saved cover is on this computer
pub fn resolve_cover_path(cover_path: &str) -> PathBuf {
    let is_cached_file_name = !cover_path.is_empty() && !cover_path.contains(['/', '\\', ':']);
    match kuramoto_data_dir() {
        Ok(data_dir) if is_cached_file_name => data_dir.join("covers").join(cover_path),
        _ => PathBuf::from(cover_path),
    }
}
///downloads a cover into the cache dir, returns the file name to save as the cover path
pub async fn cache_cover_image(
    vn_id: u64,
    cover_url: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let cache_dir = cover_cache_dir()?;
    let file_name = format!("{vn_id}.jpg");
    let image_bytes = reqwest::get(cover_url)
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    fs::write(cache_dir.join(&file_name), image_bytes)?;
    Ok(file_name)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cached_covers_from_any_computer_become_file_names() {
        for path in [
            r"C:\Users\noah1\AppData\Roaming\kuramoto\covers\5.jpg",
            r"C:\Users\Noah\AppData\Roaming\kuramoto\covers\5.jpg",
            "/home/noah/.local/share/kuramoto/covers/5.jpg",
            "/home/noah/.var/app/io.github.leeni514.Kuramoto/data/kuramoto/covers/5.jpg",
            "5.jpg",
        ] {
            assert_eq!(portable_cover_path_in(path, None), "5.jpg");
        }
        //a data folder in an unusual place
        let cache_dir = std::env::temp_dir().join("custom").join("covers");
        let cover = cache_dir.join("5.jpg").to_string_lossy().to_string();
        assert_eq!(portable_cover_path_in(&cover, Some(&cache_dir)), "5.jpg");
    }
    #[test]
    fn covers_picked_from_elsewhere_are_kept() {
        for path in [
            r"D:\Pictures\my-cover.png",
            r"D:\Pictures\kuramoto\covers\my-cover.png",
            r"C:\Users\noah1\AppData\Roaming\kakera\covers\5.jpg",
            "/home/noah/Pictures/my-cover.png",
            "",
        ] {
            assert_eq!(portable_cover_path_in(path, None), path);
            assert_eq!(resolve_cover_path(path), PathBuf::from(path));
        }
    }
    #[test]
    fn file_names_resolve_into_the_cover_cache() {
        let expected = kuramoto_data_dir().unwrap().join("covers").join("5.jpg");
        assert_eq!(resolve_cover_path("5.jpg"), expected);
    }
}
