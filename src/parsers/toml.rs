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
            o => toml::to_string(o).unwrap(),
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
