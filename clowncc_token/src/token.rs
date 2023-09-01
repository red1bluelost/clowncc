use TokenKind::*;

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
    Whitespace { splits_lines: bool },
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
}

impl TokenKind {
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(
            self,
            Unknown | BadRawStr(_) | StrayBackSlash | StrayNumPrefix { .. }
        )
    }

    #[must_use]
    pub const fn is_multi_char(self) -> bool {
        matches!(
            self,
            Unknown
                | BadRawStr(_)
                | StrayBackSlash
                | StrayNumPrefix { .. }
                | LineComment
                | BlockComment
                | Identifier
                | Whitespace { .. }
                | Number { .. }
                | SystemHeader
                | Header
                | CharSeq { .. }
                | Str { .. }
                | RawStr { .. }
        )
    }

    #[must_use]
    pub const fn is_single_char(self) -> bool {
        matches!(
            self,
            StrayBackSlash
                | SemiColon
                | Pound
                | Ampersand
                | Pipe
                | Dot
                | Comma
                | QuestionMark
                | Colon
                | Equal
                | Plus
                | Minus
                | Star
                | Slash
                | Percent
                | Exclamation
                | Tilde
                | Caret
                | GreaterThan
                | LessThan
                | OpenParen
                | CloseParen
                | OpenBrace
                | CloseBrace
                | OpenBracket
                | CloseBracket
        )
    }

    /// Indicates multi character sequence with an open and close delimiter.
    #[must_use]
    pub const fn is_delimited(self) -> bool {
        let result = matches!(
            self,
            BlockComment
                | SystemHeader
                | Header
                | CharSeq { .. }
                | Str { .. }
                | RawStr { .. }
        );
        debug_assert!(self.is_multi_char() || !result);
        result
    }
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
    d_char: u8,
    count: u8,
}

impl DCharSeq {
    pub const MAX_LEN: u8 = 16;

    #[must_use]
    pub(crate) fn new(d_char: char, count: u8) -> Self {
        debug_assert!(d_char.is_ascii() && (count <= 16));
        let d_char = d_char as u8;
        Self { d_char, count }
    }

    #[must_use]
    pub(crate) fn empty() -> Self {
        let (d_char, count) = (0, 0);
        Self { d_char, count }
    }

    #[must_use]
    pub fn d_char(self) -> char {
        self.d_char as char
    }

    #[must_use]
    pub fn count(self) -> u8 {
        self.count
    }
}

// TODO: make an actual error type
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RawStrErr {
    NotDChar,
    PrefixMultiChar,
    PrefixTooLong,
    Unterminated,
    UnterminatedInPrefix,
    UnterminatedInSuffix,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    length: u32,
    flags: TokenFlags,
}

impl Token {
    // Constructor

    /// Constructor with debug assertions on invariants
    #[must_use]
    pub(crate) const fn new(
        kind: TokenKind,
        length: u32,
        flags: TokenFlags,
    ) -> Self {
        let token = Token {
            kind,
            length,
            flags,
        };
        debug_assert!(!kind.is_single_char() || length == 1);
        debug_assert!(kind.is_multi_char() || !token.has_new_line());
        debug_assert!(
            matches!(kind, Identifier) || !token.has_universal_char()
        );
        debug_assert!(kind.is_delimited() || !token.is_unterminated());
        token
    }

    // Accessors:
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.length
    }

    #[must_use]
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }

    // Flag Queries:

    /// Indicates if token contains a new line, escaped and unescaped
    #[must_use]
    pub const fn has_new_line(&self) -> bool {
        self.flags.contains(TokenFlags::NEWLINE)
    }

    /// Indicates if number token contains a separator
    #[must_use]
    pub const fn has_num_separator(&self) -> bool {
        self.flags.contains(TokenFlags::NUM_SEPARATOR)
    }

    /// Indicates if [`Token`] consumed a universal character.
    ///
    /// Will only occur inside a [`TokenKind::Identifier`]. [`TokenKind::Str`]
    /// and [`TokenKind::CharSeq`] may contain universal characters but the
    /// lexer treats them as any other escape.
    #[must_use]
    pub const fn has_universal_char(&self) -> bool {
        self.flags.contains(TokenFlags::UNIV_CHAR)
    }

    /// Indicates if token consumed is not terminated by a closing delimiter.
    /// Will only occur for delimited token kinds.
    #[must_use]
    pub const fn is_unterminated(&self) -> bool {
        self.flags.contains(TokenFlags::UNTERMINATED)
    }
}

bitflags::bitflags! {
    /// Packed booleans for [`Token`] to indicate edge scenarios that may need
    /// processing.
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct TokenFlags: u8 {
        /// Indicates a new line was consumed within the token.
        const NEWLINE = (1 << 0);
        /// Indicates a number contains at least one separator
        const NUM_SEPARATOR = (1 << 1);
        /// Indicates a valid universal character was consumed within the token.
        const UNIV_CHAR = (1 << 2);
        /// Indicates if delimited token has a closing delimiter.
        const UNTERMINATED = (1 << 3);
    }
}
