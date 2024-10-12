use crate::prelude::v1::{Version, VO};

pub trait Entity: Eq + PartialEq + Send + Sync {
    type Id: Eq + PartialEq + VO + Send + Sync;

    type Event: Send;

    fn apply(&mut self, events: &[Self::Event]);

    fn apply_single(&mut self, event: Self::Event) {
        self.apply(&[event]);
    }

    fn id(&self) -> &Self::Id;

    fn version(&self) -> &Version;
}
