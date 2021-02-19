use json::JsonSolver;

mod json;

#[derive(Debug)]
pub enum Solver {
    Json,
}

impl Solver {
    pub fn create_solver(file_type: &str) -> Solver {
        match file_type.to_ascii_lowercase().as_ref() {
            "json" => Solver::Json,
            _ => panic!("Unknown file_type:{}", file_type),
        }
    }

    pub fn solve(&self, input: &String, expression: &str) -> String {
        match self {
            Solver::Json => JsonSolver::solve(input, expression),
        }
    }
}
