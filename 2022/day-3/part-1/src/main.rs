use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let rucksacks : Vec<Rucksack> = input.lines().map(|line| Rucksack::new(line)).collect();

    let mut sum = 0;
    for rucksack in rucksacks {
        let error = rucksack.get_error();
        if let Some(error) = error {
            sum += error.get_priotity();
        }
    }

    println!("Result = {}", sum);
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

pub struct Rucksack {
    first: Vec<char>,
    second: Vec<char>,
}

impl Rucksack {
    pub fn new(input: &str) -> Self {
        let split = input.len() / 2;
        Rucksack { 
            first: input[0..split].chars().collect(), 
            second: input[split..input.len()].chars().collect(),
        }
    }

    pub fn get_error(&self) -> Option<char> {
        let errors = self.get_errors();
        if errors.len() != 1 {
            return None;
        }

        // No prices for this one...
        return Some(errors.iter().next().unwrap().clone());
    }

    pub fn get_errors(&self) -> HashSet<char> {
        let mut result = HashSet::new();
        for item in &self.first {
            if self.second.contains(&item) {
                result.insert(item.clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_score() {
        assert_eq!('p', Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp").get_error().unwrap());
        assert_eq!('L', Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").get_error().unwrap());
        assert_eq!('P', Rucksack::new("PmmdzqPrVvPwwTWBwg").get_error().unwrap());
        assert_eq!('v', Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").get_error().unwrap());
        assert_eq!('t', Rucksack::new("ttgJtRGJQctTZtZT").get_error().unwrap());
        assert_eq!('s', Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw").get_error().unwrap());
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