fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;

    let mut elves: Vec<_> = input
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        })
        .collect();

    elves.sort();
    elves.reverse();
    let calories: u32 = elves.iter().take(3).sum();
    println!("The three elves carying the most calories are carrying a total of {} calories.", calories);

    Ok(())
}
