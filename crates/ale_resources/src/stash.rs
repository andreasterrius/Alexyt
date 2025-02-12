use std::marker::PhantomData;
use log::info;
use ale_data::alevec::{AleVec, AleVecIter, AleVecIterMut, Key};
use crate::path::ResourcePath;

pub trait Load<Resource, Err> {
    fn load(&self, path: &str) -> Result<Vec<Resource>, Err>;
}

pub struct Stash<Resource, Err, Loader: Load<Resource, Err> + Default> {
    resources: AleVec<Resource>,
    loader: Loader,

    phantom_err: PhantomData<Err>,
}

impl<Resource, Err, Loader: Load<Resource, Err> + Default> Stash<Resource, Err, Loader> {
    pub fn new() -> Stash<Resource, Err, Loader> {
        return Stash {
            resources: AleVec::new(),
            loader: Loader::default(),
            phantom_err: Default::default(),
        };
    }

    pub fn load(&mut self, path: &str) -> Result<Vec<Key<Resource>>, Err> {
        let resource_path = &ResourcePath::find(path);
        let res = self.loader.load(resource_path)?;

        info!("load: {}", resource_path);

        let mut keys = vec!();
        for r in res {
            keys.push(self.resources.push(r));
        }
        Ok(keys)
    }

    pub fn register(&mut self, resource: Resource) -> Key<Resource> {
        self.resources.push(resource)
    }

    pub fn get(&mut self, key: Key<Resource>) -> Option<&Resource> {
        self.resources.get(key)
    }

    pub fn get_mut(&mut self, key: Key<Resource>) -> Option<&mut Resource> {
        self.resources.get_mut(key)
    }

    pub fn remove(&mut self, key: Key<Resource>) {
        self.resources.remove_no_drop(key)
    }

    pub fn iter(&self) -> AleVecIter<Resource> {
        self.resources.iter()
    }

    pub fn iter_mut(&mut self) -> AleVecIterMut<Resource> {
        self.resources.iter_mut()
    }
}