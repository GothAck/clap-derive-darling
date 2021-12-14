impl clap_derive_darling::Args for Application {
    fn augment_args(app: clap::App<'_>, prefix: Option<String>) -> clap::App<'_> {
        let app = app.arg({
            let ___name = "name";
            let ___value = "NAME";
            let ___long = "name";
            clap::Arg::new(___name)
                .short('n')
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .help("Name")
                .long_help("Longer name")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name = "option";
            let ___value = "OPTION";
            let ___long = "option";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
            let ___name = "opt-arg-enum";
            let ___value = "OPT_ARG_ENUM";
            let ___long = "opt-arg-enum";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name = "opt-opt-arg-enum";
            let ___value = "OPT_OPT_ARG_ENUM";
            let ___long = "opt-opt-arg-enum";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
            let ___name = "bool";
            let ___long = "bool";
            clap::Arg::new(___name).long(___long).takes_value(false)
        });
        let app = app.arg({
            let ___name = "opt-opt-t";
            let ___value = "OPT_OPT_T";
            let ___long = "opt-opt-t";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: u64| ()))
        });
        let app = app.arg({
            let ___name = "vec-str";
            let ___value = "VEC_STR";
            let ___long = "vec-str";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name = "opt-vec-str";
            let ___value = "OPT_VEC_STR";
            let ___long = "opt-vec-str";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
        let app = app.arg({
            let ___name = "name";
            let ___value = "NAME";
            let ___long = "name";
            clap::Arg::new(___name)
                .short('n')
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .help("Name")
                .long_help("Longer name")
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name = "option";
            let ___value = "OPTION";
            let ___long = "option";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
            let ___name = "opt-arg-enum";
            let ___value = "OPT_ARG_ENUM";
            let ___long = "opt-arg-enum";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .possible_values(
                    <MyArgEnum as clap_derive_darling::ArgEnum>::value_variants()
                        .iter()
                        .filter_map(clap_derive_darling::ArgEnum::to_possible_value),
                )
        });
        let app = app.arg({
            let ___name = "opt-opt-arg-enum";
            let ___value = "OPT_OPT_ARG_ENUM";
            let ___long = "opt-opt-arg-enum";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
            let ___name = "bool";
            let ___long = "bool";
            clap::Arg::new(___name).long(___long).takes_value(false)
        });
        let app = app.arg({
            let ___name = "opt-opt-t";
            let ___value = "OPT_OPT_T";
            let ___long = "opt-opt-t";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .min_values(0)
                .max_values(1)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: u64| ()))
        });
        let app = app.arg({
            let ___name = "vec-str";
            let ___value = "VEC_STR";
            let ___long = "vec-str";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
                .required(false)
                .multiple_occurrences(true)
                .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
        });
        let app = app.arg({
            let ___name = "opt-vec-str";
            let ___value = "OPT_VEC_STR";
            let ___long = "opt-vec-str";
            clap::Arg::new(___name)
                .long(___long)
                .takes_value(true)
                .value_name(___value)
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
                let ___name = "name";
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
                let ___name = "option";
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
                let ___name = "opt-arg-enum";
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
                let ___name = "opt-opt-arg-enum";
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
                let ___name = "bool";
                arg_matches.is_present(___name)
            },
            opt_opt_t: {
                let ___name = "opt-opt-t";
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
                let ___name = "vec-str";
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
                let ___name = "opt-vec-str";
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
                let ___name = "name";
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
                let ___name = "option";
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
                let ___name = "opt-arg-enum";
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
                let ___name = "opt-opt-arg-enum";
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
                let ___name = "bool";
                arg_matches.is_present(___name)
            };
        }
        {
            #[allow(non_snake_case)]
            let opt_opt_t = &mut self.opt_opt_t;
            *opt_opt_t = {
                let ___name = "opt-opt-t";
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
                let ___name = "vec-str";
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
                let ___name = "opt-vec-str";
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
            )?;
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
