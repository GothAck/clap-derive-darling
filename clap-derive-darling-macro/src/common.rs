use darling::util::Override;
use quote::quote;
use syn::{AttrStyle, Attribute, LitStr};

pub trait ClapParserArgsCommon {
    fn to_tokens_name_storage(&self) -> proc_macro2::TokenStream {
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

    fn format_author_and_version(
        &self,
        author: Option<&Override<String>>,
        version: Option<&Override<String>>,
    ) -> proc_macro2::TokenStream {
        let author = author
            .map(|or| match or {
                Override::Explicit(author) => author,
                Override::Inherit => env!("CARGO_PKG_AUTHORS"),
            })
            .map(|s| {
                quote! { .author(#s) }
            });

        let version = version
            .map(|or| match or {
                Override::Explicit(version) => version,
                Override::Inherit => env!("CARGO_PKG_VERSION"),
            })
            .map(|s| {
                quote! { .version(#s) }
            });

        quote! { #author #version }
    }

    fn format_help_heading(
        &self,
        help_heading: Option<&String>,
    ) -> Option<proc_macro2::TokenStream> {
        help_heading.map(|string| {
            quote! {
                let app = app.help_heading(#string);
            }
        })
    }
}

pub trait ClapHelpCommon {
    fn attrs_to_docstring_iter<'a>(
        &self,
        attrs: &'a [Attribute],
    ) -> Box<dyn Iterator<Item = String> + 'a> {
        Box::new(
            attrs
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
                        .collect::<proc_macro2::TokenStream>();
                    syn::parse2(ts).unwrap()
                })
                .map(|v| {
                    let v = v.value();
                    v.trim().to_string()
                }),
        )
    }

    fn docstring_iter_to_opt_str(
        &self,
        iter: Box<dyn Iterator<Item = String> + '_>,
    ) -> (Option<String>, Option<String>) {
        let mut doc_help = Vec::new();
        let mut doc_long_help = Vec::new();
        let mut long = false;
        for docstr in iter {
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

    fn format_help(
        &self,
        iter: Box<dyn Iterator<Item = String> + '_>,
        help: Option<String>,
        long_help: Option<String>,
    ) -> proc_macro2::TokenStream {
        let (doc_help, doc_long_help) = self.docstring_iter_to_opt_str(iter);

        let help = help.or(doc_help).map(|string| {
            quote! {
                .help(#string)
            }
        });
        let long_help = long_help.or(doc_long_help).map(|string| {
            quote! {
                .long_help(#string)
            }
        });

        quote! {
            #help
            #long_help
        }
    }

    fn format_about(
        &self,
        iter: Box<dyn Iterator<Item = String> + '_>,
        about: Option<Override<String>>,
        long_about: Option<String>,
    ) -> proc_macro2::TokenStream {
        let (doc_about, doc_long_about) = self.docstring_iter_to_opt_str(iter);

        let about = about
            .map(|v| match v {
                Override::Explicit(v) => {
                    quote!(#v)
                }
                Override::Inherit => quote! { env!("CARGO_PKG_DESCRIPTION") },
            })
            .or_else(|| doc_about.map(|v| quote!(#v)))
            .map(|tokens| {
                quote! {
                    .about(#tokens)
                }
            });
        let long_about = long_about.or(doc_long_about).map(|string| {
            quote! {
                .long_about(#string)
            }
        });

        quote! {
            #about
            #long_about
        }
    }
}
