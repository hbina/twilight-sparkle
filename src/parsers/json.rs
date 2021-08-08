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
        JsonSolver { expression, pretty }
    }
}

impl JsonSolver {
    pub fn resolve_value<'a>(&self, value: &'a str) -> Result<String, crate::TError> {
        let mut result = vec![serde_json::from_str::<serde_json::Value>(value)?];
        for expr in &self.expression {
            result = result
                .into_iter()
                .map(|reader| match reader {
                    serde_json::Value::Array(v) => v
                        .into_iter()
                        .map(|o| {
                            o.get(expr.as_str())
                                .map(|v| v.clone())
                                .ok_or_else(|| crate::TError::KeyNotExist(expr.clone()))
                        })
                        .collect::<Result<Vec<_>, _>>(),
                    o => o
                        .get(expr.as_str())
                        .map(|v| vec![v.clone()])
                        .ok_or_else(|| crate::TError::KeyNotExist(expr.clone())),
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
        }
        Ok(result
            .iter()
            .map(|s| JsonSolver::value_to_string(self.pretty, s))
            .collect::<Vec<_>>()
            .join("\n"))
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
            clap::Arg::with_name("file")
                .long("file")
                .help(
                    "Input file. \
                        If not specified, will read from stdin.",
                )
                .takes_value(true),
        )
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
