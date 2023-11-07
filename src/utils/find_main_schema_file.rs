use teo_result::{Error, Result};
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn find_main_schema_file(file: Option<&str>, base_directory: &Path) -> Result<PathBuf> {
    if let Some(file) = file {
        let file_path = base_directory.join(file);
        if file_path.is_file() {
            return Ok(file_path);
        } else {
            return Err(Error::new(format!("cannot find schema file '{}'", file)));
        }
    }
    let default = vec!["schema.teo", "index.teo", "src/schema.teo", "src/index.teo", "schema/index.teo", "src/schema/index.teo"];
    for name in default {
        let file_path = base_directory.join(name);
        if file_path.is_file() {
            return Ok(file_path);
        }
    }
    Err(Error::new("cannot find default schema file"))
}

