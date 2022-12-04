use std::result;

fn main() {
    let rounds = parse_input();
    let result = rounds.iter().map(|round| round.get_player_score()).sum::<i32>();

    println!("Result = {}", result);
}

fn parse_input() -> Vec<Round> {
    let input = include_str!("../input.txt");
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(Round::parse(line));
    }

    result
}

pub struct Round {
    opponent: Hand,
    player: Hand,
}

impl Round {
    pub fn new(opponent: Hand, player: Hand) -> Self {
        Self {
            opponent,
            player,
        }
    }

    pub fn parse(input: &str) -> Round {
        let inputs = input.split(' ').collect::<Vec<&str>>();
        if inputs.len() != 2 {
            panic!("invalid round");
        }

        Round::new(
            Hand::parse(inputs[0]),
            Hand::parse(inputs[1])
        )
    }

    pub fn get_player_score(&self) -> i32 {
        self.player.get_score() + match (&self.opponent, &self.player) {
            (Hand::Rock, Hand::Rock) => 3,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Scissor, Hand::Scissor) => 3,
            (Hand::Rock, Hand::Paper) => 6,
            (Hand::Rock, Hand::Scissor) => 0,
            (Hand::Paper, Hand::Scissor) => 6,
            (Hand::Paper, Hand::Rock) => 0,
            (Hand::Scissor, Hand::Rock) => 6,
            (Hand::Scissor, Hand::Paper) => 0,
        }
    }
}

#[derive(PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl Hand {
    pub fn parse(input: &str) -> Hand {
        match input {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
            "X" => Hand::Rock,
            "Y" => Hand::Paper,
            "Z" => Hand::Scissor,
            _ => panic!("unknown hand")
        }
    }
    pub fn get_score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_should_return_correct_player_outcome() {
        assert_eq!(8, Round::new(Hand::Rock, Hand::Paper).get_player_score());
        assert_eq!(1, Round::new(Hand::Paper, Hand::Rock).get_player_score());
        assert_eq!(6, Round::new(Hand::Scissor, Hand::Scissor).get_player_score());
    }
}