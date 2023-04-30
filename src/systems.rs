use crate::prelude::Context;

pub trait System: 'static + Send {
    fn run(&mut self, ctx: Context);
}

impl<F: Fn(Context) + 'static + Send> System for F {
    fn run(&mut self, ctx: Context) {
        self(ctx);
    }
}

pub struct StageManager {
    pub(crate) stages: Vec<Stage>,
}

pub struct Stage {
    pub(crate) idx: usize,
    pub(crate) systems: Vec<Box<dyn System>>,
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
