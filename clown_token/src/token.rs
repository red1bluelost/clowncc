#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // Error tokens:
    Unknown,
    BadRawStr(RawStrErr),
    StrayBackSlash,
    StrayNumPrefix { base: NumberBase },

    // Multi-char tokens:
    LineComment,
    BlockComment,

    Identifier,
    Whitespace { no_bare_newline: bool },
    Number { base: NumberBase },

    SystemHeader,
    Header,
    CharSeq { lit_type: LitType, has_esc: bool },
    Str { lit_type: LitType, has_esc: bool },
    RawStr { lit_type: LitType, delim: DCharSeq },

    // One-char tokens:
    SemiColon,
    Pound,
    Ampersand,
    Pipe,
    Dot,
    Comma,

    QuestionMark,
    Colon,

    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Exclamation,
    Tilde,
    Caret,
    GreaterThan,
    LessThan,

    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,

    // No-char tokens:
    Eof,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberBase {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexidecimal = 16,
}

impl NumberBase {
    pub(crate) const fn matches(self, c: char) -> bool {
        match self {
            NumberBase::Binary => matches!(c, '0' | '1'),
            NumberBase::Octal => matches!(c, '0'..='7'),
            NumberBase::Decimal => c.is_ascii_digit(),
            NumberBase::Hexidecimal => {
                matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F')
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LitType {
    /// String or char literal with no prefix
    Default,
    Wide,
    Utf8,
    Utf16,
    Utf32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DCharSeq {
    pub d_char: char,
    pub count: u8,
}

impl DCharSeq {
    pub const MAX_LEN: u8 = 16;

    pub(crate) fn empty() -> Self {
        Self {
            d_char: '\0',
            count: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RawStrErr {
    NotDChar(char),
    MultipleChar { first: char, other: char },
    PrefixTooLong,
    Unterminated,
    UnterminatedInPrefix,
    UnterminatedInSuffix,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub length: u32,
    pub(crate) flags: TokenFlags,
}

impl Token {
    pub fn has_new_line(&self) -> bool {
        self.flags.contains(TokenFlags::NEWLINE)
    }

    pub fn has_universal_char(&self) -> bool {
        self.flags.contains(TokenFlags::UNIV_CHAR)
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct TokenFlags: u8 {
        const NEWLINE = (1 << 0);
        const UNIV_CHAR = (1 << 1);
    }
}
