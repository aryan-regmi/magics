use std::{
    any::TypeId,
    sync::{Arc, Mutex},
};

use crate::{prelude::World, Component, ComponentVec};

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

    pub fn query(&mut self, query_builder: QueryBuilder) -> Query {
        query_builder.build()
    }
}

pub struct QueryBuilder {
    component_types: Vec<TypeId>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            component_types: vec![],
        }
    }

    pub fn with<T: Component>(mut self) -> Self {
        self.component_types.push(TypeId::of::<T>());
        self
    }

    fn build(self) -> Query {
        todo!()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Query {
    _component_vecs: Box<dyn ComponentVec>,
}

pub struct Entity {}

impl Entity {
    pub fn get_component<T>(&self) -> Option<T> {
        todo!()
    }
}

impl Iterator for Query {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct EntityBuilder {
    id: usize,
    ctx: Context,
}

impl EntityBuilder {
    // pub fn with<T: Component + Clone>(self, component: T) -> Self {
    pub fn with<T: Component>(self, component: T) -> Self {
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
