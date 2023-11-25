use crate::domain::user::Error;
use crate::util::ddd::VO;

pub(crate) struct Name(String);

impl Name {
    const MAX_LEN: usize = 30;

    fn max_len() -> usize {
        Self::MAX_LEN
    }
}

impl Clone for Name {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl TryFrom<String> for Name {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value).and_then(|| Ok(Self(value)))
    }
}

impl VO<String> for Name {
    type ValueError = Error;

    fn value(&self) -> &String {
        &self.0
    }

    fn validate(value: &String) -> Result<(), Self::ValueError> {
        if value.len() > Self::max_len() {
            Err(Error::UserNameTooLong)
        }

        Ok(())
    }
}
