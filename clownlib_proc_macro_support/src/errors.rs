use quote::ToTokens;

use std::fmt::Display;

pub struct ErrorsBuilder(Vec<syn::Error>);

impl ErrorsBuilder {
    pub fn new() -> ErrorsBuilder {
        Self(Vec::new())
    }

    pub fn push(&mut self, e: syn::Error) {
        self.0.push(e)
    }

    pub fn emplace<T: ToTokens, U: Display>(&mut self, tokens: T, message: U) {
        self.0.push(syn::Error::new_spanned(tokens, message))
    }

    pub fn collect(self) -> syn::Result<()> {
        self.0
            .into_iter()
            .reduce(|mut f, r| {
                f.combine(r);
                f
            })
            .map_or(Ok(()), Err)
    }
}
