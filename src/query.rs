use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use crate::{concrete_component, concrete_component_mut, prelude::World, Component};

// Values stored by queries
pub struct Entity<'e> {
    components: Vec<&'e dyn Component>,
    mutable_components: Vec<&'e mut dyn Component>,
}

impl<'e> Entity<'e> {
    pub fn get<C: Component>(&'e self) -> Option<&C> {
        for component in &self.components {
            if component.get_type_id() == TypeId::of::<C>() {
                let concerete = concrete_component::<C>(*component);
                return concerete;
            }
        }

        None
    }

    pub fn get_mut<C: Component>(&'e mut self) -> Option<&mut C> {
        for component in &mut self.mutable_components {
            if component.get_type_id() == TypeId::of::<C>() {
                let concerete = concrete_component_mut::<C>(*component);
                return concerete;
            }
        }

        None
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

        // Get all components that have 'Some'
        let mut immutable_components = vec![];
        for component in self.components {
            if world.component_vecs.contains_key(&component) {
                // Grab all `Some` components from each of the vectors
                let component_vec = world.component_vecs.get(&component).unwrap();
                let mut filtered_components = component_vec
                    .iter()
                    .filter_map(|c| c.as_ref())
                    .collect::<Vec<_>>();
                immutable_components.append(&mut filtered_components);
            }
        }

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
