use crate::TError;

#[derive(Debug)]
pub struct CsvSolver {
    pub column_index: Option<usize>,
    pub row_index: Option<usize>,
}

impl From<&clap::ArgMatches<'_>> for CsvSolver {
    fn from(input: &clap::ArgMatches<'_>) -> CsvSolver {
        let column_index = input
            .value_of("column-index")
            .map(|s| usize::from_str_radix(s, 10).unwrap());
        let row_index = input
            .value_of("row-index")
            .map(|s| usize::from_str_radix(s, 10).unwrap());
        CsvSolver {
            column_index,
            row_index,
        }
    }
}

impl CsvSolver {
    pub fn from_reader<R>(&self, value: R) -> Result<(), TError>
    where
        R: std::io::BufRead,
    {
        let mut rdr = csv::Reader::from_reader(value);
        for (idx, result) in rdr.records().enumerate() {
            if let Some(row_idx) = self.row_index {
                if idx != row_idx {
                    continue;
                }
            }
            let record = result?;
            let result = match self.column_index {
                None => record
                    .into_iter()
                    .map(|s| String::from(s))
                    .collect::<Vec<_>>()
                    .join(" "),
                Some(col_idx) => record
                    .into_iter()
                    .nth(col_idx)
                    .map(|s| String::from(s))
                    .unwrap_or_default(),
            };
            println!("{}", result);
        }
        Ok(())
    }
}

pub fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new("csv")
        .about("Perform queries on CSV files")
        .arg(
            clap::Arg::with_name("column-index")
                .long("column-index")
                .help("Index of column to select")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("row-index")
                .long("row-index")
                .help("Index of row to select")
                .takes_value(true),
        )
        .author(clap::crate_authors!())
}
