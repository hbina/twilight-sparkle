use crate::TError;

#[derive(Debug)]
pub struct JsonSolver {
    expression: Vec<String>,
    pretty: bool,
}

impl From<&clap::ArgMatches<'_>> for JsonSolver {
    fn from(input: &clap::ArgMatches<'_>) -> JsonSolver {
        let expression = input
            .value_of("expression")
            // TODO: I am pretty sure its perfectly legal to use "." as a value key in JSON?
            .map(|s| s.split(".").map(String::from).collect::<_>())
            .unwrap_or_default();
        let pretty = input.is_present("pretty");
        println!("expression:{:#?}", expression);
        JsonSolver { expression, pretty }
    }
}

impl JsonSolver {
    pub fn resolve_value<'a>(&self, value: &'a str) -> Result<String, TError> {
        let root = serde_json::from_str::<serde_json::Value>(value)?;
        let mut result = vec![&root];
        for expr in &self.expression {
            result = result
                .into_iter()
                .map(
                    |reader| -> Box<dyn Iterator<Item = Result<&serde_json::Value, TError>>> {
                        match reader {
                            serde_json::Value::Array(v) => {
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
            .map(|s| format!("{}\n", JsonSolver::value_to_string(self.pretty, s)))
            .collect::<String>())
    }

    fn value_to_string(pretty: bool, value: &serde_json::Value) -> String {
        if pretty {
            serde_json::to_string_pretty(value).unwrap()
        } else {
            serde_json::to_string(value).unwrap()
        }
    }
}

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("json")
        .about("Perform queries on JSON files")
        .arg(
            clap::Arg::with_name("expression")
                .long("expression")
                .help("Expression to evaluate the input with.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty prints the output")
                .takes_value(false),
        )
        .author(clap::crate_authors!())
}
