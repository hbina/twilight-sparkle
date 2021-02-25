#[derive(Debug)]
pub struct YamlSolver;

#[derive(Debug)]
pub enum YamlSolverError {
    ValueNotObject(String),
    ConversionError(String, Box<dyn std::error::Error>),
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

impl std::convert::From<serde_yaml::Error> for YamlSolverError {
    fn from(err: serde_yaml::Error) -> Self {
        YamlSolverError::Other(Box::new(err))
    }
}

impl YamlSolver {
    fn resolve_and_replace_yaml_value<'a>(
        value: &'a mut serde_yaml::Value,
        expression: &Vec<&str>,
        replace: serde_yaml::Value,
    ) -> Result<(), YamlSolverError> {
        let mut reader = &mut *value;
        for expr in expression.iter() {
            match reader {
                serde_yaml::Value::Mapping(map) => {
                    let key = serde_yaml::from_str(expr).unwrap();
                    let value = map.get_mut(&key).unwrap();
                    reader = value;
                }
                serde_yaml::Value::Sequence(arr) => {
                    let index = expr.parse::<usize>().unwrap();
                    reader = arr.get_mut(index).unwrap();
                }
                _ => {
                    return Err(YamlSolverError::ValueNotObject(
                        YamlSolver::yaml_value_to_string(reader),
                    ))
                }
            }
        }
        *reader = replace;
        Ok(())
    }

    fn resolve_yaml_value_with_expression<'a>(
        value: &'a serde_yaml::Value,
        expression: &Vec<&str>,
    ) -> Result<&'a serde_yaml::Value, YamlSolverError> {
        let mut reader = value;
        for expr in expression.iter() {
            match reader {
                serde_yaml::Value::Mapping(map) => {
                    let key = serde_yaml::from_str(expr).unwrap();
                    let value = map.get(&key).unwrap();
                    reader = value;
                }
                serde_yaml::Value::Sequence(arr) => {
                    let index = expr.parse::<usize>().unwrap();
                    reader = arr.get(index).unwrap();
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
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String {
        let mut json_value = serde_yaml::from_str::<serde_yaml::Value>(&input)
            .map_err(|x| YamlSolverError::ConversionError(input.to_string(), Box::new(x)))
            .unwrap();
        let expression = if let Some(expression) = expression {
            expression.split('.').collect()
        } else {
            vec![]
        };
        if let Some(replace) = replace {
            let replace_value = serde_yaml::from_str::<serde_yaml::Value>(&replace)
                .map_err(|x| YamlSolverError::ConversionError(input.to_string(), Box::new(x)))
                .unwrap();
            YamlSolver::resolve_and_replace_yaml_value(&mut json_value, &expression, replace_value)
                .unwrap();
            YamlSolver::yaml_value_to_string(&json_value)
        } else {
            let resolved_value =
                YamlSolver::resolve_yaml_value_with_expression(&json_value, &expression).unwrap();
            YamlSolver::yaml_value_to_string(resolved_value)
        }
    }
}
