use std::ops::Deref;

use darling::{util::Override, Error, FromMeta, Result};
use dyn_clone::{clone_trait_object, DynClone};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{AttrStyle, Attribute, Lit, LitStr};

use crate::{field::ClapField, RenameAll};

pub(crate) trait ClapIdentName {
    fn get_ident(&self) -> Option<Ident>;
    fn get_name(&self) -> Option<String>;

    fn get_parent(&self) -> Option<Option<Box<dyn ClapFieldParent>>> {
        None
    }

    fn get_ident_or(&self) -> Result<Ident> {
        self.get_ident()
            .ok_or_else(|| Error::custom("Failed to get ident"))
    }
    fn get_name_or(&self) -> Result<String> {
        let ident = self.get_ident_or()?;

        self.get_name().ok_or_else(|| {
            Error::custom(format!("Couldn't get name for {}", ident)).with_span(&ident)
        })
    }
    fn get_parent_or(&self) -> Result<Box<dyn ClapFieldParent>> {
        let ident = self.get_ident_or()?;

        self.get_parent().flatten().ok_or_else(|| {
            Error::custom(format!("Couldn't get parent for {}", ident)).with_span(&ident)
        })
    }
}

pub(crate) trait ClapCommonIdents {
    fn get_name_ident(&self) -> Ident {
        format_ident!("___name")
    }
    fn get_value_ident(&self) -> Ident {
        format_ident!("___value")
    }
    fn get_long_ident(&self) -> Ident {
        format_ident!("___long")
    }
    fn get_env_ident(&self) -> Ident {
        format_ident!("___env")
    }
    fn get_app_ident(&self) -> Ident {
        format_ident!("___app")
    }
    fn get_prefix_ident(&self) -> Ident {
        format_ident!("___prefix")
    }
    fn get_arg_matches_ident(&self) -> Ident {
        format_ident!("___arg_matches")
    }
}

pub(crate) trait ClapFieldParent: ClapIdentName + DynClone {
    fn get_ident_with_parent(&self) -> Result<Ident>;
}

#[derive(Clone)]
pub(crate) struct ClapIdentNameContainer(
    Option<Ident>,
    Option<Option<Box<dyn ClapFieldParent>>>,
    Option<String>,
);

impl ClapIdentNameContainer {
    pub fn from(s: &impl ClapIdentName) -> Self {
        let ident = s.get_ident();
        let parent_ident = s.get_parent();
        let name = s.get_name();

        Self(ident, parent_ident, name)
    }
}

impl ClapIdentName for ClapIdentNameContainer {
    fn get_ident(&self) -> Option<Ident> {
        self.0.clone()
    }
    fn get_name(&self) -> Option<String> {
        self.2.clone()
    }

    fn get_parent(&self) -> Option<Option<Box<dyn ClapFieldParent>>> {
        self.1.clone()
    }
}

impl ClapFieldParent for ClapIdentNameContainer {
    fn get_ident_with_parent(&self) -> Result<Ident> {
        let ident = self.get_ident_or()?;

        let parent_ident = self
            .get_parent()
            .map(|o| {
                o.ok_or_else(|| {
                    Error::custom(format!("Failed to get parent for {}", ident)).with_span(&ident)
                })
            })
            .transpose()?
            .map(|p| p.get_ident_with_parent())
            .transpose()?
            .map(|i| i.to_string())
            .unwrap_or_default();

        Ok(format_ident!("{}{}", parent_ident, ident))
    }
}

clone_trait_object!(ClapFieldParent);

impl std::fmt::Debug for Box<dyn ClapFieldParent> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Box<ClapFieldParent>")
    }
}

pub(crate) trait ClapTokensResult {
    fn to_tokens_result(&self) -> Result<TokenStream>;
}

pub(crate) trait ClapTokensResultAuto: ClapTokensResult {
    fn to_tokens(&self) -> TokenStream {
        match self.to_tokens_result() {
            Ok(tokens) => tokens,
            Err(error) => error.write_errors(),
        }
    }
}

