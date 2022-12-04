use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.lines().collect();

    println!("Result = {}", get_score(lines));
}

fn get_score(lines: Vec<&str>) -> usize {
    lines
        .chunks(3)
        .map(|c| {
            Group::new(
                Rucksack::new(c[0]),
                Rucksack::new(c[1]),
                Rucksack::new(c[2]),
            )
        })
        .map(|group| group.get_badge())
        .map(|badge| badge.get_priotity())
        .sum()
}

pub trait Priority {
    fn get_priotity(&self) -> usize;
}

impl Priority for char {
    fn get_priotity(&self) -> usize {
        match self {
            'a'..='z' => 1 + (*self as u8 - b'a') as usize,
            'A'..='Z' => 27 + (*self as u8 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

pub struct Group {
    pub first: Rucksack,
    pub second: Rucksack,
    pub third: Rucksack,
}

impl Group {
    pub fn new(first: Rucksack, second: Rucksack, third: Rucksack) -> Self {
        Self {
            first,
            second,
            third,
        }
    }

    pub fn get_badge(&self) -> char {
        // Don't expect any prices for this...
        let foo: HashSet<char> = self
            .first
            .items
            .intersection(&self.second.items)
            .copied()
            .collect();
        let bar: HashSet<char> = foo.intersection(&self.third.items).copied().collect();
        bar.iter().next().unwrap().clone()
    }
}

pub struct Rucksack {
    pub source: String,
    pub items: HashSet<char>,
}

impl Rucksack {
    pub fn new(input: &str) -> Self {
        Self {
            source: input.to_string(),
            items: HashSet::from_iter(input.chars()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_badges() {
        let groups = vec![
            Group::new(
                Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
                Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                Rucksack::new("PmmdzqPrVvPwwTWBwg"),
            ),
            Group::new(
                Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                Rucksack::new("ttgJtRGJQctTZtZT"),
                Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ),
        ];

        assert_eq!('r', groups[0].get_badge());
        assert_eq!('Z', groups[1].get_badge());
    }

    #[test]
    fn should_return_correct_score() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];

        assert_eq!(70, get_score(lines));
    }

    #[test]
    fn characters_should_return_correct_priority() {
        assert_eq!(16, 'p'.get_priotity());
        assert_eq!(38, 'L'.get_priotity());
        assert_eq!(42, 'P'.get_priotity());
        assert_eq!(22, 'v'.get_priotity());
        assert_eq!(20, 't'.get_priotity());
        assert_eq!(19, 's'.get_priotity());
    }
}
