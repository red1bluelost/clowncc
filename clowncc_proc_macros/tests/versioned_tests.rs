use clowncc_proc_macros::Versioned;
use clowncc_version::{
    Language, LanguageSupported, StdVersion, StdVersionSupported,
};

use strum::IntoEnumIterator;

#[test]
fn test_universal() {
    #[derive(Versioned)]
    enum E {
        #[versioned(universal)]
        A,
    }

    StdVersion::iter().for_each(|sv| assert!(E::A.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::A.is_in_language(l)));
}

#[test]
fn test_langs() {
    #[derive(Versioned)]
    enum E {
        #[versioned(lang C, lang Cpp)]
        A,
        #[versioned(lang C)]
        B(),
        #[versioned(lang Cpp)]
        C,
    }

    // A
    StdVersion::iter().for_each(|sv| assert!(E::A.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::A.is_in_language(l)));

    // B
    StdVersion::iter()
        .filter(|sv| sv.is_c())
        .for_each(|sv| assert!(E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_c())
        .for_each(|l| assert!(E::B().is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_cpp())
        .for_each(|sv| assert!(!E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_cpp())
        .for_each(|l| assert!(!E::B().is_in_language(l)));

    // C
    StdVersion::iter()
        .filter(|sv| sv.is_c())
        .for_each(|sv| assert!(!E::C.is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_c())
        .for_each(|l| assert!(!E::C.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_cpp())
        .for_each(|sv| assert!(E::C.is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_cpp())
        .for_each(|l| assert!(E::C.is_in_language(l)));
}

#[test]
fn test_sinces() {
    #[derive(Versioned)]
    enum E {
        #[versioned(since C99, since Cpp17)]
        A,
        #[versioned(since Cpp14)]
        B(),
        #[versioned(lang C, since Cpp20)]
        C { i: i32 },
    }

    // A
    StdVersion::iter()
        .filter(|sv| sv.is_since_c99() || sv.is_since_cpp17())
        .for_each(|sv| assert!(E::A.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::A.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_before_c99() || sv.is_before_cpp17())
        .for_each(|sv| assert!(!E::A.is_in_std_version(sv)));

    // B
    StdVersion::iter()
        .filter(|sv| sv.is_since_cpp14())
        .for_each(|sv| assert!(E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_cpp())
        .for_each(|l| assert!(E::B().is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_c() || sv.is_before_cpp14())
        .for_each(|sv| assert!(!E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_c())
        .for_each(|l| assert!(!E::B().is_in_language(l)));

    // C
    StdVersion::iter()
        .filter(|sv| sv.is_c() || sv.is_since_cpp20())
        .for_each(|sv| assert!(E::C { i: 3 }.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::C { i: 3 }.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_before_cpp20())
        .for_each(|sv| assert!(!E::C { i: 3 }.is_in_std_version(sv)));
}

#[test]
fn test_untils() {
    #[derive(Versioned)]
    enum E {
        #[versioned(until C99, until Cpp17)]
        A,
        #[versioned(until Cpp14)]
        B(),
        #[versioned(lang C, until Cpp20)]
        C { i: i32 },
    }

    // A
    StdVersion::iter()
        .filter(|sv| sv.is_before_c99() || sv.is_before_cpp17())
        .for_each(|sv| assert!(E::A.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::A.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_since_c99() || sv.is_since_cpp17())
        .for_each(|sv| assert!(!E::A.is_in_std_version(sv)));

    // B
    StdVersion::iter()
        .filter(|sv| sv.is_before_cpp14())
        .for_each(|sv| assert!(E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_cpp())
        .for_each(|l| assert!(E::B().is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_c() || sv.is_since_cpp14())
        .for_each(|sv| assert!(!E::B().is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_c())
        .for_each(|l| assert!(!E::B().is_in_language(l)));

    // C
    StdVersion::iter()
        .filter(|sv| sv.is_c() || sv.is_before_cpp20())
        .for_each(|sv| assert!(E::C { i: 3 }.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(E::C { i: 3 }.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| sv.is_since_cpp20())
        .for_each(|sv| assert!(!E::C { i: 3 }.is_in_std_version(sv)));
}

#[test]
fn test_sinces_and_untils() {
    #[derive(Versioned)]
    enum E {
        #[versioned(since Cpp11, until Cpp20)]
        A,
    }

    // A
    StdVersion::iter()
        .filter(|sv| sv.is_since_cpp11() && sv.is_before_cpp20())
        .for_each(|sv| assert!(E::A.is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_cpp())
        .for_each(|l| assert!(E::A.is_in_language(l)));
    StdVersion::iter()
        .filter(|sv| !sv.is_since_cpp11() || !sv.is_before_cpp20())
        .for_each(|sv| assert!(!E::A.is_in_std_version(sv)));
    Language::iter()
        .filter(|l| l.is_c())
        .for_each(|l| assert!(!E::A.is_in_language(l)));
}

#[test]
fn test_struct() {
    #[derive(Versioned, Default)]
    #[versioned(since C89, since Cpp11)]
    struct S;

    StdVersion::iter().for_each(|sv| assert!(S.is_in_std_version(sv)));
    Language::iter().for_each(|l| assert!(S.is_in_language(l)));
}
