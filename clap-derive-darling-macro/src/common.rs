use quote::quote;

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
}
