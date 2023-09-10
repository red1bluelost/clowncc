use crate::{DCharSeq, NumberBase, RawStrErr, Token, TokenKind};

/// TODO: Move to support macro crate
macro_rules! static_assert_size_eq {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::core::mem::size_of::<$ty>()];
    };
}

// Assertions to keep the token size small
static_assert_size_eq!(DCharSeq, 2);
static_assert_size_eq!(RawStrErr, 1);
static_assert_size_eq!(NumberBase, 1);
static_assert_size_eq!(TokenKind, 3);
static_assert_size_eq!(Token, 8);
