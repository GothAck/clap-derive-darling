use std::vec;

use darling::{ast, util::Override, FromDeriveInput, FromVariant, ToTokens};
use quote::quote;
use syn::Ident;

use crate::{
    common::{
        ClapDocAboutMarker, ClapDocCommon, ClapDocCommonAuto, ClapFieldStructs, ClapFields,
        ClapParserArgsCommon, ClapRename, ClapTraitImpls,
    },
    field::ClapField,
    RenameAll, RenameAllCasing,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clap), supports(enum_any))]
pub struct ClapSubcommand {
    ident: Ident,
    data: ast::Data<ClapSubcommandVariant, ()>,
}

impl ToTokens for ClapSubcommand {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.to_tokens_impl_from_arg_matches());
        tokens.extend(self.to_tokens_impl_subcommand());
    }
}

impl ClapParserArgsCommon for ClapSubcommand {
    fn get_author(&self) -> Option<&Override<String>> {
        None
    }
    fn get_version(&self) -> Option<&Override<String>> {
        None
    }
    fn get_help_heading(&self) -> Option<&String> {
        None
    }
}

impl ClapSubcommand {
    fn get_variants(&self) -> Vec<ClapSubcommandVariant> {
        self.data
            .as_ref()
            .take_enum()
            .expect("Should always be an enum")
            .iter()
            .cloned()
            .cloned()
            .map(|mut v| {
                v.parent_ident = Some(self.ident.clone());
                v
            })
            .collect()
    }

    fn to_tokens_impl_from_arg_matches(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let from_arg_matches_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_from_arg_matches_variant())
            .collect::<Vec<_>>();

        let update_from_arg_matches_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_update_from_arg_matches_variant())
            .collect::<Vec<_>>();

        quote! {
            impl clap_derive_darling::FromArgMatches for #ident {
                fn from_arg_matches(arg_matches: &clap::ArgMatches, prefix: Option<String>) -> Result<Self, clap::Error> {
                    if let Some((clap_name, sub_arg_matches)) = arg_matches.subcommand() {
                        {
                            let arg_matches = sub_arg_matches;

                            #(#from_arg_matches_variants)*
                        }
                        Err(clap::Error::raw(clap::ErrorKind::UnrecognizedSubcommand, format!("The subcommand '{}' watn't recognized", clap_name)))
                    } else {
                        Err(clap::Error::raw(clap::ErrorKind::MissingSubcommand, "A subcommand is required but one was not provided"))
                    }
                }
                fn update_from_arg_matches(&mut self, arg_matches: &clap::ArgMatches, prefix: Option<String>) -> Result<(), clap::Error> {
                    if let Some((clap_name, sub_arg_matches)) = arg_matches.subcommand() {
                        match self {
                            #(#update_from_arg_matches_variants)*

                            s => {
                                *s = <Self as clap_derive_darling::FromArgMatches>::from_arg_matches(arg_matches, prefix)?;
                            }
                        }
                    }
                    Ok(())
                }
            }
        }
    }

    fn to_tokens_impl_subcommand(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let name_storage = self.to_tokens_name_storage();

        let augment_subcommands_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokents_augment_subcommands_variant())
            .collect::<Vec<_>>();

        let augment_subcommands_for_update_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokents_augment_subcommands_for_update_variant())
            .collect::<Vec<_>>();

        let has_subcommands = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_has_subcommand())
            .collect::<Vec<_>>();

        quote! {

            impl clap_derive_darling::Subcommand for #ident {
                fn augment_subcommands<'b>(clap_app: clap::App<'b>, prefix: Option<String>) -> clap::App<'b> {
                    #name_storage

                    let clap_app = clap_app;

                    #(#augment_subcommands_variants)*

                    clap_app
                }
                fn augment_subcommands_for_update<'b>(clap_app: clap::App<'b>, prefix: Option<String>) -> clap::App<'b> {
                    #name_storage

                    let clap_app = clap_app;

                    #(#augment_subcommands_for_update_variants)*

                    clap_app
                }
                fn has_subcommand(clap_name: &str) -> bool {
                    #(#has_subcommands)*

                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(clap), forward_attrs(doc))]
