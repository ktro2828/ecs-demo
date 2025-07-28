use std::collections::HashMap;

use crate::{
    component::Component,
    entity::{ComponentStorage, EntityId},
};

#[derive(Debug, Default)]
pub struct World {
    components: HashMap<EntityId, ComponentStorage>,
    next_id: EntityId,
}

impl World {
    pub fn spawn(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.components.insert(id, ComponentStorage::new());
        id
    }

    pub fn entities(&self) -> Vec<EntityId> {
        let mut all = self.components.keys().cloned().collect::<Vec<_>>();
        all.sort();
        all
    }

    pub fn insert<T: Component>(&mut self, entity: EntityId, component: T) {
        if let Some(storage) = self.components.get_mut(&entity) {
            storage.insert(component);
        }
    }

    pub fn get<T: Component>(&self, entity: EntityId) -> Option<&T> {
        self.components
            .get(&entity)
            .and_then(|storage| storage.get())
    }

    pub fn get_mut<T: Component>(&mut self, entity: EntityId) -> Option<&mut T> {
        self.components
            .get_mut(&entity)
            .and_then(|storage| storage.get_mut())
    }
}
