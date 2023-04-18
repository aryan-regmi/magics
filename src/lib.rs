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
pub trait Component: 'static + Send + std::fmt::Debug + Sync {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
