use std::{
    any::{Any, TypeId},
    collections::{BTreeSet, HashMap},
};

use crate::component::Component;

pub type EntityId = u32;

#[derive(Debug, Default)]
pub struct Archetype {
    components: HashMap<TypeId, Box<dyn Any>>,
    pub entities: Vec<EntityId>,
}

impl Archetype {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T: Component>(&mut self) {
        if !self.contains::<T>() {
            let type_id = TypeId::of::<T>();
            self.components.insert(type_id, Box::new(Vec::<T>::new()));
        }
    }

    /// Try to return vector of immutable `Components`.
    pub fn get<T: Component>(&self) -> Option<&Vec<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<Vec<T>>())
    }

    /// Try to return vector of mutable `Components`.
    pub fn get_mut<T: Component>(&mut self) -> Option<&mut Vec<T>> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<Vec<T>>())
    }

    /// Return true if the specified component has already been registered.
    pub fn contains<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct ArchetypeId {
    component_ids: BTreeSet<TypeId>,
}

impl ArchetypeId {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_slice(type_ids: &[TypeId]) -> Self {
        Self {
            component_ids: type_ids.iter().cloned().collect(),
        }
    }
}
