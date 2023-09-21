use quote::ToTokens;
use syn::Fields;

pub(crate) trait ResultExt: Sized {
    fn into_token_stream(self) -> proc_macro::TokenStream {
        self.into_token_stream2().into()
    }
    fn into_token_stream2(self) -> proc_macro2::TokenStream;
}

impl<T: ToTokens> ResultExt for syn::Result<T> {
    fn into_token_stream2(self) -> proc_macro2::TokenStream {
        self.map_or_else(|e| e.into_compile_error(), |o| o.into_token_stream())
    }
}

pub(crate) trait FieldsExt {
    fn is_unit(&self) -> bool;
}

impl FieldsExt for Fields {
    fn is_unit(&self) -> bool {
        matches!(self, Self::Unit)
    }
}
