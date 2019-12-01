use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Record {
    pub name: String,
    pub shop: String,
    pub category: String,
    pub grades: Vec<Option<f32>>,
}

#[derive(Debug)]
pub struct Cake {
    pub name: String,
    pub shop: String,
    pub category: String,
}

impl Cake {
    pub fn from_record(record: Record) -> Self {
        Cake {
            name: record.name,
            shop: record.shop,
            category: record.category,
        }
    }
}

#[derive(Debug)]
pub struct Grade {
    pub cake: Cake,
    pub grade: u32,
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}): {}",
            self.cake.name,
            self.cake.shop,
            self.grade as f32 / 100.0
        )
    }
}

#[derive(Debug)]
pub struct Stats {
    pub averages: Vec<Grade>,
}

impl Stats {
    pub fn from_records(records: Vec<Record>) -> Self {
        let mut averages: Vec<Grade> = Vec::new();
        for record in records {
            let grades: Vec<f32> = record.grades.iter().filter_map(|g| *g).collect();
            let cake = Cake::from_record(record);
            let average_grade = grades.iter().sum::<f32>() / grades.len() as f32;
            averages.push(Grade {
                cake,
                grade: (average_grade * 100.0) as u32,
            });
        }
        Stats { averages }
    }
}
