use crate::components::Spawnable;
use crate::world::BoxEntity;
use ale_data::indexmap::Key;
use std::any::{Any, TypeId};

pub struct SpawnEvent {
  pub(crate) type_id: TypeId,
  pub(crate) entity: Box<dyn Any>,
  pub(crate) entity_key: Key<BoxEntity>,
}

impl SpawnEvent {
  pub fn new<T: 'static + Spawnable>(entity: T) -> SpawnEvent {
    let entity_key = entity.get_key();
    return SpawnEvent {
      type_id: TypeId::of::<T>(),
      entity: Box::new(entity),
      entity_key
    };
  }
}

pub struct KillEvent {
  pub(crate) entity_key: Key<BoxEntity>,
}

impl KillEvent {
  pub fn new(entity_key: Key<BoxEntity>) -> KillEvent {
    return KillEvent { entity_key };
  }
}
