use std::ops::Range;

trait ContainsRange {
    fn contains_range(&self, range: &Self) -> bool;
}

impl<T: PartialOrd> ContainsRange for Range<T> {
    fn contains_range(&self, range: &Range<T>) -> bool {
        self.start <= range.start && self.end >= range.end
    }
}

fn parse_range(range: &str) -> Range<usize> {
    let (start, end) = range.split_once("-").unwrap();
    let (start, end) = (
        start.parse::<usize>().unwrap(),
        // Half-open range
        end.parse::<usize>().unwrap() + 1,
    );
    Range { start, end }
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;

    let result = input
        .split_ascii_whitespace()
        .map(|line| line.split_once(",").unwrap())
        .map(|(first, second)| (parse_range(first), parse_range(second)))
        .filter(|(first, second)| first.contains_range(second) || second.contains_range(first));

    println!(
        "{} pairs have one assignment that completely contains the other.",
        result.count()
    );

    Ok(())
}
