use darling::{util::Override, Error, FromField, FromMeta, Result, ToTokens};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Expr, GenericArgument, Ident, LitStr, Path, PathArguments, Type};

use crate::common::{ClapDocCommon, ClapDocCommonAuto, ClapDocHelpMarker, ClapTokensResult};

use super::RenameAll;

enum ClapArgType {
    Bool,
    OptionT,
    OptionOptionT,
    T,
    VecT,
    OptionVecT,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(clap), forward_attrs(doc))]
pub(crate) struct ClapField {
    pub ident: Option<Ident>,
    pub ty: syn::Type,
    pub attrs: Vec<syn::Attribute>,

    #[darling(default)]
    pub name: Option<String>,
    #[darling(default)]
    pub help: Option<String>,
    #[darling(default)]
    pub long_help: Option<String>,
    #[allow(dead_code)]
    #[darling(default)]
    pub verbatim_doc_comment: bool,
    #[darling(default)]
    pub short: Option<Override<String>>,
    #[darling(default)]
    pub long: Option<Override<String>>,
    #[darling(default)]
    pub env: Option<Override<String>>,
    #[darling(default)]
    pub flatten: Option<Override<String>>,
    #[darling(default)]
    pub subcommand: bool,
    #[allow(dead_code)]
    #[darling(default)]
    pub from_global: bool,
    #[darling(default)]
    pub parse: Option<ClapFieldParse>,
    #[darling(default)]
    pub arg_enum: bool,
    #[darling(default)]
    pub skip: Option<Override<Path>>,
    #[darling(default)]
    pub default_value: Option<String>,

    #[darling(skip, default = "crate::default_rename_all")]
    pub rename_all: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_env")]
    pub rename_all_env: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_value")]
    pub rename_all_value: RenameAll,
}

#[derive(Debug, Clone, FromMeta)]
#[darling(rename_all = "snake_case")]
pub(crate) enum ClapFieldParse {
    FromStr(Override<LitStr>),
    TryFromStr(Override<LitStr>),
    FromOsStr(Override<LitStr>),
    TryFromOsStr(Override<LitStr>),
    FromOccurrences(Override<LitStr>),
    FromFlag(Override<LitStr>),
}

impl ClapFieldParse {
    pub fn defaulted(&self) -> Result<Self> {
        use ClapFieldParse::*;
        use Override::{Explicit, Inherit};

        Ok(match self {
            FromStr(Inherit) => FromStr(Explicit(LitStr::new(
                "::std::convert::From::from",
                Span::call_site(),
            ))),
            TryFromStr(Inherit) => TryFromStr(Explicit(LitStr::new(
                "::std::str::FromStr::from_str",
                Span::call_site(),
            ))),
            FromOsStr(Inherit) => FromOsStr(Explicit(LitStr::new(
                "::std::convert::From::from",
                Span::call_site(),
            ))),
            TryFromOsStr(Inherit) => {
                return Err(Error::unknown_value("No default for try_from_os_str"))
            }
            FromOccurrences(Inherit) => {
                FromOccurrences(Explicit(LitStr::new("value as T", Span::call_site())))
            }
            FromFlag(Inherit) => FromFlag(Explicit(LitStr::new(
                "::std::convert::From::from",
                Span::call_site(),
            ))),

            FromStr(Explicit(..)) => self.clone(),
            TryFromStr(Explicit(..)) => self.clone(),
            FromOsStr(Explicit(..)) => self.clone(),
            TryFromOsStr(Explicit(..)) => self.clone(),
            FromOccurrences(Explicit(..)) => self.clone(),
            FromFlag(Explicit(..)) => self.clone(),
        })
    }
    pub fn parse(&self) -> Result<Expr> {
        use ClapFieldParse::*;

        Ok(match self {
            FromStr(Override::Explicit(exp)) => exp.parse()?,
            TryFromStr(Override::Explicit(exp)) => exp.parse()?,
            FromOsStr(Override::Explicit(exp)) => exp.parse()?,
            TryFromOsStr(Override::Explicit(exp)) => exp.parse()?,
            FromOccurrences(Override::Explicit(exp)) => exp.parse()?,
            FromFlag(Override::Explicit(exp)) => exp.parse()?,
            _ => return Err(Error::unknown_value("Parse should have been defaulted...")),
        })
    }
}

