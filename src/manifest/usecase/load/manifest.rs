use crate::manifest::domain::model::entity::Manifest;
use crate::manifest::domain::repository::ManifestRepository;

pub struct LoadManifestUseCase<R>
where
    R: ManifestRepository,
{
    pub repository: R,
}


impl<R> LoadManifestUseCase<R>
where
    R: ManifestRepository,
{
    pub fn new(repository: R) -> Self {
        Self{ repository }
    }
    pub fn load(&self) -> anyhow::Result<Manifest> {
        self.repository.load()
    }
}