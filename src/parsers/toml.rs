pub struct TomlSolver;

impl TomlSolver {
    fn resolve_and_replace_value<'a>(
        value: &'a mut toml::Value,
        expression: &Vec<&str>,
        replace: toml::Value,
    ) -> Result<(), crate::TError> {
        let mut reader = &mut *value;
        for expr in expression.iter() {
            if let Ok(index) = expr.parse::<usize>() {
                reader = reader
                    .get_mut(index)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            } else {
                reader = reader
                    .get_mut(expr)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            }
        }
        *reader = replace;
        Ok(())
    }

    fn resolve_value<'a>(
        value: &'a toml::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a toml::Value, crate::TError> {
        let mut reader = value;
        for expr in expression {
            if let Ok(index) = expr.parse::<usize>() {
                reader = reader
                    .get(index)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            } else {
                reader = reader
                    .get(expr)
                    .ok_or_else(|| crate::TError::KeyNotExist(expr.to_string()))?;
            }
        }
        Ok(reader)
    }

    fn value_to_string(value: &toml::Value) -> String {
        match value {
            toml::Value::String(s) => s.clone(),
            o => toml::to_string_pretty(o).unwrap(),
        }
    }
}

impl crate::Solver for TomlSolver {
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String {
        let mut toml_value = toml::from_str::<toml::Value>(&input)
            .map_err(|x| crate::TError::ConversionError(input.to_string(), Box::new(x)))
            .unwrap();
        let expression = if let Some(expression) = expression {
            expression.split('.').collect()
        } else {
            vec![]
        };
        if let Some(replace) = replace {
            let replace_value = toml::from_str::<toml::Value>(&replace)
                .map_err(|x| crate::TError::ConversionError(replace.to_string(), Box::new(x)))
                .unwrap();
            TomlSolver::resolve_and_replace_value(&mut toml_value, &expression, replace_value)
                .unwrap();
            TomlSolver::value_to_string(&toml_value)
        } else {
            let resolved_value = TomlSolver::resolve_value(&toml_value, &expression).unwrap();
            TomlSolver::value_to_string(resolved_value)
        }
    }
}

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("toml")
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
            clap::Arg::with_name("write")
                .long("write")
                .help(
                    "Output file. \
                        If not specified, will write to stdout.",
                )
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("file-type")
                .long("file-type")
                .help(
                    "What to interpret the input as. \
                        This is usually helpful if using stdin because \
                        we only infer the type from the extension.",
                )
                .takes_value(true)
                .required(false)
                .possible_values(&["json", "yaml", "toml"]),
        )
        .arg(
            clap::Arg::with_name("expression")
                .long("expression")
                .help("Expression to evaluate the input with.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("replace")
                .long("replace")
                .help("Value to replace the value resolved by the expression with.")
                .takes_value(true),
        )
        .author(clap::crate_authors!())
}
