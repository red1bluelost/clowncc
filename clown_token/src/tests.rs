use crate::{Cursor, DCharSeq, NumberBase, RawStrErr, Token, TokenKind};

use clown_version::StdVersion;
use expect_test::{expect, Expect};

/// TODO: Move to support macro crate
macro_rules! static_assert_size_eq {
    ($ty:ty, $size:expr) => {
        const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
    };
}

// Assertions to keep the token size small
static_assert_size_eq!(DCharSeq, 2);
static_assert_size_eq!(RawStrErr, 1);
static_assert_size_eq!(NumberBase, 1);
static_assert_size_eq!(TokenKind, 3);
static_assert_size_eq!(Token, 8);

fn check_tokens_impl<'c>(
    code: &'c str,
    expect: Expect,
    mut tok_fn: impl FnMut(&mut Cursor<'c>) -> Option<Token>,
) {
    let mut cursor = Cursor::new(code, StdVersion::Cpp26);
    let mut length_acc = 0;
    let tokens: String = std::iter::from_fn(move || tok_fn(&mut cursor))
        .map(|t| {
            length_acc += t.length() as usize;
            t
        })
        .map(|t| format!("{:?}\n", t))
        .collect();
    expect.assert_eq(&tokens);
    assert_eq!(code.len(), length_acc);
}

fn check_basic_tokens(code: &str, expect: Expect) {
    check_tokens_impl(code, expect, Cursor::next_token);
}

fn check_header_tokens(code: &str, expect: Expect) {
    check_tokens_impl(code, expect, Cursor::next_token_header);
}

#[test]
fn hello_world_test() {
    check_basic_tokens(
        r#"
int main() {
    puts("hello world")
}
"#,
        expect![[r#"
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
            Token { kind: Identifier, length: 3, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Identifier, length: 4, flags: TokenFlags(0x0) }
            Token { kind: OpenParen, length: 1, flags: TokenFlags(0x0) }
            Token { kind: CloseParen, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: OpenBrace, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 5, flags: TokenFlags(NEWLINE) }
            Token { kind: Identifier, length: 4, flags: TokenFlags(0x0) }
            Token { kind: OpenParen, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Str { lit_type: Default, has_esc: false }, length: 13, flags: TokenFlags(0x0) }
            Token { kind: CloseParen, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
            Token { kind: CloseBrace, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}

#[test]
fn spliced_universal_char() {
    check_basic_tokens(
        r#"
int \\
\
\
u\
0\
3\
\
9\
1 = 0;
"#,
        expect![[r#"
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
            Token { kind: Identifier, length: 3, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Identifier, length: 22, flags: TokenFlags(NEWLINE | UNIV_CHAR) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Equal, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Number { base: Decimal }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: SemiColon, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}

#[test]
fn spliced_line_comment() {
    check_basic_tokens(
        "//   \\       \n\\\nint main() {}",
        expect![[r#"
            Token { kind: LineComment, length: 29, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}

#[test]
fn unterminated_string() {
    check_basic_tokens(
        r#""hello
"gary"#,
        expect![[r#"
            Token { kind: Str { lit_type: Default, has_esc: false }, length: 7, flags: TokenFlags(NEWLINE | UNTERMINATED) }
            Token { kind: Str { lit_type: Default, has_esc: false }, length: 5, flags: TokenFlags(UNTERMINATED) }
        "#]],
    );
}

#[test]
fn string_double_backslash() {
    check_basic_tokens(
        "const char *ignore = \"\\\\\nf\";",
        expect![[r#"
            Token { kind: Identifier, length: 5, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Identifier, length: 4, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Star, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Identifier, length: 6, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Equal, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Str { lit_type: Default, has_esc: true }, length: 6, flags: TokenFlags(NEWLINE) }
            Token { kind: SemiColon, length: 1, flags: TokenFlags(0x0) }
        "#]],
    );
}

#[test]
fn system_header() {
    check_header_tokens(
        " <stdio.h> // something\n",
        expect![[r#"
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: SystemHeader, length: 9, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: LineComment, length: 12, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}

#[test]
fn local_header() {
    check_header_tokens(
        " /*hi*/ \"llvm/ADT/SmallVector.h\" //\n",
        expect![[r#"
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: BlockComment, length: 6, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: Header, length: 24, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: LineComment, length: 2, flags: TokenFlags(0x0) }
            Token { kind: Whitespace { splits_lines: true }, length: 1, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}

#[test]
fn spliced_system_header() {
    check_header_tokens(
        " <stdl\\\nib.h\\\n>",
        expect![[r#"
            Token { kind: Whitespace { splits_lines: false }, length: 1, flags: TokenFlags(0x0) }
            Token { kind: SystemHeader, length: 14, flags: TokenFlags(NEWLINE) }
        "#]],
    );
}
