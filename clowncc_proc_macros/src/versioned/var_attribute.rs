use clownlib_proc_macro_support::errors::ErrorsBuilder;

use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Meta, MetaList, Token,
};
use synstructure::VariantInfo;

const LANGS: [&str; 2] = ["C", "Cpp"];
const STD_VERSIONS: [&str; 12] = [
    "C89", "C95", "C99", "C11", "C17", "C23", "Cpp11", "Cpp14", "Cpp17",
    "Cpp20", "Cpp23", "Cpp26",
];

#[test]
fn matches_version_package() {
    use clowncc_version::{Language, StdVersion};
    use strum::IntoEnumIterator;

    let version_strings: Vec<_> =
        StdVersion::iter().map(StdVersion::as_str).collect();
    let macro_strings: Vec<_> = STD_VERSIONS
        .into_iter()
        .map(|s| s.to_lowercase().replace("pp", "++"))
        .collect();
    assert_eq!(version_strings, macro_strings);

    let version_strings: Vec<_> =
        Language::iter().map(Language::as_str).collect();
    let macro_strings: Vec<_> = LANGS
        .into_iter()
        .map(|s| s.to_lowercase().replace("pp", "++"))
        .collect();
    assert_eq!(version_strings, macro_strings);
}

fn verify_impl(id: Ident, ty: &str, supported: &[&str]) -> syn::Result<Ident> {
    let name = id.to_string();
    if name.starts_with("C++") {
        return Err(Error::new_spanned(
            id,
            format_args!(
                "unknown {} `{}`, use `Cpp` instead of `C++` only supports {}",
                name,
                ty,
                supported.join(", ")
            ),
        ));
    }
    if !supported.contains(&name.as_str()) {
        return Err(Error::new_spanned(
            id,
            format_args!(
                "unknown {} `{}`, only supports {}",
                ty,
                name,
                supported.join(", ")
            ),
        ));
    }
    Ok(id)
}

fn verify_lang(lang: Ident) -> syn::Result<Ident> {
    verify_impl(lang, "language", &LANGS)
}

fn verify_std_version(sv: Ident) -> syn::Result<Ident> {
    verify_impl(sv, "STD version", &STD_VERSIONS)
}

pub(super) struct VarAttribute<'var> {
    pub(super) var_info: &'var VariantInfo<'var>,
    pub(super) langs: Vec<Ident>,
    pub(super) sinces: Vec<Ident>,
    pub(super) untils: Vec<Ident>,
}

impl VarAttribute<'_> {
    pub(super) fn is_universal(&self) -> bool {
        [&self.langs, &self.sinces, &self.untils]
            .iter()
            .cloned()
            .all(Vec::is_empty)
    }
}

#[derive(Clone)]
enum Item {
    Universal(Ident),
    Lang(Ident, Ident),
    Since(Ident, Ident),
    Until(Ident, Ident),
}
impl Item {
    fn language(&self) -> Option<&Ident> {
        match self {
            Item::Universal(_) => None,
            Item::Lang(_, l) | Item::Since(_, l) | Item::Until(_, l) => Some(l),
        }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use Item as I;
        match self.clone() {
            I::Universal(i) => tokens.append(i),
            I::Lang(k, v) | I::Since(k, v) | I::Until(k, v) => {
                tokens.append_all([k, v])
            }
        }
    }
}

struct Items(Punctuated<Item, Token![,]>);

impl Parse for Items {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated_with(input, |input| {
            use Item as I;
            let key: Ident = input.parse()?;
            let key_str = key.to_string();
            let (ctor, verify): (fn(_, _) -> _, fn(_) -> _) =
                match key_str.as_str() {
                    "universal" => return Ok(I::Universal(key)),
                    "lang" => (I::Lang, verify_lang),
                    "since" => (I::Since, verify_std_version),
                    "until" => (I::Until, verify_std_version),
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
            let ident = verify(input.parse()?)?;
            Ok(ctor(key, ident))
        })?))
    }
}

fn check_duplication(
    item_iter: &Punctuated<Item, Token![,]>,
) -> syn::Result<()> {
    let mut errors = ErrorsBuilder::new();
    for l in LANGS {
        let mut first_lang = None;
        let mut first_since = None;
        let mut first_until = None;
        for item in item_iter.iter().filter(|item| {
            item.language().map_or(false, |i| {
                i.to_string()
                    .trim_start_matches(l)
                    .chars()
                    .next()
                    .map_or(true, |c| c.is_ascii_digit())
            })
        }) {
            match item {
                Item::Universal(_) => unreachable!("should have been filtered"),
                Item::Lang(_, _) if first_lang.is_none() => {
                    first_lang = Some(item)
                }
                Item::Since(_, _) if first_since.is_none() => {
                    first_since = Some(item)
                }
                Item::Until(_, _) if first_until.is_none() => {
                    first_until = Some(item)
                }
                Item::Lang(k, _) | Item::Since(k, _) | Item::Until(k, _) => {
                    errors.emplace(
                        item,
                        format_args!(
                            "each language may contain only one `{}`",
                            k
                        ),
                    )
                }
            }
        }
        if first_lang.and(first_since.or(first_until)).is_some() {
            errors.emplace(
                first_lang,
                "`lang` exists so `since` and `until` should not be present",
            );
        }
    }
    errors.collect()
}

fn collect_attr_from_tokens(
    tokens: TokenStream,
    langs: &mut Vec<Ident>,
    sinces: &mut Vec<Ident>,
    untils: &mut Vec<Ident>,
) -> syn::Result<()> {
    let item_list = syn::parse2::<Items>(tokens)?.0;
    if item_list.is_empty() {
        return Err(Error::new_spanned(
            item_list,
            "expected key values in the attribute list",
        ));
    }

    check_duplication(&item_list)?;

    let mut is_first = true;
    let mut is_universal = false;
    for item in item_list {
        use Item as I;
        match item {
            I::Universal(_) if is_first => is_universal = true,
            I::Lang(_, l) if !is_universal => langs.push(l),
            I::Since(_, s) if !is_universal => sinces.push(s),
            I::Until(_, u) if !is_universal => untils.push(u),
            I::Universal(i) => {
                return Err(Error::new_spanned(
                    i,
                    "unexpected universal, universal must be alone",
                ));
            }
            I::Lang(k, _) | I::Since(k, _) | I::Until(k, _) => {
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

pub(super) fn collect_attribute<'var>(
    var_info: &'var VariantInfo<'_>,
) -> syn::Result<VarAttribute<'var>> {
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
    let mut errors = ErrorsBuilder::new();
    if let Some(attr) = attr_iter.next() {
        errors.emplace(
            attr,
            "duplicate `versioned` attribute in `VariantVersion` derive",
        );
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
        errors.emplace(
            &attribute.meta,
            "unexpected attribute format, \
            use list format: i.e. `#[versioned(<options>)]`",
        );
    }

    errors.collect().map(move |()| VarAttribute {
        var_info,
        langs,
        sinces,
        untils,
    })
}
