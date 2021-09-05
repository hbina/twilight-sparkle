use parsers::{self, json::JsonSolver, toml::TomlSolver, yaml::YamlSolver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .subcommand(crate::parsers::json::clap_app())
        .subcommand(crate::parsers::toml::clap_app())
        .subcommand(crate::parsers::yaml::clap_app())
        .arg(
            clap::Arg::with_name("input-file")
                .long("input-file")
                .takes_value(true),
        )
        .get_matches();

    let buffer = if atty::is(atty::Stream::Stdin) {
        if let Some(file) = matches.value_of("input-file") {
            std::fs::read_to_string(file)
        } else {
            eprintln!("Please provide an input either by piping something in or specifying a file with '--input-file <file>'");
            return Ok(());
        }
    } else {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input)?;
        Ok(input)
    }?;

    let (command, args) = matches.subcommand();
    if let Some(matches) = args {
        match parsers::SupportedFiles::from_str(command) {
            Some(parsers::SupportedFiles::JSON) => {
                let result = JsonSolver::from(matches).resolve_value(&buffer).unwrap();
                println!("{}", result);
            }
            Some(parsers::SupportedFiles::TOML) => {
                let result = TomlSolver::from(matches).resolve_value(&buffer).unwrap();
                println!("{}", result);
            }
            Some(parsers::SupportedFiles::YAML) => {
                let result = YamlSolver::from(matches).resolve_value(&buffer).unwrap();
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
