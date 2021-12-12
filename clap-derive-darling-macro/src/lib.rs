//! # An alternative implementation of the Clap Derive macros
//!
//! ## Why?
//! Mostly so that I have an excuse to play with proc macros
//!
//! ## But why?
//! Yeah I know, reinventing the wheel, etc. I needed a project.

mod arg_enum;
mod args;
mod common;
mod field;
mod parser;
mod subcommand;

use darling::{FromDeriveInput, FromMeta, ToTokens};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use arg_enum::ClapArgEnum;
use args::ClapArgs;
use parser::ClapParser;
use subcommand::ClapSubcommand;

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

#[proc_macro_derive(Subcommand, attributes(clap))]
pub fn derive_subcommand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let conf_struct = ClapSubcommand::from_derive_input(&input).expect("Wrong options");

    let tokens = quote! { #conf_struct };

    tokens.into()
}

#[proc_macro_derive(ArgEnum, attributes(clap))]
pub fn derive_arg_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let conf_struct = ClapArgEnum::from_derive_input(&input).expect("Wrong options");

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
    Camel,
    #[darling(rename = "kebab-case")]
    Kebab,
    #[darling(rename = "PascalCase")]
    Pascal,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
    #[darling(rename = "snake_case")]
    Snake,
    #[darling(rename = "lower")]
    Lower,
    #[darling(rename = "UPPER")]
    Upper,
    #[darling(rename = "verbatim")]
    Verbatim,
}

pub(crate) trait RenameAllCasing {
    fn to_rename_all_case(&self, case: RenameAll) -> String;
}

impl RenameAllCasing for str {
    fn to_rename_all_case(&self, case: RenameAll) -> String {
        use convert_case::{Case, Casing};
        match case {
            RenameAll::Camel => self.to_case(Case::Camel),
            RenameAll::Kebab => self.to_case(Case::Kebab),
            RenameAll::Pascal => self.to_case(Case::Pascal),
            RenameAll::ScreamingSnake => self.to_case(Case::ScreamingSnake),
            RenameAll::Snake => self.to_case(Case::Snake),
            RenameAll::Lower => self.to_case(Case::Lower),
            RenameAll::Upper => self.to_case(Case::Upper),
            RenameAll::Verbatim => self.to_string(),
        }
    }
}

impl RenameAllCasing for String {
    fn to_rename_all_case(&self, case: RenameAll) -> String {
        use convert_case::{Case, Casing};
        match case {
            RenameAll::Camel => self.to_case(Case::Camel),
            RenameAll::Kebab => self.to_case(Case::Kebab),
            RenameAll::Pascal => self.to_case(Case::Pascal),
            RenameAll::ScreamingSnake => self.to_case(Case::ScreamingSnake),
            RenameAll::Snake => self.to_case(Case::Snake),
            RenameAll::Lower => self.to_case(Case::Lower),
            RenameAll::Upper => self.to_case(Case::Upper),
            RenameAll::Verbatim => self.to_string(),
        }
    }
}

pub(crate) fn default_rename_all() -> RenameAll {
    RenameAll::Kebab
}

pub(crate) fn default_rename_all_env() -> RenameAll {
    RenameAll::ScreamingSnake
}

pub(crate) fn default_rename_all_value() -> RenameAll {
    RenameAll::ScreamingSnake
}

impl ToTokens for RenameAll {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use RenameAll::*;

        tokens.extend(match self {
            Camel => quote! { clap_derive_darling::rename::camel_case },
            Kebab => quote! { clap_derive_darling::rename::kebab_case },
            Pascal => quote! { clap_derive_darling::rename::pascal_case },
            ScreamingSnake => quote! { clap_derive_darling::rename::screaming_snake_case },
            Snake => quote! { clap_derive_darling::rename::snake_case },
            Lower => quote! { clap_derive_darling::rename::lower_case },
            Upper => quote! { clap_derive_darling::rename::upper_case },
            Verbatim => quote! { clap_derive_darling::rename::verbatim_case },
        });
    }
}
