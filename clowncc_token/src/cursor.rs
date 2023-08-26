use QuoteType::*;

use crate::token::{LitType, NumberBase, RawStrErr, Token, TokenFlags};
use crate::{CharInfo, DCharSeq, TokenKind};

use std::str::Chars;

use clowncc_version::StdVersion;

type TK = TokenKind;

/// Representing a single character delimited string
#[derive(Copy, Clone, Eq, PartialEq)]
enum QuoteType {
    /// Include string for system header files: `<file>`
    SysHeader,
    /// Include string for regular header files: `"file"`
    Header,
    /// String literal (not raw): `"chars"`
    String,
    /// Character sequence: `'chars'`
    CharSeq,
}

impl QuoteType {
    /// Checks if a character is the terminator for the given quote type
    const fn matches_terminator(self, c: char) -> bool {
        matches!(
            (self, c),
            (SysHeader, '>') | (String | Header, '"') | (CharSeq, '\'')
        )
    }
}

clowncc_macros::define_yes_no! {
    /// When parsing a universal character or escaped newline, the caller may
    /// want the initial backslash consumed or not consumed during the
    /// speculative parse.
    EatSlash;
}
clowncc_macros::define_yes_no! {
    /// Tokenizing has no state so the caller must choose when to treat quotes
    /// `"` or angle brackets `<` as headers or regular tokens.
    ExpectHeader;
}

#[derive(Clone)]
struct TokenBuilder {
    start_len_from_end: u32,
    flags: TokenFlags,
}

impl TokenBuilder {
    fn set_newline(&mut self) {
        self.flags |= TokenFlags::NEWLINE;
        let _ = true < false;
    }

    fn set_univ_char(&mut self) {
        self.flags |= TokenFlags::UNIV_CHAR;
    }

    fn set_unterminated(&mut self) {
        self.flags |= TokenFlags::UNTERMINATED;
    }

    const fn build(self, kind: TokenKind, end_len_from_end: u32) -> Token {
        let Self {
            start_len_from_end,
            flags,
        } = self;
        Token::new(kind, start_len_from_end - end_len_from_end, flags)
    }
}

pub struct Cursor<'chars> {
    chars: Chars<'chars>,
    std_vers: StdVersion,
    #[cfg(debug_assertions)]
    cur_char: char,
}

