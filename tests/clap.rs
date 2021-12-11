use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "my_app", author, version, help_heading = "Test heading")]
/// App help
///
/// Longer
struct Application {
    #[clap(long, short)]
    /// Name
    ///
    /// Longer name
    name: String,
    #[clap(long, help = "Option", long_help = "Longer help for Option")]
    option: Option<String>,

    #[clap(flatten)]
    flatten: Flatten,

    #[clap(long)]
    bool: bool,

    #[clap(long)]
    opt_opt_t: Option<Option<u64>>,

    #[clap(long)]
    vec_str: Vec<String>,

    #[clap(long)]
    opt_vec_str: Option<Vec<String>>,
    // This doesn't work, Clap doesn't think it's an Option.
    // #[clap(long)]
    // core_opt_str: std::option::Option<String>,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Args)]
#[clap(
    name = "TestArgs",
    version,
    author = "Test Author",
    help_heading = "Other heading"
)]
struct Flatten {
    #[clap(skip)]
    other: u64,

    flattened: Option<u32>,
}

#[derive(Subcommand)]
enum Command {
    First(FirstCommand),
    #[clap(name = "2nd", version = "9.9.9", help_heading = "Rar")]
    Second {
        #[clap(long)]
        embedded: Option<String>,
    },
}

#[derive(Args)]
struct FirstCommand {
    #[clap(long)]
    arg: Option<String>,
}
