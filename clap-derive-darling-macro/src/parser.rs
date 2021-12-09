use std::vec;

use darling::{
    ast,
    util::{Ignored, Override},
    FromDeriveInput, ToTokens,
};
use quote::quote;
use syn::Ident;

use crate::{common::ClapParserArgsCommon, field::ClapField, RenameAll};

#[allow(dead_code)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clap), forward_attrs(doc), supports(struct_named))]
pub(crate) struct ClapParser {
    ident: Ident,
    data: ast::Data<Ignored, ClapField>,
    attrs: Vec<syn::Attribute>,

    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    version: Option<Override<String>>,
    #[darling(default)]
    author: Option<Override<String>>,
    #[darling(default)]
    about: Option<Override<String>>,
    #[darling(default)]
    long_about: Option<Override<String>>,
    #[darling(default)]
    verbatim_doc_comment: bool,
    #[darling(default)]
    help_heading: Option<String>,
    #[darling(default)]
    rename_all: Option<super::RenameAll>,
    #[darling(default)]
    rename_all_env: Option<super::RenameAll>,
    #[darling(default)]
    rename_all_value: Option<super::RenameAll>,
}

impl ToTokens for ClapParser {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.to_tokens_args());
    }
}

impl ClapParser {
    fn fields(&self) -> Vec<ClapField> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Should never be enum")
            .fields
            .iter()
            .cloned()
            .cloned()
            .map(|mut v| {
                v.rename_all = self.get_rename_all();
                v.rename_all_env = self.get_rename_all_env();
                v.rename_all_value = self.get_rename_all_value();
                v
            })
            .collect::<Vec<_>>()
    }

    fn get_rename_all(&self) -> RenameAll {
        match &self.rename_all {
            Some(rename_all) => *rename_all,
            None => crate::default_rename_all(),
        }
    }

    fn get_rename_all_env(&self) -> RenameAll {
        match &self.rename_all_env {
            Some(rename_all) => *rename_all,
            None => crate::default_rename_all_env(),
        }
    }

    fn get_rename_all_value(&self) -> RenameAll {
        match &self.rename_all_value {
            Some(rename_all) => *rename_all,
            None => crate::default_rename_all_value(),
        }
    }

    fn to_tokens_args(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        let augment_args_fields = self
            .fields()
            .iter()
            .map(|f| f.to_tokens_augment())
            .collect::<Vec<_>>();

        let augment_args_for_update_fields = self
            .fields()
            .iter()
            .map(|f| f.to_tokens_augment_for_update())
            .collect::<Vec<_>>();

        let from_arg_matches_fields = self
            .fields()
            .iter()
            .map(|f| f.to_tokens_from_arg_matches())
            .collect::<Vec<_>>();

        let update_from_arg_matches_fields = self
            .fields()
            .iter()
            .map(|f| f.to_tokens_update_from_arg_matches())
            .collect::<Vec<_>>();

        let name_storage = self.to_tokens_name_storage();

        let name = self
            .name
            .clone()
            .unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());

        quote! {
            impl clap_derive_darling::Args for #ident {
                fn augment_args<'a>(app: clap::App<'a>, prefix: &Option<String>) -> clap::App<'a> {
                    #name_storage

                    #(#augment_args_fields)*

                    app
                }
                fn augment_args_for_update<'a>(app: clap::App<'a>, prefix: &Option<String>) -> clap::App<'a> {
                    #name_storage

                    #(#augment_args_for_update_fields)*

                    app
                }
            }

            impl clap_derive_darling::FromArgMatches for #ident {
                fn from_arg_matches(arg_matches: &clap::ArgMatches, prefix: &Option<String>) -> Result<Self, clap::Error> {
                    let v = #ident {
                        #(#from_arg_matches_fields)*
                    };

                    Ok(v)
                }
                fn update_from_arg_matches(&mut self, arg_matches: &clap::ArgMatches, prefix: &Option<String>) -> Result<(), clap::Error> {
                    #(#update_from_arg_matches_fields)*

                    Ok(())
                }
            }

            impl clap::IntoApp for #ident {
                fn into_app<'help>() -> clap::App<'help> {
                    let app = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args(app, &None)
                }
                fn into_app_for_update<'help>() -> clap::App<'help> {
                    let app = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args_for_update(app, &None)
                }
            }

            impl clap_derive_darling::Clap for #ident {}
        }
    }
}

impl ClapParserArgsCommon for ClapParser {}
