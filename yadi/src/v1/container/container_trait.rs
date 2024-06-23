use crate::v1::{Builder, Injectable};

pub trait Container {
    fn builder() -> impl Builder;
    fn resolve<T: Injectable>(&self) -> &T;
}
