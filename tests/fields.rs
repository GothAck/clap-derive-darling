use clap_derive_darling::{Args, Clap, Parser};

#[derive(Parser)]
struct Application {
    #[clap(long, short)]
    name: String,
    #[clap(long)]
    option: Option<String>,

    #[clap(flatten = "flatten")]
    flatten: Flatten,

    #[clap(long)]
    bool: bool,

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
