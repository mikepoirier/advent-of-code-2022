use std::collections::{HashMap, HashSet};

use advent_of_code_2022::prelude::*;

fn main() -> Result<()> {
    let input = Input::Full(3);
    let priority: Supplies = input.try_into()?;

    println!("Part 1: {}", priority.sum_of_duplicates());
    println!("Part 2: {}", priority.sum_of_badges());

    Ok(())
}

struct Supplies {
    input: String,
}

impl Supplies {
    fn sum_of_duplicates(&self) -> i32 {
        self.input.lines()
            .map(|line| Rucksack::from(line.to_string()))
            .filter_map(|r| r.find_duplicate_item())
            .filter_map(|i| i.value())
            .sum()
    }

    fn sum_of_badges(&self) -> i32 {
        self.input.lines()
            .collect::<Vec<_>>()
            .chunks(3)
            .filter_map(|c| find_common_item(c))
            .filter_map(|i| i.value())
            .sum()
    }
}

impl TryFrom<Input> for Supplies {
    type Error = anyhow::Error;

    fn try_from(value: Input) -> Result<Self> {
        let input = read_input(value)?;
        Ok(Self {
            input
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rucksack {
    contents: String,
    split_at: usize,
}

impl Rucksack {
    fn find_duplicate_item(&self) -> Option<Item> {
        let (first, second) = self.contents.split_at(self.split_at);
        find_common_item(&[first, second])
    }
}

fn find_common_item<'a, T>(items: &[T]) -> Option<Item>
where
    T: AsRef<str>
{
    items.iter()
        .map(|a| a.as_ref().chars().collect::<HashSet<_>>())
        .reduce(|acc, next| {
            acc.intersection(&next)
                .cloned()
                .collect()
        })
        .map(|set| set.into_iter().next())
        .flatten()
        .map(|c| Item(c.to_owned()))
    
}

impl From<String> for Rucksack {
    fn from(contents: String) -> Self {
        let split_at = contents.len() / 2;
        Self {
            contents,
            split_at
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn value(&self) -> Option<i32> {
        let a_range: Vec<_> = ('a'..='z').into_iter()
            .chain('A'..='Z')
            .collect();
        let n_range: Vec<_> = (1..=52).into_iter().collect();
        let map: HashMap<_, _> = a_range.iter().zip(n_range).collect();

        map.get(&self.0).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_duplicates_works() {
        let supplies = Supplies::try_from(Input::Test(3)).unwrap();

        let actual = supplies.sum_of_duplicates();

        assert_eq!(actual, 157)
    }

    #[test]
    fn sum_of_badges_works() {
        let supplies = Supplies::try_from(Input::Test(3)).unwrap();

        let actual = supplies.sum_of_badges();

        assert_eq!(actual, 70)
    }

    #[test]
    fn char_value_lower_works() {
        let a_range: Vec<_> = ('a'..='z').into_iter().collect();
        let n_range: Vec<_> = (1..=26).into_iter().collect();
        let expected: HashMap<_, _> = a_range.iter().zip(n_range).collect();

        for (c, num) in expected.into_iter() {
            let item = Item(c.to_owned());
            assert_eq!(item.value(), Some(num))
        }
        assert_eq!(Item(';').value(), None)
    }

    #[test]
    fn char_value_upper_works() {
        let a_range: Vec<_> = ('A'..='Z').into_iter().collect();
        let n_range: Vec<_> = (27..=52).into_iter().collect();
        let expected: HashMap<_, _> = a_range.iter().zip(n_range).collect();

        for (c, num) in expected.into_iter() {
            let item = Item(c.to_owned());
            assert_eq!(item.value(), Some(num))
        }
    }

    #[test]
    fn rucksack_new_works() {
        let contents = "abcd".to_string();

        let actual = Rucksack::from(contents.clone());

        assert_eq!(actual, Rucksack {
            contents,
            split_at: 2
        })
    }

    #[test]
    fn rucksack_find_duplicate_item_works() {
        let contents = "abca".to_string();
        let rucksack = Rucksack::from(contents.clone());

        let actual = rucksack.find_duplicate_item();

        assert_eq!(actual, Some(Item('a')))
    }

    #[test]
    fn find_common_items_works() {
        let items_1 = "abcdef";
        let items_2 = "fizz";
        let items_3 = "friend";

        let actual = find_common_item(&[items_1, items_2, items_3]);

        assert_eq!(actual, Some(Item('f')))
    }
}
