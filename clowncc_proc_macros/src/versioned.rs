use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Error, Meta, MetaList, Token};
use synstructure::{Structure, VariantInfo};

struct VarAttribute<'var, 'tok> {
    var_info: &'var VariantInfo<'tok>,
    langs: Vec<Ident>,
    sinces: Vec<Ident>,
    untils: Vec<Ident>,
}

impl VarAttribute<'_, '_> {
    fn is_universal(&self) -> bool {
        [&self.langs, &self.sinces, &self.untils]
            .iter()
            .cloned()
            .all(Vec::is_empty)
    }
}

fn collect_errors(errors: Vec<Error>) -> syn::Result<()> {
    errors
        .into_iter()
        .reduce(|mut f, r| {
            f.combine(r);
            f
        })
        .map_or(Ok(()), Err)
}

fn collect_attr_from_tokens(
    tokens: TokenStream,
    langs: &mut Vec<Ident>,
    sinces: &mut Vec<Ident>,
    untils: &mut Vec<Ident>,
) -> syn::Result<()> {
    #[derive(Clone)]
    enum Item {
        Universal(Ident),
        Lang(Ident, Ident),
        Since(Ident, Ident),
        Until(Ident, Ident),
    }
    impl ToTokens for Item {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            use Item::*;
            match self.clone() {
                Universal(i) => tokens.append(i),
                Lang(k, v) | Since(k, v) | Until(k, v) => {
                    tokens.append_all([k, v])
                }
            }
        }
    }
    struct Items(Punctuated<Item, Token![,]>);
    impl Parse for Items {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self(Punctuated::parse_terminated_with(input, |input| {
                use Item::*;
                let key: Ident = input.parse()?;
                let key_str = key.to_string();
                let ctor = match key_str.as_str() {
                    "universal" => return Ok(Universal(key)),
                    "lang" => Lang,
                    "since" => Since,
                    "until" => Until,
                    "before" => {
                        return Err(Error::new_spanned(
                            key,
                            "unknown key, perhaps meant `until`, \
                            only supports [universal, lang, since, until]",
                        ))
                    }
                    _ => {
                        return Err(Error::new_spanned(
                            key,
                            "unknown key, only supports \
                            [universal, lang, since, until]",
                        ))
                    }
                };
                Ok(ctor(key, input.parse()?))
            })?))
        }
    }

    let item_list = syn::parse2::<Items>(tokens)?.0;
    if item_list.is_empty() {
        return Err(Error::new_spanned(
            item_list,
            "expected key values in the attribute list",
        ));
    }

    let mut is_first = true;
    let mut is_universal = false;
    for item in item_list {
        use Item::*;
        match item {
            Universal(_) if is_first => is_universal = true,
            Lang(_, l) if !is_universal => langs.push(l),
            Since(_, s) if !is_universal => sinces.push(s),
            Until(_, u) if !is_universal => untils.push(u),
            Universal(i) => {
                return Err(Error::new_spanned(
                    i,
                    "unexpected universal, universal must be alone",
                ));
            }
            Lang(k, _) | Since(k, _) | Until(k, _) => {
                return Err(Error::new_spanned(
                    k,
                    "unexpected key, either remove `universal` or \
                    remove all other key values",
                ));
            }
        };
        is_first = false;
    }
    Ok(())
}

fn collect_attribute<'var, 'tok>(
    var_info: &'var VariantInfo<'tok>,
) -> syn::Result<VarAttribute<'var, 'tok>> {
    let mut attr_iter = var_info
        .ast()
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("versioned"));
    let attribute = attr_iter.next().ok_or_else(|| {
        Error::new_spanned(
            var_info.ast().ident,
            "variant missing attribute with name `versioned`",
        )
    })?;
    let mut errors = Vec::new();
    if let Some(attr) = attr_iter.next() {
        errors.push(Error::new_spanned(
            attr,
            "duplicate `versioned` attribute in `VariantVersion` derive",
        ));
    }

    let mut langs = vec![];
    let mut sinces = vec![];
    let mut untils = vec![];
    if let Meta::List(MetaList { tokens, .. }) = &attribute.meta {
        collect_attr_from_tokens(
            tokens.clone(),
            &mut langs,
            &mut sinces,
            &mut untils,
        )
        .map_err(|e| errors.push(e))
        .ok();
    } else {
        errors.push(Error::new_spanned(
            &attribute.meta,
            "unexpected attribute format, \
            use list format: i.e. `#[versioned(<options>)]`",
        ));
    }

    collect_errors(errors)?;

    Ok(VarAttribute {
        var_info,
        langs,
        sinces,
        untils,
    })
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
