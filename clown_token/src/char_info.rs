pub trait CharInfo: Copy {
    fn is_id_start(self) -> bool;
    fn is_id_continue(self) -> bool;
    fn is_in_basic_set(self) -> bool;
    fn is_in_basic_literal_set(self) -> bool;
    fn is_in_translation_set(self) -> bool;
    fn is_d_char(self) -> bool;
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

    fn is_in_basic_set(self) -> bool {
        todo!()
    }

    fn is_in_basic_literal_set(self) -> bool {
        todo!()
    }

    fn is_in_translation_set(self) -> bool {
        todo!()
    }

    fn is_d_char(self) -> bool {
        todo!()
    }
}
