mod var_attribute;

use var_attribute::VarAttribute;

use clownlib_proc_macro_support::errors::ErrorsBuilder;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;
use synstructure::Structure;

fn gen_std_version_condition(
    langs: TokenStream,
    sinces: TokenStream,
    untils: TokenStream,
) -> TokenStream {
    quote! {
        const __langs: &[::clowncc_version::Language] = &[#langs];
        const __sinces: &[::clowncc_version::StdVersion] = &[#sinces];
        const __untils: &[::clowncc_version::StdVersion] = &[#untils];
        let __lang = __langs.iter().filter(|&&l| l == __svlang).next();
        let __since = __sinces.iter().filter(|&&sv| *sv == __svlang).next();
        let __until = __untils.iter().filter(|&&sv| *sv == __svlang).next();
        match (__since, __until) {
            (None, None) => __lang.is_some(),
            (Some(&ssv), None) => sv.is_since(ssv),
            (None, Some(&usv)) => sv.is_before(usv),
            (Some(&ssv), Some(&usv)) => sv.is_since(ssv) && sv.is_before(usv),
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
        let __lang = __langs.iter().filter(|&&l| l == lang).next();
        let __since = __sinces.iter().filter(|&&sv| *sv == lang).next();
        let __until = __untils.iter().filter(|&&sv| *sv == lang).next();
        __lang.is_some() || __since.is_some() || __until.is_some()
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

pub(crate) fn versioned(definition: Structure) -> syn::Result<TokenStream> {
    let num_variants = definition.variants().len();
    if num_variants == 0 {
        return Err(Error::new_spanned(
            definition.ast(),
            "does not support zero variant enums",
        ));
    }

    let mut var_attrs = Vec::with_capacity(num_variants);
    let mut errors = ErrorsBuilder::new();
    definition
        .variants()
        .iter()
        .map(var_attribute::collect_attribute)
        .for_each(|res| {
            res.map_or_else(|e| errors.push(e), |va| var_attrs.push(va))
        });

    errors.collect()?;

    let std_version_body = generate_body(&var_attrs, gen_std_version_condition);
    let language_body = generate_body(&var_attrs, gen_language_condition);

    Ok(definition.gen_impl(quote! {
        extern crate clowncc_version;
        gen impl ::clowncc_version::StdVersionSupported for @Self {
            fn is_in_std_version(
                &self,
                sv: ::clowncc_version::StdVersion
            ) -> bool {
                let __svlang = sv.as_language();
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
