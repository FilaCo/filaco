use crate::v1::{Container, Injectable};

pub trait Entry<T: Injectable> {
    fn get(&self, container: &impl Container) -> &T;
}