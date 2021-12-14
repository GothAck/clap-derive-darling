fn rar() {
    let ___app = ___app.arg({
        let ___name = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "prefix-0-name"
                } else if ___prefix == "prefix1" {
                    "prefix-1-name"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
                }
            } else {
                "name"
            }
        };
        let ___value = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "PREFIX_0_NAME"
                } else if ___prefix == "prefix1" {
                    "PREFIX_1_NAME"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
                }
            } else {
                "NAME"
            }
        };
        let ___long = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "prefix-0-name"
                } else if ___prefix == "prefix1" {
                    "prefix-1-name"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
                }
            } else {
                "name"
            }
        };
        let ___env = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "PREFIX_0_NAME"
                } else if ___prefix == "prefix1" {
                    "PREFIX_1_NAME"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
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
    let ___app = ___app.arg({
        let ___name = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "prefix-0-lala"
                } else if ___prefix == "prefix1" {
                    "prefix-1-lala"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
                }
            } else {
                "lala"
            }
        };
        let ___value = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "PREFIX_0_LALA"
                } else if ___prefix == "prefix1" {
                    "PREFIX_1_LALA"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
                }
            } else {
                "LALA"
            }
        };
        let ___long = {
            if let Some(___prefix) = &___prefix {
                if ___prefix == "prefix0" {
                    "prefix-0-rar"
                } else if ___prefix == "prefix1" {
                    "prefix-1-rar"
                } else {
                    panic!("Prefix {} not defined for {}", ___prefix, "Test");
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
    let old_heading = ___app.get_help_heading();
    let ___subprefix = {
        let mut vec = Vec::new();
        if let Some(___prefix) = ___prefix.as_ref() {
            vec.push(___prefix.to_string());
        }
        vec.push("demo".to_string());
        if vec.is_empty() {
            None
        } else {
            Some(vec.join("-"))
        }
    };
    let ___app = <Other as clap_derive_darling::Args>::augment_args(___app, ___subprefix.clone());
    let ___app = ___app.help_heading(old_heading);
}
