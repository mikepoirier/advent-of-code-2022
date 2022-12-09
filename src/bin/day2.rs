use std::ops::Deref;

use advent_of_code_2022::prelude::*;
use anyhow::anyhow;

fn main() -> Result<()> {

    let strategy_guide: StrategyGuide = Input::Full(2).try_into()?;

    println!("Day 1: {}", strategy_guide.total_score());

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct StrategyGuide(Vec<Round>);

impl StrategyGuide {
    fn total_score(&self) -> i32 {
        self.iter()
            .map(|round| round.score())
            .sum()
    }
}

impl Deref for StrategyGuide {
    type Target = Vec<Round>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Input> for StrategyGuide {
    type Error = anyhow::Error;

    fn try_from(value: Input) -> Result<Self> {
        let input = read_input(value)?;
        input.try_into()
    }
}

impl TryFrom<Vec<String>> for StrategyGuide {
    type Error = anyhow::Error;

    fn try_from(value: Vec<String>) -> Result<Self> {
        let mut results = vec![];
        let pairs = value.iter()
            .filter_map(|s| s.split_once(' '));
        for pair in pairs {
            results.push(pair.try_into()?);
        }
        Ok(Self(results))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round(Hand, Hand);

impl Round {
    fn score(&self) -> i32 {
        let mut score: i32 = 0;

        score += match (&self.0, &self.1) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scissors) => 0,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Paper, Hand::Scissors) => 6,
            (Hand::Scissors, Hand::Rock) => 6,
            (Hand::Scissors, Hand::Paper) => 0,
            (Hand::Scissors, Hand::Scissors) => 3,
        };

        score += match &self.1 {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        score
    }
}

impl From<(Hand, Hand)> for Round {
    fn from(value: (Hand, Hand)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<(&str, &str)> for Round {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self> {
        Ok(Self(value.0.try_into()?, value.1.try_into()?))
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err(anyhow!("Invalid Hand Value: {}", value))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hand_from_a_works() {
        let hand = Hand::try_from("A").unwrap();

        assert_eq!(hand, Hand::Rock)
    }

    #[test]
    fn hand_from_b_works() {
        let hand = Hand::try_from("B").unwrap();

        assert_eq!(hand, Hand::Paper)
    }

    #[test]
    fn hand_from_c_works() {
        let hand = Hand::try_from("C").unwrap();

        assert_eq!(hand, Hand::Scissors)
    }

    #[test]
    fn hand_from_x_works() {
        let hand = Hand::try_from("X").unwrap();

        assert_eq!(hand, Hand::Rock)
    }

    #[test]
    fn hand_from_y_works() {
        let hand = Hand::try_from("Y").unwrap();

        assert_eq!(hand, Hand::Paper)
    }

    #[test]
    fn hand_from_z_works() {
        let hand = Hand::try_from("Z").unwrap();

        assert_eq!(hand, Hand::Scissors)
    }

    #[test]
    fn round_from_strs_works() {
        let round = Round::try_from(("A", "Y")).unwrap();

        assert_eq!(round, Round(Hand::Rock, Hand::Paper))
    }

    #[test]
    fn strategy_guide_from_input_works() {
        let strategy_guide = StrategyGuide::try_from(Input::Test(2)).unwrap();

        assert_eq!(strategy_guide, StrategyGuide(vec![
            Round(Hand::Rock, Hand::Paper),
            Round(Hand::Paper, Hand::Rock),
            Round(Hand::Scissors, Hand::Scissors),
        ]))
    }

    #[test]
    fn strategy_guide_total_score_works() {
        let strategy_guide = StrategyGuide::try_from(Input::Test(2)).unwrap();

        let total_score = strategy_guide.total_score();

        assert_eq!(total_score, 15);
    }
}

