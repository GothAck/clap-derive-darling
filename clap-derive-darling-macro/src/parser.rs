use std::vec;

use darling::{
    ast,
    util::{Ignored, Override},
    FromDeriveInput, ToTokens,
};
use quote::quote;
use syn::Ident;

use crate::{
    common::{
        ClapDocAboutMarker, ClapDocCommon, ClapDocCommonAuto, ClapFieldStructs, ClapFields,
        ClapParserArgsCommon, ClapTraitImpls,
    },
    field::ClapField,
    RenameAll, RenameAllCasing,
};

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
    long_about: Option<String>,
    #[allow(dead_code)]
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
    fn to_tokens_args(&self) -> proc_macro2::TokenStream {
        let impl_args = self.to_tokens_impl_args();
        let impl_from_arg_matches = self.to_tokens_impl_from_arg_matches();
        let impl_into_app = self.to_tokens_impl_into_app();

        quote! {
            #impl_args
            #impl_from_arg_matches
            #impl_into_app
        }
    }
}

impl ClapFields for ClapParser {
    fn get_fields(&self) -> Vec<&ClapField> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Should always be a struct")
            .fields
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
}
impl ClapFieldStructs for ClapParser {}
impl ClapTraitImpls for ClapParser {
    fn get_ident(&self) -> &Ident {
        &self.ident
    }
    fn get_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            self.ident
                .to_string()
                .to_rename_all_case(self.get_rename_all())
        })
    }
}
impl ClapParserArgsCommon for ClapParser {
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
impl ClapDocCommon for ClapParser {
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
impl ClapDocCommonAuto for ClapParser {
    type Marker = ClapDocAboutMarker;
}
