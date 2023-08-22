use crate::{Language, StdVersion};

#[test]
fn deref_to_lang() {
    assert_eq!(Language::C, *StdVersion::C89);
    assert_eq!(Language::C, *StdVersion::C95);
    assert_eq!(Language::C, *StdVersion::C99);
    assert_eq!(Language::C, *StdVersion::C11);
    assert_eq!(Language::C, *StdVersion::C17);
    assert_eq!(Language::C, *StdVersion::C23);
    assert_ne!(Language::Cpp, *StdVersion::C89);
    assert_ne!(Language::Cpp, *StdVersion::C95);
    assert_ne!(Language::Cpp, *StdVersion::C99);
    assert_ne!(Language::Cpp, *StdVersion::C11);
    assert_ne!(Language::Cpp, *StdVersion::C17);
    assert_ne!(Language::Cpp, *StdVersion::C23);


    assert_eq!(Language::Cpp, *StdVersion::Cpp11);
    assert_eq!(Language::Cpp, *StdVersion::Cpp14);
    assert_eq!(Language::Cpp, *StdVersion::Cpp17);
    assert_eq!(Language::Cpp, *StdVersion::Cpp20);
    assert_eq!(Language::Cpp, *StdVersion::Cpp23);
    assert_eq!(Language::Cpp, *StdVersion::Cpp26);
    assert_ne!(Language::C, *StdVersion::Cpp11);
    assert_ne!(Language::C, *StdVersion::Cpp14);
    assert_ne!(Language::C, *StdVersion::Cpp17);
    assert_ne!(Language::C, *StdVersion::Cpp20);
    assert_ne!(Language::C, *StdVersion::Cpp23);
    assert_ne!(Language::C, *StdVersion::Cpp26);
}
