use yaml_rust::Yaml;
use std::path::PathBuf;

pub trait ModblockHandler<'a> {
    fn save_modblock(&self) -> anyhow::Result<()>;
    fn modblock(&self, path: &'a PathBuf) -> anyhow::Result<String>;
    fn upstream_modblock(&self, upstream: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn upstream_modblock_with_path(&self, _upstream: &Vec<Yaml>, _mod_block: &mut String, _tabs: &'a str, _path: &'a PathBuf) -> anyhow::Result<()> {
        Ok(())
    }
    fn codefile_modblock(&self, codefile: &Vec<Yaml>, mod_block: &mut String, tabs: &'a str) -> anyhow::Result<()>;
    fn modblock_pattern(&self, path: &'a PathBuf) -> &str {
        let path = path.to_str().unwrap();
        if path.to_string().contains("lib.rs") {
            r"pub mod[\s\S]*//.*Automatically exported by saba\."
        }
        else {
            r"mod[\s\S]*//.*Automatically exported by saba\."
        }
    }
}