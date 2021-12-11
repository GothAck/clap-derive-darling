use darling::{util::Override, FromField, FromMeta, ToTokens};
use quote::{format_ident, quote};
use syn::{GenericArgument, Ident, LitStr, PathArguments, Type};

use crate::common::{ClapDocCommon, ClapDocCommonAuto, ClapDocHelpMarker};

use super::RenameAll;
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
    #[allow(dead_code)]
    #[darling(default)]
    pub arg_enum: bool,
    #[darling(default)]
    pub skip: Option<Override<String>>,
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

impl ToTokens for ClapFieldParse {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use ClapFieldParse::*;

        match self {
            FromStr(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            TryFromStr(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            FromOsStr(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            TryFromOsStr(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            FromOccurrences(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            FromFlag(Override::Explicit(exp)) => {
                tokens.extend(quote! {
                    #exp
                });
            }
            _ => {}
        }
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

    fn get_type_new_strip_option(&self, input: &OptionSynPath, level: usize) -> OptionSynPath {
        self.get_type_new_strip_types_impl(
            input,
            &[
                quote!(std::option::Option),
                quote!(core::option::Option),
                quote!(option::Option),
                quote!(Option),
            ],
            level,
        )
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
        types: &[proc_macro2::TokenStream],
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
        types: &[proc_macro2::TokenStream],
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

    fn types_eq(&self, input: &OptionSynPath, types: &[proc_macro2::TokenStream]) -> OptionSynPath {
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

    // fn get_vec_option_prefixes(&self) -> (Vec<&'static str>, OptionSynPath) {
    //     let mut prefixes = Vec::new();
    //     let mut ty = self.get_type_path();

    //     for _ in 0..100 {
    //         let mut set = false;
    //         if self.types_without_generics_eq_vec(&ty).is_some() {
    //             prefixes.push("Vec");
    //             set = true;
    //         } else if self.types_without_generics_eq_option(&ty).is_some() {
    //             prefixes.push("Option");
    //             set = true;
    //         }
    //         if set {
    //             ty = self.get_type_new_strip_vec_option(&ty, 1);
    //         } else {
    //             break;
    //         }
    //     }

    //     (prefixes, ty)
    // }

    fn get_parse(&self) -> ClapFieldParse {
        use ClapFieldParse::*;

        if let Some(parse) = &self.parse {
            parse.clone()
        } else if let Some(ty_path) = self.get_type_path() {
            if ty_path.len() == 1 && ty_path[0].ident == "bool" {
                FromFlag(Override::Inherit)
            } else {
                TryFromStr(Override::Inherit)
            }
        } else {
            TryFromStr(Override::Inherit)
        }
    }

    // fn get_parse_defaulted(&self) -> ClapFieldParse {
    //     use ClapFieldParse::*;

    //     let parse = self.get_parse();

    //     match &parse {
    //         FromStr(Override::Inherit) => FromStr(Override::Explicit(LitStr::new(
    //             "::std::convert::From::from",
    //             Span::call_site(),
    //         ))),
    //         FromStr(Override::Explicit(..)) => parse,
    //         TryFromStr(Override::Inherit) => FromStr(Override::Explicit(LitStr::new(
    //             "::std::str::FromStr::from_str",
    //             Span::call_site(),
    //         ))),
    //         TryFromStr(Override::Explicit(..)) => parse,
    //         FromOsStr(Override::Inherit) => FromStr(Override::Explicit(LitStr::new(
    //             "::std::convert::From::from",
    //             Span::call_site(),
    //         ))),
    //         FromOsStr(Override::Explicit(..)) => parse,
    //         TryFromOsStr(Override::Inherit) => FromStr(Override::Inherit),
    //         TryFromOsStr(Override::Explicit(..)) => parse,
    //         FromOccurrences(Override::Inherit) => FromStr(Override::Explicit(LitStr::new(
    //             "value as T",
    //             Span::call_site(),
    //         ))),
    //         FromOccurrences(Override::Explicit(..)) => parse,
    //         FromFlag(Override::Inherit) => FromStr(Override::Explicit(LitStr::new(
    //             "::std::convert::From::from",
    //             Span::call_site(),
    //         ))),
    //         FromFlag(Override::Explicit(..)) => parse,
    //     }
    // }

    fn get_takes_value(&self) -> proc_macro2::TokenStream {
        if matches!(self.get_parse(), ClapFieldParse::FromFlag(..)) {
            quote! { .takes_value(false) }
        } else {
            quote! { .takes_value(true) }
        }
    }

    fn is_required(&self) -> bool {
        if self.default_value.is_some() || matches!(self.get_parse(), ClapFieldParse::FromFlag(..))
        {
            return false;
        }

        let type_path = self.get_type_path();
        if self.types_without_generics_eq_option(&type_path).is_some()
            || self.types_without_generics_eq_vec(&type_path).is_some()
        {
            return false;
        }

        true
    }

    fn get_required(&self) -> Option<proc_macro2::TokenStream> {
        if self.is_required() {
            Some(quote! { .required(true) })
        } else {
            None
        }
    }

    fn get_short(&self) -> Option<proc_macro2::TokenStream> {
        let name = self.get_name().chars().next().unwrap();

        match &self.short {
            Some(Override::Explicit(short)) => Some(quote! { .short(#short) }),
            Some(Override::Inherit) => Some(quote! { .short(#name) }),
            None => None,
        }
    }

    fn get_long(&self) -> Option<proc_macro2::TokenStream> {
        match &self.long {
            Some(Override::Explicit(long)) => {
                Some(self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#long)))
            }
            Some(Override::Inherit) => {
                let name = self.get_name();
                Some(self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name)))
            }
            None => None,
        }
    }

    fn get_env(&self) -> Option<proc_macro2::TokenStream> {
        match &self.env {
            Some(Override::Explicit(env)) => {
                let name_rename =
                    self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#env));

                Some(quote! { .env(&#name_rename) })
            }
            Some(Override::Inherit) => {
                let name = self.get_name();
                let name_rename =
                    self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

                Some(quote! { .env(&#name_rename) })
            }
            None => None,
        }
    }

    fn get_flatten(&self) -> (proc_macro2::TokenStream, Option<proc_macro2::TokenStream>) {
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

    pub fn to_tokens_augment(&self) -> proc_macro2::TokenStream {
        if self.subcommand {
            let ty = &self.ty;

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
            let ty_no_opt_vec = self.get_type_new_strip_vec_option(&self.get_type_path(), 10);

            let name = self.get_name();
            let name_renamed =
                self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));
            let name_renamed_value =
                self.rename_field(self.rename_all_value, Some(quote!(prefix)), quote!(#name));

            let takes_value = self.get_takes_value();
            let required = self.get_required();
            let short = self.get_short();
            let env = self.get_env();
            let help = self.to_tokens_app_call_help_about();

            let multiple_values = if self
                .types_without_generics_eq_vec(&self.get_type_path())
                .is_some()
            {
                Some(quote! {.multiple_values(true)})
            } else {
                None
            };

            let (long_var, long_call) = {
                if let Some(long) = self.get_long() {
                    let ident = format_ident!("___name_long");
                    (
                        Some(
                            quote! { let #ident = get_cache_str_keyed("name_long", #name, &prefix, || #long); },
                        ),
                        Some(quote! { .long(#ident) }),
                    )
                } else {
                    (None, None)
                }
            };

            let value_name = if matches!(self.get_parse(), ClapFieldParse::FromFlag(..)) {
                None
            } else {
                Some(quote! { .value_name(&___name_value) })
            };

            quote! {
                let ___name = get_cache_str_keyed("name", #name, &prefix, || #name_renamed );
                let ___name_value = get_cache_str_keyed("name_value", #name, &prefix, || #name_renamed_value );
                #long_var

                let app = app.arg(
                    clap::Arg::new(&*___name)
                        #help
                        #takes_value
                        #multiple_values
                        #value_name
                        .validator(|s| ::std::str::FromStr::from_str(s).map(|_: #ty_no_opt_vec| ()))
                        #required
                        #short
                        #long_call
                        #env
                );
            }
        }
    }

    pub fn to_tokens_augment_for_update(&self) -> proc_macro2::TokenStream {
        self.to_tokens_augment()
    }

    pub fn to_tokens_from_arg_matches(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let type_path = self.get_type_path();

        let name = self.get_name();
        let field_name_renamed =
            self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

        if self.subcommand {
            let ty = &self.ty;

            quote! {
                #ident: { <#ty as clap_derive_darling::FromArgMatches>::from_arg_matches(arg_matches, prefix.clone())? },
            }
        } else if self.skip.is_some() {
            quote! {
                #ident: std::default::Default::default(),
            }
        } else if self.flatten.is_some() {
            let (prefix_ident, subprefix) = self.get_flatten();

            quote! {
                #ident: {
                    #subprefix

                    clap_derive_darling::FromArgMatches::from_arg_matches(arg_matches, #prefix_ident).unwrap()
                },
            }
        } else if self
            .types_without_generics_eq(&type_path, &[quote!(bool)])
            .is_some()
        {
            quote! {
                #ident: arg_matches.is_present(&#field_name_renamed),
            }
        } else if self.types_without_generics_eq_option(&type_path).is_some() {
            let next_type_path = self.get_type_new_strip_option(&type_path, 1);

            if self
                .types_without_generics_eq_vec(&next_type_path)
                .is_some()
            {
                quote! {
                    #ident: if arg_matches.is_present(&#field_name_renamed) {
                        Some(arg_matches
                            .values_of(&#field_name_renamed)
                            .map(|v| {
                                v.map::<String, _>(|s| ::std::str::FromStr::from_str(s).unwrap())
                                    .collect()
                            })
                            .unwrap_or_else(Vec::new))
                    } else {
                        None
                    },
                }
            } else if self
                .types_without_generics_eq_option(&next_type_path)
                .is_some()
            {
                quote! {
                    #ident: if arg_matches.is_present(&#field_name_renamed) {
                        Some(arg_matches
                            .value_of(&#field_name_renamed)
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap()))
                    } else {
                        None
                    },
                }
            } else {
                quote! {
                    #ident: if arg_matches.is_present(&#field_name_renamed) {
                        Some(arg_matches
                            .value_of(&#field_name_renamed)
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            .expect("app should verify arg required"))
                    } else {
                        None
                    },
                }
            }
        } else if self.types_without_generics_eq_vec(&type_path).is_some() {
            quote! {
                #ident: {
                    arg_matches
                        .values_of(&#field_name_renamed)
                        .map(|v| {
                            v.map::<String, _>(|s| ::std::str::FromStr::from_str(s).unwrap())
                                .collect()
                        })
                        .unwrap_or_else(Vec::new)
                },
            }
        } else {
            let expect = if self.is_required() {
                Some(quote! { .expect("app should verify arg required") })
            } else {
                None
            };

            quote! {
                #ident: {
                    arg_matches
                        .value_of(&#field_name_renamed)
                        .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                        #expect
                },
            }
        }
    }

    pub fn to_tokens_update_from_arg_matches(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let type_path = self.get_type_path();

        let name = self.get_name();
        let field_name_renamed =
            self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

        let update_from_arg_matches_raw = self.to_tokens_update_from_arg_matches_raw();

        if self.subcommand || self.skip.is_some() {
            quote! {
                {
                    #[allow(non_snake_case)]
                    let #ident = &mut self.#ident;

                    #update_from_arg_matches_raw
                }
            }
        } else if self.flatten.is_some() {
            let (_, subprefix) = self.get_flatten();

            quote! {
                {
                    #subprefix

                    #[allow(non_snake_case)]
                    let #ident = &mut self.#ident;

                    #update_from_arg_matches_raw
                }
            }
        } else if self
            .types_without_generics_eq(&type_path, &[quote!(bool)])
            .is_some()
        {
            quote! {
                {
                    #[allow(non_snake_case)]
                    let #ident = &mut self.#ident;

                    #update_from_arg_matches_raw
                }
            }
        } else if self.types_without_generics_eq_option(&type_path).is_some() {
            let next_type_path = self.get_type_new_strip_option(&type_path, 1);

            #[allow(clippy::if_same_then_else)]
            if self
                .types_without_generics_eq_vec(&next_type_path)
                .is_some()
            {
                // FIXME: ___name is created twice when processing subcommand
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            #[allow(non_snake_case)]
                            let #ident = &mut self.#ident;

                            #update_from_arg_matches_raw
                        }
                    }
                }
            } else if self
                .types_without_generics_eq_option(&next_type_path)
                .is_some()
            {
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            #[allow(non_snake_case)]
                            let #ident = &mut self.#ident;

                            #update_from_arg_matches_raw
                        }
                    }
                }
            } else {
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            #[allow(non_snake_case)]
                            let #ident = &mut self.#ident;

                            #update_from_arg_matches_raw
                        }
                    }
                }
            }
        } else if self.types_without_generics_eq_vec(&type_path).is_some() {
            quote! {
                {
                    let ___name = #field_name_renamed;
                    if arg_matches.is_present(&___name) {
                        #[allow(non_snake_case)]
                        let #ident = &mut self.#ident;

                        #update_from_arg_matches_raw
                    }
                }
            }
        } else {
            quote! {
                {
                    let ____name = #field_name_renamed;
                    if arg_matches.is_present(&____name) {
                        #[allow(non_snake_case)]
                        let #ident = &mut self.#ident;

                        #update_from_arg_matches_raw
                    }
                }
            }
        }
    }

    pub fn to_tokens_update_from_arg_matches_raw(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let type_path = self.get_type_path();

        let name = self.get_name();
        let field_name_renamed =
            self.rename_field(self.rename_all, Some(quote!(prefix)), quote!(#name));

        if self.subcommand {
            let ty = &self.ty;

            quote! {
                <#ty as clap_derive_darling::FromArgMatches>::update_from_arg_matches(
                    #ident,
                    arg_matches,
                    prefix,
                )?;
            }
        } else if self.skip.is_some() {
            quote! {
                *#ident = std::default::Default::default();
            }
        } else if self.flatten.is_some() {
            let (prefix_ident, subprefix) = self.get_flatten();

            quote! {
                #subprefix

                clap_derive_darling::FromArgMatches::update_from_arg_matches(#ident, arg_matches, #prefix_ident)?;
            }
        } else if self
            .types_without_generics_eq(&type_path, &[quote!(bool)])
            .is_some()
        {
            quote! {
                *#ident = arg_matches.is_present(&#field_name_renamed);
            }
        } else if self.types_without_generics_eq_option(&type_path).is_some() {
            let next_type_path = self.get_type_new_strip_option(&type_path, 1);

            if self
                .types_without_generics_eq_vec(&next_type_path)
                .is_some()
            {
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            *#ident = Some(arg_matches
                                .values_of(&___name)
                                .map(|v| {
                                    v.map::<String, _>(|s| ::std::str::FromStr::from_str(s).unwrap())
                                        .collect()
                                })
                                .unwrap_or_else(Vec::new));
                        }
                    }
                }
            } else if self
                .types_without_generics_eq_option(&next_type_path)
                .is_some()
            {
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            *#ident = Some(arg_matches
                                .value_of(&___name)
                                .map(|s| ::std::str::FromStr::from_str(s).unwrap()));
                        }
                    }
                }
            } else {
                quote! {
                    {
                        let ___name = #field_name_renamed;
                        if arg_matches.is_present(&___name) {
                            *#ident = arg_matches
                                .value_of(&___name)
                                .map(|s| ::std::str::FromStr::from_str(s).unwrap());
                        }
                    }
                }
            }
        } else if self.types_without_generics_eq_vec(&type_path).is_some() {
            quote! {
                {
                    let ___name = #field_name_renamed;
                    if arg_matches.is_present(&___name) {
                        *#ident = arg_matches
                            .values_of(&___name)
                            .map(|v| {
                                v.map::<String, _>(|s| ::std::str::FromStr::from_str(s).unwrap())
                                    .collect()
                            })
                            .unwrap_or_else(Vec::new);
                    }
                }
            }
        } else {
            let required = if self.is_required() {
                Some(quote! { .expect("App should have already required this") })
            } else {
                None
            };
            quote! {
                {
                    let ____name = #field_name_renamed;
                    if arg_matches.is_present(&____name) {
                        *#ident = arg_matches
                            .value_of(&____name)
                            .map(|s| ::std::str::FromStr::from_str(s).unwrap())
                            #required;
                    }
                }
            }
        }
    }

    fn rename_field(
        &self,
        rename: RenameAll,
        prefix: Option<proc_macro2::TokenStream>,
        name: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
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
