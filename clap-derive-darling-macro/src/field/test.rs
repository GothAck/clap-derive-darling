use std::collections::HashMap;

use darling::{
    ast,
    util::{Ignored, Override},
    FromDeriveInput,
};
use quote::quote;

use super::{ClapField, ClapFieldParse};
use crate::common::ClapTokensResult;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(clap), forward_attrs(doc), supports(struct_named))]
pub(crate) struct StructParser {
    data: ast::Data<Ignored, ClapField>,
    attrs: Vec<syn::Attribute>,
}

const INPUT_INHERIT: &str = r#"
struct Test {
    #[clap()]
    test0: String,

    #[clap(parse(from_str))]
    test1: String,

    #[clap(parse(try_from_str))]
    test2: String,

    #[clap(parse(from_os_str))]
    test3: String,

    #[clap(parse(try_from_os_str))]
    test4: String,

    #[clap(parse(from_occurrences))]
    test5: String,

    #[clap(parse(from_flag))]
    test6: String,
}
"#;

const INPUT_EXPLICIT: &str = r#"
struct Test {
    #[clap()]
    test0: String,

    #[clap(parse(from_str = "my::from_str"))]
    test1: String,

    #[clap(parse(try_from_str = "my::try_from_str"))]
    test2: String,

    #[clap(parse(from_os_str = "my::from_os_str"))]
    test3: String,

    #[clap(parse(try_from_os_str = "my::try_from_os_str"))]
    test4: String,

    #[clap(parse(from_occurrences = "my::from_occurrences"))]
    test5: String,

    #[clap(parse(from_flag = "my::from_flag"))]
    test6: String,
}
"#;

fn get_fields(struct_parser: &StructParser) -> Vec<&ClapField> {
    struct_parser
        .data
        .as_ref()
        .take_struct()
        .expect("Should always be a struct")
        .fields
}

fn get_by_name<'a>(fields: &'a Vec<&ClapField>) -> HashMap<String, Option<&'a ClapFieldParse>> {
    fields
        .iter()
        .map(|field| {
            (
                field.ident.clone().unwrap().to_string(),
                field.parse.as_ref(),
            )
        })
        .collect()
}

#[test]
fn test_inherit_and_defaults() {
    let parsed = syn::parse_str(INPUT_INHERIT).unwrap();
    let struct_parser = StructParser::from_derive_input(&parsed).unwrap();

    let fields = get_fields(&struct_parser);
    let by_name = get_by_name(&fields);

    assert!(matches!(by_name["test0"], None));
    assert!(matches!(
        by_name["test1"],
        Some(ClapFieldParse::FromStr(Override::Inherit))
    ));
    assert!(matches!(
        by_name["test2"],
        Some(ClapFieldParse::TryFromStr(Override::Inherit))
    ));
    assert!(matches!(
        by_name["test3"],
        Some(ClapFieldParse::FromOsStr(Override::Inherit))
    ));
    assert!(matches!(
        by_name["test4"],
        Some(ClapFieldParse::TryFromOsStr(Override::Inherit))
    ));
    assert!(matches!(
        by_name["test5"],
        Some(ClapFieldParse::FromOccurrences(Override::Inherit))
    ));
    assert!(matches!(
        by_name["test6"],
        Some(ClapFieldParse::FromFlag(Override::Inherit))
    ));

    assert!(matches!(
        by_name["test1"].unwrap().defaulted().unwrap(),
        ClapFieldParse::FromStr(Override::Explicit(..))
    ));
    assert!(matches!(
        by_name["test2"].unwrap().defaulted().unwrap(),
        ClapFieldParse::TryFromStr(Override::Explicit(..))
    ));
    assert!(matches!(
        by_name["test3"].unwrap().defaulted().unwrap(),
        ClapFieldParse::FromOsStr(Override::Explicit(..))
    ));
    assert!(matches!(by_name["test4"].unwrap().defaulted(), Err(..)));
    assert!(matches!(
        by_name["test5"].unwrap().defaulted().unwrap(),
        ClapFieldParse::FromOccurrences(Override::Explicit(..))
    ));
    assert!(matches!(
        by_name["test6"].unwrap().defaulted().unwrap(),
        ClapFieldParse::FromFlag(Override::Explicit(..))
    ));

    println!("{:?}", by_name);

    // assert!(false);
}

#[test]
fn test_explicit() {
    let parsed = syn::parse_str(INPUT_EXPLICIT).unwrap();
    let struct_parser = StructParser::from_derive_input(&parsed).unwrap();

    let fields = get_fields(&struct_parser);
    let by_name = get_by_name(&fields);

    assert!(matches!(by_name["test0"], None));
    assert!(matches!(
        by_name["test1"],
        Some(ClapFieldParse::FromStr(Override::Explicit(..)))
    ));
    assert!(matches!(
        by_name["test2"],
        Some(ClapFieldParse::TryFromStr(Override::Explicit(..)))
    ));
    assert!(matches!(
        by_name["test3"],
        Some(ClapFieldParse::FromOsStr(Override::Explicit(..)))
    ));
    assert!(matches!(
        by_name["test4"],
        Some(ClapFieldParse::TryFromOsStr(Override::Explicit(..)))
    ));
    assert!(matches!(
        by_name["test5"],
        Some(ClapFieldParse::FromOccurrences(Override::Explicit(..)))
    ));
    assert!(matches!(
        by_name["test6"],
        Some(ClapFieldParse::FromFlag(Override::Explicit(..)))
    ));

    assert_eq!(
        by_name["test1"]
            .as_ref()
            .unwrap()
            .to_tokens_result()
            .unwrap()
            .to_string(),
        quote!(my::from_str).to_string()
    );
}

#[test]
fn test_parse() {
    let parsed = syn::parse_str(INPUT_EXPLICIT).unwrap();
    let struct_parser = StructParser::from_derive_input(&parsed).unwrap();

    let fields = get_fields(&struct_parser);
    let by_name = get_by_name(&fields);

    assert_eq!(
        by_name["test1"]
            .unwrap()
            .to_tokens_result()
            .unwrap()
            .to_string(),
        quote!(my::from_str).to_string()
    );
    assert_eq!(
        ClapFieldParse::FromOccurrences(Override::Inherit)
            .defaulted()
            .unwrap()
            .to_tokens_result()
            .unwrap()
            .to_string(),
        quote!(value as T).to_string()
    );
}