pub(crate) struct ClapSubcommandVariant {
    ident: Ident,
    fields: ast::Fields<ClapField>,
    attrs: Vec<syn::Attribute>,

    #[darling(skip)]
    parent_ident: Option<Ident>,

    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    version: Option<Override<String>>,
    #[darling(default)]
    author: Option<Override<String>>,
    #[darling(default)]
    about: Option<Override<String>>,
    #[darling(default)]
    long_about: Option<String>,
    #[allow(dead_code)]
    #[darling(default)]
    verbatim_doc_comment: bool,
    #[darling(default)]
    help_heading: Option<String>,

    #[darling(default)]
    skip: bool,
    #[darling(default)]
    external_subcommand: bool,

    #[darling(skip, default = "crate::default_rename_all")]
    rename_all: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_env")]
    rename_all_env: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_value")]
    rename_all_value: RenameAll,
}

impl ClapSubcommandVariant {
    fn to_tokens_from_arg_matches_variant(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let name = self.get_name();
        let parent_ident = &self.parent_ident;

        let fields = self.get_fields();

        if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                if #name == clap_name {
                    return Ok(#parent_ident::#ident(
                        ::std::iter::once(::std::string::String::from(clap_name))
                            .chain(
                                arg_matches
                                    .values_of("")
                                    .into_iter()
                                    .flatten()
                                    .map(::std::string::String::from)
                            )
                            .collect::<Vec<_>>()
                    ))
                }
            }
        } else if self.fields.is_newtype() {
            let first_field_ty = &fields[0].ty;
            quote! {
                if #name == clap_name {
                    return Ok(#parent_ident::#ident(
                        <#first_field_ty as clap_derive_darling::FromArgMatches>::from_arg_matches(
                            arg_matches,
                            prefix,
                        )?,
                    ));
                }
            }
        } else if self.fields.is_struct() {
            let from_arg_matches_fields = self.to_tokens_from_arg_matches_fields();

            quote! {
                if #name == clap_name {
                    return Ok(#parent_ident::#ident {
                        #(#from_arg_matches_fields)*
                    })
                }
            }
        } else if self.fields.is_tuple() {
            unimplemented!("Variant type tuple not implemented");
        } else if self.fields.is_unit() {
            unimplemented!("Variant type unit not implemented");
        } else {
            unimplemented!("Unknown variant type")
        }
    }

    fn to_tokens_update_from_arg_matches_variant(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let name = self.get_name();
        let parent_ident = &self.parent_ident;

        let fields_ref_mut = self
            .get_fields()
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    ref mut #ident,
                }
            })
            .collect::<Vec<_>>();

        if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                #parent_ident::#ident(ref mut clap_arg) if #name == clap_name => {
                    *clap_arg = ::std::iter::once(::std::string::String::from(clap_name))
                        .chain(
                            arg_matches
                                .values_of("")
                                .into_iter()
                                .flatten()
                                .map(::std::string::String::from)
                        )
                        .collect::<Vec<_>>();
                }
            }
        } else if self.fields.is_newtype() {
            quote! {
                #parent_ident::#ident(ref mut clap_arg) if #name == clap_name => {
                    let arg_matches = sub_arg_matches;
                    clap_derive_darling::FromArgMatches::update_from_arg_matches(
                        clap_arg,
                        sub_arg_matches,
                        prefix,
                    )?
                }
            }
        } else if self.fields.is_struct() {
            let update_from_arg_matches_raw = self
                .get_fields()
                .iter()
                .map(|f| f.to_tokens_update_from_arg_matches_raw())
                .collect::<Vec<_>>();

            quote! {
                #parent_ident::#ident { #(#fields_ref_mut)* } if #name == clap_name => {
                    let arg_matches = sub_arg_matches;
                    {
                        #(#update_from_arg_matches_raw)*
                    }
                }
            }
        } else if self.fields.is_tuple() {
            unimplemented!("Variant type tuple not implemented");
        } else if self.fields.is_unit() {
            unimplemented!("Variant type unit not implemented");
        } else {
            unimplemented!("Unknown variant type")
        }
    }

    fn to_tokents_augment_subcommands_variant(&self) -> proc_macro2::TokenStream {
        let name = self.get_name();

        let author_and_version = self.to_tokens_author_and_version();
        let app_call_help_about = self.to_tokens_app_call_help_about();

        let fields = self.get_fields();

        if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                let clap_app = clap_app.subcommand({
                    let clap_subcommand = clap::App::new(#name);

                    clap_subcommand
                });
                let clap_app = clap_app.setting(clap::AppSettings::AllowExternalSubcommands);
            }
        } else if self.fields.is_newtype() {
            let first_field_ty = &fields[0].ty;
            quote! {
                let clap_app = clap_app.subcommand({
                    let clap_subcommand = clap::App::new(#name);

                    let clap_subcommand = {
                        <#first_field_ty as clap_derive_darling::Args>::augment_args(clap_subcommand, None)
                    };

                    clap_subcommand
                        #author_and_version
                        #app_call_help_about
                });
            }
        } else if self.fields.is_struct() {
            let augment = self
                .get_fields()
                .iter()
                .map(|f| f.to_tokens_augment())
                .collect::<Vec<_>>();

            quote! {
                let clap_app = clap_app.subcommand({
                    let clap_subcommand = clap::App::new(#name);
                    {
                        let app = clap_subcommand;

                        #(#augment)*

                        app
                            #author_and_version
                            #app_call_help_about
                    }
                });
            }
        } else if self.fields.is_tuple() {
            unimplemented!("Variant type tuple not implemented");
        } else if self.fields.is_unit() {
            unimplemented!("Variant type unit not implemented");
        } else {
            unimplemented!("Unknown variant type")
        }
    }
    fn to_tokents_augment_subcommands_for_update_variant(&self) -> proc_macro2::TokenStream {
        self.to_tokents_augment_subcommands_variant()
    }
    fn to_tokens_has_subcommand(&self) -> proc_macro2::TokenStream {
        let name = self.get_name();

        if self.skip {
            quote! {}
        } else {
            quote! {
                {
                    let name = #name;
                    if name == clap_name {
                        return true;
                    }
                }
            }
        }
    }
}

