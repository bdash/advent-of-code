use std::ops::Range;

trait ContainsRange {
    fn contains_range(&self, range: &Self) -> bool;
}

trait OverlapsRange {
    fn overlaps_range(&self, range: &Self) -> bool;
}

impl<T: PartialOrd> ContainsRange for Range<T> {
    fn contains_range(&self, range: &Range<T>) -> bool {
        self.start <= range.start && self.end >= range.end
    }
}

impl<T: PartialOrd + core::fmt::Debug> OverlapsRange for Range<T> {
    fn overlaps_range(&self, range: &Range<T>) -> bool {
        self.contains(&range.start) || range.contains(&self.start)
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
        .filter(|(first, second)| first.overlaps_range(second) || second.overlaps_range(first));

    println!("{} pairs have assignments that overlap.", result.count());

    Ok(())
}
