use std::fmt;

use crate::{Change, Version};

#[derive(Debug, Clone, Default)]
pub struct NextOptions {
    change: Option<Change>,
}

impl NextOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change(mut self, change: Change) -> Self {
        self.change = Some(change);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NextError {
    Overflow,
}

impl fmt::Display for NextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NextError::Overflow => write!(f, "a version part is too large to increment"),
        }
    }
}

impl std::error::Error for NextError {}

impl Version {
    /// Generates the next version from `self`, the previous version.
    ///
    /// Without a change the default is [`Change::Feature`]. Releasing from a pre-release
    /// without a change drops the pre-release and keeps the core.
    ///
    /// ```
    /// use version::{Change, NextOptions, Version};
    ///
    /// let previous = Version::parse_loose("v1.2.3").unwrap();
    ///
    /// let next = previous.next(&NextOptions::new()).unwrap();
    /// assert_eq!(next.to_string(), "1.3.0");
    ///
    /// let breaking = previous.next(&NextOptions::new().change(Change::Breaking)).unwrap();
    /// assert_eq!(breaking.to_string(), "2.0.0");
    /// ```
    pub fn next(&self, options: &NextOptions) -> Result<Version, NextError> {
        match (&self.pre, options.change) {
            (Some(_), None) => Ok(Version::new(self.major, self.minor, self.patch)),
            (_, change) => self
                .bump(change.unwrap_or(Change::Feature))
                .ok_or(NextError::Overflow),
        }
    }
}
