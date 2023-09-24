#[macro_export]
macro_rules! size_eq {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::core::mem::size_of::<$ty>()];
    };
}
