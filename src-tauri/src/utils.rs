use std::path::{Path, PathBuf};

use sysinfo::{DiskExt, RefreshKind, System, SystemExt};
use tauri::AppHandle;

pub fn cache_dir(handle: &AppHandle) -> PathBuf {
    let cache_dir = handle.path_resolver().app_cache_dir().unwrap();

    if !cache_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&cache_dir) {
            println!("{:?}", e);
        };
    }

    cache_dir
}

pub fn query_available_space(path: &str) -> u64 {
    let disk_path = Path::new(path);

    let mut system = System::new_with_specifics(RefreshKind::new().with_disks());
    system.refresh_disks_list();

    match system
        .disks()
        .iter()
        .find(|disk| disk_path.starts_with(disk.mount_point()))
    {
        Some(disk) => disk.available_space(),
        None => 0,
    }
}
