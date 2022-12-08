fn update_visible_trees_in_sequence<'a>(
    trees: impl Iterator<Item = &'a u8>,
    visible: impl Iterator<Item = &'a mut bool>,
) {
    let mut last = None;
    for (&item, visible) in trees.zip(visible) {
        if last.is_none() || last.unwrap() < item {
            last = Some(item);
            *visible = true;
        }
    }
}

#[cfg(test)]
fn visible_trees_in_sequence<'a>(trees: &[u8]) -> Vec<bool> {
    let mut result = vec![false; trees.len()];
    update_visible_trees_in_sequence(trees.iter(), result.iter_mut());
    result
}

#[derive(Default, Debug)]
struct Grid {
    width: usize,
    height: usize,
    tree_heights: Vec<u8>,
}

fn parse(input: &str) -> Grid {
    let width = input.find("\n").unwrap();
    let tree_heights: Vec<_> = input
        .as_bytes()
        .into_iter()
        .filter(|b| **b != ('\n' as u8))
        .map(|b| (b - '0' as u8) as u8)
        .collect();
    let height = tree_heights.len() / width;
    Grid {
        width,
        height,
        tree_heights,
    }
}

fn count_trees_visible_from_perimeter(grid: &Grid) -> usize {
    let mut visible_trees = vec![false; grid.width * grid.height];
    for row in 0..grid.height {
        let row_start = row * grid.width;
        let range = row_start..(row_start + grid.width);
        // Rows from left
        update_visible_trees_in_sequence(
            grid.tree_heights[range.clone()].iter(),
            visible_trees[range.clone()].iter_mut(),
        );
        // Rows from right
        update_visible_trees_in_sequence(
            grid.tree_heights[range.clone()].iter().rev(),
            visible_trees[range].iter_mut().rev(),
        );
    }
    for col in 0..grid.width {
        let range = col..;
        // Columns from top
        update_visible_trees_in_sequence(
            grid.tree_heights[range.clone()].iter().step_by(grid.height),
            visible_trees[range.clone()].iter_mut().step_by(grid.height),
        );
        // Columns from bottom
        update_visible_trees_in_sequence(
            grid.tree_heights[range.clone()]
                .iter()
                .step_by(grid.height)
                .rev(),
            visible_trees[range.clone()]
                .iter_mut()
                .step_by(grid.height)
                .rev(),
        );
    }

    visible_trees.iter().filter(|visible| **visible).count()
}

fn part_1(input: &str) {
    let grid = parse(&input);
    let result = count_trees_visible_from_perimeter(&grid);
    println!("There are {} trees visible from the perimeter.", result);
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;
    part_1(&input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::visible_trees_in_sequence;

    #[test]
    fn test_number_of_visible_trees() {
        assert_eq!(
            visible_trees_in_sequence(&[3 as u8, 0, 3, 7, 3]),
            [true, false, false, true, false]
        );
        assert_eq!(
            visible_trees_in_sequence(&[2 as u8, 5, 5, 1, 2]),
            [true, true, false, false, false]
        );
        assert_eq!(
            visible_trees_in_sequence(&[6 as u8, 5, 3, 3, 2]),
            [true, false, false, false, false]
        );
        assert_eq!(
            visible_trees_in_sequence(&[3 as u8, 3, 5, 4, 9]),
            [true, false, true, false, true]
        );
        assert_eq!(
            visible_trees_in_sequence(&[3 as u8, 5, 3, 9, 0]),
            [true, true, false, true, false]
        );
    }
}
