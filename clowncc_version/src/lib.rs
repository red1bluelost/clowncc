#![no_std]

mod common_macros;
mod language;
mod std_version;

use strum_macros::EnumIter;

use core::marker::PhantomData;

#[cfg(test)]
mod tests;

/// TODO: Make actual error type
#[derive(Debug)]
pub struct FromStrError<T>(PhantomData<T>);

language::implement! {
    [c, "c"],
    [cpp, "c++"],
}

std_version::implement! {
    // c
    [C, c89, "c89"],
    [C, c95, "c95"],
    [C, c99, "c99"],
    [C, c11, "c11"],
    [C, c17, "c17"],
    [C, c23, "c23"],

    // c++
    [Cpp, cpp11, "c++11"],
    [Cpp, cpp14, "c++14"],
    [Cpp, cpp17, "c++17"],
    [Cpp, cpp20, "c++20"],
    [Cpp, cpp23, "c++23"],
    [Cpp, cpp26, "c++26"],
}

pub trait StdVersionSupported {
    fn is_in_std_version(&self, sv: StdVersion) -> bool;
}

pub trait LanguageSupported {
    fn is_in_language(&self, lang: Language) -> bool;
}
