use crate::StdVersion;

macro_rules! implement {
    ($([$lang:ident, $id_snake:ident, $name_str:expr]),* $(,)?) => {
        ::paste::paste! {
            crate::common_macros::define_info_enum!{
                #[derive(Ord, PartialOrd)]
                StdVersion: $([$id_snake, $name_str]),*,
            }

            impl StdVersion {
                #[must_use]
                pub const fn as_language(self) -> Language {
                    match self {
                        $(Self::[<$id_snake:camel>] => Language::$lang),*
                    }
                }

                $(
                    #[must_use]
                    pub fn [<is_since_ $id_snake>](self) -> bool {
                        self.is_since(Self::[<$id_snake:camel>])
                    }

                    #[must_use]
                    pub fn [<is_before_ $id_snake>](self) -> bool {
                        self.is_before(Self::[<$id_snake:camel>])
                    }
                )*
            }

            impl ::core::ops::Deref for StdVersion {
                type Target = Language;
                #[must_use]
                fn deref(&self) -> &Self::Target {
                    match self {
                        $(Self::[<$id_snake:camel>] => &Language::$lang),*
                    }
                }
            }
        }
    }
}
pub(super) use implement;

impl StdVersion {
    pub const C_DEFAULT_VERSION: StdVersion = StdVersion::C17;
    pub const CPP_DEFAULT_VERSION: StdVersion = StdVersion::Cpp17;
    pub const C_EARLIEST_VERSION: StdVersion = StdVersion::C89;
    pub const CPP_EARLIEST_VERSION: StdVersion = StdVersion::Cpp11;

    #[must_use]
    pub fn is_since(self, since: StdVersion) -> bool {
        since.as_language() == self.as_language() && since <= self
    }

    #[must_use]
    pub fn is_before(self, since: StdVersion) -> bool {
        since.as_language() == self.as_language() && since > self
    }
}
