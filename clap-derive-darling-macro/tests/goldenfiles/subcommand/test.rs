impl clap_derive_darling::FromArgMatches for Command {
    fn from_arg_matches(
        ___arg_matches: &clap::ArgMatches,
        ___prefix: Vec<&'static str>,
    ) -> Result<Self, clap::Error> {
        if let Some((clap_name, sub_arg_matches)) = ___arg_matches.subcommand() {
            {
                let ___arg_matches = sub_arg_matches;
                if "first" == clap_name {
                    return Ok(Command::First(
                        <FirstCommand as clap_derive_darling::FromArgMatches>::from_arg_matches(
                            ___arg_matches,
                            ___prefix,
                        )?,
                    ));
                }
                if "2nd" == clap_name {
                    return Ok(Command::Second {
                        embedded: {
                            let ___name = "embedded";
                            ___arg_matches
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
                    });
                }
                if "external" == clap_name {
                    return Ok(Command::External(
                        ::std::iter::once(::std::string::String::from(clap_name))
                            .chain(
                                ___arg_matches
                                    .values_of("")
                                    .into_iter()
                                    .flatten()
                                    .map(::std::string::String::from),
                            )
                            .collect::<Vec<_>>(),
                    ));
                }
            }
            Err(clap::Error::raw(
                clap::ErrorKind::UnrecognizedSubcommand,
                format!("The subcommand '{}' watn't recognized", clap_name),
            ))
        } else {
            Err(clap::Error::raw(
                clap::ErrorKind::MissingSubcommand,
                "A subcommand is required but one was not provided",
            ))
        }
    }
    fn update_from_arg_matches(
        &mut self,
        ___arg_matches: &clap::ArgMatches,
        ___prefix: Vec<&'static str>,
    ) -> Result<(), clap::Error> {
        if let Some((clap_name, sub_arg_matches)) = ___arg_matches.subcommand() {
            match self {
                Command::First(ref mut clap_arg) if "first" == clap_name => {
                    let ___arg_matches = sub_arg_matches;
                    clap_derive_darling::FromArgMatches::update_from_arg_matches(
                        clap_arg,
                        sub_arg_matches,
                        ___prefix,
                    )?
                }
                Command::Second { ref mut embedded } if "2nd" == clap_name => {
                    let ___arg_matches = sub_arg_matches;
                    {
                        *embedded = {
                            let ___name = "embedded";
                            ___arg_matches
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
                }
                Command::External(ref mut clap_arg) if "external" == clap_name => {
                    *clap_arg = ::std::iter::once(::std::string::String::from(clap_name))
                        .chain(
                            ___arg_matches
                                .values_of("")
                                .into_iter()
                                .flatten()
                                .map(::std::string::String::from),
                        )
                        .collect::<Vec<_>>();
                }
                s => {
                    *s = <Self as clap_derive_darling::FromArgMatches>::from_arg_matches(
                        ___arg_matches,
                        ___prefix,
                    )?;
                }
            }
        }
        Ok(())
    }
}
impl clap_derive_darling::Subcommand for Command {
    fn augment_subcommands<'b>(
        ___app: clap::App<'b>,
        ___prefix: Vec<&'static str>,
    ) -> clap::App<'b> {
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("first");
            let clap_subcommand = {
                <FirstCommand as clap_derive_darling::Args>::augment_args(
                    clap_subcommand,
                    Vec::new(),
                )
            };
            clap_subcommand
        });
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("2nd");
            {
                let ___app = clap_subcommand;
                let ___app = ___app.arg({
                    let ___name = "embedded";
                    let ___value = "EMBEDDED";
                    let ___long = "embedded";
                    clap::Arg::new(___name)
                        .long(___long)
                        .takes_value(true)
                        .value_name(___value)
                        .required(false)
                        .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
                });
                ___app.version("9.9.9")
            }
        });
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("external");
            clap_subcommand
        });
        let ___app = ___app.setting(clap::AppSettings::AllowExternalSubcommands);
        ___app
    }
    fn augment_subcommands_for_update<'b>(
        ___app: clap::App<'b>,
        ___prefix: Vec<&'static str>,
    ) -> clap::App<'b> {
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("first");
            let clap_subcommand = {
                <FirstCommand as clap_derive_darling::Args>::augment_args(
                    clap_subcommand,
                    Vec::new(),
                )
            };
            clap_subcommand
        });
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("2nd");
            {
                let ___app = clap_subcommand;
                let ___app = ___app.arg({
                    let ___name = "embedded";
                    let ___value = "EMBEDDED";
                    let ___long = "embedded";
                    clap::Arg::new(___name)
                        .long(___long)
                        .takes_value(true)
                        .value_name(___value)
                        .required(false)
                        .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
                });
                ___app.version("9.9.9")
            }
        });
        let ___app = ___app.subcommand({
            let clap_subcommand = clap::App::new("external");
            clap_subcommand
        });
        let ___app = ___app.setting(clap::AppSettings::AllowExternalSubcommands);
        ___app
    }
    fn has_subcommand(clap_name: &str) -> bool {
        {
            let name = "first";
            if name == clap_name {
                return true;
            }
        }
        {
            let name = "2nd";
            if name == clap_name {
                return true;
            }
        }
        {
            let name = "external";
            if name == clap_name {
                return true;
            }
        }
        false
    }
}
