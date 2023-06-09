use std::any::{Any, TypeId};

mod app;
mod context;
mod query;
mod systems;
mod world;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::context::*;
    pub use crate::query::*;
    pub use crate::systems::*;
    pub use crate::*;

    pub(crate) use crate::world::*;

    pub use std::any::Any;
}

// FIXME: Remove Debug
pub trait Component: 'static + Send + std::fmt::Debug + Sync {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    // fn as_any(&self) -> &dyn Any;
    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

// FIX: Remove Debug
pub(crate) trait ComponentVec: Send + std::fmt::Debug {
    fn push_none(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// impl<T: Component + Clone> ComponentVec for Vec<Option<T>> {
impl<T: Component> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
