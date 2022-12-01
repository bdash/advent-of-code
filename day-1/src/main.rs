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
    println!("The elf with the most calories is carrying {} calories.", elves.last().unwrap());

    Ok(())
}
