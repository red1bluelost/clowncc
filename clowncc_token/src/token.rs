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

    Identifier { has_univ_char: bool },
    Whitespace { splits_lines: bool },
    Number { base: NumberBase, has_sep: bool },

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
                | Identifier { .. }
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
        debug_assert!(kind.is_multi_char() || !flags.has_new_line());
        debug_assert!(kind.is_delimited() || !flags.is_unterminated());
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

    #[must_use]
    pub const fn flags(&self) -> TokenFlags {
        self.flags
    }
    // Flag Queries:
}

bitflags::bitflags! {
    /// Packed booleans for [`Token`] to indicate edge scenarios that may need
    /// processing. These flags should occur in multiple kinds of tokens,
    /// otherwise it is best to encode within the [`TokenKind`] variant.
    #[derive(Copy, Clone, Debug)]
    pub struct TokenFlags: u8 {
        /// Indicates a new line was consumed within the token.
        const NEWLINE = (1 << 0);
        /// Indicates if delimited token has a closing delimiter.
        const UNTERMINATED = (1 << 1);
    }
}

impl TokenFlags {
    /// Indicates if token contains a new line, escaped and unescaped
    #[must_use]
    pub const fn has_new_line(self) -> bool {
        self.contains(Self::NEWLINE)
    }

    /// Indicates if token consumed is not terminated by a closing delimiter.
    /// Will only occur for delimited token kinds.
    #[must_use]
    pub const fn is_unterminated(self) -> bool {
        self.contains(Self::UNTERMINATED)
    }
}
