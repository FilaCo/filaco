use yadi::{Container, Injectable};

#[derive(Debug)]
pub struct CommandHandler {}

impl Injectable for CommandHandler {
    fn from_container(container: &Container) -> Self {
        todo!()
    }
}
