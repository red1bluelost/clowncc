use clown_version::StdVersion;

pub trait CharInfo: Copy {
    fn is_id_start(self) -> bool;
    fn is_id_continue(self) -> bool;
    fn is_in_basic_set(self, sv: StdVersion) -> bool;
    fn is_in_basic_literal_set(self, sv: StdVersion) -> bool;
    fn is_in_translation_set(self) -> bool;
    fn is_c_char(self, sv: StdVersion) -> bool;
    fn is_d_char(self, sv: StdVersion) -> bool;
    fn is_r_char(self) -> bool;
    fn is_s_char(self, sv: StdVersion) -> bool;
}

impl CharInfo for char {
    #[inline]
    fn is_id_start(self) -> bool {
        self == '_' || unicode_ident::is_xid_start(self)
    }

    #[inline]
    fn is_id_continue(self) -> bool {
        unicode_ident::is_xid_continue(self)
    }

    fn is_in_basic_set(self, sv: StdVersion) -> bool {
        if matches!(
            self,
            '\u{09}' // '\t'
                | '\u{0B}' // '\v'
                | '\u{0C}' // '\f'
                | '\u{20}' // ' '
                | '\u{21}' // '!'
                | '\u{22}' // '"'
                | '\u{23}' // '#'
                | '\u{25}' // '%'
                | '\u{26}' // '&'
                | '\u{27}' // '''
                | '\u{28}' // '('
                | '\u{29}' // ')'
                | '\u{2A}' // '*'
                | '\u{2B}' // '+'
                | '\u{2C}' // ','
                | '\u{2D}' // '-'
                | '\u{2E}' // '.'
                | '\u{2F}' // '/'
                | '\u{30}'..='\u{39}' // '0'..='9'
                | '\u{3A}' // ':'
                | '\u{3B}' // ';'
                | '\u{3C}' // '<'
                | '\u{3D}' // '='
                | '\u{3E}' // '>'
                | '\u{3F}' // '?'
                | '\u{41}'..='\u{5A}' // 'A'..='Z'
                | '\u{5B}' // '['
                | '\u{5C}' // '\'
                | '\u{5D}' // ']'
                | '\u{5E}' // '^'
                | '\u{5F}' // '_'
                | '\u{61}'..='\u{7A}' // 'a'..='z'
                | '\u{7B}' // '{'
                | '\u{7C}' // '|'
                | '\u{7D}' // '}'
                | '\u{7E}' // '~'
        ) {
            return true;
        }
        if sv.is_c() && matches!(self, '\u{0A}' /*'\n'*/) {
            return true;
        }
        if sv.is_cpp26()
            && matches!(
                self,
                '\u{7C}' // '|'
                    | '\u{7D}' // '}'
                    | '\u{7E}' // '~'
            )
        {
            return true;
        }
        false
    }

    fn is_in_basic_literal_set(self, sv: StdVersion) -> bool {
        matches!(
            self,
            '\0'
                | '\u{07}' // Bell
                | '\u{08}' // Backspace
                | '\r'
        ) || self.is_in_basic_set(sv)
    }

    fn is_in_translation_set(self) -> bool {
        todo!()
    }

    fn is_c_char(self, sv: StdVersion) -> bool {
        self.is_in_basic_set(sv) && !matches!(self, '\'' | '\\' | '\n')
    }

    fn is_d_char(self, sv: StdVersion) -> bool {
        self.is_in_basic_set(sv) && !matches!(self, '(' | ')' | '\\' | ' ')
    }

    fn is_r_char(self) -> bool {
        self.is_in_translation_set()
    }

    fn is_s_char(self, sv: StdVersion) -> bool {
        self.is_in_basic_set(sv) && !matches!(self, '"' | '\\' | '\n')
    }
}
