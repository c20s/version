//! API tests.
//!
//! These tests only use the public API of the crate. If one of them has to
//! change, it is a breaking change and needs a major version bump.
//!
//! Other tests (unit tests inside `src/`) can change freely.

use std::cmp::Ordering;
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
fn parse_rejects_wrong_number_of_parts() {
    for input in [
        "", "a", "1", "1.2", "1.2.3.4", "1.2.3.a", "1.2.3.", "-1.2.3", "+1.2.3",
    ] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::PartCount));
    }
}

#[test]
fn parse_rejects_non_numeric_parts() {
    for input in ["a.2.3", "1.b.3", "1.2.c", "1..3", "1.2.", ".1.2", "1.2.3 "] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::InvalidNumber));
    }
}

#[test]
fn parse_rejects_leading_zeros() {
    for input in ["01.2.3", "1.02.3", "1.2.03", "00.0.0"] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::LeadingZero));
    }
}

#[test]
fn parse_accepts_zero_as_a_part() {
    let v: Version = "0.0.0".parse().unwrap();

    assert_eq!((v.major(), v.minor(), v.patch()), (0, 0, 0));
}

#[test]
fn parse_rejects_a_v_prefix() {
    for input in ["v1.2.3", "V1.2.3", "v1.2.3-beta+x"] {
        assert_eq!(input.parse::<Version>(), Err(ParseError::InvalidNumber));
    }
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
fn parse_loose_cuts_everything_before_the_version() {
    for (input, expected) in [
        ("v1.2.3", "1.2.3"),
        ("release-1.2.3", "1.2.3"),
        ("refs/tags/v1.2.3", "1.2.3"),
        ("app@1.2.3-beta.1+sha", "1.2.3-beta.1+sha"),
    ] {
        let v = Version::parse_loose(input).unwrap();

        assert_eq!(v.to_string(), expected);
    }
}

#[test]
fn parse_loose_keeps_the_whole_version_when_it_is_valid() {
    for (input, expected) in [
        ("10.2.3", "10.2.3"),
        ("v10.20.30", "10.20.30"),
        ("1.2.3-beta+x", "1.2.3-beta+x"),
    ] {
        let v = Version::parse_loose(input).unwrap();

        assert_eq!(v.to_string(), expected);
    }
}

#[test]
fn parse_loose_matches_strict_parsing_for_valid_input() {
    for input in ["0.0.0", "1.2.3", "1.2.3-beta.1", "1.2.3+sha", "1.2.3-rc.1+sha.5"] {
        let loose = Version::parse_loose(input).unwrap();
        let strict: Version = input.parse().unwrap();

        assert_eq!(loose, strict);
        assert_eq!(loose.build_metadata(), strict.build_metadata());
    }
}

#[test]
fn parse_loose_does_not_accept_short_forms() {
    for input in ["", "latest", "build42", "1", "1.2", "v1.2"] {
        assert!(Version::parse_loose(input).is_err());
    }
}

#[test]
fn parse_loose_returns_the_strict_error_when_nothing_is_found() {
    for input in ["", "latest", "build42", "v1.2", "1.2.3.a"] {
        assert_eq!(
            Version::parse_loose(input),
            input.parse::<Version>()
        );
    }
}

fn v(s: &str) -> Version {
    s.parse().unwrap()
}

#[test]
fn ordering_compares_major_minor_patch_numerically() {
    let ascending = ["0.9.9", "1.0.0", "1.2.3", "1.2.4", "1.3.0", "1.10.0", "2.0.0", "10.0.0"];

    for pair in ascending.windows(2) {
        assert!(v(pair[0]) < v(pair[1]), "{} < {}", pair[0], pair[1]);
    }
}

#[test]
fn ordering_puts_a_pre_release_below_its_release() {
    assert!(v("1.0.0-rc.1") < v("1.0.0"));
    assert!(v("1.0.0") < v("1.0.1-alpha"));
}

#[test]
fn ordering_follows_the_semver_precedence_example() {
    let expected = [
        "1.0.0-alpha",
        "1.0.0-alpha.1",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
    ];

    let mut versions: Vec<Version> = expected.iter().rev().map(|s| v(s)).collect();
    versions.sort();

    let sorted: Vec<String> = versions.iter().map(Version::to_string).collect();
    assert_eq!(sorted, expected);
}

#[test]
fn ordering_ranks_numeric_pre_release_identifiers_below_alphanumeric() {
    assert!(v("1.0.0-1") < v("1.0.0-alpha"));
    assert!(v("1.0.0-alpha.1") < v("1.0.0-alpha.a"));
}

#[test]
fn ordering_handles_numeric_pre_release_identifiers_larger_than_u64() {
    assert!(v("1.0.0-99999999999999999999") < v("1.0.0-100000000000000000000"));
}

#[test]
fn ordering_ignores_build_metadata() {
    assert_eq!(v("1.2.3+a").cmp(&v("1.2.3+b")), Ordering::Equal);
    assert_eq!(v("1.2.3+a").partial_cmp(&v("1.2.3")), Some(Ordering::Equal));
    assert!(v("1.2.3+z") < v("1.2.4+a"));
}

#[test]
fn ordering_agrees_with_equality() {
    let versions = [
        "1.0.0",
        "1.0.0+a",
        "1.0.0-alpha",
        "1.0.0-alpha+b",
        "1.0.0-alpha.1",
        "1.0.1",
    ];

    for a in versions {
        for b in versions {
            assert_eq!(v(a) == v(b), v(a).cmp(&v(b)) == Ordering::Equal, "{a} {b}");
        }
    }
}

#[test]
fn parse_error_is_a_displayable_error() {
    let err = "nope".parse::<Version>().unwrap_err();
    let boxed: Box<dyn std::error::Error> = Box::new(err);

    assert!(!boxed.to_string().is_empty());
}
