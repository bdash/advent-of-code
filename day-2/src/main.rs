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
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Unknown selection".into()),
        }
    }
}

enum Winner {
    Us,
    Them,
    Draw,
}

impl Winner {
    fn new(them: Selection, us: Selection) -> Winner {
        use Selection::*;

        match (us, them) {
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Winner::Draw,
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Winner::Us,
            _ => Winner::Them,
        }
    }

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
        .map(str::parse)
        .map(Result::unwrap)
        .tuples()
        .map(|(them, us)| Winner::new(them, us).score() + us.score())
        .sum();

    println!("Score after all rounds was {score}");

    Ok(())
}
