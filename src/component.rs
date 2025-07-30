use std::any::{Any, TypeId};

use crate::archetype::{Archetype, EntityId};

pub trait Component: Any + Send + Sync + 'static {}

impl<T: Any + Send + Sync + 'static> Component for T {}

pub trait ComponentBundle {
    fn type_ids(&self) -> Vec<TypeId>;
    fn insert_into(&self, archetype: &mut Archetype, entity_id: EntityId);
    fn register(&self, archetype: &mut Archetype);
}

macro_rules! impl_component_bundle {
    ($(($($T:ident),+)),+) => {
        $(
            #[allow(non_snake_case, non_camel_case_types)]
            impl<$($T),+> ComponentBundle for ($($T,)+)
            where
                $($T: Component + Clone),+
            {
                fn type_ids(&self) -> Vec<TypeId> {
                    vec![$(TypeId::of::<$T>()),+]
                }

                fn register(&self, archetype: &mut Archetype) {
                    $(archetype.register::<$T>();)+
                }

                fn insert_into(&self, archetype: &mut Archetype, entity: EntityId) {
                    archetype.entities.push(entity);
                    let ($($T,)+) = self;
                    $(archetype.get_mut::<$T>().unwrap().push($T.clone());)+
                }
            }
        )+
    };
}

impl_component_bundle!(
    (T1),
    (T1, T2),
    (T1, T2, T3),
    (T1, T2, T3, T4),
    (T1, T2, T3, T4, T5),
    (T1, T2, T3, T4, T5, T6),
    (T1, T2, T3, T4, T5, T6, T7),
    (T1, T2, T3, T4, T5, T6, T7, T8)
);
