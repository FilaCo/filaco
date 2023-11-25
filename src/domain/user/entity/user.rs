use crate::domain::user::vo::user::{Id, Name};
use crate::util::ddd::{Aggregate, Entity};

pub(crate) struct User {
    id: Id,
    name: Name,
}

impl User {
    pub fn new(id: Id, name: Name) -> Self {
        Self { id, name }
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl Entity for User {
    type Id = ();

    fn id(&self) -> &Self::Id {
        todo!()
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Aggregate for User {}
