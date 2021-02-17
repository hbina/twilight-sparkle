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
        serde_json::Value::Array(v1) => v1
            .iter()
            .map(json_value_to_string)
            .collect::<Vec<String>>()
            .join(","),
        _ => unimplemented!("Deserializing back to JSON is not yet supported"),
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
}

impl std::error::Error for JsonSolverError {}

impl std::fmt::Display for JsonSolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl JsonSolver {
    fn parse_expression<'a>(
        value: &'a serde_json::Value,
        expr: &str,
    ) -> Result<&'a serde_json::Value, JsonSolverError> {
        let mut reader = value;
        let err = || JsonSolverError::KeyNotExist(value.clone(), expr.to_string());
        for item in expr.split('.') {
            reader = reader.get(item).ok_or_else(err)?;
        }
        Ok(reader)
    }
}
