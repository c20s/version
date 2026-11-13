//! Human readable version scheme based on [SemVer](https://semver.org/).

/// A version with the form `major.minor.patch`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
}

impl Version {
    #[allow(dead_code)]
    pub(crate) fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn patch(&self) -> u64 {
        self.patch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_the_three_parts() {
        let v = Version::new(1, 2, 3);

        assert_eq!(v.major(), 1);
        assert_eq!(v.minor(), 2);
        assert_eq!(v.patch(), 3);
    }

    #[test]
    fn versions_with_the_same_parts_are_equal() {
        assert_eq!(Version::new(1, 2, 3), Version::new(1, 2, 3));
        assert_ne!(Version::new(1, 2, 3), Version::new(1, 2, 4));
    }

    #[test]
    fn version_is_clone_and_debug() {
        let v = Version::new(1, 2, 3);

        assert_eq!(v.clone(), v);
        assert!(!format!("{v:?}").is_empty());
    }
}
