use crate::domain::user::Error;
use crate::util::ddd::VO;
use uuid::Uuid;

pub(crate) struct Id(Uuid);

impl Clone for Id {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TryFrom<Uuid> for Id {
    type Error = Error;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl VO<Uuid> for Id {
    type ValueError = Error;

    fn value(&self) -> &Uuid {
        &self.0
    }

    fn validate(_value: &Uuid) -> Result<(), Self::ValueError> {
        Ok(())
    }
}