impl<T: ClapTokensResult> ClapTokensResultAuto for T {}

pub(crate) trait ClapFields {
    fn get_fields(&self) -> Vec<&ClapField>;
    fn get_rename_all(&self) -> RenameAll;
    fn get_rename_all_env(&self) -> RenameAll;
    fn get_rename_all_value(&self) -> RenameAll;
}

pub(crate) trait ClapFieldStructs: ClapIdentName + ClapFields + Clone {
    fn augment_field(&self, _field: &mut ClapField) {}

    fn get_fieldstructs(&self) -> Vec<ClapField> {
        self.get_fields()
            .iter()
            .cloned()
            .cloned()
            .map(|mut v| {
                let container = ClapIdentNameContainer::from(self);
                v.parent = Some(Box::new(container));
                v.rename_all = self.get_rename_all();
                v.rename_all_env = self.get_rename_all_env();
                v.rename_all_value = self.get_rename_all_value();
                self.augment_field(&mut v);
                v
            })
            .collect()
    }

    fn to_tokens_augment_args_fields(&self) -> Result<Vec<TokenStream>> {
        self.get_fieldstructs()
            .iter()
            .map(|f| f.to_tokens_augment())
            .collect()
    }

    fn to_tokens_augment_args_for_update_fields(&self) -> Result<Vec<TokenStream>> {
        self.get_fieldstructs()
            .iter()
            .map(|f| f.to_tokens_augment_for_update())
            .collect()
    }

    fn to_tokens_from_arg_matches_fields(&self) -> Result<Vec<TokenStream>> {
        self.get_fieldstructs()
            .iter()
            .map(|f| f.to_tokens_from_arg_matches())
            .collect()
    }

    fn to_tokens_update_from_arg_matches_fields(&self) -> Result<Vec<TokenStream>> {
        self.get_fieldstructs()
            .iter()
            .map(|f| f.to_tokens_update_from_arg_matches())
            .collect()
    }
}

pub(crate) trait ClapTraitImpls:
    ClapCommonIdents + ClapIdentName + ClapFieldStructs + ClapParserArgsCommon + ClapDocCommon
{
    fn to_tokens_impl_args(&self) -> Result<TokenStream> {
        let ident = self.get_ident_or()?;
        let app_ident = self.get_app_ident();
        let prefix_ident = self.get_prefix_ident();

        let help_heading = self.to_tokens_help_heading();
        let author_and_version = self.to_tokens_author_and_version();
        let app_call_help_about = self.to_tokens_app_call_help_about();

        let augment_args_fields = self.to_tokens_augment_args_fields()?;
        let augment_args_for_update_fields = self.to_tokens_augment_args_for_update_fields()?;

        Ok(quote! {
            impl clap_derive_darling::Args for #ident {
                fn augment_args<'a>(#app_ident: clap::App<'a>, #prefix_ident: Vec<&'static str>) -> clap::App<'a> {
                    #help_heading

                    #(#augment_args_fields)*

                    #app_ident
                        #author_and_version
                        #app_call_help_about
                }
                fn augment_args_for_update<'a>(#app_ident: clap::App<'a>, #prefix_ident: Vec<&'static str>) -> clap::App<'a> {
                    #help_heading

                    #(#augment_args_for_update_fields)*

                    #app_ident
                        #author_and_version
                        #app_call_help_about
                }
            }
        })
    }

    fn to_tokens_impl_from_arg_matches(&self) -> Result<TokenStream> {
        let ident = self.get_ident_or()?;
        let arg_matches_ident = self.get_arg_matches_ident();
        let prefix_ident = self.get_prefix_ident();

        let from_arg_matches_fields = self.to_tokens_from_arg_matches_fields()?;
        let update_from_arg_matches_fields = self.to_tokens_update_from_arg_matches_fields()?;

        Ok(quote! {
            impl clap_derive_darling::FromArgMatches for #ident {
                fn from_arg_matches(#arg_matches_ident: &clap::ArgMatches, #prefix_ident: Vec<&'static str>) -> Result<Self, clap::Error> {
                    let v = #ident {
                        #(#from_arg_matches_fields)*
                    };

                    Ok(v)
                }
                fn update_from_arg_matches(&mut self, #arg_matches_ident: &clap::ArgMatches, #prefix_ident: Vec<&'static str>) -> Result<(), clap::Error> {
                    #(#update_from_arg_matches_fields)*

                    Ok(())
                }
            }
        })
    }

    fn to_tokens_impl_into_app(&self) -> Result<TokenStream> {
        let ident = self.get_ident_or()?;
        let app_ident = self.get_app_ident();
        let name = self.get_name_or()?;

        Ok(quote! {
            impl clap::IntoApp for #ident {
                fn into_app<'help>() -> clap::App<'help> {
                    let #app_ident = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args(#app_ident, Vec::new())
                }
                fn into_app_for_update<'help>() -> clap::App<'help> {
                    let #app_ident = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args_for_update(#app_ident, Vec::new())
                }
            }

            impl clap_derive_darling::Clap for #ident {}
        })
    }
}

