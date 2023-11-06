/// Defines an enumeration with two values, `Yes` and `No`. Meant to act as
/// a self documenting boolean. The name provided should be an option action
/// which functions may perform. This way the call sight has more context
/// that a regular `true`/`false`.
///
/// # Example
/// ```rust
/// clowncc_macros::yes_no!{
///     pub enum ClearFirst;
/// }
///
/// pub fn add_n_zeros(n: usize, clear_first: ClearFirst, v: &mut Vec<i32>) {
///     if clear_first.is_yes() {
///         v.clear();
///     }
///     v.append(&mut vec![0; n]);
/// }
///
/// let mut v = vec![1, 2, 3];
///
/// add_n_zeros(5, ClearFirst::No, &mut v);
/// assert_eq!(v, vec![1, 2, 3, 0, 0, 0, 0, 0]);
///
/// add_n_zeros(5, ClearFirst::Yes, &mut v);
/// assert_eq!(v, vec![0, 0, 0, 0, 0]);
/// ```
#[macro_export]
macro_rules! yes_no {
    {
        $(#[$attrs:meta])*
        $access:vis enum $name:ident;
    } => {
        $crate::custom_bool!{
            $(#[$attrs])*
            #[derive(Default)]
            $access enum $name {
                #[default]
                No = false,
                Yes = true,
            }
        }
    }
}

#[macro_export]
macro_rules! custom_bool {
    (
        $(#[$attrs:meta])*
        $access:vis enum $name:ident {
            $(#[$a_attrs:meta])* $a_id:ident $(= $a_bool:tt)?,
            $(#[$b_attrs:meta])* $b_id:ident $(= $b_bool:tt)? $(,)?
        }
    ) => {
        $(#[$attrs])*
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        $access enum $name {
            $(#[$a_attrs])*
            $a_id,
            $(#[$b_attrs])*
            $b_id,
        }

        impl $name {
            $crate::__internal::__paste::paste! {
                #[inline]
                #[must_use]
                $access const fn [<is_ $a_id:snake>](self) -> bool {
                    matches!(self, Self::$a_id)
                }

                #[inline]
                #[must_use]
                $access const fn [<is_ $b_id:snake>](self) -> bool {
                    matches!(self, Self::$b_id)
                }
            }
        }

        $crate::__to_from_bool!{
            $name,
            $a_id $(= $a_bool)?,
            $b_id $(= $b_bool)?,
        }
    }
}

#[macro_export]
macro_rules! __to_from_bool {
    [$name:ident, $a_id:ident = true, $b_id:ident = false,] => {
        $crate::__to_from_bool_unchecked!($name, $a_id = true, $b_id = false,);
    };

    [$name:ident, $a_id:ident = false, $b_id:ident = true,] => {
        $crate::__to_from_bool_unchecked!($name, $a_id = false, $b_id = true,);
    };

    [$name:ident, $a_id:ident, $b_id:ident,] => {};

    ($($_:tt)*) => {
        compile_error!(
r#"when assigning discriminant you must use `true` and `false`, example:
custom_bool! {
    pub What {
        NuhUh = false,
        YuhHuh = true,
    }
}"#
        );
    }
}

#[macro_export]
macro_rules! __to_from_bool_unchecked {
    {
        $name:ident,
        $a_id:ident = $a_bool:literal,
        $b_id:ident = $b_bool:literal,
    } => {
        impl From<$name> for bool {
            fn from(v: $name) -> bool {
                match v {
                    $name::$a_id => $a_bool,
                    $name::$b_id => $b_bool,
                }
            }
        }

        impl From<bool> for $name {
            fn from(v: bool) -> $name {
                match v {
                    $a_bool => $name::$a_id,
                    $b_bool => $name::$b_id,
                }
            }
        }
    };
}
