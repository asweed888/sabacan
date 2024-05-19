use yaml_rust::Yaml;
use std::path::PathBuf;

pub trait ModblockHandler<'a> {
    fn save_modblock(&self) -> anyhow::Result<()>;
    fn modblock(&self, _path: &'a PathBuf) -> anyhow::Result<String> {
        Ok("modblock default".to_string())
    }
    fn upstream_modblock(&self, upstream: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn codefile_modblock(&self, codefile: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn modblock_pattern(&self) -> &str {
        r"pub mod[\s\S]*//.*Automatically exported by saba\."
    }
}