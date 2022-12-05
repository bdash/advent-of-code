use lazy_static::lazy_static;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug)]
struct State(Vec<Vec<char>>);

impl State {
    fn apply(&mut self, moves: &Vec<Move>) {
        for m in moves {
            self.apply_move(&m);
        }
    }

    fn apply_move(&mut self, m: &Move) {
        let range = (self.0[m.source].len() - m.number)..;
        let mut moving: Vec<_> = self.0[m.source].drain(range).rev().collect();
        self.0[m.dest].append(&mut moving);
    }

    fn summary(&self) -> String {
        String::from_iter(self.0.iter().map(|stack| stack.last().unwrap().to_string()))
    }
}

impl FromStr for State {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().rev();
        // Final line contains only indices. Use it to determine how many stacks we need.
        let stack_count = lines.next().unwrap().split_whitespace().count();
        let stacks  = lines
            .map(|line| {
                line.as_bytes().chunks(4).map(|chunk| match *chunk {
                    // [<letter>]<optional space>
                    [91, letter, 93, ..] => Some(letter as char),
                    _ => None,
                })
            })
            .fold(vec![vec![]; stack_count], |mut stacks, items| {
                for (idx, item) in items.enumerate() {
                    if let Some(item) = item {
                        stacks[idx].push(item);
                    }
                }
                stacks
            });

        Ok(State(stacks))
    }
}

struct Move {
    number: usize,
    source: usize,
    dest: usize,
}

impl FromStr for Move {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let captures = RE.captures(s).ok_or("Failed to match pattern")?;
        let capture = |idx, name: &str| -> Result<usize, Self::Err> {
            Ok(captures
                .get(idx)
                .ok_or_else(|| format!("Failed to find {} in line", name))?
                .as_str()
                .parse::<usize>()?)
        };

        Ok(Move {
            number: capture(1, "number")?,
            source: capture(2, "source")? - 1,
            dest: capture(3, "dest")? - 1,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input")?;

    let (initial_state, instructions) = input.split_once("\n\n").unwrap();
    let mut state: State = initial_state.parse()?;
    let moves = instructions
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    state.apply(&moves);

    println!(
        "Top of the stack after applying the move instructions is: {}",
        state.summary(),
    );

    Ok(())
}
