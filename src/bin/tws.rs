use std::io::BufRead;

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

    fn read_line(&mut self, buffer: &mut String) -> Result<bool, TError> {
        let buffer = match self {
            InputType::Stdin(s) => s.read_line(buffer)? != 0,
            InputType::BufReader(reader) => reader.read_line(buffer)? != 0,
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

    let mut handle = if atty::is(atty::Stream::Stdin) {
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
        match parsers::SupportedFiles::maybe_from_str(command) {
            Some(parsers::SupportedFiles::Json) => {
                let solver = JsonSolver::from(matches);

                if solver.json_line {
                    let mut buffer = String::new();
                    while handle.read_line(&mut buffer)? {
                        for x in JsonSolver::from(matches)
                            .resolve_line(buffer.as_str())
                            .unwrap()
                        {
                            println!("{}", x);
                        }
                        buffer.clear();
                    }
                } else {
                    for x in JsonSolver::from(matches)
                        .resolve_value(handle.read_everything()?.as_ref())
                        .unwrap()
                    {
                        println!("{}", x);
                    }
                }
            }
            Some(parsers::SupportedFiles::Toml) => {
                let result = TomlSolver::from(matches)
                    .resolve_value(handle.read_everything()?.as_ref())
                    .unwrap();
                println!("{}", result);
            }
            Some(parsers::SupportedFiles::Yaml) => {
                let result = YamlSolver::from(matches)
                    .resolve_value(handle.read_everything()?.as_ref())
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