impl<'chars> Cursor<'chars> {
    #[must_use]
    pub fn new(code: &'chars str, std_vers: StdVersion) -> Cursor<'chars> {
        Cursor {
            chars: code.chars(),
            std_vers,
            #[cfg(debug_assertions)]
            cur_char: '\0',
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.next_token_impl(ExpectHeader::No)
    }

    pub fn next_token_header(&mut self) -> Option<Token> {
        self.next_token_impl(ExpectHeader::Yes)
    }

    #[must_use]
    fn peek_first(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn next_char(&mut self, tb: &mut TokenBuilder) -> Option<char> {
        let c = self.chars.next()?;
        if c == '\n' {
            tb.set_newline();
        }
        #[cfg(debug_assertions)]
        (self.cur_char = c);
        Some(c)
    }

    fn eat_while(
        &mut self,
        tb: &mut TokenBuilder,
        mut predicate: impl FnMut(char) -> bool,
    ) -> Option<char> {
        loop {
            let c = self.peek_first()?;
            if !predicate(c) {
                return Some(c);
            }
            self.next_char(tb);
        }
    }

    #[must_use]
    fn len_from_end(&self) -> u32 {
        self.chars
            .as_str()
            .len()
            .try_into()
            .expect("Input too large to handle")
    }

    #[must_use]
    fn make_token_builder(&self) -> TokenBuilder {
        TokenBuilder {
            start_len_from_end: self.len_from_end(),
            flags: TokenFlags::empty(),
        }
    }

    fn next_token_impl(&mut self, header: ExpectHeader) -> Option<Token> {
        let mut token_builder = self.make_token_builder();
        let tb = &mut token_builder;

        let kind = match self.next_char(tb)? {
            c if c.is_whitespace() => self.eat_whitespace(c == '\n', tb),

            '/' => match self.peek_first() {
                Some('/') => self.eat_line_comment(tb),
                Some('*') => self.eat_block_comment(tb),
                Some(_) | None => TK::Slash,
            },

            'L' => self.eat_lit_or_identifier(LitType::Wide, tb),
            'U' if self.std_vers.is_since_c11()
                || self.std_vers.is_since_cpp11() =>
            {
                self.eat_lit_or_identifier(LitType::Utf32, tb)
            }
            'u' if self.std_vers.is_since_c11()
                || self.std_vers.is_since_cpp11() =>
            {
                self.eat_lit_or_identifier_u(tb)
            }
            'R' if self.std_vers.is_since_cpp11() => {
                self.eat_raw_str_or_identifier(LitType::Default, tb)
            }

            c if c.is_id_start() => self.eat_identifier(tb),
            c @ '0'..='9' => self.eat_numbers(c, tb),

            '"' if header.is_yes() => {
                self.eat_quoted_list(Header, LitType::Default, tb)
            }
            '<' if header.is_yes() => {
                self.eat_quoted_list(SysHeader, LitType::Default, tb)
            }

            '"' => self.eat_quoted_list(String, LitType::Default, tb),
            '\'' => self.eat_quoted_list(CharSeq, LitType::Default, tb),

            ';' => TK::SemiColon,
            '#' => TK::Pound,
            '&' => TK::Ampersand,
            '|' => TK::Pipe,
            '.' => TK::Dot,
            ',' => TK::Comma,

            '?' => TK::QuestionMark,
            ':' => TK::Colon,

            '=' => TK::Equal,
            '+' => TK::Plus,
            '-' => TK::Minus,
            '*' => TK::Star,
            '%' => TK::Percent,
            '!' => TK::Exclamation,
            '~' => TK::Tilde,
            '^' => TK::Caret,

            '<' => TK::GreaterThan,
            '>' => TK::LessThan,

            '(' => TK::OpenParen,
            ')' => TK::CloseParen,
            '{' => TK::OpenBrace,
            '}' => TK::CloseBrace,
            '[' => TK::OpenBracket,
            ']' => TK::CloseBracket,

            '\\' => match self.peek_first() {
                Some('u' | 'U' | '\\')
                    if self.try_eat_universal_char(EatSlash::No, tb) =>
                {
                    self.eat_identifier(tb)
                }
                Some(c) if c.is_whitespace() => {
                    debug_assert!(!self.try_eat_esc_newline(EatSlash::No, tb));
                    TK::StrayBackSlash
                }
                Some(_) | None => TK::StrayBackSlash,
            },
            _ => TK::Unknown,
        };

        Some(token_builder.build(kind, self.len_from_end()))
    }

    fn eat_line_comment(&mut self, tb: &mut TokenBuilder) -> TokenKind {
        debug_assert!(self.cur_char == '/' && self.peek_first() == Some('/'));

        while let Some(c) = self.eat_while(tb, |c| !matches!(c, '\\' | '\n')) {
            match c {
                '\n' => break,
                '\\' => {
                    self.next_char(tb);
                    self.try_eat_esc_newline(EatSlash::No, tb);
                }
                _ => unreachable!(),
            }
        }

        debug_assert!(matches!(self.peek_first(), Some('\n') | None));
        TK::LineComment
    }

    fn eat_block_comment(&mut self, tb: &mut TokenBuilder) -> TokenKind {
        debug_assert!(self.cur_char == '/' && self.peek_first() == Some('*'));
        self.next_char(tb); // Consume first star as part of opener
        while let Some(c) = self.next_char(tb) {
            if c != '*' || self.peek_first() != Some('/') {
                continue;
            }
            self.next_char(tb);
            return TK::BlockComment;
        }
        tb.set_unterminated();
        TK::BlockComment
    }

    fn eat_whitespace(
        &mut self,
        mut splits_lines: bool,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(self.cur_char.is_whitespace());
        while let Some('\\') = self.eat_while(tb, |c| {
            splits_lines = splits_lines || c == '\n';
            c.is_whitespace()
        }) {
            if !self.try_eat_esc_newline(EatSlash::Yes, tb) {
                break;
            }
        }
        TK::Whitespace { splits_lines }
    }

    fn eat_identifier(&mut self, tb: &mut TokenBuilder) -> TokenKind {
        while let Some('\\') = self.eat_while(tb, char::is_id_continue) {
            if !self.try_eat_universal_char(EatSlash::Yes, tb) {
                break;
            }
        }
        TK::Identifier
    }

    fn eat_numbers(
        &mut self,
        first_char: char,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(
            self.cur_char.is_ascii_digit() && self.cur_char == first_char
        );
        let base = if first_char == '0' {
            self.eat_number_base(tb)
        } else {
            NumberBase::Decimal
        };

        if base != NumberBase::Decimal
            && self.peek_first().map_or(true, |c| base.matches(c))
        {
            return TK::StrayNumPrefix { base };
        }

        self.eat_while(tb, |c| base.matches(c));

        TK::Number { base }
    }

    fn eat_number_base(&mut self, tb: &mut TokenBuilder) -> NumberBase {
        debug_assert!(self.cur_char == '0');
        let base = match self.peek_first() {
            Some('b' | 'B') => NumberBase::Binary,
            Some('0') => NumberBase::Octal,
            Some('x' | 'X') => NumberBase::Hexidecimal,
            // Return here to not eat the non-number character that follows
            Some(_) | None => return NumberBase::Decimal,
        };
        self.next_char(tb);
        base
    }

    fn eat_lit_or_identifier(
        &mut self,
        prefix: LitType,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(matches!(
            (self.cur_char, prefix),
            ('L', LitType::Wide)
                | ('8', LitType::Utf8)
                | ('u', LitType::Utf16)
                | ('U', LitType::Utf32)
        ));
        match self.peek_first() {
            None => TK::Identifier,
            // TODO: gaurd based on stdversion
            Some('\'') => self.eat_quoted_list(CharSeq, prefix, tb),
            Some('"') => self.eat_quoted_list(String, prefix, tb),
            Some('R') => self.eat_raw_string(prefix, tb),
            Some(_) => self.eat_identifier(tb),
        }
    }

    fn eat_lit_or_identifier_u(&mut self, tb: &mut TokenBuilder) -> TokenKind {
        debug_assert!(self.cur_char == 'u');
        match self.peek_first() {
            None => TK::Identifier,
            Some('8') => {
                self.next_char(tb);
                self.eat_lit_or_identifier(LitType::Utf8, tb)
            }
            Some(_) => self.eat_lit_or_identifier(LitType::Utf16, tb),
        }
    }

    fn eat_quoted_list(
        &mut self,
        quote_ty: QuoteType,
        lit_type: LitType,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(matches!(
            (quote_ty, lit_type, self.cur_char),
            (CharSeq, _, '\'')
                | (String, _, '"')
                | (SysHeader, LitType::Default, '<')
                | (Header, LitType::Default, '"')
        ));
        let mut has_esc = false;
        let unterminated = loop {
            match self.next_char(tb) {
                // Stop tokenizing if non-escaped newline is encountered.
                // This is an unterminated string which is always an error. We
                // are fine with consuming the new line since it will error
                // later.
                Some('\n') | None => break true,
                // Close quote if character matches terminator
                Some(c) if quote_ty.matches_terminator(c) => {
                    break false;
                }
                Some('\\') => {
                    has_esc = true;
                    match self.peek_first() {
                        None | Some('\\') => {}
                        Some(c) if c.is_whitespace() => {
                            self.try_eat_esc_newline(EatSlash::No, tb);
                        }
                        Some(_) => {
                            self.next_char(tb);
                        }
                    }
                }
                _ => continue,
            }
        };
        if unterminated {
            tb.set_unterminated();
        }
        match quote_ty {
            String => TK::Str { lit_type, has_esc },
            CharSeq => TK::CharSeq { lit_type, has_esc },
            SysHeader => TK::SystemHeader,
            Header => TK::Header,
        }
    }

    fn eat_raw_str_or_identifier(
        &mut self,
        prefix: LitType,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(self.cur_char == 'R');
        match self.peek_first() {
            None => TK::Identifier,
            Some('"') => self.eat_raw_string(prefix, tb),
            Some(_) => self.eat_identifier(tb),
        }
    }

    fn eat_raw_string(
        &mut self,
        prefix: LitType,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        debug_assert!(self.cur_char == 'R' && self.peek_first() == Some('"'));
        self.next_char(tb); // Consume the starting quote

        let d_char_seq = match self.eat_raw_d_char_prefix(tb) {
            Ok(dcs) => dcs,
            Err(invalid) => return invalid,
        };
        debug_assert!(self.cur_char == '(');

        self.eat_raw_str_after_prefix(d_char_seq, prefix, tb)
    }

    fn eat_raw_d_char_prefix(
        &mut self,
        tb: &mut TokenBuilder,
    ) -> Result<DCharSeq, TokenKind> {
        debug_assert!(self.cur_char == '"');
        let prefix_start = self.len_from_end();
        let prefix_char = match self.next_char(tb) {
            Some('(') => return Ok(DCharSeq::empty()),
            Some(c) if c.is_d_char(self.std_vers) => c,
            Some(_) => return Err(TK::BadRawStr(RawStrErr::NotDChar)),
            None => return Err(TK::BadRawStr(RawStrErr::UnterminatedInPrefix)),
        };

        let last_char = self.eat_while(tb, |c| c == prefix_char);

        let prefix_len_so_far = prefix_start - self.len_from_end();
        if prefix_len_so_far > DCharSeq::MAX_LEN.into() {
            return Err(TK::BadRawStr(RawStrErr::PrefixTooLong));
        }

        #[allow(clippy::cast_possible_truncation)]
        let delim = DCharSeq::new(prefix_char, prefix_len_so_far as u8);
        match last_char {
            Some('(') => {
                self.next_char(tb);
                Ok(delim)
            }
            Some(_) => Err(TK::BadRawStr(RawStrErr::PrefixMultiChar)),
            None => Err(TK::BadRawStr(RawStrErr::UnterminatedInPrefix)),
        }
    }

    fn eat_raw_str_after_prefix(
        &mut self,
        delim: DCharSeq,
        lit_type: LitType,
        tb: &mut TokenBuilder,
    ) -> TokenKind {
        loop {
            match self.eat_while(tb, |c| c != ')') {
                None => return TK::BadRawStr(RawStrErr::Unterminated),
                Some(')') => {}
                Some(_) => unreachable!(),
            }
            if let Some(result) = self.eat_raw_str_suffix(delim, lit_type, tb) {
                return result;
            }
        }
    }

    fn eat_raw_str_suffix(
        &mut self,
        delim: DCharSeq,
        lit_type: LitType,
        tb: &mut TokenBuilder,
    ) -> Option<TokenKind> {
        let start = self.len_from_end();
        let expected = delim.count().into();
        match self.eat_while(tb, |c| c == delim.d_char()) {
            Some('"') if start - self.len_from_end() != expected => None,
            Some('"') => {
                self.next_char(tb);
                Some(TK::RawStr { lit_type, delim })
            }
            Some(_) => None,
            None => Some(TK::BadRawStr(RawStrErr::UnterminatedInSuffix)),
        }
    }

    fn try_eat_esc_newline(
        &mut self,
        eat_slash: EatSlash,
        tb: &mut TokenBuilder,
    ) -> bool {
        let mut chars_dup = self.chars.clone();
        let mut tb_dup = tb.clone();
        if eat_slash.is_yes() && self.peek_first() == Some('\\') {
            self.next_char(tb);
        }
        while let Some('\n') =
            self.eat_while(tb, |c| c.is_whitespace() && c != '\n')
        {
            self.next_char(tb); // eat new line
            if self.peek_first() == Some('\\') {
                chars_dup = self.chars.clone();
                tb_dup = tb.clone();
                self.next_char(tb); // eat slash
            } else {
                return true;
            }
        }
        *tb = tb_dup;
        self.chars = chars_dup;
        false
    }

    #[cold]
    #[inline(never)]
    fn try_eat_universal_char(
        &mut self,
        eat_slash: EatSlash,
        tb: &mut TokenBuilder,
    ) -> bool {
        fn internal(this: &mut Cursor<'_>, tb: &mut TokenBuilder) -> bool {
            let start_char = match this.peek_first() {
                Some('\\') if this.try_eat_esc_newline(EatSlash::Yes, tb) => {
                    this.peek_first()
                }
                c => c,
            };
            let max_count = match start_char {
                Some('u') => 4,
                Some('U') => 8,
                Some(_) | None => {
                    return false;
                }
            };
            this.next_char(tb);
            let mut count = 0;
            while let Some(c) = this.next_char(tb) {
                match c {
                    c if NumberBase::Hexidecimal.matches(c) => {
                        count += 1;
                        if count == max_count {
                            return true;
                        }
                    }
                    '\\' if this.try_eat_esc_newline(EatSlash::No, tb) => {
                        continue
                    }
                    _ => break,
                }
            }
            false
        }

        let chars_dup = self.chars.clone();
        let tb_dup = tb.clone();
        if eat_slash.is_yes() && self.peek_first() == Some('\\') {
            self.next_char(tb);
        }

        let result = internal(self, tb);
        if result {
            tb.set_univ_char();
        } else {
            *tb = tb_dup;
            self.chars = chars_dup;
        }
        result
    }
}
