fn rar() {
    let app = app.arg({
        let ___name = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "prefix-0-name"
                } else if prefix == "prefix1" {
                    "prefix-1-name"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "name"
            }
        };
        let ___value = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "PREFIX_0_NAME"
                } else if prefix == "prefix1" {
                    "PREFIX_1_NAME"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "NAME"
            }
        };
        let ___long = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "prefix-0-name"
                } else if prefix == "prefix1" {
                    "prefix-1-name"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "name"
            }
        };
        let ___env = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "PREFIX_0_NAME"
                } else if prefix == "prefix1" {
                    "PREFIX_1_NAME"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "NAME"
            }
        };
        clap::Arg::new(___name)
            .long(___long)
            .env(___env)
            .takes_value(true)
            .value_name(___value)
            .required(false)
            .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
    });
    let app = app.arg({
        let ___name = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "prefix-0-lala"
                } else if prefix == "prefix1" {
                    "prefix-1-lala"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "lala"
            }
        };
        let ___value = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "PREFIX_0_LALA"
                } else if prefix == "prefix1" {
                    "PREFIX_1_LALA"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "LALA"
            }
        };
        let ___long = {
            if let Some(prefix) = &prefix {
                if prefix == "prefix0" {
                    "prefix-0-rar"
                } else if prefix == "prefix1" {
                    "prefix-1-rar"
                } else {
                    panic!("Prefix {} not defined for {}", prefix, "Test");
                }
            } else {
                "rar"
            }
        };
        clap::Arg::new(___name)
            .long(___long)
            .takes_value(true)
            .value_name(___value)
            .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
    });
    let old_heading = app.get_help_heading();
    let subprefix = {
        let mut vec = Vec::new();
        if let Some(prefix) = prefix.as_ref() {
            vec.push(prefix.to_string());
        }
        vec.push("demo".to_string());
        if vec.is_empty() {
            None
        } else {
            Some(vec.join("-"))
        }
    };
    let app = <Other as clap_derive_darling::Args>::augment_args(app, subprefix.clone());
    let app = app.help_heading(old_heading);
}
