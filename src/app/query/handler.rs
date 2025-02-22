use yadi::{Container, Injectable};

#[derive(Debug)]
pub struct QueryHandler {}

impl Injectable for QueryHandler {
    fn from_container(container: &Container) -> Self {
        todo!()
    }
}
