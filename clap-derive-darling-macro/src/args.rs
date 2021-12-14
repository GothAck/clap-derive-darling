use darling::{
    ast,
    util::{Ignored, Override},
    FromDeriveInput, Result,
};
use quote::quote;
use syn::Ident;

use crate::{
    common::{
        ClapDocAboutMarker, ClapDocCommon, ClapDocCommonAuto, ClapFieldStructs, ClapFields,
        ClapIdentName, ClapParserArgsCommon, ClapTokensResult, ClapTraitImpls, VecStringAttr,
    },
    field::ClapField,
    RenameAll,
};
#[derive(Clone, Debug, FromDeriveInput)]
#[darling(attributes(clap), forward_attrs(doc), supports(struct_named))]
pub(crate) struct ClapArgs {
    ident: Ident,
    data: ast::Data<Ignored, ClapField>,
    attrs: Vec<syn::Attribute>,

    #[allow(dead_code)]
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
    flatten: VecStringAttr,

    #[darling(skip, default = "crate::default_rename_all")]
    rename_all: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_env")]
    rename_all_env: RenameAll,
    #[darling(skip, default = "crate::default_rename_all_value")]
    rename_all_value: RenameAll,
}

impl ClapIdentName for ClapArgs {
    fn get_ident(&self) -> Option<Ident> {
        Some(self.ident.clone())
    }
    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
}

impl ClapTokensResult for ClapArgs {
    fn to_tokens_result(&self) -> Result<proc_macro2::TokenStream> {
        let impl_args = self.to_tokens_impl_args()?;
        let impl_from_arg_matches = self.to_tokens_impl_from_arg_matches()?;

        Ok(quote! {
            #impl_args
            #impl_from_arg_matches
        })
    }
}

impl ClapFields for ClapArgs {
    fn get_fields(&self) -> Vec<&ClapField> {
        self.data
            .as_ref()
            .take_struct()
            .expect("Should always be a struct")
            .fields
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
impl ClapFieldStructs for ClapArgs {
    fn augment_field(&self, field: &mut ClapField) {
        field.flatten_args = self.flatten.to_strings();
    }
}
impl ClapTraitImpls for ClapArgs {}
impl ClapParserArgsCommon for ClapArgs {
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
impl ClapDocCommon for ClapArgs {
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
impl ClapDocCommonAuto for ClapArgs {
    type Marker = ClapDocAboutMarker;
}
