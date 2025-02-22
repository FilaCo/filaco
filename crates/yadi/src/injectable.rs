use crate::Container;

pub trait Injectable: 'static + Send + Sync {
    fn from_container(container: &Container) -> Self;
}
