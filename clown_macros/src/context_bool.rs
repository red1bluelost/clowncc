#[macro_export]
macro_rules! define_yes_no {
    (
        $(#[$attrs:meta])*
        $access:vis $name:ident
    ) => {
        $(#[$attrs])*
        #[must_use]
        #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
        $access enum $name {
            #[default]
            No,
            Yes,
        }

        impl $name {
            #[must_use]
            $access const fn is_no(self) -> bool {
                matches!(self, Self::No)
            }
            #[must_use]
            $access const fn is_yes(self) -> bool {
                matches!(self, Self::Yes)
            }
        }

        impl From<$name> for bool {
            fn from(v: $name) -> bool {
                v.is_yes()
            }
        }

        impl From<bool> for $name {
            fn from(v: bool) -> $name {
                if v { $name::Yes } else { $name::No }
            }
        }
    };
}
