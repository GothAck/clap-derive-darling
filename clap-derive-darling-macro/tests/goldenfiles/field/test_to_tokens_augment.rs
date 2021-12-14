fn rar() {
    let ___app = ___app.arg({
        let ___name = {
            if ___prefix.is_empty() {
                "name"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "prefix-0-prefix-1-name"
                } else if ___prefix == ["prefix2"] {
                    "prefix-2-name"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
            }
        };
        let ___value = {
            if ___prefix.is_empty() {
                "NAME"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "PREFIX_0_PREFIX_1_NAME"
                } else if ___prefix == ["prefix2"] {
                    "PREFIX_2_NAME"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
            }
        };
        let ___long = {
            if ___prefix.is_empty() {
                "name"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "prefix-0-prefix-1-name"
                } else if ___prefix == ["prefix2"] {
                    "prefix-2-name"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
            }
        };
        let ___env = {
            if ___prefix.is_empty() {
                "NAME"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "PREFIX_0_PREFIX_1_NAME"
                } else if ___prefix == ["prefix2"] {
                    "PREFIX_2_NAME"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
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
            if ___prefix.is_empty() {
                "lala"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "prefix-0-prefix-1-lala"
                } else if ___prefix == ["prefix2"] {
                    "prefix-2-lala"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
            }
        };
        let ___value = {
            if ___prefix.is_empty() {
                "LALA"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "PREFIX_0_PREFIX_1_LALA"
                } else if ___prefix == ["prefix2"] {
                    "PREFIX_2_LALA"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
            }
        };
        let ___long = {
            if ___prefix.is_empty() {
                "rar"
            } else {
                if ___prefix == ["prefix0", "prefix1"] {
                    "prefix-0-prefix-1-rar"
                } else if ___prefix == ["prefix2"] {
                    "prefix-2-rar"
                } else {
                    panic!("Prefix {:?} not defined for {}", ___prefix, "Test");
                }
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
        let mut vec = ___prefix.clone();
        vec.push("demo");
        vec
    };
    let ___app = <Other as clap_derive_darling::Args>::augment_args(___app, ___subprefix.clone());
    let ___app = ___app.help_heading(old_heading);
}
