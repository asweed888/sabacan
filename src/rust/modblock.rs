use crate::manifest::domain::model::entity::Manifest;
use yaml_rust::Yaml;
use std::path::PathBuf;

pub trait Handler<'a> {
    fn save_modblock(&self, manifest: &'a Manifest) -> anyhow::Result<()>;
    fn modblock(&self, manifest: &'a Manifest, path: &'a PathBuf) -> anyhow::Result<String>;
    fn upstream_modblock(&self, upstream: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn codefile_modblock(&self, codefile: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn modblock_pattern(&self, path: PathBuf) -> &str {
        if path.to_str().unwrap().to_string().contains("lib.rs") {
            r"pub mod[\s\S]*//.*Automatically exported by saba\."
        }
        else {
            r"mod[\s\S]*//.*Automatically exported by saba\."
        }
    }
}