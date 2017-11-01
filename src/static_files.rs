use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> NamedFile {
    static_assets(PathBuf::from("/index.html")).expect("Should always be able to open /index.html")
}

#[get("/<path..>")]
pub fn static_assets(path: PathBuf) -> Option<NamedFile> {
    if path.starts_with("api") {
        None 
    } else {
        let asset_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("frontend").join("build");
        let full_path = asset_dir.join(path);
        let f = NamedFile::open(full_path)
            .unwrap_or_else(|_| NamedFile::open(asset_dir.join("index.html"))
            .expect("Unable to locate index.html"));
        Some(f)
    }
}