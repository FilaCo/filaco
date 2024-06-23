use std::marker::PhantomData;
use std::sync::Arc;

use crate::v1::{Container, Entry, Injectable};

pub struct Transient<T: Injectable> {
    _m: PhantomData<T>,
}

impl<T: Injectable> Entry<T> for Transient<T> {
    fn get(&self, container: &impl Container) -> Arc<T> {
        Arc::new(T::from_container(container))
    }
}