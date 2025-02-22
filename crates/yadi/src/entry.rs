use crate::{Container, Factory, Injectable};
use std::sync::{Arc, OnceLock};

#[derive(Debug)]
pub enum Entry<T: Injectable> {
    Transient(Factory<T>),
    Lazy {
        instance: OnceLock<Arc<T>>,
        factory: Factory<T>,
    },
}

impl<T: Injectable> Entry<T> {
    pub fn get(&self, container: &Container) -> Arc<T> {
        match self {
            Entry::Transient(factory) => Arc::new(factory(container)),
            Entry::Lazy { instance, factory } => instance
                .get_or_init(|| Arc::new(factory(container)))
                .clone(),
        }
    }
}
