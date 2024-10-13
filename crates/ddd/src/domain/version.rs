use crate::prelude::v1::VO;
use thiserror::Error;

pub const MIN_VERSION: u64 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
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

impl VO for Version {}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_version_from_zero_value_error_returned() {
        // arrange
        let value = 0u64;

        // act
        let result = Version::try_from(value);

        // assert
        assert!(result.is_err());
    }

    #[test]
    fn test_convert_version_from_non_zero_value_version_returned() {
        // arrange
        let value = 123u64;

        // act
        let result = Version::try_from(value);

        // assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_min_version_version_returned() {
        // arrange
        let expected =
            Version::try_from(1).expect("unable to init version with min value, update the test");

        // act
        let actual = Version::min();

        // assert
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_increment_version_bumped_version_returned() {
        // arrange
        let expected = Version::try_from(124)
            .expect("unable to init version with valid value, update the test");

        let start_version = Version::try_from(123)
            .expect("unable to init version with valid value, update the test");

        // act
        let actual = start_version.increment();

        // assert
        assert_eq!(expected, actual);
    }
}
