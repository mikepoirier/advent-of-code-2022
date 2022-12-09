use std::ops::{Deref, DerefMut};

use advent_of_code_2022::prelude::*;
use anyhow::anyhow;

fn main() -> Result<()> {
    let input: ElfCalories = Input::Full(1).try_into()?;

    println!("Day 1: {}", input.max_calories()?);

    println!("Day 2: {}", input.top_three_calories()?);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct ElfCalories(Vec<Vec<i32>>);

impl ElfCalories {
    fn sum_groups(&self) -> Vec<i32> {
        self.iter().map(|items| items.iter().sum()).collect()
    }

    fn max_calories(&self) -> Result<i32> {
        self.sum_groups()
            .into_iter()
            .max()
            .ok_or(anyhow!("Could not find max calories"))
    }

    fn top_three_calories(&self) -> Result<i32> {
        let top_three = self.sum_groups()
            .into_iter()
            .rev()
            .take(3)
            .sum();
        Ok(top_three)
    }
}

impl Default for ElfCalories {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Deref for ElfCalories {
    type Target = Vec<Vec<i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ElfCalories {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Input> for ElfCalories {
    type Error = anyhow::Error;

    fn try_from(value: Input) -> CoreResult<Self, Self::Error> {
        let input = read_input(value)?;
        input.try_into()
    }
}

impl TryFrom<Vec<String>> for ElfCalories {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> CoreResult<Self, Self::Error> {
        let mut parsed_input = ElfCalories::default();
        let mut inner_vec: Vec<i32> = vec![];
        for item in value {
            if item.is_empty() {
                parsed_input.push(inner_vec);
                inner_vec = vec![];
            } else {
                inner_vec.push(item.parse()?);
            }
        }
        parsed_input.push(inner_vec);
        Ok(parsed_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_works() {
        let actual: ElfCalories = Input::Test(1).try_into().unwrap();

        assert_eq!(
            actual,
            ElfCalories(vec![
                vec![1000, 2000, 3000],
                vec![4000],
                vec![5000, 6000],
                vec![7000, 8000, 9000],
                vec![10000],
            ])
        )
    }

    #[test]
    fn day_1_works() {
        let input: ElfCalories = Input::Test(1).try_into().unwrap();

        let actual = input.max_calories().unwrap();

        assert_eq!(actual, 24000);
    }

    #[test]
    fn day_2_works() {
        let input: ElfCalories = Input::Test(1).try_into().unwrap();

        let actual = input.top_three_calories().unwrap();

        assert_eq!(actual, 45000);
    }
}
