pub trait OptionExt: crate::internal::Sealed {
    type T;
    fn err(self) -> Result<(), Self::T>;
    fn err_or<O>(self, ok: O) -> Result<O, Self::T>;
    fn err_or_else<O>(self, ok: impl FnOnce() -> O) -> Result<O, Self::T>;
}

impl<T> crate::internal::Sealed for Option<T> {}

impl<T> OptionExt for Option<T> {
    type T = T;

    fn err(self) -> Result<(), Self::T> {
        self.map_or(Ok(()), Err)
    }

    fn err_or<O>(self, ok: O) -> Result<O, Self::T> {
        self.map_or(Ok(ok), Err)
    }

    fn err_or_else<O>(self, ok: impl FnOnce() -> O) -> Result<O, Self::T> {
        self.map_or_else(|| Ok(ok()), Err)
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
}
