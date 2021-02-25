mod json;
mod toml;
mod yaml;

pub fn solve(
    file_type: &str,
    input: &str,
    expression: Option<&str>,
    replace: Option<&str>,
) -> String {
    match file_type.to_ascii_lowercase().as_ref() {
        "json" => json::JsonSolver::solve(input, expression, replace),
        "yaml" | "yml" => yaml::YamlSolver::solve(input, expression, replace),
        "toml" => toml::TomlSolver::solve(input, expression, replace),
        _ => panic!("Unknown file_type:{}", file_type),
    }
}

#[derive(Debug)]
pub enum TError {
    KeyNotExist(String),
    ConversionError(String, Box<dyn std::error::Error>),
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for TError {}

impl std::fmt::Display for TError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

macro_rules! impl_error {
    ($ty:ty) => {
        impl std::convert::From<$ty> for TError {
            fn from(err: $ty) -> Self {
                TError::Other(Box::new(err))
            }
        }
    };
}

impl_error!(std::num::ParseIntError);
impl_error!(serde_json::Error);
impl_error!(serde_yaml::Error);
impl_error!(::toml::de::Error);

trait Solver {
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String;
}
