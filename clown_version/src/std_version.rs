macro_rules! implement {
    ($([$lang:ident, $id_snake:ident, $name_str:expr]),* $(,)?) => {
        ::paste::paste! {
            crate::common_macros::define_info_enum!{
                StdVersion: $([$id_snake, $name_str]),*,
            }

            impl ::std::ops::Deref for StdVersion {
                type Target = Language;
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
