use itertools::Itertools;

 const LENGTH: usize = 4;

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;
    let first = input
        .as_bytes()
        .windows(LENGTH)
        .enumerate()
        .filter(|(_, window)| window.into_iter().unique().count() == window.len())
        .next()
        .expect("Failed to find marker");
    println!(
        "First start-of-packet marker appears {} characters into stream.",
        first.0 + first.1.len()
    );
    Ok(())
}
