pub struct TomlSolver;

impl TomlSolver {
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
        let resolved_value = TomlSolver::resolve_value(&toml_value, &expression).unwrap();
        TomlSolver::value_to_string(resolved_value)
    }
}
