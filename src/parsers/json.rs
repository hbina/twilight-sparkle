#[derive(Debug)]
pub struct JsonSolver;

#[derive(Debug)]
pub enum JsonSolverError {
    KeyNotExist(Vec<String>, String),
    IndexOutOfBound(usize, usize),
    ValueNotObject(String),
    NonNumericIndex(String),
    ConversionError(String, Box<dyn std::error::Error>),
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for JsonSolverError {}

impl std::fmt::Display for JsonSolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::convert::From<std::num::ParseIntError> for JsonSolverError {
    fn from(err: std::num::ParseIntError) -> Self {
        JsonSolverError::Other(Box::new(err))
    }
}

impl JsonSolver {
    fn resolve_and_replace_json_value<'a>(
        value: &'a mut serde_json::Value,
        expression: &Vec<&str>,
        replace: serde_json::Value,
    ) -> Result<(), JsonSolverError> {
        let mut reader = &mut *value;
        for expr in expression.iter() {
            match reader {
                serde_json::Value::Object(map) => {
                    let value = map.get_mut(*expr).unwrap();
                    reader = value;
                }
                serde_json::Value::Array(arr) => {
                    let index = expr.parse::<usize>().unwrap();
                    reader = arr.get_mut(index).unwrap();
                }
                _ => {
                    return Err(JsonSolverError::ValueNotObject(
                        JsonSolver::json_value_to_string(reader),
                    ))
                }
            }
        }
        *reader = replace;
        Ok(())
    }

    fn resolve_json_value_with_expression<'a>(
        value: &'a serde_json::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a serde_json::Value, JsonSolverError> {
        let mut reader = value;
        for expr in expression {
            match reader {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(*expr) {
                        reader = value;
                    } else {
                        return Err(JsonSolverError::KeyNotExist(
                            map.keys().cloned().collect(),
                            expr.to_string(),
                        ));
                    }
                }
                serde_json::Value::Array(arr) => {
                    let index = expr
                        .parse::<usize>()
                        .map_err(|_| JsonSolverError::NonNumericIndex(expr.to_string()))?;
                    reader = arr
                        .get(index)
                        .ok_or_else(|| JsonSolverError::IndexOutOfBound(index, arr.len()))?;
                }
                _ => {
                    return Err(JsonSolverError::ValueNotObject(
                        JsonSolver::json_value_to_string(reader),
                    ))
                }
            }
        }
        Ok(reader)
    }

    fn json_value_to_string(value: &serde_json::Value) -> String {
        match value {
            serde_json::Value::String(s) => s.clone(),
            o => serde_json::to_string(o).unwrap(),
        }
    }
}

impl crate::Solver for JsonSolver {
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String {
        let mut json_value = serde_json::from_str::<serde_json::Value>(&input)
            .map_err(|x| JsonSolverError::ConversionError(input.to_string(), Box::new(x)))
            .unwrap();
        let expression = if let Some(expression) = expression {
            expression.split('.').collect()
        } else {
            vec![]
        };
        if let Some(replace) = replace {
            let replace_value = serde_json::from_str::<serde_json::Value>(&replace)
                .map_err(|x| JsonSolverError::ConversionError(replace.to_string(), Box::new(x)))
                .unwrap();
            JsonSolver::resolve_and_replace_json_value(&mut json_value, &expression, replace_value)
                .unwrap();
            JsonSolver::json_value_to_string(&json_value)
        } else {
            let resolved_value =
                JsonSolver::resolve_json_value_with_expression(&json_value, &expression).unwrap();
            JsonSolver::json_value_to_string(resolved_value)
        }
    }
}
