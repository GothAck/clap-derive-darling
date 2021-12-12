use clap::App;
use clap_derive_darling::{ArgEnum, Args, Clap, Parser, Subcommand};

#[derive(Parser)]
/// Application description
///
/// Application long description
struct Application {
    /// Name short help
    #[clap(long, short)]
    name: String,
    /// Option short help
    #[clap(long)]
    option: Option<String>,

    #[clap(flatten = "flatten")]
    flatten: Flatten,

    #[clap(long)]
    bool: bool,

    /// OptOptT short help
    ///
    /// OptOptT long help...
    #[clap(long)]
    opt_opt_t: Option<Option<u64>>,

    #[clap(long)]
    vec_str: Vec<String>,

    #[clap(long)]
    opt_vec_str: Option<Vec<String>>,

    #[clap(long)]
    core_opt_str: std::option::Option<String>,
}

#[derive(Args)]
struct Flatten {
    #[clap(skip)]
    other: u64,

    #[clap(long)]
    flattened: Option<u32>,
}

#[test]
fn test_parse() {
    let args = vec!["app_name", "--name", "My app name"];

    let flags = Application::try_parse_from(args).unwrap();

    assert_eq!(flags.name, "My app name");
    assert_eq!(flags.flatten.flattened, None);
}

#[test]
fn test_flatten_prefix() {
    let args = vec![
        "app_name",
        "--name",
        "My app name",
        "--flatten-flattened",
        "128",
    ];

    let flags = Application::try_parse_from(args).unwrap();

    assert_eq!(flags.name, "My app name");
    assert_eq!(flags.flatten.flattened, Some(128));
}

#[test]
fn test_reuse_same_struct() {
    #[derive(Parser)]
    #[clap(about, long_about = "rar")]
    struct Application {
        #[clap(long, short)]
        name: String,

        #[clap(flatten = "db")]
        db: Settings,
        #[clap(flatten = "api")]
        api: Settings,
    }

    #[derive(Args)]
    struct Settings {
        #[clap(long)]
        uri: Option<String>,
        #[clap(long)]
        timeout_ms: Option<u64>,
    }

    let args = vec![
        "app_name",
        "--name",
        "My app name",
        "--db-uri",
        "MY_DB_URI",
        "--api-timeout-ms",
        "64",
    ];

    let flags = Application::try_parse_from(args).unwrap();

    assert_eq!(flags.name, "My app name");
    assert_eq!(flags.db.uri, Some("MY_DB_URI".to_string()));
    assert_eq!(flags.api.timeout_ms, Some(64));
}

#[test]
fn test_subcommand() {
    #[derive(Parser)]
    #[clap(help_heading = "123")]
    struct Application {
        /// Name short help
        #[clap(long, short)]
        name: String,

        #[clap(subcommand)]
        command: Command,
    }

    #[derive(Subcommand)]
    enum Command {
        /// First about
        ///
        /// First long about
        First(FirstCommand),
        #[clap(name = "2nd", version = "9.9.9", help_heading = "Rar")]
        Second {
            #[clap(long)]
            /// Embedded option
            ///
            /// Longer...
            embedded: Option<String>,
        },
        #[clap(skip)]
        SkipMe,
        #[clap(external_subcommand)]
        External(Vec<String>),
    }

    #[derive(Args)]
    struct FirstCommand {
        #[clap(long)]
        arg: Option<String>,
    }

    let args = vec!["app_name", "--name", "rar", "first", "--arg", "thing"];

    let flags = Application::try_parse_from(args).unwrap();

    assert_eq!(flags.name, "rar");

    assert!(matches!(flags.command, Command::First(..)));

    if let Command::First(command) = flags.command {
        assert_eq!(command.arg, Some("thing".to_string()));
    }

    let args = vec!["app_name", "--name", "lala", "2nd", "--embedded", "yes"];

    let flags = Application::try_parse_from(args).unwrap();

    assert_eq!(flags.name, "lala");

    assert!(matches!(flags.command, Command::Second { .. }));

    if let Command::Second { embedded } = flags.command {
        assert_eq!(embedded, Some("yes".to_string()));
    }

    let args = vec!["app_name", "--name", "skip-me"];

    assert!(Application::try_parse_from(args).is_err());

    // External commands are broken

    // let args = vec!["app_name", "--name", "external", "--external-arg"];

    // let flags = Application::try_parse_from(args).unwrap();

    // assert!(matches!(flags.command, Command::External(..)));

    // if let Command::External(external) = flags.command {
    //     assert_eq!(external, vec!["--external-arg"]);
    // }
}

#[test]
fn test_arg_enum() {
    #[derive(Parser)]
    struct Application {
        #[clap(long, short, arg_enum)]
        first: First,

        #[clap(long, short, arg_enum)]
        second: Option<Second>,

        #[clap(long, short, arg_enum)]
        third: Option<Option<Second>>,
    }

    #[derive(ArgEnum, Clone)]
    enum First {
        #[clap(help = "First variant 0 help")]
        Variant0,
        Variant1,
    }

    #[derive(ArgEnum, Clone)]
    #[clap(rename_all = "SCREAMING_SNAKE_CASE")]
    enum Second {
        Variant0,
        #[clap(help = "Second variant 1 help")]
        Variant1,
    }

    let args = vec!["app_name", "--first", "variant-0", "--second", "VARIANT_1"];

    // FIXME: --third should be able to be added without a value...

    let flags = Application::try_parse_from(args).unwrap();

    assert!(matches!(flags.first, First::Variant0));
    assert!(matches!(flags.second, Some(Second::Variant1)));
    assert!(matches!(flags.third, None));
}
