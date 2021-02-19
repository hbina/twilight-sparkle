#[derive(Debug)]
pub struct JsonSolver;

#[derive(Debug)]
pub enum JsonSolverError {
    KeyNotExist(Vec<String>, String),
    IndexOutOfBound(usize, usize),
    ValueNotObject(String),
    NonNumericIndex(String),
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
    fn resolve_json_value_with_expression<'a>(
        value: &'a serde_json::Value,
        expression: &str,
    ) -> Result<&'a serde_json::Value, JsonSolverError> {
        let mut reader = value;
        for expr in expression.split('.') {
            match reader {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(expr) {
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
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Array(arr) => format!("{:?}", arr),
            o => serde_json::to_string(o).unwrap(),
        }
    }

    pub fn solve(input: &String, expression: &str) -> String {
        let json_value = serde_json::from_str::<serde_json::Value>(&input)
            .expect("Cannot parse input as a JSON file");
        let resolved_value =
            JsonSolver::resolve_json_value_with_expression(&json_value, expression).unwrap();
        JsonSolver::json_value_to_string(resolved_value)
    }
}
