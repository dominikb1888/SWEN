use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Biomarker {
}

impl Biomarker {
    pub fn new() -> Self {
    }

    pub fn next_value(&self) -> i64 {
    }
}

pub fn load_biomarker_data(filename: &str) -> io::Result<Vec<Biomarker>> {
}

pub fn sum_extrapolated_values(biomarkers: Vec<Biomarker>) -> i64 {
}

