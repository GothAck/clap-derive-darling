use std::vec;

use darling::{
    ast,
    util::{Ignored, Override},
    FromDeriveInput, ToTokens,
};
use quote::quote;
use syn::Ident;

use super::{common::ClapParserArgsCommon, field::ClapField, RenameAll};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clap), forward_attrs(doc), supports(struct_named))]
pub(crate) struct ClapArgs {
    ident: Ident,
    data: ast::Data<Ignored, ClapField>,
    #[allow(dead_code)]
    attrs: Vec<syn::Attribute>,

    #[allow(dead_code)]
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    version: Option<Override<String>>,
    #[darling(default)]
    author: Option<Override<String>>,
    #[allow(dead_code)]
    #[darling(default)]
    about: Option<Override<String>>,
    #[allow(dead_code)]
    #[darling(default)]
    long_about: Option<Override<String>>,
    #[allow(dead_code)]
    #[darling(default)]
    verbatim_doc_comment: bool,
    #[darling(default)]
    help_heading: Option<String>,
    #[darling(default)]
    rename_all: Option<RenameAll>,
    #[darling(default)]
    rename_all_env: Option<RenameAll>,
    #[darling(default)]
    rename_all_value: Option<RenameAll>,
}

impl ToTokens for ClapArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.to_tokens_args());
    }
}

impl ClapArgs {
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

        let author_and_version =
            self.format_author_and_version(self.author.as_ref(), self.version.as_ref());

        let help_heading = self.format_help_heading(self.help_heading.as_ref());

        quote! {
            impl clap_derive_darling::Args for #ident {
                fn augment_args<'a>(app: clap::App<'a>, prefix: &Option<String>) -> clap::App<'a> {
                    #name_storage

                    #help_heading

                    #(#augment_args_fields)*
                    app
                        #author_and_version
                }
                fn augment_args_for_update<'a>(app: clap::App<'a>, prefix: &Option<String>) -> clap::App<'a> {
                    #name_storage

                    #help_heading

                    #(#augment_args_for_update_fields)*

                    app
                        #author_and_version
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
        }
    }
}

impl ClapParserArgsCommon for ClapArgs {}
