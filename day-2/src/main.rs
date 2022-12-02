use std::str::FromStr;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Selection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            _ => Err("Unknown selection".into()),
        }
    }
}

enum Winner {
    Us,
    Them,
    Draw,
}

impl FromStr for Winner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Them),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Us),
            _ => Err("Unknown winner".into()),
        }
    }
}

impl Winner {
    #[allow(unused)]
    fn new(them: Selection, us: Selection) -> Winner {
        use Selection::*;

        match (us, them) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Winner::Draw,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Winner::Us,
            _ => Winner::Them,
        }
    }

    fn our_move(&self, them: Selection) -> Selection {
        use Selection::*;
        use Winner::*;

        match (self, them) {
            (Us, Scissors) | (Draw, Rock) | (Them, Paper) => Rock,
            (Us, Paper) | (Draw, Scissors) | (Them, Rock) => Scissors,
            _ => Paper,
        }
    }

    #[allow(unused)]
    fn score(&self) -> u32 {
        match self {
            Self::Us => 6,
            Self::Draw => 3,
            Self::Them => 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?;

    let score: u32 = input
        .split_ascii_whitespace()
        .tuples()
        .map(|(them, winner)| (them.parse().unwrap(), winner.parse::<Winner>().unwrap()))
        .map(|(them, winner)| winner.score() + winner.our_move(them).score())
        .sum();

    println!("Score after all rounds was {score}");

    Ok(())
}
