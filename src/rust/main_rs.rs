use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

pub fn path<'a>(path: &'a PathBuf) -> anyhow::Result<PathBuf> {
    let fpath1 = PathBuf::from(path.to_str().unwrap().to_string() + "/main.rs");
    let fpath2 = PathBuf::from(path.to_str().unwrap().to_string() + "/lib.rs");
    if fpath1.as_path().exists() {
        return Ok(fpath1)
    }

    if fpath2.as_path().exists() {
        return Ok(fpath2)
    }

    Ok(fpath1)
}

pub fn gen(path: PathBuf) -> anyhow::Result<()> {
    let path_exists = path.as_path().exists();
    let path = path.to_str().unwrap();

    println!("path: {}", path);

    if !path_exists {
        let mut file = File::create(path)?;
        file.write_all("".as_bytes())?;
    }
    Ok(())
}