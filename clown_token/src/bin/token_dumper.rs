use clown_token::{Cursor, TokenKind};
use std::io::Read;

#[derive(Copy, Clone, Eq, PartialEq)]
enum ParseHeader {
    None,
    Pound,
    Include,
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("Should be passing in a file name");
    let code = if file_name == "-" {
        let mut str_out = String::new();
        std::io::stdin().read_to_string(&mut str_out).unwrap();
        str_out
    } else {
        std::fs::read_to_string(file_name).unwrap()
    };

    let mut cursor = Cursor::new(&code);

    let mut parse_header = ParseHeader::None;
    let mut token_start = 0;
    while let Some(token) = if parse_header == ParseHeader::Include {
        cursor.next_token_header()
    } else {
        cursor.next_token()
    } {
        let len = token.length as usize;
        let code_slice = &code[token_start..token_start + len];
        println!("{:?} = (\"{}\")", token, code_slice);
        token_start += len;

        match token.kind {
            TokenKind::BlockComment
            | TokenKind::Whitespace {
                no_bare_newline: true,
            } => continue,
            TokenKind::Pound if parse_header == ParseHeader::None => {
                parse_header = ParseHeader::Pound;
            }
            TokenKind::Identifier
                if matches!(
                    (parse_header, code_slice),
                    (ParseHeader::Pound, "include")
                ) =>
            {
                parse_header = ParseHeader::Include;
            }
            _ => {
                parse_header = ParseHeader::None;
            }
        }
    }
    println!("remaining = (\"{}\")", &code[token_start..]);
    assert_eq!(token_start, code.len());
}
