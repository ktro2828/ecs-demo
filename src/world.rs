use std::collections::HashMap;

use crate::{
    archetype::{Archetype, ArchetypeId, EntityId},
    component::{Component, ComponentBundle},
};

#[derive(Debug, Default)]
pub struct World {
    pub archetypes: HashMap<ArchetypeId, Archetype>,
    entity_to_archetype: HashMap<EntityId, ArchetypeId>,
    next_id: EntityId,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn entity and insert to `Archetype`
    pub fn spawn<B: ComponentBundle>(&mut self, bundle: B) -> EntityId {
        let type_ids = bundle.type_ids();
        let archetype_id = ArchetypeId::from_slice(&type_ids);

        let archetype = self
            .archetypes
            .entry(archetype_id.clone())
            .or_insert_with(|| {
                let mut arch = Archetype::new();
                bundle.register(&mut arch);
                arch
            });

        let entity_id = self.next_id;
        self.next_id += 1;

        bundle.insert_into(archetype, entity_id);
        self.entity_to_archetype.insert(entity_id, archetype_id);
        entity_id
    }

    pub fn entities(&self) -> Vec<EntityId> {
        self.entity_to_archetype.keys().cloned().collect()
    }

    pub fn get_component<T: Component>(&self, entity_id: EntityId) -> Option<&T> {
        if let Some(archetype) = self
            .entity_to_archetype
            .get(&entity_id)
            .and_then(|aid| self.archetypes.get(aid))
        {
            let index = archetype.entities.iter().position(|&id| id == entity_id)?;
            archetype.get::<T>().and_then(|d| d.get(index))
        } else {
            None
        }
    }
}
