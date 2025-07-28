use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::component::Component;

pub type EntityId = u32;

#[derive(Debug, Default)]
pub(crate) struct ComponentStorage {
    storage: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ComponentStorage {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn insert<T: Component>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.storage.insert(type_id, Box::new(component));
    }

    pub(crate) fn get<T: Component>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.storage
            .get(&type_id)
            .and_then(|boxed| boxed.downcast_ref())
    }

    pub(crate) fn get_mut<T: Component>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.storage
            .get_mut(&type_id)
            .and_then(|boxed| boxed.downcast_mut())
    }
}
