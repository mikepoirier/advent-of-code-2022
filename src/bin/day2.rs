use advent_of_code_2022::prelude::*;
use anyhow::anyhow;

fn main() -> Result<()> {
    let input = Input::Full(2);
    let strategy_one = StrategyOne::try_from(input)?;
    let strategy_two = StrategyTwo::try_from(input)?;

    println!("Day 1: {}", strategy_one.score());
    println!("Day 2: {}", strategy_two.score());

    Ok(())
}

struct StrategyOne {
    input: String
}

impl StrategyOne {
    fn score(&self) -> i32 {
        self.input.lines()
            .filter_map(|s| s.split_once(' '))
            .filter_map(|pair| pair.try_into().ok())
            .map(|round: Round| round.score())
            .sum()
    }
}

impl TryFrom<Input> for StrategyOne {
    type Error = anyhow::Error;

    fn try_from(value: Input) -> Result<Self> {
        let input = read_input(value)?;
        Ok(Self {
            input
        })
    }
}

struct StrategyTwo {
    input: String
}

impl StrategyTwo {
    fn score(&self) -> i32 {
        self.input.lines()
            .filter_map(|s| s.split_once(' '))
            .filter_map(|pair| pair.try_into().ok())
            .map(|round: Round2| round.score())
            .sum()
    }
}

impl TryFrom<Input> for StrategyTwo {
    type Error = anyhow::Error;

    fn try_from(value: Input) -> Result<Self> {
        let input = read_input(value)?;
        Ok(Self {
            input
        })
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

impl TryFrom<(&str, &str)> for Round {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self> {
        Ok(Self(value.0.try_into()?, value.1.try_into()?))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round2(Hand, Outcome);

impl Round2 {
    fn score(&self) -> i32 {
        let my_hand = match &self {
            Self(Hand::Rock, Outcome::Win) => Hand::Paper,
            Self(Hand::Rock, Outcome::Lose) => Hand::Scissors,
            Self(Hand::Rock, Outcome::Draw) => Hand::Rock,
            Self(Hand::Paper, Outcome::Win) => Hand::Scissors,
            Self(Hand::Paper, Outcome::Lose) => Hand::Rock,
            Self(Hand::Paper, Outcome::Draw) => Hand::Paper,
            Self(Hand::Scissors, Outcome::Win) => Hand::Rock,
            Self(Hand::Scissors, Outcome::Lose) => Hand::Paper,
            Self(Hand::Scissors, Outcome::Draw) => Hand::Scissors,
        };
        let round = Round(self.0.clone(), my_hand);

        round.score()
    }
}

impl TryFrom<(&str, &str)> for Round2 {
    type Error = anyhow::Error;

    fn try_from(value: (&str, &str)) -> Result<Self> {
        let hand = Hand::try_from(value.0)?;
        let outcome = Outcome::try_from(value.1)?;

        Ok(Self(hand, outcome))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Hand
{
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

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl TryFrom<&str> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Invalid Outcome Value: {}", value))
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
    fn strategy_one_score_works() {
        let strategy_one = StrategyOne::try_from(Input::Test(2)).unwrap();

        let score = strategy_one.score();

        assert_eq!(score, 15)
    }

    #[test]
    fn strategy_two_score_works() {
        let strategy_two = StrategyTwo::try_from(Input::Test(2)).unwrap();

        let score = strategy_two.score();

        assert_eq!(score, 12)
    }
}

