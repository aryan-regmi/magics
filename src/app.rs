use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    context::Context,
    prelude::{Stage, StageManager, System, World},
};

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
