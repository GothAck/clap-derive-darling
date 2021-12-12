use std::vec;

use darling::{ast, util::Override, FromDeriveInput, FromVariant, ToTokens};
use quote::quote;
use syn::Ident;

use crate::{
    common::{ClapParserArgsCommon, ClapRename},
    RenameAll,
};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clap), supports(enum_any))]
pub struct ClapArgEnum {
    ident: Ident,
    data: ast::Data<ClapArgEnumVariant, ()>,

    #[darling(default = "crate::default_rename_all")]
    rename_all: RenameAll,
}

impl ToTokens for ClapArgEnum {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.to_tokens_impl_arg_enum().unwrap());
    }
}

impl ClapParserArgsCommon for ClapArgEnum {
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

impl ClapArgEnum {
    fn get_variants(&self) -> Vec<ClapArgEnumVariant> {
        self.data
            .as_ref()
            .take_enum()
            .expect("Should always be an enum")
            .iter()
            .cloned()
            .cloned()
            .map(|mut v| {
                v.parent_ident = Some(self.ident.clone());
                v.rename_all = self.rename_all;
                v
            })
            .collect()
    }

    fn to_tokens_impl_arg_enum(&self) -> darling::Result<proc_macro2::TokenStream> {
        let ident = &self.ident;

        let name_storage = self.to_tokens_name_storage();

        let self_variants = self
            .get_variants()
            .iter()
            .map(|v| {
                let v_ident = &v.ident;
                quote! { Self::#v_ident, }
            })
            .collect::<Vec<_>>();

        let match_to_possible_value = self
            .get_variants()
            .iter()
            .map(|v| v.to_tokens_match_to_possible_value())
            .collect::<darling::Result<Vec<_>>>()?;

        Ok(quote! {
            impl clap_derive_darling::ArgEnum for #ident {
                fn value_variants<'a>() -> &'a [Self] {
                    &[
                        #(#self_variants)*
                    ]
                }
                fn to_possible_value<'a>(&self) -> Option<clap::PossibleValue<'a>> {
                    #name_storage

                    match self {
                        #(#match_to_possible_value)*
                        _ => None,
                    }
                }
            }
        })
    }
}

#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(clap), forward_attrs(doc))]
pub struct ClapArgEnumVariant {
    ident: Ident,
    fields: ast::Fields<()>,

    #[darling(default)]
    help: Option<String>,

    #[darling(skip)]
    parent_ident: Option<Ident>,
    #[darling(skip, default = "crate::default_rename_all")]
    rename_all: RenameAll,
}

impl ClapArgEnumVariant {
    fn to_tokens_match_to_possible_value(&self) -> darling::Result<proc_macro2::TokenStream> {
        let ident = &self.ident;
        let parent_ident = self
            .parent_ident
            .as_ref()
            .ok_or_else(|| darling::Error::custom("Missing parent_ident").with_span(&ident))?;

        if !self.fields.is_unit() {
            return Err(darling::Error::custom(format!(
                "Enum variant {}::{} should not have any fields",
                parent_ident, ident
            ))
            .with_span(&ident));
        }

        let ident = &self.ident;
        let name = self.get_name();
        let name_renamed =
            self.to_tokens_rename_all(self.rename_all, None, quote! { #name.to_string() });
        let help = self.help.as_ref().map(|v| quote! { .help(#v) });

        Ok(quote! {
            Self::#ident => Some(clap::PossibleValue::new(
                get_cache_str_keyed("name", #name, &None, || #name_renamed)
            ) #help),
        })
    }
}

impl ClapRename for ClapArgEnumVariant {
    fn get_name(&self) -> String {
        self.ident.to_string()
    }
}
