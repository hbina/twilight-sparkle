use crate::TError;

pub struct TomlSolver {
    expression: Vec<String>,
}

impl From<&clap::ArgMatches<'_>> for TomlSolver {
    fn from(input: &clap::ArgMatches<'_>) -> TomlSolver {
        let expression = input
            .value_of("expression")
            .map(|s| s.split(".").map(String::from).collect::<_>())
            .unwrap_or_default();
        TomlSolver { expression }
    }
}

impl TomlSolver {
    pub fn resolve_value<'a>(&self, value: &'a str) -> Result<String, TError> {
        let root = toml::from_str::<toml::Value>(value)?;
        let mut result = vec![&root];
        for expr in &self.expression {
            result = result
                .into_iter()
                .map(
                    |reader| -> Box<dyn Iterator<Item = Result<&toml::Value, TError>>> {
                        match reader {
                            toml::Value::Array(v) => {
                                let next = v.into_iter().map(|o| {
                                    o.get(expr.as_str())
                                        .ok_or_else(|| TError::KeyNotExist(expr.clone()))
                                });
                                Box::new(next)
                            }
                            o => {
                                let next = std::iter::once(
                                    o.get(expr.as_str())
                                        .ok_or_else(|| TError::KeyNotExist(expr.clone())),
                                );
                                Box::new(next)
                            }
                        }
                    },
                )
                .flatten()
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(result
            .iter()
            .map(|s| format!("{}\n", TomlSolver::value_to_string(s)))
            .collect::<String>())
    }

    fn value_to_string(value: &toml::Value) -> String {
        serde_yaml::to_string(value).unwrap()
    }
}

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("toml")
        .about("Perform queries on TOML files")
        .arg(
            clap::Arg::with_name("expression")
                .long("expression")
                .help("Expression to evaluate the input with.")
                .takes_value(true),
        )
        .author(clap::crate_authors!())
}
