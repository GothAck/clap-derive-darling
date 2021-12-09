use clap::{Args, Parser};

#[derive(Parser)]
#[clap(name = "my_app", author, version)]
struct Application {
    #[clap(long, short)]
    name: String,
    #[clap(long)]
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
}

#[derive(Args)]
#[clap(name = "TestArgs", version, author = "Test Author")]
struct Flatten {
    #[clap(skip)]
    other: u64,

    flattened: Option<u32>,
}
