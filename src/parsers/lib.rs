mod json;
mod yaml;

pub fn solve(file_type: &str, input: &str, expression: &str) -> String {
    match file_type.to_ascii_lowercase().as_ref() {
        "json" => json::JsonSolver::solve(input, expression),
        "yaml" | "yml" => yaml::YamlSolver::solve(input, expression),
        _ => panic!("Unknown file_type:{}", file_type),
    }
}

trait Solver {
    fn solve(input: &str, expression: &str) -> String;
}
