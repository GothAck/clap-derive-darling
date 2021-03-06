use std::vec;

use darling::{ast, util::Override, FromDeriveInput, FromVariant, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{
    common::{
        ClapCommonIdents, ClapDocAboutMarker, ClapDocCommon, ClapDocCommonAuto, ClapFieldParent,
        ClapFieldStructs, ClapFields, ClapIdentName, ClapIdentNameContainer, ClapParserArgsCommon,
        ClapTokensResult, ClapTraitImpls,
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

impl ClapIdentName for ClapSubcommand {
    fn get_ident(&self) -> Option<Ident> {
        Some(self.ident.clone())
    }
    fn get_name(&self) -> Option<String> {
        None
    }
}
impl ClapCommonIdents for ClapSubcommand {}
impl ClapTokensResult for ClapSubcommand {
    fn to_tokens_result(&self) -> Result<TokenStream> {
        let impl_from_arg_matches = self.to_tokens_impl_from_arg_matches()?;
        let impl_subcommand = self.to_tokens_impl_subcommand()?;

        Ok(quote! {
            #impl_from_arg_matches
            #impl_subcommand
        })
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
                let container = ClapIdentNameContainer::from(self);
                v.parent = Some(Box::new(container));
                v
            })
            .collect()
    }

    fn to_tokens_impl_from_arg_matches(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let arg_matches_ident = self.get_arg_matches_ident();
        let prefix_ident = self.get_prefix_ident();

        let from_arg_matches_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_from_arg_matches_variant())
            .collect::<Result<Vec<_>>>()?;

        let update_from_arg_matches_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_update_from_arg_matches_variant())
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            impl clap_derive_darling::FromArgMatches for #ident {
                fn from_arg_matches(#arg_matches_ident: &clap::ArgMatches, #prefix_ident: Vec<&'static str>) -> Result<Self, clap::Error> {
                    if let Some((clap_name, sub_arg_matches)) = #arg_matches_ident.subcommand() {
                        {
                            let #arg_matches_ident = sub_arg_matches;

                            #(#from_arg_matches_variants)*
                        }
                        Err(clap::Error::raw(clap::ErrorKind::UnrecognizedSubcommand, format!("The subcommand '{}' watn't recognized", clap_name)))
                    } else {
                        Err(clap::Error::raw(clap::ErrorKind::MissingSubcommand, "A subcommand is required but one was not provided"))
                    }
                }
                fn update_from_arg_matches(&mut self, #arg_matches_ident: &clap::ArgMatches, #prefix_ident: Vec<&'static str>) -> Result<(), clap::Error> {
                    if let Some((clap_name, sub_arg_matches)) = #arg_matches_ident.subcommand() {
                        match self {
                            #(#update_from_arg_matches_variants)*

                            s => {
                                *s = <Self as clap_derive_darling::FromArgMatches>::from_arg_matches(#arg_matches_ident, #prefix_ident)?;
                            }
                        }
                    }
                    Ok(())
                }
            }
        })
    }

    fn to_tokens_impl_subcommand(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let app_ident = self.get_app_ident();
        let prefix_ident = self.get_prefix_ident();

        let augment_subcommands_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokents_augment_subcommands_variant())
            .collect::<Result<Vec<_>>>()?;

        let augment_subcommands_for_update_variants = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokents_augment_subcommands_for_update_variant())
            .collect::<Result<Vec<_>>>()?;

        let has_subcommands = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_has_subcommand())
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! {
            impl clap_derive_darling::Subcommand for #ident {
                fn augment_subcommands<'b>(#app_ident: clap::App<'b>, #prefix_ident: Vec<&'static str>) -> clap::App<'b> {
                    #(#augment_subcommands_variants)*

                    #app_ident
                }
                fn augment_subcommands_for_update<'b>(#app_ident: clap::App<'b>, #prefix_ident: Vec<&'static str>) -> clap::App<'b> {
                    #(#augment_subcommands_for_update_variants)*

                    #app_ident
                }
                fn has_subcommand(clap_name: &str) -> bool {
                    #(#has_subcommands)*

                    false
                }
            }
        })
    }
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(clap), forward_attrs(doc))]
pub(crate) struct ClapSubcommandVariant {
    ident: Ident,
    fields: ast::Fields<ClapField>,
    attrs: Vec<syn::Attribute>,

