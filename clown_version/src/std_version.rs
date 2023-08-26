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
                        Language::$lang == self.as_language()
                        && Self::[<$id_snake:camel>] <= self
                    }

                    #[must_use]
                    pub fn [<is_before_ $id_snake>](self) -> bool {
                        Language::$lang == self.as_language()
                        && Self::[<$id_snake:camel>] > self
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
    };
}

pub(super) use implement;
