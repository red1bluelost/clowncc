use proc_macro2;
use quote::ToTokens;
use syn::Fields;

pub trait ResultExt: Sized {
    fn into_token_stream2(self) -> proc_macro2::TokenStream;
}

impl<T: ToTokens> ResultExt for syn::Result<T> {
    fn into_token_stream2(self) -> proc_macro2::TokenStream {
        self.map_or_else(|e| e.into_compile_error(), |o| o.into_token_stream())
    }
}

pub trait FieldsExt {
    fn is_unit(&self) -> bool;
}

impl FieldsExt for Fields {
    fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }
}
