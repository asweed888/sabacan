use crate::manifest::domain::model::entity::Manifest;
use std::path::PathBuf;

pub trait Handler<'a> {
    fn write_modblock(&self, manifest: &'a Manifest);
    fn main_rs_path(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn old_modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn upstream_modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
    fn codefile_modblock(&self, manifest: &'a Manifest, path: &'a PathBuf);
}