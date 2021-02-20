#[derive(Debug)]
pub struct YamlSolver;

#[derive(Debug)]
pub enum YamlSolverError {
    KeyNotExist(Vec<(String, String)>, String),
    IndexOutOfBound(usize, usize),
    ValueNotObject(String),
    NonNumericIndex(String),
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for YamlSolverError {}

impl std::fmt::Display for YamlSolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::convert::From<std::num::ParseIntError> for YamlSolverError {
    fn from(err: std::num::ParseIntError) -> Self {
        YamlSolverError::Other(Box::new(err))
    }
}

impl YamlSolver {
    fn resolve_yaml_value_with_expression<'a>(
        value: &'a serde_yaml::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a serde_yaml::Value, YamlSolverError> {
        let mut reader = value;
        for expr in expression.iter() {
            match reader {
                serde_yaml::Value::Mapping(map) => {
                    let key = serde_yaml::from_str(expr).unwrap();
                    if let Some(value) = map.get(&key) {
                        reader = value;
                    } else {
                        let keys = map
                            .iter()
                            .map(|(k, v)| {
                                match (serde_yaml::to_string(k), serde_yaml::to_string(v)) {
                                    (Ok(k), Ok(v)) => Ok((k, v)),
                                    (Err(e), _) => Err(e),
                                    (_, Err(e)) => Err(e),
                                }
                            })
                            .collect::<serde_yaml::Result<_>>()
                            .unwrap();
                        return Err(YamlSolverError::KeyNotExist(keys, expr.to_string()));
                    }
                }
                serde_yaml::Value::Sequence(arr) => {
                    let index = expr
                        .parse::<usize>()
                        .map_err(|_| YamlSolverError::NonNumericIndex(expr.to_string()))?;
                    reader = arr
                        .get(index)
                        .ok_or_else(|| YamlSolverError::IndexOutOfBound(index, arr.len()))?;
                }
                _ => {
                    return Err(YamlSolverError::ValueNotObject(
                        YamlSolver::yaml_value_to_string(reader),
                    ))
                }
            }
        }
        Ok(reader)
    }

    fn yaml_value_to_string(value: &serde_yaml::Value) -> String {
        match value {
            serde_yaml::Value::String(s) => s.clone(),
            o => serde_yaml::to_string(o).unwrap(),
        }
    }
}

impl crate::Solver for YamlSolver {
    fn solve(input: &str, expression: &str) -> String {
        let json_value = serde_yaml::from_str::<serde_yaml::Value>(&input)
            .expect("Cannot parse input as a JSON file");
        let expression = if expression.is_empty() {
            vec![]
        } else {
            expression.split('.').collect()
        };
        let resolved_value =
            YamlSolver::resolve_yaml_value_with_expression(&json_value, &expression).unwrap();
        YamlSolver::yaml_value_to_string(resolved_value)
    }
}