pub(crate) trait ClapParserArgsCommon: ClapCommonIdents {
    fn get_author(&self) -> Option<&Override<String>>;
    fn get_version(&self) -> Option<&Override<String>>;
    fn get_help_heading(&self) -> Option<&String>;

    fn to_tokens_author_and_version(&self) -> TokenStream {
        let author = self
            .get_author()
            .map(|or| match or {
                Override::Explicit(author) => author,
                Override::Inherit => env!("CARGO_PKG_AUTHORS"),
            })
            .map(|s| {
                quote! { .author(#s) }
            });

        let version = self
            .get_version()
            .map(|or| match or {
                Override::Explicit(version) => version,
                Override::Inherit => env!("CARGO_PKG_VERSION"),
            })
            .map(|s| {
                quote! { .version(#s) }
            });

        quote! { #author #version }
    }

    fn to_tokens_help_heading(&self) -> Option<TokenStream> {
        let app_ident = self.get_app_ident();

        self.get_help_heading().map(|string| {
            quote! {
                let #app_ident = #app_ident.help_heading(#string);
            }
        })
    }
}

pub(crate) trait ClapDocCommon: ClapDocCommonAuto {
    fn get_attrs(&self) -> Vec<Attribute>;
    fn get_help_about(&self) -> Option<String>;
    fn get_long_help_about(&self) -> Option<String>;

    fn to_tokens_app_call_help_about(&self) -> Option<TokenStream> {
        let help_about = self.get_help_about();
        let long_help_about = self.get_long_help_about();

        let (doc_help_about, doc_long_help_about) = self.get_docs_short_long();

        let app_call_help_about_ident = self.get_app_call_help_about_ident();
        let help_about = help_about.or(doc_help_about).map(|string| {
            quote! {
                .#app_call_help_about_ident(#string)
            }
        });

        let app_call_long_help_about_ident = self.get_app_call_long_help_about_ident();
        let long_help_about = long_help_about.or(doc_long_help_about).map(|string| {
            quote! {
                .#app_call_long_help_about_ident(#string)
            }
        });

        if help_about.is_some() || long_help_about.is_some() {
            Some(quote! {
                #help_about
                #long_help_about
            })
        } else {
            None
        }
    }

    fn get_docs(&self) -> Vec<String> {
        self.get_attrs()
            .iter()
            .filter(|a| {
                a.style == AttrStyle::Outer
                    && a.path.segments.len() == 1
                    && a.path.segments[0].ident == "doc"
            })
            .map(|a| -> LitStr {
                let ts = a
                    .tokens
                    .clone()
                    .into_iter()
                    .skip(1)
                    .collect::<TokenStream>();
                syn::parse2(ts).unwrap()
            })
            .map(|v| {
                let v = v.value();
                v.trim().to_string()
            })
            .collect()
    }

    fn get_docs_short_long(&self) -> (Option<String>, Option<String>) {
        let mut doc_help = Vec::new();
        let mut doc_long_help = Vec::new();
        let mut long = false;
        for docstr in self.get_docs() {
            if long {
                doc_long_help.push(docstr);
            } else if docstr.is_empty() {
                long = true;
            } else {
                doc_help.push(docstr);
            }
        }
        (
            if doc_help.is_empty() {
                None
            } else {
                Some(doc_help.join("\n"))
            },
            if doc_long_help.is_empty() {
                None
            } else {
                Some(doc_long_help.join("\n"))
            },
        )
    }
}

pub(crate) trait ClapDocCommonAuto {
    type Marker: ClapDocCommonAutoMarker;
    fn get_app_call_help_about_ident(&self) -> Ident {
        Self::Marker::get_app_call_help_about_ident()
    }
    fn get_app_call_long_help_about_ident(&self) -> Ident {
        Self::Marker::get_app_call_long_help_about_ident()
    }
}

pub(crate) trait ClapDocCommonAutoMarker: sealed::Sealed {
    fn get_app_call_help_about_ident() -> Ident;
    fn get_app_call_long_help_about_ident() -> Ident;
}

pub(crate) struct ClapDocHelpMarker;
impl ClapDocCommonAutoMarker for ClapDocHelpMarker {
    fn get_app_call_help_about_ident() -> Ident {
        format_ident!("help")
    }
    fn get_app_call_long_help_about_ident() -> Ident {
        format_ident!("long_help")
    }
}
pub(crate) struct ClapDocAboutMarker;
impl ClapDocCommonAutoMarker for ClapDocAboutMarker {
    fn get_app_call_help_about_ident() -> Ident {
        format_ident!("about")
    }
    fn get_app_call_long_help_about_ident() -> Ident {
        format_ident!("long_about")
    }
}

pub(crate) trait ClapDocHelp: ClapDocCommonAuto<Marker = ClapDocHelpMarker> {}
pub(crate) trait ClapDocAbout: ClapDocCommonAuto<Marker = ClapDocAboutMarker> {}

impl<T: ClapDocCommonAuto<Marker = ClapDocHelpMarker>> ClapDocHelp for T {}
impl<T: ClapDocCommonAuto<Marker = ClapDocAboutMarker>> ClapDocAbout for T {}
mod sealed {
    pub trait Sealed {}

    impl Sealed for super::ClapDocHelpMarker {}
    impl Sealed for super::ClapDocAboutMarker {}
}

#[derive(Clone, Debug, Default)]
pub(crate) struct VecStringAttr(Vec<String>);

impl VecStringAttr {
    pub fn new<T: Into<String>>(vals: Vec<T>) -> Self {
        Self(vals.into_iter().map(T::into).collect())
    }

    pub fn to_strings(&self) -> Vec<String> {
        self.0.to_vec()
    }
}

impl Deref for VecStringAttr {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<String>> for VecStringAttr {
    fn from(v: Vec<String>) -> Self {
        VecStringAttr::new(v)
    }
}

impl From<VecStringAttr> for Vec<String> {
    fn from(v: VecStringAttr) -> Self {
        v.to_strings()
    }
}

impl FromMeta for VecStringAttr {
    fn from_list(items: &[syn::NestedMeta]) -> Result<Self> {
        let mut vec = Vec::with_capacity(items.len());
        for item in items {
            if let syn::NestedMeta::Lit(Lit::Str(ref str)) = *item {
                vec.push(str.value());
            } else {
                return Err(Error::custom("not a string").with_span(item));
            }
        }
        Ok(VecStringAttr::new(vec))
    }
}
