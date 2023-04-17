use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::{context::Context, Component};

pub trait System: 'static + Send {
    fn run(&mut self, ctx: Context);
}

impl<F: Fn(Context) + 'static + Send> System for F {
    fn run(&mut self, ctx: Context) {
        self(ctx);
    }
}

pub struct World {
    num_entities: usize,
    pub(crate) component_vecs: HashMap<TypeId, Vec<Option<Box<dyn Component>>>>,
}

impl World {
    pub(crate) fn spawn_entity(&mut self) -> usize {
        let entity = self.num_entities;
        self.num_entities += 1;

        for component_vec in self.component_vecs.values_mut() {
            component_vec.push(None);
        }

        entity
    }

    pub(crate) fn add_component_to_entity(&mut self, entity: usize, component: Box<dyn Component>) {
        // If component vec already exists, just add the component at the entity index
        let type_id = component.get_type_id();
        if self.component_vecs.contains_key(&type_id) {
            let component_vec = self.component_vecs.get_mut(&type_id).unwrap();

            component_vec[entity] = Some(component);
            return;
        }

        // Create a new component vec and add it to world otherwise:
        let mut component_vec: Vec<Option<Box<dyn Component>>> =
            Vec::with_capacity(self.num_entities);
        for _ in 0..self.num_entities {
            component_vec.push(None);
        }
        component_vec[entity] = Some(component);
        self.component_vecs.insert(type_id, component_vec);
    }
}

// TODO: Add stages!!!
pub struct App {
    world: Arc<Mutex<World>>,
    systems: Vec<Box<dyn System>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(World {
                component_vecs: HashMap::new(),
                num_entities: 0,
            })),
            systems: vec![],
        }
    }

    pub fn add_system<F: System>(mut self, system: F) -> Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn run(self) {
        // TODO: Add scheduler w/ thread pool

        // Run each system in a separate thread
        let mut threads = vec![];
        for mut system in self.systems {
            let world = Arc::clone(&self.world);
            threads.push(thread::spawn(move || system.run(Context::new(world))));
        }

        for thread in threads {
            thread.join().expect("Unable to join thread");
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
