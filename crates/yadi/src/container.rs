use crate::{Builder, Entry, Injectable};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Container {
    entries: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn resolve<T: Injectable>(&self) -> Arc<T> {
        self.entries
            .get(&TypeId::of::<T>())
            .expect("unable to resolve tag")
            .downcast_ref::<Entry<T>>()
            .expect("oopsie, wrong type associated with tag")
            .get(self)
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub(crate) fn new(entries: HashMap<TypeId, Arc<dyn Any + Send + Sync>>) -> Self {
        Self { entries }
    }
}
