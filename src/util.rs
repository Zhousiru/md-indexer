use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

pub fn get_key(path: &str, base_path: &str) -> Result<String, Box<dyn Error>> {
    let full_path = fs::canonicalize(&path)?;
    let full_base = fs::canonicalize(&base_path)?;

    let mut ret = full_path
        .strip_prefix(full_base)?
        .to_string_lossy()
        .to_string()
        // Convert slash.
        .replace(r"\", "/");

    // Prepend slash.
    ret.insert_str(0, "/");

    Ok(ret)
}

pub fn get_path_by_key(key: &str, base_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let relative_path = key.strip_prefix("/").ok_or("invalid key")?;
    Ok(Path::new(base_path).join(relative_path))
}
