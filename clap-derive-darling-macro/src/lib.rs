//! # An alternative implementation of the Clap Derive macros
//!
//! ## Why?
//! Mostly so that I have an excuse to play with proc macros
//!
//! ## But why?
//! Yeah I know, reinventing the wheel, etc. I needed a project.

mod args;
mod common;
mod field;
mod parser;

use darling::{FromDeriveInput, FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use args::ClapArgs;
use parser::ClapParser;

#[proc_macro_derive(Parser, attributes(clap, doc))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let conf_struct = ClapParser::from_derive_input(&input).expect("Wrong options");

    let tokens = quote! { #conf_struct };

    tokens.into()
}

#[proc_macro_derive(Args, attributes(clap, doc))]
pub fn derive_args(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let conf_struct = ClapArgs::from_derive_input(&input).expect("Wrong options");

    let tokens = quote! { #conf_struct };

    tokens.into()
}

#[derive(Debug, Clone, Copy, FromMeta)]
enum ClapType {
    Parser,
    Args,
}

impl Default for ClapType {
    fn default() -> Self {
        ClapType::Parser
    }
}

#[derive(Debug, Clone, Copy, FromMeta)]
pub(crate) enum RenameAll {
    #[darling(rename = "camelCase")]
    CamelCase,
    #[darling(rename = "kebab-case")]
    KebabCase,
    #[darling(rename = "PascalCase")]
    PascalCase,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnakeCase,
    #[darling(rename = "snake_case")]
    SnakeCase,
    #[darling(rename = "lower")]
    Lower,
    #[darling(rename = "UPPER")]
    Upper,
    #[darling(rename = "verbatim")]
    Verbatim,
}

pub(crate) fn default_rename_all() -> RenameAll {
    RenameAll::KebabCase
}

pub(crate) fn default_rename_all_env() -> RenameAll {
    RenameAll::ScreamingSnakeCase
}

pub(crate) fn default_rename_all_value() -> RenameAll {
    RenameAll::ScreamingSnakeCase
}

impl ToTokens for RenameAll {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use RenameAll::*;

        tokens.extend(match self {
            CamelCase => quote! { clap_derive_darling::rename::camel_case },
            KebabCase => quote! { clap_derive_darling::rename::kebab_case },
            PascalCase => quote! { clap_derive_darling::rename::pascal_case },
            ScreamingSnakeCase => quote! { clap_derive_darling::rename::screaming_snake_case },
            SnakeCase => quote! { clap_derive_darling::rename::snake_case },
            Lower => quote! { clap_derive_darling::rename::lower_case },
            Upper => quote! { clap_derive_darling::rename::upper_case },
            Verbatim => quote! { clap_derive_darling::rename::verbatim_case },
        });
    }
}
