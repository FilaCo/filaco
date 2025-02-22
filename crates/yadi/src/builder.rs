use crate::{Container, Entry, Injectable};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use thiserror::Error;

#[derive(Debug)]
pub struct Builder {
    entries: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn register_lazy<T: Injectable>(self) -> Self {
        self.register(Entry::Lazy {
            factory: T::from_container,
            instance: OnceLock::new(),
        })
    }

    pub fn register_transient<T: Injectable>(self) -> Self {
        self.register(Entry::Transient(T::from_container))
    }

    fn register<T: Injectable>(mut self, entry: Entry<T>) -> Self {
        self.entries.insert(TypeId::of::<T>(), Arc::new(entry));

        self
    }

    pub fn build(self) -> Result<Container, BuildError> {
        // TODO: add validation
        Ok(Container::new(self.entries))
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum BuildError {}
