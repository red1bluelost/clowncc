use crate::{DCharSeq, NumberBase, RawStrErr, Token, TokenKind};

// Assertions to keep the token size small
clownlib_static_assert::size_eq!(DCharSeq, 2);
clownlib_static_assert::size_eq!(RawStrErr, 1);
clownlib_static_assert::size_eq!(NumberBase, 1);
clownlib_static_assert::size_eq!(TokenKind, 3);
clownlib_static_assert::size_eq!(Token, 8);
