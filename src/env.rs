use std::path::{Component, Path, PathBuf};

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref SHARE: PathBuf = Path::new(&Component::RootDir)
        .join("usr")
        .join("share")
        .join(NAME);
}

#[cfg(not(debug_assertions))]
pub fn etc() -> PathBuf {
    Path::new(&Component::RootDir).join("etc").join(NAME)
}

#[cfg(debug_assertions)]
pub fn etc() -> PathBuf {
    Path::new(".etc").to_path_buf()
}

#[cfg(not(debug_assertions))]
pub fn var() -> PathBuf {
    Path::new(&Component::RootDir).join("var").join(NAME)
}

#[cfg(debug_assertions)]
pub fn var() -> PathBuf {
    Path::new(".var").to_path_buf()
}

#[cfg(not(debug_assertions))]
pub fn third() -> PathBuf {
    SHARE.join("node_modules")
}

#[cfg(debug_assertions)]
pub fn third() -> PathBuf {
    Path::new("node_modules").to_path_buf()
}
