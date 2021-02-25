mod json;
mod json_yaml;
mod yaml;

pub fn solve(
    file_type: &str,
    input: &str,
    expression: Option<&str>,
    replace: Option<&str>,
) -> String {
    match file_type.to_ascii_lowercase().as_ref() {
        "json" => json::JsonSolver::solve(input, expression, replace),
        "yaml" | "yml" => yaml::YamlSolver::solve(input, expression, replace),
        _ => panic!("Unknown file_type:{}", file_type),
    }
}

trait Solver {
    fn solve(input: &str, expression: Option<&str>, replace: Option<&str>) -> String;
}
