//! Low-level C and C++ tokenizer
//!
//! `clowncc_token` aims to provide simple lexer that can separate out tokens
//! from source code in a minimal representation. A cursor [`Cursor`] can
//! iterate over source code [`&str`] to generate small tokens [`Token`].
//!
//! # Errors
//!
//! Errors that prevent lexing are encoded as unique tokens with some useful
//! information.
//!
//! Delimited tokens (`BlockComment`, `SystemHeader`, `Header`, `Str`,
//! `CharSeq`) may be unterminated and contain a flag that indicates this error.

mod char_info;
mod cursor;
mod token;

#[cfg(test)]
mod tests;

pub use char_info::CharInfo;
pub use cursor::Cursor;
pub use token::{DCharSeq, LitType, NumberBase, RawStrErr, Token, TokenKind};
