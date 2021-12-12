impl clap_derive_darling::Args for Application {
    fn augment_args(app: clap::App<'_>, prefix: Option<String>) -> clap::App<'_> {
        use clap_derive_darling::OnceBox;
        use std::{collections::HashMap, sync::Mutex};
        static STR_CACHE: OnceBox<Mutex<HashMap<String, &'static str>>> = OnceBox::new();
        fn string_to_static_str(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }
        fn get_cache_str<F>(key: String, or_else: F) -> &'static str
        where
            F: Fn() -> String,
        {
            let mut str_cache = STR_CACHE
                .get_or_init(|| Box::from(Mutex::from(HashMap::new())))
                .lock()
                .unwrap();
            str_cache
                .entry(key)
                .or_insert_with(|| string_to_static_str(or_else()))
        }
        fn get_cache_str_keyed<F>(
            ty: &str,
            string: &str,
            prefix: &Option<String>,
            or_else: F,
        ) -> &'static str
        where
            F: Fn() -> String,
        {
            get_cache_str(
                clap_derive_darling::rename::cache_key(ty, string, prefix),
                or_else,
            )
        }
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "name", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("name", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "name", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "name", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "name", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "name", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .short('n')
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(true)
                .help("Name")
                .long_help("Longer name")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "option", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("option", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "option", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "option", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "option", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "option", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .help("Option")
                .long_help("Longer help for Option")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let old_heading = app.get_help_heading();
        let subprefix = {
            let mut vec = Vec::new();
            if let Some(prefix) = prefix.as_ref() {
                vec.push(prefix.to_string());
            }
            if vec.is_empty() {
                None
            } else {
                Some(vec.join("-"))
            }
        };
        let app = <Flatten as clap_derive_darling::Args>::augment_args(app, subprefix.clone());
        let app = app.help_heading(old_heading);
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_arg_enum", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_arg_enum",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_arg_enum",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name_value =
                get_cache_str_keyed("name_value", "opt_opt_arg_enum", &prefix, || {
                    clap_derive_darling::rename::screaming_snake_case(
                        clap_derive_darling::rename::prefix("opt_opt_arg_enum", &prefix),
                    )
                });
            let ___name_long =
                get_cache_str_keyed("name_long", "opt_opt_arg_enum", &prefix, || {
                    clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                        "opt_opt_arg_enum",
                        &prefix,
                    ))
                });
            let ___name = get_cache_str_keyed("name", "opt_opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_arg_enum",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name_long = get_cache_str_keyed("name_long", "bool", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "bool", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "bool", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "bool", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(false)
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_opt_t", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_t",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_t",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: u64| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "vec_str", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("vec_str", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "vec_str", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "vec_str", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_vec_str", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_vec_str",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_vec_str",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app =
            <Command as clap_derive_darling::Subcommand>::augment_subcommands(app, prefix.clone());
        let app = app.setting(clap::AppSettings::SubcommandRequiredElseHelp);
        app
    }
    fn augment_args_for_update(app: clap::App<'_>, prefix: Option<String>) -> clap::App<'_> {
        use clap_derive_darling::OnceBox;
        use std::{collections::HashMap, sync::Mutex};
        static STR_CACHE: OnceBox<Mutex<HashMap<String, &'static str>>> = OnceBox::new();
        fn string_to_static_str(s: String) -> &'static str {
            Box::leak(s.into_boxed_str())
        }
        fn get_cache_str<F>(key: String, or_else: F) -> &'static str
        where
            F: Fn() -> String,
        {
            let mut str_cache = STR_CACHE
                .get_or_init(|| Box::from(Mutex::from(HashMap::new())))
                .lock()
                .unwrap();
            str_cache
                .entry(key)
                .or_insert_with(|| string_to_static_str(or_else()))
        }
        fn get_cache_str_keyed<F>(
            ty: &str,
            string: &str,
            prefix: &Option<String>,
            or_else: F,
        ) -> &'static str
        where
            F: Fn() -> String,
        {
            get_cache_str(
                clap_derive_darling::rename::cache_key(ty, string, prefix),
                or_else,
            )
        }
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "name", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("name", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "name", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "name", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "name", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "name", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .short('n')
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(true)
                .help("Name")
                .long_help("Longer name")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "option", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("option", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "option", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "option", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "option", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "option", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .help("Option")
                .long_help("Longer help for Option")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let old_heading = app.get_help_heading();
        let subprefix = {
            let mut vec = Vec::new();
            if let Some(prefix) = prefix.as_ref() {
                vec.push(prefix.to_string());
            }
            if vec.is_empty() {
                None
            } else {
                Some(vec.join("-"))
            }
        };
        let app = <Flatten as clap_derive_darling::Args>::augment_args(app, subprefix.clone());
        let app = app.help_heading(old_heading);
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_arg_enum", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_arg_enum",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_arg_enum",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name_value =
                get_cache_str_keyed("name_value", "opt_opt_arg_enum", &prefix, || {
                    clap_derive_darling::rename::screaming_snake_case(
                        clap_derive_darling::rename::prefix("opt_opt_arg_enum", &prefix),
                    )
                });
            let ___name_long =
                get_cache_str_keyed("name_long", "opt_opt_arg_enum", &prefix, || {
                    clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                        "opt_opt_arg_enum",
                        &prefix,
                    ))
                });
            let ___name = get_cache_str_keyed("name", "opt_opt_arg_enum", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_arg_enum",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name_long = get_cache_str_keyed("name_long", "bool", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "bool", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "bool", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "bool", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(false)
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_opt_t", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_t",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_opt_t", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_opt_t",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: u64| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "vec_str", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("vec_str", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "vec_str", &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "vec_str", &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name_value = get_cache_str_keyed("name_value", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::screaming_snake_case(
                    clap_derive_darling::rename::prefix("opt_vec_str", &prefix),
                )
            });
            let ___name_long = get_cache_str_keyed("name_long", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_vec_str",
                    &prefix,
                ))
            });
            let ___name = get_cache_str_keyed("name", "opt_vec_str", &prefix, || {
                clap_derive_darling::rename::kebab_case(clap_derive_darling::rename::prefix(
                    "opt_vec_str",
                    &prefix,
                ))
            });
            clap::Arg::new(___name)
                .long(___name_long)
                .takes_value(true)
                .value_name(___name_value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app =
            <Command as clap_derive_darling::Subcommand>::augment_subcommands(app, prefix.clone());
        let app = app.setting(clap::AppSettings::SubcommandRequiredElseHelp);
        app
    }
}
impl clap_derive_darling::FromArgMatches for Application {
    fn from_arg_matches(
        arg_matches: &clap::ArgMatches,
        prefix: Option<String>,
    ) -> Result<Self, clap::Error> {
        let v = Application {
            name: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("name", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        ::std::str::FromStr::from_str(s).map_err(|err| {
                            clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                format!("Invalid value for {}: {}", &___name, &err),
                            )
                        })
                    })
                    .transpose()?
                    .ok_or_else(|| {
                        clap::Error::raw(
                            clap::ErrorKind::ValueValidation,
                            format!("Invalid value for {}", &___name),
                        )
                    })?
            },
            option: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("option", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        ::std::str::FromStr::from_str(s).map_err(|err| {
                            clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                format!("Invalid value for {}: {}", &___name, &err),
                            )
                        })
                    })
                    .transpose()?
            },
            flatten: {
                let subprefix = {
                    let mut vec = Vec::new();
                    if let Some(prefix) = prefix.as_ref() {
                        vec.push(prefix.to_string());
                    }
                    if vec.is_empty() {
                        None
                    } else {
                        Some(vec.join("-"))
                    }
                };
                clap_derive_darling::FromArgMatches::from_arg_matches(arg_matches, subprefix)
                    .unwrap()
            },
            opt_arg_enum: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_arg_enum", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        <MyArgEnum as clap_derive_darling::ArgEnum>::from_str(s, false).map_err(
                            |err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            },
                        )
                    })
                    .transpose()?
            },
            opt_opt_arg_enum: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_opt_arg_enum", &prefix),
                );
                if arg_matches.is_present(&___name) {
                    Some(
                        arg_matches
                            .value_of(&___name)
                            .map(|s| {
                                <MyArgEnum as clap_derive_darling::ArgEnum>::from_str(s, false)
                                    .map_err(|err| {
                                        clap::Error::raw(
                                            clap::ErrorKind::ValueValidation,
                                            format!("Invalid value for {}: {}", &___name, &err),
                                        )
                                    })
                            })
                            .transpose()?,
                    )
                } else {
                    None
                }
            },
            bool: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("bool", &prefix),
                );
                arg_matches.is_present(___name)
            },
            opt_opt_t: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_opt_t", &prefix),
                );
                if arg_matches.is_present(&___name) {
                    Some(
                        arg_matches
                            .value_of(&___name)
                            .map(|s| {
                                ::std::str::FromStr::from_str(s).map_err(|err| {
                                    clap::Error::raw(
                                        clap::ErrorKind::ValueValidation,
                                        format!("Invalid value for {}: {}", &___name, &err),
                                    )
                                })
                            })
                            .transpose()?,
                    )
                } else {
                    None
                }
            },
            vec_str: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("vec_str", &prefix),
                );
                arg_matches
                    .values_of(&___name)
                    .map(|v| {
                        v.map(|s| {
                            ::std::str::FromStr::from_str(s).map_err(|err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            })
                        })
                    })
                    .map(|v| v.collect::<Result<Vec<_>, _>>())
                    .unwrap_or_else(|| Ok(Vec::new()))?
            },
            opt_vec_str: {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_vec_str", &prefix),
                );
                arg_matches
                    .values_of(&___name)
                    .map(|v| {
                        v.map(|s| {
                            ::std::str::FromStr::from_str(s).map_err(|err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            })
                        })
                    })
                    .map(|v| v.collect::<Result<Vec<_>, _>>())
                    .transpose()?
            },
            command: <Command as clap_derive_darling::FromArgMatches>::from_arg_matches(
                arg_matches,
                prefix.clone(),
            )?,
        };
        Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        arg_matches: &clap::ArgMatches,
        prefix: Option<String>,
    ) -> Result<(), clap::Error> {
        {
            #[allow(non_snake_case)]
            let name = &mut self.name;
            *name = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("name", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        ::std::str::FromStr::from_str(s).map_err(|err| {
                            clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                format!("Invalid value for {}: {}", &___name, &err),
                            )
                        })
                    })
                    .transpose()?
                    .ok_or_else(|| {
                        clap::Error::raw(
                            clap::ErrorKind::ValueValidation,
                            format!("Invalid value for {}", &___name),
                        )
                    })?
            };
        }
        {
            #[allow(non_snake_case)]
            let option = &mut self.option;
            *option = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("option", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        ::std::str::FromStr::from_str(s).map_err(|err| {
                            clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                format!("Invalid value for {}: {}", &___name, &err),
                            )
                        })
                    })
                    .transpose()?
            };
        }
        {
            #[allow(non_snake_case)]
            let flatten = &mut self.flatten;
            {
                let subprefix = {
                    let mut vec = Vec::new();
                    if let Some(prefix) = prefix.as_ref() {
                        vec.push(prefix.to_string());
                    }
                    if vec.is_empty() {
                        None
                    } else {
                        Some(vec.join("-"))
                    }
                };
                clap_derive_darling::FromArgMatches::update_from_arg_matches(
                    flatten,
                    arg_matches,
                    subprefix,
                )
            };
        }
        {
            #[allow(non_snake_case)]
            let opt_arg_enum = &mut self.opt_arg_enum;
            *opt_arg_enum = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_arg_enum", &prefix),
                );
                arg_matches
                    .value_of(&___name)
                    .map(|s| {
                        <MyArgEnum as clap_derive_darling::ArgEnum>::from_str(s, false).map_err(
                            |err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            },
                        )
                    })
                    .transpose()?
            };
        }
        {
            #[allow(non_snake_case)]
            let opt_opt_arg_enum = &mut self.opt_opt_arg_enum;
            *opt_opt_arg_enum = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_opt_arg_enum", &prefix),
                );
                if arg_matches.is_present(&___name) {
                    Some(
                        arg_matches
                            .value_of(&___name)
                            .map(|s| {
                                <MyArgEnum as clap_derive_darling::ArgEnum>::from_str(s, false)
                                    .map_err(|err| {
                                        clap::Error::raw(
                                            clap::ErrorKind::ValueValidation,
                                            format!("Invalid value for {}: {}", &___name, &err),
                                        )
                                    })
                            })
                            .transpose()?,
                    )
                } else {
                    None
                }
            };
        }
        {
            #[allow(non_snake_case)]
            let bool = &mut self.bool;
            *bool = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("bool", &prefix),
                );
                arg_matches.is_present(___name)
            };
        }
        {
            #[allow(non_snake_case)]
            let opt_opt_t = &mut self.opt_opt_t;
            *opt_opt_t = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_opt_t", &prefix),
                );
                if arg_matches.is_present(&___name) {
                    Some(
                        arg_matches
                            .value_of(&___name)
                            .map(|s| {
                                ::std::str::FromStr::from_str(s).map_err(|err| {
                                    clap::Error::raw(
                                        clap::ErrorKind::ValueValidation,
                                        format!("Invalid value for {}: {}", &___name, &err),
                                    )
                                })
                            })
                            .transpose()?,
                    )
                } else {
                    None
                }
            };
        }
        {
            #[allow(non_snake_case)]
            let vec_str = &mut self.vec_str;
            *vec_str = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("vec_str", &prefix),
                );
                arg_matches
                    .values_of(&___name)
                    .map(|v| {
                        v.map(|s| {
                            ::std::str::FromStr::from_str(s).map_err(|err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            })
                        })
                    })
                    .map(|v| v.collect::<Result<Vec<_>, _>>())
                    .unwrap_or_else(|| Ok(Vec::new()))?
            };
        }
        {
            #[allow(non_snake_case)]
            let opt_vec_str = &mut self.opt_vec_str;
            *opt_vec_str = {
                let ___name = clap_derive_darling::rename::kebab_case(
                    clap_derive_darling::rename::prefix("opt_vec_str", &prefix),
                );
                arg_matches
                    .values_of(&___name)
                    .map(|v| {
                        v.map(|s| {
                            ::std::str::FromStr::from_str(s).map_err(|err| {
                                clap::Error::raw(
                                    clap::ErrorKind::ValueValidation,
                                    format!("Invalid value for {}: {}", &___name, &err),
                                )
                            })
                        })
                    })
                    .map(|v| v.collect::<Result<Vec<_>, _>>())
                    .transpose()?
            };
        }
        {
            #[allow(non_snake_case)]
            let command = &mut self.command;
            <Command as clap_derive_darling::FromArgMatches>::update_from_arg_matches(
                command,
                arg_matches,
                prefix,
            );
        }
        Ok(())
    }
}
impl clap::IntoApp for Application {
    fn into_app<'help>() -> clap::App<'help> {
        let app = clap::App::new("application");
        <Self as clap_derive_darling::Args>::augment_args(app, None)
    }
    fn into_app_for_update<'help>() -> clap::App<'help> {
        let app = clap::App::new("application");
        <Self as clap_derive_darling::Args>::augment_args_for_update(app, None)
    }
}
impl clap_derive_darling::Clap for Application {}
