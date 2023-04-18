use std::{
    any::TypeId,
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::{context::Context, Component, ComponentVec};

pub trait System: 'static + Send {
    fn run(&mut self, ctx: Context);
}

impl<F: Fn(Context) + 'static + Send> System for F {
    fn run(&mut self, ctx: Context) {
        self(ctx);
    }
}

pub(crate) struct World {
    num_entities: usize,
    pub(crate) component_vecs: HashMap<TypeId, Box<dyn ComponentVec>>,
}

impl World {
    pub(crate) fn spawn_entity(&mut self) -> usize {
        let entity = self.num_entities;
        self.num_entities += 1;

        for component_vec in self.component_vecs.values_mut() {
            component_vec.push_none();
        }

        entity
    }

    pub(crate) fn add_component_to_entity<C: Component>(&mut self, entity: usize, component: C) {
        // If component vec already exists, just add the component at the entity index
        let type_id = component.get_type_id();
        if self.component_vecs.contains_key(&type_id) {
            let component_vec = self
                .component_vecs
                .get_mut(&type_id)
                .unwrap()
                .as_any_mut()
                .downcast_mut::<Vec<Option<C>>>()
                .expect("Error downcasting component vector to Vec<Option<C>>");

            component_vec.push(Some(component));
            return;
        }

        // Create a new component vec and add it to world otherwise:
        let mut component_vec: Vec<Option<C>> = Vec::with_capacity(self.num_entities);
        for _ in 0..self.num_entities {
            component_vec.push(None);
        }
        component_vec[entity] = Some(component);
        self.component_vecs.insert(type_id, Box::new(component_vec));
    }
}

pub struct StageManager {
    stages: Vec<Stage>,
}

pub struct Stage {
    idx: usize,
    systems: Vec<Box<dyn System>>,
}

impl Stage {
    pub fn run_order(idx: usize) -> Self {
        Self {
            idx,
            systems: vec![],
        }
    }

    pub fn with<F: System>(mut self, system: F) -> Self {
        self.systems.push(Box::new(system));
        self
    }
}

pub struct App {
    world: Arc<Mutex<World>>,
    systems: Vec<Box<dyn System>>,
    stage_manager: StageManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: Arc::new(Mutex::new(World {
                component_vecs: HashMap::new(),
                num_entities: 0,
            })),
            systems: vec![],
            stage_manager: StageManager { stages: vec![] },
        }
    }

    pub fn add_system<F: System>(mut self, system: F) -> Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_stage(mut self, stage: Stage) -> Self {
        self.stage_manager.stages.push(stage);
        self
    }

    pub fn run(mut self) {
        // TODO: Add scheduler w/ thread pool & premptive work stealing queue

        // Run each system in a separate thread
        let mut threads = vec![];
        for mut system in self.systems {
            let world = Arc::clone(&self.world);
            threads.push(thread::spawn(move || system.run(Context::new(world))));
        }

        // Run each stage sequentially, but run all systems inside the stage in parallel
        let world = Arc::clone(&self.world);
        self.stage_manager.stages.sort_by(|a, b| a.idx.cmp(&b.idx));
        threads.push(thread::spawn(move || {
            for stage in self.stage_manager.stages {
                // Run each system in a separate thread
                let mut threads = vec![];
                for mut sys in stage.systems {
                    let world = Arc::clone(&world);
                    threads.push(thread::spawn(move || sys.run(Context::new(world))));
                }

                // NOTE: All systems in a stage are joined before running the next stage i.e the next
                // stage only runs after all the systems of the current stage are finished.
                for thread in threads {
                    thread.join().expect("Unable to join thread");
                }
            }
        }));

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
