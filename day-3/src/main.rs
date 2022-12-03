use itertools::Itertools;

type HashSet<T> = std::collections::HashSet<T>;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;

    let result: u32 = input
        .split_ascii_whitespace()
        .tuples()
        .map(|(first, second, third)| {
            (
                HashSet::from_iter(first.chars()),
                HashSet::from_iter(second.chars()),
                HashSet::from_iter(third.chars()),
            )
        })
        .map(|(first, second, third)| {
            (
                first.intersection(&second).cloned().collect::<HashSet<_>>(),
                third,
            )
        })
        .map(|(first, second)| first.intersection(&second).next().unwrap().clone())
        .map(|dupe| match dupe {
            'a'..='z' => 1 + ((dupe as u32) - ('a' as u32)),
            'A'..='Z' => 27 + ((dupe as u32) - ('A' as u32)),
            _ => unreachable!(),
        })
        .sum();

    println!("Sum of priorities of badge types is {result}");
    Ok(())
}