    #[darling(skip)]
    parent: Option<Box<dyn ClapFieldParent>>,

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

impl ClapIdentName for ClapSubcommandVariant {
    fn get_ident(&self) -> Option<Ident> {
        Some(self.ident.clone())
    }
    fn get_parent(&self) -> Option<Option<Box<dyn ClapFieldParent>>> {
        Some(self.parent.clone())
    }
    fn get_name(&self) -> Option<String> {
        Some(self.name.clone().unwrap_or_else(|| {
            self.ident
                .to_string()
                .to_rename_all_case(self.get_rename_all())
        }))
    }
}
impl ClapCommonIdents for ClapSubcommandVariant {}

impl ClapSubcommandVariant {
    fn to_tokens_from_arg_matches_variant(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let arg_matches_ident = self.get_arg_matches_ident();
        let prefix_ident = self.get_prefix_ident();
        let name = self.get_name_or()?;
        let parent_ident = self.get_parent_or()?.get_ident_or()?;

        let fields = self.get_fieldstructs();

        Ok(if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                if #name == clap_name {
                    return Ok(#parent_ident::#ident(
                        ::std::iter::once(::std::string::String::from(clap_name))
                            .chain(
                                #arg_matches_ident
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
                            #arg_matches_ident,
                            #prefix_ident,
                        )?,
                    ));
                }
            }
        } else if self.fields.is_struct() {
            let from_arg_matches_fields = self.to_tokens_from_arg_matches_fields()?;

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
        })
    }

    fn to_tokens_update_from_arg_matches_variant(&self) -> Result<TokenStream> {
        let ident = &self.ident;
        let arg_matches_ident = self.get_arg_matches_ident();
        let prefix_ident = self.get_prefix_ident();

        let name = self.get_name_or()?;
        let parent_ident = self.get_parent_or()?.get_ident_or()?;

        let fields_ref_mut = self
            .get_fieldstructs()
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    ref mut #ident,
                }
            })
            .collect::<Vec<_>>();

        Ok(if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                #parent_ident::#ident(ref mut clap_arg) if #name == clap_name => {
                    *clap_arg = ::std::iter::once(::std::string::String::from(clap_name))
                        .chain(
                            #arg_matches_ident
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
                    let #arg_matches_ident = sub_arg_matches;
                    clap_derive_darling::FromArgMatches::update_from_arg_matches(
                        clap_arg,
                        sub_arg_matches,
                        #prefix_ident,
                    )?
                }
            }
        } else if self.fields.is_struct() {
            let update_from_arg_matches_raw = self
                .get_fieldstructs()
                .iter()
                .map(|f| f.to_tokens_update_from_arg_matches_raw())
                .collect::<Result<Vec<_>>>()?;

            quote! {
                #parent_ident::#ident { #(#fields_ref_mut)* } if #name == clap_name => {
                    let #arg_matches_ident = sub_arg_matches;
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
        })
    }

    fn to_tokents_augment_subcommands_variant(&self) -> Result<TokenStream> {
        let app_ident = self.get_app_ident();

        let name = self.get_name_or()?;

        let author_and_version = self.to_tokens_author_and_version();
        let app_call_help_about = self.to_tokens_app_call_help_about();

        let fields = self.get_fieldstructs();

        Ok(if self.skip {
            quote! {}
        } else if self.external_subcommand {
            quote! {
                let #app_ident = #app_ident.subcommand({
                    let clap_subcommand = clap::App::new(#name);

                    clap_subcommand
                });
                let #app_ident = #app_ident.setting(clap::AppSettings::AllowExternalSubcommands);
            }
        } else if self.fields.is_newtype() {
            let first_field_ty = &fields[0].ty;
            quote! {
                let #app_ident = #app_ident.subcommand({
                    let clap_subcommand = clap::App::new(#name);

                    let clap_subcommand = {
                        <#first_field_ty as clap_derive_darling::Args>::augment_args(clap_subcommand, Vec::new())
                    };

                    clap_subcommand
                        #author_and_version
                        #app_call_help_about
                });
            }
        } else if self.fields.is_struct() {
            let augment = self
                .get_fieldstructs()
                .iter()
                .map(|f| f.to_tokens_augment())
                .collect::<Result<Vec<_>>>()?;

            quote! {
                let #app_ident = #app_ident.subcommand({
                    let clap_subcommand = clap::App::new(#name);
                    {
                        let #app_ident = clap_subcommand;

                        #(#augment)*

                        #app_ident
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
        })
    }
    fn to_tokents_augment_subcommands_for_update_variant(&self) -> Result<TokenStream> {
        self.to_tokents_augment_subcommands_variant()
    }
    fn to_tokens_has_subcommand(&self) -> Result<TokenStream> {
        let name = self.get_name_or()?;

        Ok(if self.skip {
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
        })
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
impl ClapTraitImpls for ClapSubcommandVariant {}
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

#[cfg(test)]
mod test;
