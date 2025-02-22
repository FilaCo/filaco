use crate::Container;

pub type Factory<T> = fn(&Container) -> T;
