use parsers;

fn main() {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("file")
                .long("file")
                .help("Input file. If not specified, will read from stdin.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("write")
                .long("write")
                .help("Output file. If not specified, will write to stdout.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("file-type")
                .long("file-type")
                .help(
                    r##"What to interpret the input as. This is usually helpful if using stdin because we only infer the type from the extension."##,
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            clap::Arg::with_name("expression")
                .long("expression")
                .help("Expression to evaluate the input with.")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("replace")
                .long("replace")
                .help("Value to replace the value resolved by the expression with.")
                .takes_value(true),
        )
        .get_matches();
    let (input, ext) = if let Some(path) = matches.value_of("file") {
        (
            std::fs::read_to_string(path).unwrap(),
            matches
                .value_of("file-type")
                .map(String::from)
                .unwrap_or_else(|| {
                    get_extension_from_filename(path)
                        .expect("Cannot parse the extension from the file path.")
                }),
        )
    } else {
        let mut input = String::new();
        std::io::Read::read_to_string(&mut std::io::stdin(), &mut input).unwrap();
        let file_type = matches.value_of("file-type").map(String::from).expect("Please provide argument `file-type` or else we can't infer what file type of the stdin.");
        (input, file_type)
    };
    let expression = matches.value_of("expression");
    let replace = matches.value_of("replace");
    let result = parsers::solve(ext.as_ref(), &input, expression, replace);
    if let Some(output) = matches.value_of("write") {
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .append(false)
            .open(output)
            .unwrap();
        std::io::Write::write_all(&mut file, result.as_bytes()).unwrap();
        std::io::Write::flush(&mut file).unwrap();
    } else {
        println!("{}", result);
    }
}

fn get_extension_from_filename(filename: &str) -> Option<String> {
    std::path::Path::new(filename)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(String::from)
}
