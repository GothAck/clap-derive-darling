use convert_case::{Case, Casing};

pub fn camel_case(string: String) -> String {
    string.to_case(Case::Camel)
}

pub fn kebab_case(string: String) -> String {
    string.to_case(Case::Kebab)
}

pub fn pascal_case(string: String) -> String {
    string.to_case(Case::Pascal)
}

pub fn screaming_snake_case(string: String) -> String {
    string.to_case(Case::ScreamingSnake)
}

pub fn snake_case(string: String) -> String {
    string.to_case(Case::Snake)
}

pub fn lower_case(string: String) -> String {
    string.to_case(Case::Lower)
}

pub fn upper_case(string: String) -> String {
    string.to_case(Case::Upper)
}

pub fn verbatim_case(string: String) -> String {
    string
}

pub fn prefix(string: &str, prefix: &Option<String>) -> String {
    if let Some(prefix) = prefix {
        format!("{}-{}", prefix, string)
    } else {
        string.to_string()
    }
}

pub fn cache_key(ty: &str, string: &str, prefix: &Option<String>) -> String {
    format!("{}|{:?}|{}", ty, prefix, string)
}