impl Default for ClapFieldParse {
    fn default() -> Self {
        Self::TryFromStr(Override::Inherit)
    }
}

impl ClapTokensResult for ClapFieldParse {
    fn to_tokens_result(&self) -> Result<TokenStream> {
        let parsed = self.parse()?;
        Ok(quote!(#parsed))
    }
}

type SynPath = syn::punctuated::Punctuated<syn::PathSegment, syn::token::Colon2>;
type OptionSynPath = Option<SynPath>;

impl ClapField {
    fn get_name(&self) -> String {
        self.name
            .clone()
            .or_else(|| self.ident.as_ref().map(|i| i.to_string()))
            .expect("Field should have name or ident")
    }

    fn get_type_path(&self) -> OptionSynPath {
        match &self.ty {
            Type::Path(type_path) => Some(type_path.path.segments.clone()),
            _ => None,
        }
    }

    fn get_type_new_strip_vec_option(&self, input: &OptionSynPath, level: usize) -> OptionSynPath {
        self.get_type_new_strip_types_impl(
            input,
            &[
                quote!(std::option::Option),
                quote!(core::option::Option),
                quote!(option::Option),
                quote!(Option),
                quote!(std::vec::Vec),
                quote!(core::vec::Vec),
                quote!(vec::Vec),
                quote!(Vec),
            ],
            level,
        )
    }

    fn get_type_new_strip_types_impl(
        &self,
        input: &OptionSynPath,
        types: &[TokenStream],
        level: usize,
    ) -> OptionSynPath {
        if level == 0 {
            input.clone()
        } else if self.types_without_generics_eq(input, types).is_some() {
            let type_path = input.clone().unwrap();
            for path_entry in type_path.iter() {
                if let PathArguments::AngleBracketed(args) = &path_entry.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(Type::Path(arg_type_path)) = &args.args[0] {
                            return self.get_type_new_strip_types_impl(
                                &Some(arg_type_path.path.segments.clone()),
                                types,
                                level.saturating_sub(1),
                            );
                        }
                    }
                }
            }
            input.clone()
        } else {
            input.clone()
        }
    }

    fn types_without_generics_eq_vec(&self, input: &OptionSynPath) -> OptionSynPath {
        self.types_without_generics_eq(
            input,
            &[
                quote!(std::vec::Vec),
                quote!(core::vec::Vec),
                quote!(vec::Vec),
                quote!(Vec),
            ],
        )
    }

    fn types_without_generics_eq_option(&self, input: &OptionSynPath) -> OptionSynPath {
        self.types_without_generics_eq(
            input,
            &[
                quote!(std::option::Option),
                quote!(core::option::Option),
                quote!(option::Option),
                quote!(Option),
            ],
        )
    }

    fn types_without_generics_eq(
        &self,
        input: &OptionSynPath,
        types: &[TokenStream],
    ) -> OptionSynPath {
        if let Some(type_path) = input {
            let type_path_no_generic = type_path
                .iter()
                .map(|p| {
                    let mut p = p.clone();
                    p.arguments = Default::default();
                    p
                })
                .collect::<SynPath>();

            self.types_eq(&Some(type_path_no_generic), types)
        } else {
            None
        }
    }

