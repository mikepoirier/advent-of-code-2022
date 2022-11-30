use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

use crate::prelude::*;

const INPUT_DIR: &str = "./inputs";

pub enum Input {
    Full(usize),
    Test(usize),
}
pub fn read_input(input: Input) -> Result<Vec<String>> {
    let mut path = PathBuf::from(INPUT_DIR);
    let filename = match input {
        Input::Full(day) => format!("day_{}.txt", day),
        Input::Test(day) => format!("day_{}_test.txt", day),
    };
    path.push(filename);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<_>>();

    Ok(lines)
}
