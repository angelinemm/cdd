mod cdd;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

use crate::cdd::{Record, Stats};

fn main() {
    let path: &str = "data.csv";
    let mut stats = parse_file(path).expect("Nooooooo!");

    // Show Ranking:
    stats
        .averages
        .sort_by(|grade_1, grade_2| grade_2.grade.cmp(&grade_1.grade));
    for grade in stats.averages {
        println!("{}", grade);
    }
}

fn parse_file(path: &str) -> Result<Stats, Box<dyn Error>> {
    let data_file = File::open(&path)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data_file);
    let records: Vec<Record> = rdr.deserialize::<Record>().flatten().collect();
    Ok(Stats::from_records(records))
}
