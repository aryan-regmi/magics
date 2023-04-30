use std::{any::TypeId, collections::HashMap};

use crate::{Component, ComponentVec};

pub(crate) struct World {
    pub(crate) num_entities: usize,
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
