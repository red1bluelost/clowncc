mod keyword_enum;
mod versioned;

use synstructure::{decl_attribute, decl_derive};

decl_derive!([Versioned, attributes(versioned)] => versioned::versioned);
decl_derive!([KeywordEnum, attributes(keyword)] => keyword_enum::keyword_enum);

decl_attribute!([scratch] => crate::scratch_impl);

fn scratch_impl(
    attr: proc_macro2::TokenStream,
    s: synstructure::Structure,
) -> proc_macro2::TokenStream {
    println!("scratch:");
    println!("{:?}", attr);
    println!("{:?}", s);
    let s = s.ast();
    quote::quote!(
        const YEET: &str = stringify!(#s);
    )
}
