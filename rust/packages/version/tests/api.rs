//! API tests.
//!
//! These tests only use the public API of the crate. If one of them has to
//! change, it is a breaking change and needs a major version bump.
//!
//! Other tests (unit tests inside `src/`) can change freely.

use std::collections::HashSet;

use version::{ParseError, Version};

#[test]
fn parse_exposes_the_three_parts() {
    let v: Version = "1.2.3".parse().unwrap();

    assert_eq!(v.major(), 1);
    assert_eq!(v.minor(), 2);
    assert_eq!(v.patch(), 3);
}

#[test]
fn display_round_trips_with_parse() {
    for input in ["0.0.0", "1.2.3", "10.20.30"] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), input);
    }
}

#[test]
fn equal_versions_are_equal() {
    let a: Version = "1.2.3".parse().unwrap();
    let b: Version = "1.2.3".parse().unwrap();
    let c: Version = "1.2.4".parse().unwrap();

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn parse_fills_missing_minor_and_patch_with_zero() {
    for (input, expected) in [("1", "1.0.0"), ("1.2", "1.2.0"), ("0", "0.0.0")] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), expected);
    }
}

#[test]
fn parse_rejects_more_than_three_parts() {
    for input in ["1.2.3.4", "1.2.3.a", "1.2.3."] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::PartCount));
    }
}

#[test]
fn parse_rejects_non_numeric_parts() {
    for input in [
        "", "1.", ".1", "a", "a.2.3", "1.b.3", "1.2.c", "1..3", "-1.2.3", "+1.2.3", "1.2.3 ",
    ] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::InvalidNumber));
    }
}

#[test]
fn parse_rejects_leading_zeros() {
    for input in ["01", "01.2", "01.2.3", "1.02.3", "1.2.03", "00.0.0"] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::LeadingZero));
    }
}

#[test]
fn parse_accepts_zero_as_a_part() {
    let v: Version = "0.0.0".parse().unwrap();

    assert_eq!((v.major(), v.minor(), v.patch()), (0, 0, 0));
}

#[test]
fn parse_exposes_the_pre_release() {
    let with: Version = "1.2.3-beta.1".parse().unwrap();
    let without: Version = "1.2.3".parse().unwrap();

    assert_eq!(with.pre_release(), Some("beta.1"));
    assert_eq!(without.pre_release(), None);
}

#[test]
fn display_round_trips_with_pre_release() {
    for input in [
        "1.2.3-alpha",
        "1.0.0-alpha.1",
        "1.0.0-0.3.7",
        "1.0.0-x.7.z.92",
        "1.0.0-alpha-a.b-c",
    ] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), input);
    }
}

#[test]
fn short_forms_combine_with_pre_release() {
    for (input, expected) in [("1-beta", "1.0.0-beta"), ("1.2-rc.1", "1.2.0-rc.1")] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), expected);
    }
}

#[test]
fn versions_with_different_pre_releases_are_not_equal() {
    let a: Version = "1.2.3-alpha".parse().unwrap();
    let b: Version = "1.2.3-beta".parse().unwrap();
    let stable: Version = "1.2.3".parse().unwrap();

    assert_ne!(a, b);
    assert_ne!(a, stable);
}

#[test]
fn parse_rejects_invalid_pre_release() {
    for input in [
        "1.2.3-",
        "1.2.3-beta.",
        "1.2.3-.beta",
        "1.2.3-be..ta",
        "1.2.3-beta!",
        "1.2.3-b_c",
        "1.2.3-é",
    ] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::InvalidPreRelease));
    }
}

#[test]
fn parse_rejects_leading_zeros_in_numeric_pre_release_identifiers() {
    for input in ["1.0.0-01", "1.0.0-00", "1.0.0-alpha.01", "1.0.0-01.alpha"] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::LeadingZero));
    }
}

#[test]
fn parse_accepts_zero_in_pre_release_identifiers() {
    for input in ["1.0.0-0", "1.0.0-alpha.0", "1.0.0-0a", "1.0.0-01a", "1.0.0-a01"] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), input);
    }
}

#[test]
fn parse_exposes_the_build_metadata() {
    let with: Version = "1.2.3+sha.abc".parse().unwrap();
    let without: Version = "1.2.3".parse().unwrap();

    assert_eq!(with.build_metadata(), Some("sha.abc"));
    assert_eq!(without.build_metadata(), None);
}

#[test]
fn display_round_trips_with_build_metadata() {
    for input in [
        "1.2.3+sha.abc",
        "1.0.0+001",
        "1.0.0+20130313144700",
        "1.0.0-beta+exp.sha.5114f85",
        "1.0.0+21AF26D3-117B344092BD",
    ] {
        let v: Version = input.parse().unwrap();

        assert_eq!(v.to_string(), input);
    }
}

#[test]
fn pre_release_and_build_metadata_are_separate() {
    let v: Version = "1.2.3-beta-1+sha-2".parse().unwrap();

    assert_eq!(v.pre_release(), Some("beta-1"));
    assert_eq!(v.build_metadata(), Some("sha-2"));
}

#[test]
fn short_forms_combine_with_build_metadata() {
    let v: Version = "1.2+sha".parse().unwrap();

    assert_eq!(v.to_string(), "1.2.0+sha");
}

#[test]
fn equality_ignores_build_metadata() {
    let a: Version = "1.2.3+a".parse().unwrap();
    let b: Version = "1.2.3+b".parse().unwrap();
    let none: Version = "1.2.3".parse().unwrap();

    assert_eq!(a, b);
    assert_eq!(a, none);
}

#[test]
fn equality_still_compares_pre_release() {
    let a: Version = "1.2.3-alpha+x".parse().unwrap();
    let b: Version = "1.2.3-beta+x".parse().unwrap();

    assert_ne!(a, b);
}

#[test]
fn hash_agrees_with_equality() {
    let a: Version = "1.2.3+a".parse().unwrap();
    let b: Version = "1.2.3+b".parse().unwrap();

    let set: HashSet<Version> = [a, b].into_iter().collect();

    assert_eq!(set.len(), 1);
}

#[test]
fn parse_rejects_invalid_build_metadata() {
    for input in [
        "1.2.3+",
        "1.2.3+sha.",
        "1.2.3+.sha",
        "1.2.3+sha..abc",
        "1.2.3+sha!",
        "1.2.3+a+b",
        "1.2.3-beta+",
    ] {
        assert_eq!(
            input.parse::<Version>(),
            Err(ParseError::InvalidBuildMetadata)
        );
    }
}

#[test]
fn parse_error_is_a_displayable_error() {
    let err = "nope".parse::<Version>().unwrap_err();
    let boxed: Box<dyn std::error::Error> = Box::new(err);

    assert!(!boxed.to_string().is_empty());
}
