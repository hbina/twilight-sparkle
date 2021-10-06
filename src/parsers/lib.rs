pub mod csv_parser;
pub mod json_parser;
pub mod toml_parser;
pub mod yaml_parser;

pub enum SupportedFiles {
    Json,
    Toml,
    Yaml,
    Csv,
}

impl SupportedFiles {
    pub fn maybe_from_str(input: &str) -> Option<SupportedFiles> {
        match input {
            "json" => Some(SupportedFiles::Json),
            "toml" => Some(SupportedFiles::Toml),
            "yaml" => Some(SupportedFiles::Yaml),
            "csv" => Some(SupportedFiles::Csv),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum TError {
    NoInput,
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
impl_error!(std::io::Error);
impl_error!(csv::Error);

trait Solver {
    fn solve(input: &str, expression: Option<&str>) -> String;
}
