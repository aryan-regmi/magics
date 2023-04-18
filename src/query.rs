use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use itertools::{concat, Itertools};

use crate::{prelude::World, Component};

#[derive(Debug)]
// Values stored by queries
pub struct Entity<'e> {
    components: Vec<&'e dyn Component>,
}

impl<'e> Entity<'e> {
    pub fn get<C: Component>(&'e self) -> Option<&C> {
        todo!()
    }

    pub fn get_mut<C: Component>(&'e mut self) -> Option<&mut C> {
        todo!()
    }
}

pub struct QueryBuilder {
    components: Vec<TypeId>,
    mutable_components: Vec<TypeId>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            components: vec![],
            mutable_components: vec![],
        }
    }

    pub fn with<T: Component>(mut self) -> Self {
        self.components.push(TypeId::of::<T>());
        self
    }

    pub fn with_mut<T: Component>(mut self) -> Self {
        self.mutable_components.push(TypeId::of::<T>());
        self
    }

    /// For all components in world, grab the ones that match the query, and create entities
    /// from them
    pub(crate) fn build<'e>(self, world: Arc<Mutex<World>>) -> Query<'e> {
        // Get lock on the world
        let world = world.lock().expect("World mutex has been poisoned");
        let num_components = self.components.len();

        // Get all components that have 'Some'
        let mut valid_component_vecs = vec![];
        let mut min_components = usize::max_value();
        for component in self.components {
            if world.component_vecs.contains_key(&component) {
                let component_vec = world.component_vecs.get(&component).unwrap();
                let some_components = component_vec
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.is_some())
                    .map(|(i, c)| (i, c.as_ref().unwrap()))
                    .collect::<Vec<_>>();

                if min_components > some_components.len() {
                    min_components = some_components.len();
                }

                valid_component_vecs.push(some_components);
            }
        }
        let valid_components = concat(valid_component_vecs)
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect::<Vec<_>>();

        let mut entities: Vec<Entity> = vec![];
        let mut entity_idx = 0;
        let mut last_index = usize::max_value();
        for (i, component) in valid_components {
            // If the component in valid_components belongs to the same entity as before, then add
            // the component to the entity
            if last_index == i {
                entities[entity_idx].components.push(&**component);
            } else {
                let mut entity = Entity { components: vec![] };
                entity.components.push(&**component);
                entities.push(entity);

                entity_idx += 1;
            }

            last_index = i;
        }

        let query_entities = entities
            .into_iter()
            .filter(|v| v.components.len() == num_components)
            .collect::<Vec<_>>();

        // Query {
        //     entities: query_entities,
        // }
        todo!()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Query<'q> {
    #[allow(unused)]
    entities: Vec<Entity<'q>>,
}

impl<'q> IntoIterator for Query<'q> {
    type Item = Entity<'q>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}
