use std::iter::Peekable;

fn evaluate(input: &str) -> Vec<usize> {
    let mut lines = input.lines().peekable();

    let mut directory_sizes = vec![];
    visit_directory(&mut lines, &mut directory_sizes);

    directory_sizes.sort();
    directory_sizes
}

fn visit_directory<'a>(
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
    sizes: &mut Vec<usize>,
) -> usize {
    let mut size = 0;
    while let Some(line) = lines.next() {
        match &line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", ".."] => {
                sizes.push(size);
                return size;
            }
            ["$", "cd", _path] => {
                size += visit_directory(lines, sizes);
            }
            ["$", "ls"] => {
                while let Some(line) = lines.peek() {
                    match &line.split(" ").collect::<Vec<_>>()[..] {
                        ["dir", _path] => (),
                        [file_size, _path] => size += file_size.parse::<usize>().unwrap(),
                        ["$", ..] => break,
                        _ => unreachable!(),
                    }
                    lines.next();
                }
            }
            _ => unreachable!(),
        };
    }

    sizes.push(size);
    size
}

fn part_1(input: &str) {
    let directory_sizes = evaluate(input);

    const SMALL_DIRECTORY_THRESHOLD: usize = 100000;
    let total: usize = directory_sizes
        .into_iter()
        .take_while(|size| *size < SMALL_DIRECTORY_THRESHOLD)
        .sum();

    println!(
        "Total size of directories smaller than {} bytes is {} bytes",
        SMALL_DIRECTORY_THRESHOLD, total
    );
}

fn part_2(input: &str) {
    let directory_sizes = evaluate(input);

    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_FREE_DISK_SPACE: usize = 30000000;

    let total_used_disk_space = directory_sizes.last().unwrap();
    let disk_space_to_free = REQUIRED_FREE_DISK_SPACE - (TOTAL_DISK_SPACE - total_used_disk_space);
    let idx = directory_sizes
        .binary_search(&disk_space_to_free)
        .unwrap_or_else(|e| e);
    let smallest_directory_size_to_remove = directory_sizes[idx];

    println!("The smallest directory that must be removed to bring the total free space above {} bytes is {} bytes in size.", REQUIRED_FREE_DISK_SPACE, smallest_directory_size_to_remove);
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;
    part_1(&input);
    part_2(&input);
    Ok(())
}
