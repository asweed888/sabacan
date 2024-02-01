use crate::manifest::domain::model::entity::Manifest;
use std::path::PathBuf;

pub trait Handler<'a> {
    fn write_modblock(&self, manifest: &'a Manifest);
    fn modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn upstream_modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn codefile_modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn old_modblock(&self, path: &'a PathBuf) -> &str {
        if path.to_str().unwrap().to_string().contains("lib.rs") {
            r"pub mod[\s\S]*//.*Automatically exported by saba\."
        }
        else {
            r"mod[\s\S]*//.*Automatically exported by saba\."
        }
    }
}