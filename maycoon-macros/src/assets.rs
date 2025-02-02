use std::path::{Path, PathBuf};

pub fn temp_assets_folder() -> PathBuf {
    let path = std::env::temp_dir().join("maycoon-compilation-assets");

    if !path.exists() {
        std::fs::create_dir_all(&path).expect("failed to create static assets directory");
    }

    path
}

pub fn get_or_create_asset(path: &Path, name: &str, or_create: impl FnOnce() -> String) -> String {
    let asset_folder = temp_assets_folder().join(path);

    if !asset_folder.exists() {
        std::fs::create_dir_all(&asset_folder).expect("failed to create static assets directory");
    }

    let asset_path = asset_folder.join(name);

    if asset_path.exists() {
        std::fs::read_to_string(&asset_path).expect("failed to read static asset")
    } else {
        let data = or_create();

        std::fs::write(&asset_path, &data).expect("failed to write static asset");

        data
    }
}
