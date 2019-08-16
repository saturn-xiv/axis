use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::{errors::Result, orm::open as open_db};

pub fn start<P: AsRef<Path>>(etc: P, var: P) -> Result<()> {
    let etc = etc.as_ref();
    let var = var.as_ref();
    debug!(
        "load etc from {}, var from {}",
        etc.display(),
        var.display()
    );
    let db = open_db(&var.join("db").display().to_string())?;
    Ok(())
}
