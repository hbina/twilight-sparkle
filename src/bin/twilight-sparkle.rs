use parsers;

fn main() {
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
            clap::Arg::with_name("file_type")
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
        std::fs::read_to_string(path).unwrap()
    } else {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        input
    };
    let expression = matches.value_of("expression").unwrap();
    let file_type = matches.value_of("file_type").unwrap();
    let result = parsers::solve(file_type, &input, expression);
    println!("{}", result);
}
