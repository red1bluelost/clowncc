mod internal {
    pub trait Sealed<T> {}
    impl<T> Sealed<T> for Option<T> {}
}

pub trait OptionExt<T>: internal::Sealed<T> {
    fn err(self) -> Result<(), T>;
    fn err_or<O>(self, ok: O) -> Result<O, T>;
    fn err_or_else<O>(self, ok: impl FnOnce() -> O) -> Result<O, T>;

    fn map_or_default<U: Default>(self, f: impl FnOnce(T) -> U) -> U;
}

impl<T> OptionExt<T> for Option<T> {
    fn err(self) -> Result<(), T> {
        self.map_or(Ok(()), Err)
    }

    fn err_or<O>(self, ok: O) -> Result<O, T> {
        self.map_or(Ok(ok), Err)
    }

    fn err_or_else<O>(self, ok: impl FnOnce() -> O) -> Result<O, T> {
        self.map_or_else(|| Ok(ok()), Err)
    }

    fn map_or_default<U: Default>(self, f: impl FnOnce(T) -> U) -> U {
        match self {
            Some(t) => f(t),
            None => U::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OptionExt;

    #[test]
    fn option_err() {
        assert_eq!(Some(1).err(), Err(1));
        assert_eq!(None::<i32>.err(), Ok(()));
    }

    #[test]
    fn option_err_or() {
        assert_eq!(Some(3).err_or("hi"), Err(3));
        assert_eq!(None::<i32>.err_or("hello"), Ok("hello"));
    }

    #[test]
    fn option_err_or_else() {
        assert_eq!(Some(3).err_or_else(|| "hi"), Err(3));
        assert_eq!(None::<i32>.err_or_else(|| "hello"), Ok("hello"));
    }

    #[test]
    fn option_map_or_default() {
        assert_eq!(Some(4).map_or_default(|n| vec![(); n]), vec![(); 4]);
        assert_eq!(None.map_or_default(|n| vec![(); n]), vec![(); 0]);
    }
}
