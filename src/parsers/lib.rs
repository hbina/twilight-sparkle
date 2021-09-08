pub mod json_parser;
pub mod toml_parser;
pub mod yaml_parser;

pub enum SupportedFiles {
    JSON,
    TOML,
    YAML,
}

impl SupportedFiles {
    pub fn from_str(input: &str) -> Option<SupportedFiles> {
        match input {
            "json" => Some(SupportedFiles::JSON),
            "toml" => Some(SupportedFiles::TOML),
            "yaml" => Some(SupportedFiles::YAML),
            _ => None,
        }
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
impl_error!(toml::de::Error);

trait Solver {
    fn solve(input: &str, expression: Option<&str>) -> String;
}
