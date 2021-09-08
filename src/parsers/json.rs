#[derive(Debug)]
pub struct JsonSolver;

impl JsonSolver {
    fn resolve_value<'a>(
        value: &'a serde_json::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a serde_json::Value, crate::TError> {
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

    fn value_to_string(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            o => serde_json::to_string_pretty(o).unwrap(),
        }
    }
}

impl crate::Solver for JsonSolver {
    fn solve(input: &str, expression: Option<&str>) -> String {
        let mut json_value = serde_json::from_str::<serde_json::Value>(&input)
            .map_err(|x| crate::TError::ConversionError(input.to_string(), Box::new(x)))
            .unwrap();
        let expression = if let Some(expression) = expression {
            expression.split('.').collect()
        } else {
            vec![]
        };
        let resolved_value = JsonSolver::resolve_value(&json_value, &expression).unwrap();
        JsonSolver::value_to_string(resolved_value)
    }
}