    fn types_eq(&self, input: &OptionSynPath, types: &[TokenStream]) -> OptionSynPath {
        if let Some(type_path) = input {
            let type_path_str = type_path.to_token_stream().to_string();

            types.iter().find_map(|ts| {
                if ts.to_string() == type_path_str {
                    Some(type_path.clone())
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn get_arg_type(&self) -> Result<(ClapArgType, OptionSynPath)> {
        if self.ty.to_token_stream().to_string() == "bool" {
            return Ok((ClapArgType::Bool, self.get_type_path()));
        }

        let (prefixes, stripped_type_path) = self.get_vec_option_prefixes();

        if prefixes.is_empty() {
            Ok((ClapArgType::T, stripped_type_path))
        } else if prefixes == ["Option"] {
            Ok((ClapArgType::OptionT, stripped_type_path))
        } else if prefixes == ["Option", "Option"] {
            Ok((ClapArgType::OptionOptionT, stripped_type_path))
        } else if prefixes == ["Vec"] {
            Ok((ClapArgType::VecT, stripped_type_path))
        } else if prefixes == ["Option", "Vec"] {
            Ok((ClapArgType::OptionVecT, stripped_type_path))
        } else {
            Err(
                Error::custom(format!("Type {:?} does not conform to standards", &self.ty))
                    .with_span(&self.ty),
            )
        }
    }

    fn get_vec_option_prefixes(&self) -> (Vec<&'static str>, OptionSynPath) {
        let mut prefixes = Vec::new();
        let mut ty = self.get_type_path();

        for _ in 0..100 {
            let mut set = false;
            if self.types_without_generics_eq_vec(&ty).is_some() {
                prefixes.push("Vec");
                set = true;
            } else if self.types_without_generics_eq_option(&ty).is_some() {
                prefixes.push("Option");
                set = true;
            }
            if set {
                ty = self.get_type_new_strip_vec_option(&ty, 1);
            } else {
                break;
            }
        }

        (prefixes, ty)
    }

    fn get_parse_defaulted(&self) -> Result<ClapFieldParse> {
        let (arg_type, _) = self.get_arg_type()?;

        let parse = if let Some(parse) = &self.parse {
            parse.clone()
        } else if matches!(arg_type, ClapArgType::Bool) {
            ClapFieldParse::FromFlag(Override::Inherit)
        } else {
            ClapFieldParse::default()
        };

        parse.defaulted()
    }

    fn get_flatten(&self) -> (TokenStream, Option<TokenStream>) {
        if let Some(flatten) = &self.flatten {
            let prefix = match flatten {
                Override::Explicit(prefix) => Some(quote! { vec.push(#prefix.to_string()); }),
                Override::Inherit => None,
            };

            (
                quote!(subprefix),
                Some(quote! {
                    let subprefix = {
                        let mut vec = Vec::new();
                        if let Some(prefix) = prefix.as_ref() {
                            vec.push(prefix.to_string());
                        }
                        #prefix
                        if vec.is_empty() {
                            None
                        } else {
                            Some(vec.join("-"))
                        }
                    };
                }),
            )
        } else {
            (quote!(prefix), None)
        }
    }

    pub fn to_tokens_augment(&self) -> Result<TokenStream> {
        let (arg_type, stripped_type_path) = self.get_arg_type()?;

        Ok(if self.subcommand {
            let ty = &self.ty;
            if !matches!(arg_type, ClapArgType::T) {
                return Err(Error::unexpected_type(&ty.to_token_stream().to_string()).with_span(ty));
            }

            quote! {
                let app = <#ty as clap_derive_darling::Subcommand>::augment_subcommands(app, prefix.clone());
                let app = app.setting(clap::AppSettings::SubcommandRequiredElseHelp);
            }
        } else if self.skip.is_some() {
            quote! {}
        } else if self.flatten.is_some() {
            let ty = &self.ty;

            let (prefix_ident, subprefix) = self.get_flatten();

            quote! {
                let old_heading = app.get_help_heading();

                #subprefix

                let app = <#ty as clap_derive_darling::Args>::augment_args(app, #prefix_ident.clone());
                let app = app.help_heading(old_heading);
            }
        } else {
            let name = self.get_name();
            let parse = self.get_parse_defaulted()?;
            let parse_expr = parse.parse()?;
            let name_renamed =
                self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

            let name_ident = format_ident!("___name");
            let name_value_ident = format_ident!("___name_value");
            let name_long_ident = format_ident!("___name_long");
            let name_env_ident = format_ident!("___name_env");

            let builder = quote! {
                let #name_ident = get_cache_str_keyed("name", #name, &prefix, || #name_renamed);
                clap::Arg::new(#name_ident)
            };

            let builder = match &self.short {
                Some(Override::Explicit(short)) => {
                    let short = short
                        .chars()
                        .next()
                        .ok_or_else(|| Error::unknown_value(short))?;
                    quote! {
                        #builder
                            .short(#short)
                    }
                }
                Some(Override::Inherit) => {
                    let short = self.get_name().chars().next().ok_or_else(|| {
                        Error::custom("Could not build short value from field name")
                    })?;
                    quote! {
                        #builder
                            .short(#short)
                    }
                }
                None => builder,
            };

            let builder = match &self.long {
                Some(Override::Explicit(long)) => {
                    let rename =
                        self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#long));
                    quote! {
                        let #name_long_ident = get_cache_str_keyed("name_long", #name, &prefix, || #rename);

                        #builder
                            .long(#name_long_ident)
                    }
                }
                Some(Override::Inherit) => {
                    let rename =
                        self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));
                    quote! {
                        let #name_long_ident = get_cache_str_keyed("name_long", #name, &prefix, || #rename);

                        #builder
                            .long(#name_long_ident)
                    }
                }
                None => builder,
            };

            let builder = match &self.env {
                Some(Override::Explicit(env)) => {
                    let renmae =
                        self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#env));
                    quote! {
                        let #name_env_ident = get_cache_str_keyed("name_env", #name, &prefix, || #renmae);

                        #builder
                            .env(#name_env_ident)
                    }
                }
                Some(Override::Inherit) => {
                    let renmae =
                        self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));
                    quote! {
                        let #name_env_ident = get_cache_str_keyed("name_env", #name, &prefix, || #renmae);

                        #builder
                            .env(#name_env_ident)
                    }
                }
                None => builder,
            };

            let builder = match &parse {
                ClapFieldParse::FromFlag(..) => {
                    quote! {
                        #builder
                            .takes_value(false)
                    }
                }
                _ => {
                    let rename = self.rename_field(
                        self.rename_all_value,
                        Some(quote!(prefix)),
                        quote!(#name),
                    );

                    quote! {
                        let #name_value_ident = get_cache_str_keyed("name_value", #name, &prefix, || #rename);

                        #builder
                            .takes_value(true)
                            .value_name(#name_value_ident)
                    }
                }
            };

            let builder = match arg_type {
                ClapArgType::Bool => builder,
                ClapArgType::OptionT => quote! {
                    #builder
                        .required(false)
                },
                ClapArgType::OptionOptionT => quote! {
                    #builder
                        .required(false)
                        .min_values(0)
                        .max_values(1)
                },
                ClapArgType::T => {
                    let default = self.default_value.is_none();
                    quote! {
                        #builder
                            .required(#default)
                    }
                }
                ClapArgType::VecT | ClapArgType::OptionVecT => quote! {
                    #builder
                        .required(false)
                        .multiple_occurrences(true)
                },
            };

            let builder = if let Some(help) = self.to_tokens_app_call_help_about() {
                quote! {
                    #builder
                        #help
                }
            } else {
                builder
            };

            let builder = if self.arg_enum {
                quote! {
                    #builder
                        .possible_values(
                            <#stripped_type_path as clap_derive_darling::ArgEnum>::value_variants()
                                .iter()
                                .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                        )
                }
            } else if matches!(arg_type, ClapArgType::Bool) {
                builder
            } else {
                quote! {
                    #builder
                        .validator(|s| #parse_expr(s).map(|_: #stripped_type_path| ()))
                }
            };

            quote! {
                let app = app.arg({
                    #builder
                });
            }
        })
    }

    pub fn to_tokens_augment_for_update(&self) -> Result<TokenStream> {
        self.to_tokens_augment()
    }

    pub fn to_tokens_from_arg_matches(&self) -> Result<TokenStream> {
        let ident = &self.ident;

        let parse = self.to_tokens_parse(None)?;

        Ok(quote! {
            #ident: #parse,
        })
    }

    pub fn to_tokens_update_from_arg_matches(&self) -> Result<TokenStream> {
        let ident = &self.ident;

        let parse = self.to_tokens_parse(ident.clone())?;

        Ok(if self.subcommand || self.flatten.is_some() {
            quote! {
                {
                    #[allow(non_snake_case)]
                    let #ident = &mut self.#ident;

                    #parse;
                }
            }
        } else {
            quote! {
                {
                    #[allow(non_snake_case)]
                    let #ident = &mut self.#ident;

                    *#ident = #parse;
                }
            }
        })
    }

    pub fn to_tokens_update_from_arg_matches_raw(&self) -> Result<TokenStream> {
        let ident = &self.ident;

        let parse = self.to_tokens_parse(ident.clone())?;

        Ok(if self.subcommand || self.flatten.is_some() {
            parse
        } else {
            quote! {
                *#ident = #parse;
            }
        })
    }

    fn to_tokens_parse(&self, update_ident: Option<Ident>) -> Result<TokenStream> {
        let ty = &self.ty;
        let (arg_type, stripped_type_path) = self.get_arg_type()?;

        Ok(if self.subcommand {
            if !matches!(arg_type, ClapArgType::T) {
                return Err(Error::unexpected_type(&ty.to_token_stream().to_string()).with_span(ty));
            }

            if let Some(update_ident) = update_ident {
                quote! {
                    <#ty as clap_derive_darling::FromArgMatches>::update_from_arg_matches(
                        #update_ident,
                        arg_matches,
                        prefix
                    )
                }
            } else {
                quote! {
                    <#ty as clap_derive_darling::FromArgMatches>::from_arg_matches(arg_matches, prefix.clone())?
                }
            }
        } else if let Some(skip) = self.skip.as_ref() {
            match skip {
                Override::Explicit(default) => quote! { #default },
                Override::Inherit => quote! { std::default::Default::default() },
            }
        } else if self.flatten.is_some() {
            let (prefix_ident, subprefix) = self.get_flatten();

            if let Some(update_ident) = update_ident {
                quote! {
                    {
                        #subprefix

                        clap_derive_darling::FromArgMatches::update_from_arg_matches(
                            #update_ident,
                            arg_matches,
                            #prefix_ident
                        )
                    }
                }
            } else {
                quote! {
                    {
                        #subprefix

                        clap_derive_darling::FromArgMatches::from_arg_matches(arg_matches, #prefix_ident).unwrap()
                    }
                }
            }
        } else {
            let name = self.get_name();

            let parse = self.get_parse_defaulted()?;
            let parse_expr = parse.parse()?;

            let name_ident = format_ident!("___name");

            let field_name_renamed =
                self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

            let mapper = if self.arg_enum {
                quote! {
                    <#stripped_type_path as clap_derive_darling::ArgEnum>::from_str(s, false)
                        .map_err(|err| clap::Error::raw(
                            clap::ErrorKind::ValueValidation,
                            format!("Invalid value for {}: {}", &#name_ident, &err)
                        ))
                }
            } else {
                quote! {
                    #parse_expr(s).map_err(|err| {
                        clap::Error::raw(
                            clap::ErrorKind::ValueValidation,
                            format!("Invalid value for {}: {}", &#name_ident, &err)
                        )
                    })
                }
            };

            let builder = if matches!(arg_type, ClapArgType::Bool) {
                quote! {
                    arg_matches.is_present(#name_ident)
                }
            } else if matches!(arg_type, ClapArgType::VecT | ClapArgType::OptionVecT) {
                quote! {
                    arg_matches
                        .values_of(&#name_ident)
                        .map(|v| {
                            v.map(|s| #mapper)
                            // .collect()
                        })
                        .map(|v| v.collect::<Result<Vec<_>, _>>())
                }
            } else {
                quote! {
                    arg_matches
                        .value_of(&#name_ident)
                        .map(|s| #mapper)
                        .transpose()?
                }
            };

            let builder = if matches!(arg_type, ClapArgType::OptionOptionT) {
                quote! {
                    if arg_matches.is_present(&#name_ident) {
                        Some(#builder)
                    } else {
                        None
                    }
                }
            } else if matches!(
                arg_type,
                ClapArgType::OptionOptionT | ClapArgType::OptionVecT
            ) {
                quote! {
                    #builder
                        .transpose()?
                }
            } else if matches!(arg_type, ClapArgType::T) {
                quote! {
                    #builder
                        .ok_or_else(|| {
                            clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                format!("Invalid value for {}", &#name_ident)
                            )
                        })?
                }
            } else if matches!(arg_type, ClapArgType::VecT) {
                quote! {
                    #builder
                        .unwrap_or_else(|| Ok(Vec::new()))?
                }
            } else {
                builder
            };

            quote! {
                {
                    let #name_ident = #field_name_renamed;
                    #builder
                }
            }
        })
    }

    fn rename_field(
        &self,
        rename: RenameAll,
        prefix: Option<TokenStream>,
        name: TokenStream,
    ) -> TokenStream {
        if let Some(prefix) = prefix {
            quote! {
                #rename(clap_derive_darling::rename::prefix(#name, &#prefix))
            }
        } else {
            quote! {
                #rename(#name)
            }
        }
    }
}

impl ClapDocCommon for ClapField {
    fn get_attrs(&self) -> Vec<syn::Attribute> {
        self.attrs.clone()
    }
    fn get_help_about(&self) -> Option<String> {
        self.help.clone()
    }
    fn get_long_help_about(&self) -> Option<String> {
        self.long_help.clone()
    }
}
impl ClapDocCommonAuto for ClapField {
    type Marker = ClapDocHelpMarker;
}

#[cfg(test)]
mod test;
