use crate::TError;

#[derive(Debug)]
pub struct JsonSolver {
    pub expression: Vec<String>,
    pub pretty: bool,
    pub recursive: bool,
    pub json_line: bool,
    pub skip_empty: bool,
}

impl From<&clap::ArgMatches<'_>> for JsonSolver {
    fn from(input: &clap::ArgMatches<'_>) -> JsonSolver {
        let expression = input
            .value_of("expression")
            // TODO: I am pretty sure its perfectly legal to use "." as a value key in JSON?
            .map(|s| s.split('.').map(String::from).collect::<_>())
            .unwrap_or_default();
        let pretty = input.is_present("pretty");
        let recursive = input.is_present("recursive");
        let json_line = input.is_present("json-lines");
        let skip_empty = input.is_present("skip-empty");
        JsonSolver {
            expression,
            pretty,
            recursive,
            json_line,
            skip_empty,
        }
    }
}

impl JsonSolver {
    pub fn resolve_value_stream<R>(&self, value: R) -> Result<(), TError>
        where
            R: std::io::BufRead,
    {
        value
            .lines()
            .map(|value| value.map(|v| self.resolve_value_impl(&v)))
            .flatten()
            .flatten()
            .flatten()
            .map(|s| JsonSolver::value_to_string(self.pretty, &s))
            .for_each(|s| println!("{}", s));
        Ok(())
    }

    pub fn resolve_value(&self, value: &str) -> Result<String, TError> {
        let lines = if self.json_line {
            value.split('\n').filter(|v| !v.is_empty()).collect()
        } else {
            vec![value]
        };
        Ok(lines
            .into_iter()
            .map(|value| self.resolve_value_impl(value))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .map(|s| JsonSolver::value_to_string(self.pretty, &s))
            .collect::<Vec<String>>()
            .join("\n"))
    }

    fn resolve_value_impl(&self, value: &str) -> Result<Vec<serde_json::Value>, TError> {
        let root_proto = serde_json::from_str::<serde_json::Value>(value)?;
        let root = self.recursively_parse(root_proto)?;
        let resolved_value = {
            let mut result = vec![root];
            for expr in &self.expression {
                result =
                    result
                        .into_iter()
                        .map(
                            |reader| -> Box<
                                dyn Iterator<Item=Result<Option<serde_json::Value>, TError>>,
                            > {
                                match reader {
                                    serde_json::Value::Array(v) => {
                                        let next = v.into_iter().map(|values| {
                                            let result = values.get(expr.as_str()).cloned();
                                            if let Some(v) = result {
                                                Ok(Some(v))
                                            } else if self.skip_empty {
                                                Ok(None)
                                            } else {
                                                Err(TError::KeyNotExist(expr.clone()))
                                            }
                                        });
                                        Box::new(next)
                                    }
                                    o => {
                                        let next = std::iter::once({
                                            let result = o.get(expr.as_str()).cloned();
                                            if let Some(v) = result {
                                                Ok(Some(v))
                                            } else if self.skip_empty {
                                                Ok(None)
                                            } else {
                                                Err(TError::KeyNotExist(expr.clone()))
                                            }
                                        });
                                        Box::new(next)
                                    }
                                }
                            },
                        )
                        .flatten()
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>();
            }
            result
        };
        Ok(resolved_value)
    }

    fn recursively_parse(&self, value: serde_json::Value) -> Result<serde_json::Value, TError> {
        if self.recursive {
            match value {
                serde_json::Value::Array(v) => {
                    let result = v
                        .into_iter()
                        .map(|s| self.recursively_parse(s))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(serde_json::Value::Array(result))
                }
                serde_json::Value::String(s) => {
                    Ok(serde_json::from_str(&s).unwrap_or(serde_json::Value::String(s)))
                }
                serde_json::Value::Object(map) => {
                    let result = map
                        .into_iter()
                        .map(|(key, value)| {
                            self.recursively_parse(value)
                                .map(|parsed_value| (key, parsed_value))
                        })
                        .collect::<Result<serde_json::Map<_, _>, _>>()?;
                    Ok(serde_json::Value::Object(result))
                }
                v => Ok(v),
            }
        } else {
            Ok(value)
        }
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
                .help("Expression to evaluate the input with")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty prints the output")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("recursive")
                .long("recursive")
                .help(
                    "Recursively parses every string values as JSON. \
                Fallback to string if it fails",
                )
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("json-lines")
                .long("json-lines")
                .help("Enable JSON-lines mode")
                .takes_value(false),
        )
        .arg(
            clap::Arg::with_name("skip-empty")
                .long("skip-empty")
                .help("If expression fails to resolve, skip the value")
                .takes_value(false),
        )
        .author(clap::crate_authors!())
}
