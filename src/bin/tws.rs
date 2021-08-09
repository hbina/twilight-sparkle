use parsers::{self, json::JsonSolver, toml::TomlSolver, yaml::YamlSolver};

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(crate::parsers::json::clap_app())
        .subcommand(crate::parsers::toml::clap_app())
        .subcommand(crate::parsers::yaml::clap_app())
        .get_matches();

    let (command, args) = matches.subcommand();
    match parsers::SupportedFiles::from_str(command) {
        Some(parsers::SupportedFiles::JSON) => {
            if let Some(matches) = args {
                let mut input = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
                let config = JsonSolver::from(matches).resolve_value(&input).unwrap();
                println!("{}", config);
            }
        }
        Some(parsers::SupportedFiles::TOML) => {
            if let Some(matches) = args {
                let mut input = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
                let config = TomlSolver::from(matches).resolve_value(&input).unwrap();
                println!("{}", config);
            }
        }
        Some(parsers::SupportedFiles::YAML) => {
            if let Some(matches) = args {
                let mut input = String::new();
                std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
                let config = YamlSolver::from(matches).resolve_value(&input).unwrap();
                println!("{}", config);
            }
        }
        None => {
            eprintln!("Unsupport file type. Please see --help for the list of support file types");
        }
    };
}
