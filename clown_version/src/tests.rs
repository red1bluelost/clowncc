use crate::{Language, StdVersion};

use StdVersion::*;

const C_VERSIONS: [StdVersion; 6] = [C89, C95, C99, C11, C17, C23];
const CPP_VERSIONS: [StdVersion; 6] =
    [Cpp11, Cpp14, Cpp17, Cpp20, Cpp23, Cpp26];

#[test]
fn std_version_deref_to_lang() {
    for sv in C_VERSIONS {
        assert_eq!(Language::C, *sv);
        assert_ne!(Language::Cpp, *sv);
    }
    for sv in CPP_VERSIONS {
        assert_eq!(Language::Cpp, *sv);
        assert_ne!(Language::C, *sv);
    }
}

macro_rules! assert_if {
    ($cond:expr, $pred:expr) => {
        assert!(if $cond { $pred } else { !$pred })
    };
}

#[test]
fn std_version_before() {
    for (idx, sv) in C_VERSIONS.into_iter().enumerate() {
        assert!(!sv.is_before_c89());
        assert_if!(1 > idx, sv.is_before_c95());
        assert_if!(2 > idx, sv.is_before_c99());
        assert_if!(3 > idx, sv.is_before_c11());
        assert_if!(4 > idx, sv.is_before_c17());
        assert_if!(5 > idx, sv.is_before_c23());

        assert!(!sv.is_before_cpp11());
        assert!(!sv.is_before_cpp14());
        assert!(!sv.is_before_cpp17());
        assert!(!sv.is_before_cpp20());
        assert!(!sv.is_before_cpp23());
        assert!(!sv.is_before_cpp26());
    }

    for (idx, sv) in CPP_VERSIONS.into_iter().enumerate() {
        assert!(!sv.is_before_cpp11());
        assert_if!(1 > idx, sv.is_before_cpp14());
        assert_if!(2 > idx, sv.is_before_cpp17());
        assert_if!(3 > idx, sv.is_before_cpp20());
        assert_if!(4 > idx, sv.is_before_cpp23());
        assert_if!(5 > idx, sv.is_before_cpp26());

        assert!(!sv.is_before_c89());
        assert!(!sv.is_before_c95());
        assert!(!sv.is_before_c99());
        assert!(!sv.is_before_c11());
        assert!(!sv.is_before_c17());
        assert!(!sv.is_before_c23());
    }
}

#[test]
fn std_version_since() {
    for (idx, sv) in C_VERSIONS.into_iter().enumerate() {
        assert!(sv.is_since_c89());
        assert_if!(1 <= idx, sv.is_since_c95());
        assert_if!(2 <= idx, sv.is_since_c99());
        assert_if!(3 <= idx, sv.is_since_c11());
        assert_if!(4 <= idx, sv.is_since_c17());
        assert_if!(5 <= idx, sv.is_since_c23());

        assert!(!sv.is_since_cpp11());
        assert!(!sv.is_since_cpp14());
        assert!(!sv.is_since_cpp17());
        assert!(!sv.is_since_cpp20());
        assert!(!sv.is_since_cpp23());
        assert!(!sv.is_since_cpp26());
    }

    for (idx, sv) in CPP_VERSIONS.into_iter().enumerate() {
        assert!(sv.is_since_cpp11());
        assert_if!(1 <= idx, sv.is_since_cpp14());
        assert_if!(2 <= idx, sv.is_since_cpp17());
        assert_if!(3 <= idx, sv.is_since_cpp20());
        assert_if!(4 <= idx, sv.is_since_cpp23());
        assert_if!(5 <= idx, sv.is_since_cpp26());

        assert!(!sv.is_since_c89());
        assert!(!sv.is_since_c95());
        assert!(!sv.is_since_c99());
        assert!(!sv.is_since_c11());
        assert!(!sv.is_since_c17());
        assert!(!sv.is_since_c23());
    }
}

#[test]
fn std_version_from_str() {
    let p = |s: &str| -> Result<StdVersion, _> { s.parse() };
    assert!(p("c11").is_ok());
}
