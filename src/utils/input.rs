use std::path::PathBuf;

use crate::prelude::*;

const INPUT_DIR: &str = "./inputs";

#[derive(Clone, Copy)]
pub enum Input {
    Full(u8),
    Test(u8),
}

pub fn read_input(input: Input) -> Result<String> {
    let mut path = PathBuf::from(INPUT_DIR);
    let filename = match input {
        Input::Full(day) => format!("day_{}.txt", day),
        Input::Test(day) => format!("day_{}_test.txt", day),
    };
    path.push(filename);

    println!("Reading file: {:?}", path);
    let content = std::fs::read_to_string(path)?;

    Ok(content)
}
