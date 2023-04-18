use std::sync::{Arc, Mutex};

use crate::{prelude::World, Component};

pub struct EntityBuilder {
    id: usize,
    ctx: Context,
}

impl EntityBuilder {
    pub fn with<T: Component + Clone>(self, component: T) -> Self {
        self.ctx
            .world
            .lock()
            .expect("World mutex has been poisoned")
            .add_component_to_entity(self.id, component);
        self
    }

    pub fn build(self) -> usize {
        self.id
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

    pub fn spawn(&mut self) -> EntityBuilder {
        EntityBuilder {
            id: self
                .world
                .lock()
                .expect("World mutex has been poisoned")
                .spawn_entity(),
            ctx: self.clone(),
        }
    }

    // pub fn query(&mut self, query_builder: QueryBuilder) -> Query {
    //     query_builder.build(Arc::clone(&self.world))
    // }
}
