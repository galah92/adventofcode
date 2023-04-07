use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/bin/day2/input.txt")?;
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>();

    let part1_score = lines
        .iter()
        .filter_map(|line| Part1Round::from_str(line).map(|r| r.score()).ok())
        .sum::<i32>();
    println!("{part1_score}");

    let part2_score = lines
        .iter()
        .filter_map(|line| Part2Round::from_str(line).map(|r| r.score()).ok())
        .sum::<i32>();
    println!("{part2_score}");

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Hand {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Hand::Rock),
            'B' | 'Y' => Ok(Hand::Paper),
            'C' | 'Z' => Ok(Hand::Scissors),
            _ => Err("Invalid hand".into()),
        }
    }
}

impl Hand {
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn beats(&self, other: &Self) -> bool {
        matches!(
            (&self, &other),
            (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
                | (Self::Rock, Self::Scissors)
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl TryFrom<char> for Outcome {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Invalid outcome".into()),
        }
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Part1Round {
    opponent_hand: Hand,
    my_hand: Hand,
}

impl FromStr for Part1Round {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (opponent, _space, me) = value.chars().take(3).collect_tuple().unwrap();

        let opponent_hand = opponent.try_into()?;
        let my_hand = me.try_into()?;

        Ok(Part1Round {
            opponent_hand,
            my_hand,
        })
    }
}

impl Part1Round {
    fn outcome(&self) -> Outcome {
        if self.opponent_hand.beats(&self.my_hand) {
            Outcome::Lose
        } else if self.my_hand.beats(&self.opponent_hand) {
            Outcome::Win
        } else {
            Outcome::Draw
        }
    }

    fn score(&self) -> i32 {
        self.my_hand.score() + self.outcome().score()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Part2Round {
    opponent_hand: Hand,
    expected_outcome: Outcome,
}

impl FromStr for Part2Round {
    type Err = Box<dyn std::error::Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (opponent, _space, me) = value.chars().take(3).collect_tuple().unwrap();

        let opponent_hand = opponent.try_into()?;
        let expected_outcome = me.try_into()?;

        Ok(Part2Round {
            opponent_hand,
            expected_outcome,
        })
    }
}

impl Part2Round {
    fn score(&self) -> i32 {
        let my_hand = match (&self.opponent_hand, &self.expected_outcome) {
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Rock, Outcome::Lose) => Hand::Scissors,
            (Hand::Rock, Outcome::Draw) => Hand::Rock,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Paper, Outcome::Lose) => Hand::Rock,
            (Hand::Paper, Outcome::Draw) => Hand::Paper,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Scissors, Outcome::Lose) => Hand::Paper,
            (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
        };
        my_hand.score() + self.expected_outcome.score()
    }
}
