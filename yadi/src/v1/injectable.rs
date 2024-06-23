use crate::v1::Container;

pub trait Injectable: 'static {
    fn deps();
    fn from_container(container: &impl Container) -> Self;
}