use parsers::{
    self, json_parser::JsonSolver, toml_parser::TomlSolver, yaml_parser::YamlSolver, TError,
};

enum InputType {
    Stdin(std::io::Stdin),
    BufReader(std::io::BufReader<std::fs::File>),
}

impl InputType {
    fn read_everything(self) -> Result<String, TError> {
        let buffer = match self {
            InputType::Stdin(mut s) => {
                let mut result = String::new();
                std::io::Read::read_to_string(&mut s, &mut result)?;
                result
            }
            InputType::BufReader(mut reader) => {
                let mut result = String::new();
                std::io::Read::read_to_string(&mut reader, &mut result)?;
                result
            }
        };
        Ok(buffer)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(crate::parsers::json_parser::clap_app())
        .subcommand(crate::parsers::toml_parser::clap_app())
        .subcommand(crate::parsers::yaml_parser::clap_app())
        .arg(
            clap::Arg::with_name("input-file")
                .long("input-file")
                .takes_value(true),
        )
        .get_matches();

    let handle = if atty::is(atty::Stream::Stdin) {
        if let Some(file) = matches.value_of("input-file") {
            InputType::BufReader(std::io::BufReader::new(std::fs::File::open(file)?))
        } else {
            eprintln!("Please provide an input either by piping something in or specifying a file with '--input-file <file>'");
            return Err(Box::new(TError::NoInput));
        }
    } else {
        InputType::Stdin(std::io::stdin())
    };

    let (command, args) = matches.subcommand();
    if let Some(matches) = args {
        match parsers::SupportedFiles::from_str(command) {
            Some(parsers::SupportedFiles::JSON) => {
                let solver = JsonSolver::from(matches);
                if solver.json_line {
                    match handle {
                        InputType::Stdin(s) => solver.resolve_value_stream(s.lock())?,
                        InputType::BufReader(buffer) => solver.resolve_value_stream(buffer)?,
                    };
                } else {
                    let result = JsonSolver::from(matches)
                        .resolve_value(&handle.read_everything()?)
                        .unwrap();
                    println!("{}", result);
                }
            }
            Some(parsers::SupportedFiles::TOML) => {
                let result = TomlSolver::from(matches)
                    .resolve_value(&handle.read_everything()?)
                    .unwrap();
                println!("{}", result);
            }
            Some(parsers::SupportedFiles::YAML) => {
                let result = YamlSolver::from(matches)
                    .resolve_value(&handle.read_everything()?)
                    .unwrap();
                println!("{}", result);
            }
            None => {
                // TODO: This should display help instead.
                eprintln!(
                    "Unsupport file type. Please see --help for the list of support file types"
                );
            }
        }
    };
    Ok(())
}
