use yaml_rust::Yaml;
use crate::manifest::domain::model::lang::Lang;
use crate::manifest::domain::model::arch::Arch;
use crate::manifest::domain::model::root::Root;

#[derive(Debug)]
pub struct Manifest {
    pub lang: Lang,
    pub arch: Arch,
    pub root: Root,
    pub spec: Vec<Yaml>,
}