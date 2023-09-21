use crate::syn_ext;
use crate::synstructure_ext::VariantInfoExt;
use std::collections::hash_map::{Entry, HashMap};

use crate::errors::ErrorsBuilder;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataEnum, Error, Expr, ExprLit, Fields, Lit, LitStr, Meta,
    MetaNameValue,
};
use synstructure::{Structure, VariantInfo};

fn collect_keywords(data_enum: &DataEnum) -> syn::Result<Vec<&LitStr>> {
    let mut strings = Vec::with_capacity(data_enum.variants.len());
    let mut errors = ErrorsBuilder::new();
    for variant in &data_enum.variants {
        if matches!(variant.fields, Fields::Named(_) | Fields::Unnamed(_)) {
            errors.emplace(&variant.fields, "variant should be a unit")
        }

        let mut attr_iter = variant
            .attrs
            .iter()
            .filter(|a| a.path().is_ident("keyword"));
        let Some(attribute) = attr_iter.next() else {
            errors.emplace(
                &variant.ident,
                "variant missing attribute with name `keyword`",
            );
            continue;
        };
        if let Some(attr) = attr_iter.next() {
            errors.emplace(
                attr,
                "duplicate `keyword` attribute in `KeywordEnum` derive",
            );
        }

        match &attribute.meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        attrs,
                        lit: Lit::Str(lit_str),
                    }),
                ..
            }) if attrs.is_empty() && lit_str.suffix().is_empty() => {
                strings.push(lit_str)
            }
            _ => errors.emplace(
                &attribute.meta,
                r#"expected format #[keyword = "word"]"#,
            ),
        }
    }
    errors.collect().map(move |()| strings)
}

fn check_no_duplicates(lit_strs: &[&LitStr]) -> syn::Result<()> {
    let mut str_to_lit = HashMap::new();
    let mut errors = ErrorsBuilder::new();
    for &ls in lit_strs {
        let string = ls.value();
        if string == "" {
            errors.emplace(ls, "empty string not allowed in KeywordEnum");
        }
        match str_to_lit.entry(string) {
            Entry::Vacant(v) => {
                v.insert(ls);
            }
            Entry::Occupied(_) => errors
                .emplace(ls, "duplicate keywords not allowed in KeywordEnum"),
        }
    }
    errors.collect()
}

pub(crate) fn keyword_enum(structure: Structure) -> syn::Result<TokenStream> {
    let data_enum = match &structure.ast().data {
        Data::Struct(ds) => {
            return Err(Error::new_spanned(
                ds.struct_token,
                "expected an enum",
            ));
        }
        Data::Union(_) => unreachable!("synstructure does not accept unions"),
        Data::Enum(de) => de,
    };

    let strings = collect_keywords(&data_enum)?;
    let type_name = &structure.ast().ident;

    let map_elms: TokenStream = structure
        .variants()
        .iter()
        .map(VariantInfo::construct_unit)
        .zip(&strings)
        .map(|(v, &s)| quote!(#s => #v,))
        .collect();

    Ok(structure.gen_impl(quote! {
        extern crate phf;
        impl #type_name {
            pub const KEYWORDS: phf::Map<&'static str, Self> = phf::phf_map! {
                #map_elms
            };
        }

        gen impl ::std::str::FromStr for @Self {
            type Err = ();
            fn from_str(s: &str) -> ::core::result::Result<Self, ()> {
                Self::KEYWORDS.get(s).cloned().ok_or(())
            }
        }
    }))
}
