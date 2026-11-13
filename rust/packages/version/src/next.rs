use std::fmt;

use crate::{build_metadata, pre_release, Change, Version};

#[derive(Debug, Clone, Default)]
pub struct NextOptions {
    change: Option<Change>,
    pre: Option<String>,
    build: Option<String>,
}

impl NextOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn change(mut self, change: Change) -> Self {
        self.change = Some(change);
        self
    }

    pub fn pre(mut self, pre: &str) -> Self {
        self.pre = Some(pre.to_owned());
        self
    }

    pub fn build(mut self, build: &str) -> Self {
        self.build = Some(build.to_owned());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NextError {
    Overflow,
    InvalidPreRelease,
    ChangeOnPreRelease,
    InvalidBuildMetadata,
}

impl fmt::Display for NextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NextError::Overflow => write!(f, "a version part is too large to increment"),
            NextError::InvalidPreRelease => write!(f, "the requested pre-release is not valid"),
            NextError::InvalidBuildMetadata => {
                write!(f, "the requested build metadata is not valid")
            }
            NextError::ChangeOnPreRelease => write!(
                f,
                "the version is locked while on a pre-release, an explicit change is not allowed"
            ),
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
    /// A requested pre-release is either a label (`beta`), which is numbered, or explicit
    /// (`beta.5`), which is used as given. While staying on a pre-release the core is
    /// locked, so an explicit change is an error.
    ///
    /// Build metadata is never carried over from the previous version, it is only set
    /// when requested.
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
    ///
    /// let beta = previous.next(&NextOptions::new().pre("beta")).unwrap();
    /// assert_eq!(beta.to_string(), "1.3.0-beta.1");
    ///
    /// let beta2 = beta.next(&NextOptions::new().pre("beta")).unwrap();
    /// assert_eq!(beta2.to_string(), "1.3.0-beta.2");
    ///
    /// let tagged = previous.next(&NextOptions::new().build("sha.5114f85")).unwrap();
    /// assert_eq!(tagged.to_string(), "1.3.0+sha.5114f85");
    /// ```
    pub fn next(&self, options: &NextOptions) -> Result<Version, NextError> {
        let requested = options
            .pre
            .as_deref()
            .map(|pre| pre_release(pre).map_err(|_| NextError::InvalidPreRelease))
            .transpose()?;

        let build = options
            .build
            .as_deref()
            .map(|build| build_metadata(build).map_err(|_| NextError::InvalidBuildMetadata))
            .transpose()?;

        if requested.is_some() && self.pre.is_some() && options.change.is_some() {
            return Err(NextError::ChangeOnPreRelease);
        }

        let mut next = match (&self.pre, options.change) {
            (Some(_), None) => Version::new(self.major, self.minor, self.patch),
            (_, change) => self
                .bump(change.unwrap_or(Change::Feature))
                .ok_or(NextError::Overflow)?,
        };

        next.pre = requested
            .map(|requested| next_pre_release(self.pre.as_deref(), &requested))
            .transpose()?;

        next.build = build;

        Ok(next)
    }
}

fn next_pre_release(previous: Option<&str>, requested: &str) -> Result<String, NextError> {
    if split_counter(requested).1.is_some() {
        return Ok(requested.to_owned());
    }

    let counter = previous.and_then(|previous| match split_counter(previous) {
        (label, Some(counter)) if label == requested => Some(counter),
        _ => None,
    });

    let number = match counter {
        Some(counter) => counter
            .parse::<u64>()
            .ok()
            .and_then(|counter| counter.checked_add(1))
            .ok_or(NextError::Overflow)?,
        None => 1,
    };

    Ok(format!("{requested}.{number}"))
}

fn split_counter(pre: &str) -> (&str, Option<&str>) {
    let (label, last) = pre.rsplit_once('.').unwrap_or(("", pre));

    if last.bytes().all(|b| b.is_ascii_digit()) {
        (label, Some(last))
    } else {
        (pre, None)
    }
}
