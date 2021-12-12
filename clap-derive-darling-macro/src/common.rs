use darling::{util::Override, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{AttrStyle, Attribute, LitStr};

use crate::{field::ClapField, RenameAll};

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

pub(crate) trait ClapFieldStructs: ClapFields {
    fn get_fieldstructs(&self) -> Vec<ClapField> {
        self.get_fields()
            .iter()
            .cloned()
            .cloned()
            .map(|mut v| {
                v.rename_all = self.get_rename_all();
                v.rename_all_env = self.get_rename_all_env();
                v.rename_all_value = self.get_rename_all_value();
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

pub(crate) trait ClapRename {
    fn get_name(&self) -> String;

    fn to_tokens_rename_all(
        &self,
        rename: RenameAll,
        prefix: Option<TokenStream>,
        name: TokenStream,
    ) -> TokenStream {
        if let Some(prefix) = prefix {
            quote! {
                #rename(clap_derive_darling::rename::prefix(#name, &#prefix))
            }
        } else {
            quote! {
                #rename(#name)
            }
        }
    }
}

pub(crate) trait ClapTraitImpls:
    ClapRename + ClapFieldStructs + ClapParserArgsCommon + ClapDocCommon
{
    fn get_ident(&self) -> &Ident;

    fn to_tokens_impl_args(&self) -> Result<TokenStream> {
        let ident = self.get_ident();

        let name_storage = self.to_tokens_name_storage();
        let help_heading = self.to_tokens_help_heading();
        let author_and_version = self.to_tokens_author_and_version();
        let app_call_help_about = self.to_tokens_app_call_help_about();

        let augment_args_fields = self.to_tokens_augment_args_fields()?;
        let augment_args_for_update_fields = self.to_tokens_augment_args_for_update_fields()?;

        Ok(quote! {
            impl clap_derive_darling::Args for #ident {
                fn augment_args(app: clap::App<'_>, prefix: Option<String>) -> clap::App<'_> {
                    #name_storage

                    #help_heading

                    #(#augment_args_fields)*

                    app
                        #author_and_version
                        #app_call_help_about
                }
                fn augment_args_for_update(app: clap::App<'_>, prefix: Option<String>) -> clap::App<'_> {
                    #name_storage

                    #help_heading

                    #(#augment_args_for_update_fields)*

                    app
                        #author_and_version
                        #app_call_help_about
                }
            }
        })
    }

    fn to_tokens_impl_from_arg_matches(&self) -> Result<TokenStream> {
        let ident = self.get_ident();

        let from_arg_matches_fields = self.to_tokens_from_arg_matches_fields()?;
        let update_from_arg_matches_fields = self.to_tokens_update_from_arg_matches_fields()?;

        Ok(quote! {
            impl clap_derive_darling::FromArgMatches for #ident {
                fn from_arg_matches(arg_matches: &clap::ArgMatches, prefix: Option<String>) -> Result<Self, clap::Error> {
                    let v = #ident {
                        #(#from_arg_matches_fields)*
                    };

                    Ok(v)
                }
                fn update_from_arg_matches(&mut self, arg_matches: &clap::ArgMatches, prefix: Option<String>) -> Result<(), clap::Error> {
                    #(#update_from_arg_matches_fields)*

                    Ok(())
                }
            }
        })
    }

    fn to_tokens_impl_into_app(&self) -> TokenStream {
        let ident = self.get_ident();
        let name = self.get_name();

        quote! {
            impl clap::IntoApp for #ident {
                fn into_app<'help>() -> clap::App<'help> {
                    let app = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args(app, None)
                }
                fn into_app_for_update<'help>() -> clap::App<'help> {
                    let app = clap::App::new(#name);
                    <Self as clap_derive_darling::Args>::augment_args_for_update(app, None)
                }
            }

            impl clap_derive_darling::Clap for #ident {}
        }
    }
}

pub(crate) trait ClapParserArgsCommon {
    fn get_author(&self) -> Option<&Override<String>>;
    fn get_version(&self) -> Option<&Override<String>>;
    fn get_help_heading(&self) -> Option<&String>;

    fn to_tokens_name_storage(&self) -> TokenStream {
        quote! {
            use std::{collections::HashMap, sync::Mutex};
            use clap_derive_darling::OnceBox;

            static STR_CACHE: OnceBox<Mutex<HashMap<String, &'static str>>> = OnceBox::new();

            fn string_to_static_str(s: String) -> &'static str {
                Box::leak(s.into_boxed_str())
            }

            fn get_cache_str<F>(key: String, or_else: F) -> &'static str
            where
                F: Fn() -> String
            {
                let mut str_cache = STR_CACHE.get_or_init(|| Box::from(Mutex::from(HashMap::new()))).lock().unwrap();
                str_cache
                    .entry(key)
                    .or_insert_with(|| string_to_static_str(or_else()))
            }

            fn get_cache_str_keyed<F>(ty: &str, string: &str, prefix: &Option<String>, or_else: F) -> &'static str
            where
                F: Fn() -> String
            {
                get_cache_str(clap_derive_darling::rename::cache_key(ty, string, prefix), or_else)
            }
        }
    }

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
        self.get_help_heading().map(|string| {
            quote! {
                let app = app.help_heading(#string);
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
