//! Human readable version scheme based on [SemVer](https://semver.org/).

mod error;

use std::fmt;
use std::str::FromStr;

pub use error::ParseError;

/// A version with the form `major.minor.patch`.
///
/// ```
/// use version::Version;
///
/// let v: Version = "1.2.3".parse().unwrap();
/// assert_eq!(v.to_string(), "1.2.3");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<String>,
}

impl Version {
    pub(crate) fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
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

    pub fn pre_release(&self) -> Option<&str> {
        self.pre.as_deref()
    }
}

impl FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (core, pre) = match s.split_once('-') {
            Some((core, pre)) => (core, Some(pre)),
            None => (s, None),
        };

        let parts: Vec<&str> = core.split('.').collect();
        if parts.len() > 3 {
            return Err(ParseError::PartCount);
        }

        let mut numbers = [0; 3];
        for (slot, part) in numbers.iter_mut().zip(parts) {
            *slot = number(part)?;
        }

        let [major, minor, patch] = numbers;
        let mut version = Self::new(major, minor, patch);
        version.pre = pre.map(pre_release).transpose()?;
        Ok(version)
    }
}

fn pre_release(part: &str) -> Result<String, ParseError> {
    let valid = part
        .split('.')
        .all(|id| !id.is_empty() && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-'));

    if valid {
        Ok(part.to_owned())
    } else {
        Err(ParseError::InvalidPreRelease)
    }
}

fn number(part: &str) -> Result<u64, ParseError> {
    if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ParseError::InvalidNumber);
    }
    if part.len() > 1 && part.starts_with('0') {
        return Err(ParseError::LeadingZero);
    }
    part.parse().map_err(|_| ParseError::InvalidNumber)
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre {
            write!(f, "-{pre}")?;
        }
        Ok(())
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
