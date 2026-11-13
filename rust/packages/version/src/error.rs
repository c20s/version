use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    PartCount,
    InvalidNumber,
    LeadingZero,
    InvalidPreRelease,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::PartCount => write!(f, "expected at most three parts: major.minor.patch"),
            ParseError::InvalidNumber => write!(f, "version parts must be non-negative integers"),
            ParseError::LeadingZero => write!(f, "version parts must not have leading zeros"),
            ParseError::InvalidPreRelease => write!(
                f,
                "pre-release must be dot separated identifiers of [0-9A-Za-z-]"
            ),
        }
    }
}

impl std::error::Error for ParseError {}
