use std::thread;

pub mod prelude {
    pub use crate::*;
}

pub trait System: 'static + Send {
    fn run(&mut self, ctx: Context);
}

impl<F: Fn(Context) + 'static + Send> System for F {
    fn run(&mut self, ctx: Context) {
        self(ctx);
    }
}

pub struct App {
    systems: Vec<Box<dyn System>>,
}

impl App {
    pub fn new() -> Self {
        Self { systems: vec![] }
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
            threads.push(thread::spawn(move || system.run(Context::new())));
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

#[derive(Clone)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

