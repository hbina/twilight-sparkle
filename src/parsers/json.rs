#[derive(Debug)]
pub struct JsonSolver {
    expression: Vec<String>,
}

impl From<&clap::ArgMatches<'_>> for JsonSolver {
    fn from(input: &clap::ArgMatches<'_>) -> JsonSolver {
        let expression = input
            .value_of("expression")
            // TODO: I am pretty sure its perfectly legal to use "." as a value key in JSON?
            .map(|s| s.split(".").map(String::from).collect::<_>())
            .unwrap_or_default();
        JsonSolver { expression }
    }
}

impl JsonSolver {
    pub fn resolve_value<'a>(self, value: &'a str) -> Result<String, crate::TError> {
        let mut result = vec![serde_json::from_str::<serde_json::Value>(value)?];
        for expr in self.expression {
            result = result
                .iter()
                .map(|reader| {
                    reader
                        .get(expr.as_str())
                        .map(|m| {
                            let nextr = match m {
                                serde_json::Value::Array(v) => v.clone(),
                                r => vec![r.clone()],
                            };
                            nextr
                        })
                        .ok_or_else(|| crate::TError::KeyNotExist(expr.clone()))
                })
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<_>>();
        }
        Ok(result
            .iter()
            .map(JsonSolver::value_to_string)
            .collect::<Vec<_>>()
            .join("\n"))
    }

    fn value_to_string(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            o => serde_json::to_string(o).unwrap(),
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
        .author(clap::crate_authors!())
}
