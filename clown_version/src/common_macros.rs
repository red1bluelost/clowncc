macro_rules! define_info_enum {
    ($enum_id:ident: $([$id_snake:ident, $name_str:expr]),* $(,)?) => {
        ::paste::paste! {
            #[derive(Clone, Copy, Debug, Eq, PartialEq)]
            pub enum $enum_id {
                $([<$id_snake:camel>]),*
            }

            impl $enum_id {
                /// String representation of the language which can be used for
                /// argument passing and more.
                pub const fn as_str(self) -> &'static str {
                    match self {
                        $(Self::[<$id_snake:camel>] => $name_str),*
                    }
                }

                $(
                    #[inline(always)]
                    pub const fn [<is_ $id_snake>](self) -> bool {
                        matches!(self, Self::[<$id_snake:camel>])
                    }
                )*
            }

            impl ::std::str::FromStr for $enum_id {
                type Err = crate::FromStrError<$enum_id>;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        $($name_str => Ok(Self::[<$id_snake:camel>]),)*
                        _ => Err(Self::Err::make()),
                    }
                }
            }

        }

    };

}

pub(crate) use define_info_enum;