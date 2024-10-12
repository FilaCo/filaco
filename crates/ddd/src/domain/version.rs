use crate::prelude::v1::VO;
use thiserror::Error;

pub const MIN_VERSION: u64 = 1;

#[derive(Debug, Eq, PartialEq)]
pub struct Version(u64);

#[derive(Debug, Error)]
pub enum VersionError {
    #[error("Unable to init version with zero value")]
    ZeroVersion,
}

impl Version {
    pub fn min() -> Self {
        Self(MIN_VERSION)
    }

    pub fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl VO for Version {
    type Error = VersionError;
}

impl TryFrom<u64> for Version {
    type Error = VersionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < MIN_VERSION {
            Err(VersionError::ZeroVersion)
        } else {
            Ok(Self(value))
        }
    }
}
