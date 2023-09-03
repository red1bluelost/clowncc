mod var_attribute;

use var_attribute::{collect_attribute, VarAttribute};

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;
use synstructure::Structure;

fn collect_errors(errors: Vec<Error>) -> syn::Result<()> {
    errors
        .into_iter()
        .reduce(|mut f, r| {
            f.combine(r);
            f
        })
        .map_or(Ok(()), Err)
}

fn gen_std_version_condition(
    langs: TokenStream,
    sinces: TokenStream,
    untils: TokenStream,
) -> TokenStream {
    quote! {
        const __langs: &[::clowncc_version::Language] = &[#langs];
        const __sinces: &[::clowncc_version::StdVersion] = &[#sinces];
        const __untils: &[::clowncc_version::StdVersion] = &[#untils];
        let __lang_iter = __langs.iter().filter(|&&l| l == __lang);
        let __since_iter = __sinces.iter().filter(|&&sv| *sv == __lang);
        let __until_iter = __untils.iter().filter(|&&sv| *sv == __lang);
        let __lang_count = __lang_iter.clone().count();
        let __since_count = __since_iter.clone().count();
        let __until_count = __until_iter.clone().count();
        debug_assert!(
            __lang_count <= 1,
            "unexpected `lang` duplicates in `versioned` attribute",
        );
        debug_assert!(
            __since_count <= 1,
            "unexpected `since` duplicates in `versioned` attribute",
        );
        debug_assert!(
            __until_count <= 1,
            "unexpected `until` duplicates in `versioned` attribute",
        );
        let __since_opt = __since_iter.clone().next();
        let __until_opt = __until_iter.clone().next();
        match (__since_count, __until_count) {
            (0, 0) => __lang_count == 1,
            (1, 0) => sv.is_since(*__since_opt.unwrap()),
            (0, 1) => sv.is_before(*__until_opt.unwrap()),
            (1, 1) => {
                sv.is_since(*__since_opt.unwrap())
                    && sv.is_before(*__until_opt.unwrap())
            }
            _ => unreachable!(),
        }
    }
}

fn gen_language_condition(
    langs: TokenStream,
    sinces: TokenStream,
    untils: TokenStream,
) -> TokenStream {
    quote! {
        const __langs: &[::clowncc_version::Language] = &[#langs];
        const __sinces: &[::clowncc_version::StdVersion] = &[#sinces];
        const __untils: &[::clowncc_version::StdVersion] = &[#untils];
        let __lang_count = __langs.iter().filter(|&&l| l == lang).count();
        let __since_count = __sinces.iter().filter(|&&sv| *sv == lang).count();
        let __until_count = __untils.iter().filter(|&&sv| *sv == lang).count();
        debug_assert!(
            __lang_count <= 1,
            "unexpected `lang` duplicates in `versioned` attribute",
        );
        debug_assert!(
            __since_count <= 1,
            "unexpected `since` duplicates in `versioned` attribute",
        );
        debug_assert!(
            __until_count <= 1,
            "unexpected `until` duplicates in `versioned` attribute",
        );
        __lang_count != 0 || __since_count != 0 || __until_count != 0
    }
}

fn generate_body(
    var_attrs: &[VarAttribute],
    generate: impl Fn(TokenStream, TokenStream, TokenStream) -> TokenStream,
) -> TokenStream {
    let mut stream = TokenStream::new();
    for va @ VarAttribute {
        var_info,
        langs,
        sinces,
        untils,
    } in var_attrs
    {
        let pat = var_info.pat();
        if va.is_universal() {
            stream.extend(quote!(#pat => { true }));
            continue;
        }
        let langs = langs
            .iter()
            .map(|lang| quote!(::clowncc_version::Language::#lang,))
            .collect();
        let sinces = sinces
            .iter()
            .map(|since| quote!(::clowncc_version::StdVersion::#since,))
            .collect();
        let untils = untils
            .iter()
            .map(|until| quote!(::clowncc_version::StdVersion::#until,))
            .collect();

        let body = generate(langs, sinces, untils);
        stream.extend(quote!(#pat => { #body }));
    }
    stream
}

pub fn versioned(definition: Structure) -> syn::Result<TokenStream> {
    let num_variants = definition.variants().len();
    if num_variants == 0 {
        return Err(Error::new_spanned(
            definition.ast(),
            "does not support zero variant enums",
        ));
    }

    let mut var_attrs = Vec::with_capacity(num_variants);
    let mut errors = Vec::new();
    for res in definition.variants().iter().map(collect_attribute) {
        res.map_or_else(|e| errors.push(e), |va| var_attrs.push(va));
    }

    collect_errors(errors)?;

    let std_version_body = generate_body(&var_attrs, gen_std_version_condition);
    let language_body = generate_body(&var_attrs, gen_language_condition);

    Ok(definition.gen_impl(quote! {
        extern crate clowncc_version;
        gen impl ::clowncc_version::StdVersionSupported for @Self {
            fn is_in_std_version(
                &self,
                sv: ::clowncc_version::StdVersion
            ) -> bool {
                let __lang = sv.as_language();
                match self { #std_version_body }
            }
        }

        gen impl ::clowncc_version::LanguageSupported for @Self {
            fn is_in_language(
                &self,
                lang: ::clowncc_version::Language
            ) -> bool {
                match self { #language_body }
            }
        }
    }))
}
