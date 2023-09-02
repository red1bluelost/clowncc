#[macro_export]
macro_rules! debug_assert {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        (assert!($($arg)*));
    }
}
