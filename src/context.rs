use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    prelude::{Query, QueryBuilder, World},
    Component,
};

#[derive(Default)]
pub struct EntityBuilder {
    components: HashMap<TypeId, Arc<dyn Component>>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.components
            .insert(TypeId::of::<T>(), Arc::new(component));
        self
    }
}

#[derive(Clone)]
pub struct Context {
    world: Arc<Mutex<World>>,
}

impl Context {
    pub(crate) fn new(world: Arc<Mutex<World>>) -> Self {
        Self { world }
    }

    pub fn spawn(&mut self, entity_builder: EntityBuilder) -> usize {
        // Get lock on the world
        let mut world = self.world.lock().expect("World mutex has been poisoned");

        let entity_id = world.spawn_entity();
        for (_, component) in entity_builder.components {
            world.add_component_to_entity(entity_id, component);
        }

        entity_id
    }

    pub fn query(&mut self, query_builder: QueryBuilder) -> Query {
        query_builder.build(Arc::clone(&self.world))
    }
}
