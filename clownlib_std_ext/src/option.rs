mod internal {
    pub trait Sealed<T> {}
    impl<T> Sealed<T> for Option<T> {}
}

pub trait OptionExt<T>: internal::Sealed<T> {
    /// Transforms the [`Option<T>`] into a [`Result<T, ()>`], mapping
    /// [`Some(v)`] to [`Ok(v)`] and [`None`] to [`Err(())`].
    ///
    /// [`Result<T, ()>`]: Result
    /// [`Some(v)`]: Some
    /// [`Ok(v)`]: Ok
    /// [`Err(())`]: Err
    ///
    /// # Examples
    ///
    /// ```
    /// # use clownlib_std_ext::OptionExt;
    /// #
    /// let x = Some("foo");
    /// assert_eq!(x.ok(), Ok("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.ok(), Err(()));
    /// ```
    fn ok(self) -> Result<T, ()>;

    /// Transforms the [`Option<T>`] into a [`Result<(), T>`], mapping
    /// [`Some(v)`] to [`Err(v)`] and [`None`] to [`Ok(())`].
    ///
    /// [`Result<(), T>`]: Result
    /// [`Some(v)`]: Some
    /// [`Ok(())`]: Ok
    /// [`Err(v)`]: Err
    ///
    /// # Examples
    ///
    /// ```
    /// # use clownlib_std_ext::OptionExt;
    /// #
    /// let x = Some("foo");
    /// assert_eq!(x.err(), Err("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.err(), Ok(()));
    /// ```
    fn err(self) -> Result<(), T>;

    /// Transforms the [`Option<T>`] into a [`Result<O, T>`], mapping
    /// [`Some(v)`] to [`Err(v)`] and [`None`] to [`Ok(ok)`].
    ///
    /// Arguments passed to [`err_or`] are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`err_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`Result<O, T>`]: Result
    /// [`Err(v)`]: Err
    /// [`Ok(ok)`]: Ok
    /// [`Some(v)`]: Some
    /// [`err_or`]: OptionExt::err_or
    /// [`err_or_else`]: OptionExt::err_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// # use clownlib_std_ext::OptionExt;
    /// #
    /// let x = Some("foo");
    /// assert_eq!(x.err_or(0), Err("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.err_or(0), Ok(0));
    /// ```
    fn err_or<O>(self, ok: O) -> Result<O, T>;


    /// Transforms the [`Option<T>`] into a [`Result<O, T>`], mapping
    /// [`Some(v)`] to [`Err(v)`] and [`None`] to [`Ok(ok())`].
    ///
    /// [`Result<O, T>`]: Result
    /// [`Err(v)`]: Err
    /// [`Ok(ok())`]: Ok
    /// [`Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// # use clownlib_std_ext::OptionExt;
    /// #
    /// let x = Some("foo");
    /// assert_eq!(x.err_or_else(|| 0), Err("foo"));
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.err_or_else(|| 0), Ok(0));
    /// ```
    fn err_or_else<O>(self, ok: impl FnOnce() -> O) -> Result<O, T>;

    /// Constructs and returns the default value (if none),
    /// or applies a function to the contained value (if any).
    ///
    /// The call to [`default`] is lazy only if the optional is empty.
    ///
    /// [`default`]: Default::default
    ///
    /// # Examples
    ///
    /// ```
    /// # use clownlib_std_ext::OptionExt;
    /// #
    /// let x = Some("foo");
    /// assert_eq!(x.map_or_default(|v| v.len()), 3);
    ///
    /// let x: Option<&str> = None;
    /// assert_eq!(x.map_or_default(|v| v.len()), 0);
    /// ```
    fn map_or_default<U: Default>(self, f: impl FnOnce(T) -> U) -> U;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok(self) -> Result<T, ()> {
        self.map_or(Err(()), Ok)
    }

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
        self.map_or_else(U::default, f)
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
