use crate::manifest::domain::model::entity::Manifest;
use std::path::PathBuf;

pub trait Handler<'a> {
    fn write_modblock(&self, manifest: &'a Manifest);
    fn old_modblock(&self, manifest: Option<&'a Manifest>, path: Option<&'a PathBuf>);
    fn modblock(&self, manifest: Option<&'a Manifest>, path: Option<&'a PathBuf>);
    fn upstream_modblock(&self, manifest: Option<&'a Manifest>, path: Option<&'a PathBuf>);
    fn codefile_modblock(&self, manifest: Option<&'a Manifest>, path: Option<&'a PathBuf>);
}