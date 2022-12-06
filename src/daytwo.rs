use std::cmp::Ordering;

use crate::filereader;

#[derive(PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Some(Ordering::Less),
            (Hand::Rock, Hand::Scissors) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Rock) => Some(Ordering::Greater),
            (Hand::Paper, Hand::Scissors) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Rock) => Some(Ordering::Less),
            (Hand::Scissors, Hand::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn from_string(s: &str) -> Hand {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Invalid hand"),
        }
    }

    fn loser(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }

    fn winner(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn points(&self) -> u32 {
        match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }

    fn from_string(s: &str) -> GameResult {
        match s {
            "Z" => GameResult::Win,
            "X" => GameResult::Lose,
            "Y" => GameResult::Draw,
            _ => panic!("Invalid game result"),
        }
    }

    fn play(hand1: &Hand, hand2: &Hand) -> GameResult {
        if *hand1 < *hand2 {
            GameResult::Win
        } else if *hand1 > *hand2 {
            GameResult::Lose
        } else {
            GameResult::Draw
        }
    }
}

fn parse_file() -> Vec<(String, String)> {
    let mut vec: Vec<_> = Vec::new();

    for line in filereader::lines(2) {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split_whitespace();

        let col1 = split.next().unwrap().to_owned();
        let col2 = split.next().unwrap().to_owned();

        vec.push((col1, col2));
    }

    vec
}

pub fn day_2_1() -> u32 {
    parse_file()
        .iter()
        .map(|(hand1, hand2)| {
            let hand1 = Hand::from_string(hand1);
            let hand2 = Hand::from_string(hand2);

            let result = GameResult::play(&hand1, &hand2);

            result.points() + hand2.points()
        })
        .sum::<u32>()
}

pub fn day_2_2() -> u32 {
    parse_file()
        .iter()
        .map(|(hand1, res)| {
            let hand1 = Hand::from_string(hand1);
            let res = GameResult::from_string(res);

            let hand2 = match res {
                GameResult::Win => hand1.winner(),
                GameResult::Lose => hand1.loser(),
                GameResult::Draw => hand1,
            };

            hand2.points() + res.points()
        })
        .sum::<u32>()
}
