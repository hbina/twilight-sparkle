use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .long("input")
                .help("Input file. If not specified, will read from stdin")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .help("Output file. If not specified, will write to stdout")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("type")
                .short("t")
                .long("type")
                .help("What to interpret the input as")
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("expression")
                .short("e")
                .long("expression")
                .help("Expression to evaluate the input with")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let input = if let Some(path) = matches.value_of("input") {
        std::fs::read_to_string(path)?
    } else {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
        input
    };
    let expression = matches.value_of("expression").unwrap();
    let file_type: FileType = matches
        .value_of("type")
        .map(std::convert::TryFrom::try_from)
        .unwrap()
        .expect("Unsupported file type");
    match file_type {
        FileType::JSON => {
            let content = serde_json::from_str(&input)?;
            let result = JsonSolver::parse_expression(&content, &expression)?;
            let to_print = json_value_to_string(result);
            println!("{}", to_print);
        }
    };
    Ok(())
}

fn json_value_to_string(v: &serde_json::Value) -> String {
    match v {
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
        o => format!("{:?}", o),
    }
}

#[derive(Debug)]
enum FileType {
    JSON,
}

impl std::convert::TryFrom<&str> for FileType {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.to_ascii_lowercase().as_ref() {
            "json" => Ok(FileType::JSON),
            _ => Err(String::from(str)),
        }
    }
}

#[derive(Debug)]
pub struct JsonSolver;

#[derive(Debug)]
pub enum JsonSolverError {
    KeyNotExist(serde_json::Value, String),
    ValueNotObject(serde_json::Value),
    Other(Box<dyn std::error::Error>),
}

impl std::error::Error for JsonSolverError {}

impl std::fmt::Display for JsonSolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::convert::From<ParseIntError> for JsonSolverError {
    fn from(err: ParseIntError) -> Self {
        JsonSolverError::Other(Box::new(err))
    }
}

impl JsonSolver {
    fn parse_expression<'a>(
        value: &'a serde_json::Value,
        expression: &str,
    ) -> Result<&'a serde_json::Value, JsonSolverError> {
        let mut reader = value;
        for expr in expression.split(".") {
            match reader {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(expr) {
                        reader = value;
                    } else {
                        return Err(JsonSolverError::KeyNotExist(
                            reader.clone(),
                            expr.to_string(),
                        ));
                    }
                }
                serde_json::Value::Array(arr) => {
                    let index = expr.parse::<usize>()?;
                    reader = arr.get(index).ok_or(JsonSolverError::KeyNotExist(
                        reader.clone(),
                        expr.to_string(),
                    ))?;
                }
                _ => return Err(JsonSolverError::ValueNotObject(value.clone())),
            }
        }
        Ok(reader)
    }
}
