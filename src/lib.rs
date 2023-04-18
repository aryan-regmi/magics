use std::any::{Any, TypeId};

mod app;
mod context;
mod query;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::context::*;
    pub use crate::query::*;
    pub use crate::*;

    pub use std::any::Any;
}

// FIXME: Remove Debug
pub trait Component: 'static + Send + std::fmt::Debug {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

fn concrete_component<T: Component>(component: &dyn Component) -> Option<&T> {
    component.as_any().downcast_ref()
}

fn concrete_component_mut<T: Component>(component: &mut dyn Component) -> Option<&mut T> {
    component.as_any_mut().downcast_mut()
}
