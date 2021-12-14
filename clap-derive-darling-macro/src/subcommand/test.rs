use std::io::Write;

use darling::FromDeriveInput;
use goldenfile::Mint;

use crate::{common::ClapTokensResult, test_util::rustfmt_ext};

use super::ClapSubcommand;

const INPUT_VALID: &str = r#"
enum Command {
    First(FirstCommand),
    #[clap(name = "2nd", version = "9.9.9", help_heading = "Rar")]
    Second {
        #[clap(long)]
        embedded: Option<String>,
    },
    #[clap(skip)]
    SkipMe,
    #[clap(external_subcommand)]
    External(Vec<String>),
}
"#;

#[test]
fn test() {
    let mut mint = Mint::new("tests/goldenfiles/subcommand");
    let mut file = mint.new_goldenfile("test.rs").unwrap();

    let parsed = syn::parse_str(INPUT_VALID).unwrap();
    let subcommand = ClapSubcommand::from_derive_input(&parsed).unwrap();

    file.write_all(
        rustfmt_ext(subcommand.to_tokens_result().unwrap())
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
}
