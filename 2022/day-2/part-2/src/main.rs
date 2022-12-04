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
    outcome: Outcome,
}

impl Round {
    pub fn new(opponent: Hand, outcome: Outcome) -> Self {
        Self {
            opponent,
            outcome,
        }
    }

    pub fn parse(input: &str) -> Round {
        let inputs = input.split(' ').collect::<Vec<&str>>();
        if inputs.len() != 2 {
            panic!("invalid round");
        }

        Round::new(
            Hand::parse(inputs[0]),
            Outcome::parse(inputs[1])
        )
    }

    pub fn get_player_score(&self) -> i32 {
        let player = self.outcome.get_hand(&self.opponent);
        player.get_score() + match (&self.opponent, &player) {
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

#[derive(PartialEq, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissor,
}

pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn parse(input: &str) -> Outcome {
        match input {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unknown outcome")
        }
    }

    pub fn get_hand(&self, opponent: &Hand) -> Hand {
        match  self {
            Outcome::Win => match opponent {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock,
            },
            Outcome::Lose => match opponent {
                Hand::Rock => Hand::Scissor,
                Hand::Paper => Hand::Rock,
                Hand::Scissor => Hand::Paper,
            },
            Outcome::Draw => opponent.clone(),
        }
    }
}

impl Hand {
    pub fn parse(input: &str) -> Hand {
        match input {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissor,
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
        assert_eq!(4, Round::new(Hand::Rock, Outcome::Draw).get_player_score());
        assert_eq!(1, Round::new(Hand::Paper, Outcome::Lose).get_player_score());
        assert_eq!(7, Round::new(Hand::Scissor, Outcome::Win).get_player_score());
    }
}