impl ClapFields for ClapSubcommandVariant {
    fn get_fields(&self) -> Vec<&ClapField> {
        self.fields.iter().collect()
    }
    fn get_rename_all(&self) -> RenameAll {
        self.rename_all
    }

    fn get_rename_all_env(&self) -> RenameAll {
        self.rename_all_env
    }

    fn get_rename_all_value(&self) -> RenameAll {
        self.rename_all_value
    }
}
impl ClapFieldStructs for ClapSubcommandVariant {}
impl ClapRename for ClapSubcommandVariant {
    fn get_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            self.ident
                .to_string()
                .to_rename_all_case(self.get_rename_all())
        })
    }
}
impl ClapTraitImpls for ClapSubcommandVariant {
    fn get_ident(&self) -> &Ident {
        &self.ident
    }
}
impl ClapParserArgsCommon for ClapSubcommandVariant {
    fn get_author(&self) -> Option<&Override<String>> {
        self.author.as_ref()
    }
    fn get_version(&self) -> Option<&Override<String>> {
        self.version.as_ref()
    }
    fn get_help_heading(&self) -> Option<&String> {
        self.help_heading.as_ref()
    }
}
impl ClapDocCommon for ClapSubcommandVariant {
    fn get_attrs(&self) -> Vec<syn::Attribute> {
        self.attrs.clone()
    }
    fn get_help_about(&self) -> Option<String> {
        self.about.clone().map(|v| match v {
            Override::Explicit(v) => v,
            Override::Inherit => env!("CARGO_PKG_DESCRIPTION").to_string(),
        })
    }
    fn get_long_help_about(&self) -> Option<String> {
        self.long_about.clone()
    }
}
impl ClapDocCommonAuto for ClapSubcommandVariant {
    type Marker = ClapDocAboutMarker;
}
