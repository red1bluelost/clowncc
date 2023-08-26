/// Defines an enumeration with two values, `Yes` and `No`. Meant to act as
/// a self documenting boolean. The name provided should be an option action
/// which functions may perform. This way the call sight has more context
/// that a regular `true`/`false`.
///
/// # Example
/// ```rust
/// clowncc_macros::define_yes_no!{
///     pub ClearFirst;
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
macro_rules! define_yes_no {
    (
        $(#[$attrs:meta])*
        $access:vis $name:ident;
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
