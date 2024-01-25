use crate::manifest::domain::model::entity::Manifest;

pub trait ManifestRepository {
    fn load(&self) -> anyhow::Result<Manifest>;
}