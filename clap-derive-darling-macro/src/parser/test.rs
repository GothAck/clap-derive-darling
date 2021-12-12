use std::io::Write;

use darling::FromDeriveInput;
use goldenfile::Mint;

use super::ClapParser;
use crate::{common::ClapTokensResultAuto, test_util::rustfmt_ext};

#[test]
fn test() {
    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("test.rs").unwrap();

    let input = r#"
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

    #[clap(long, arg_enum)]
    opt_arg_enum: Option<MyArgEnum>,
    #[clap(long, arg_enum)]
    opt_opt_arg_enum: Option<Option<MyArgEnum>>,

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
"#;
    let parsed = syn::parse_str(input).unwrap();
    let conf_struct = ClapParser::from_derive_input(&parsed).unwrap();

    file.write_all(rustfmt_ext(conf_struct.to_tokens()).unwrap().as_bytes())
        .unwrap();
}
