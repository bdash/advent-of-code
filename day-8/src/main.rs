use itertools::iproduct;

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

fn score_view<'a>(current: u8, trees: impl Iterator<Item = &'a u8> + Clone) -> usize {
    trees
        .clone()
        .enumerate()
        .find(|(_, height)| **height >= current)
        .map_or_else(
            || {
                // We did not find a tree of height `current` or taller. We can see all of the trees.
                trees.count()
            },
            |(idx, _)| {
                // Convert 0-based index into a 1-based count.
                idx + 1
            },
        )
}

fn optimal_viewing_score_for_grid(grid: &Grid) -> usize {
    let mut best_score = 0;
    for (column, row) in iproduct!(0..grid.width, 0..grid.height) {
        let start_of_row = row * grid.width;
        let position = start_of_row + column;
        let left = grid.tree_heights[start_of_row..position]
            .iter()
            .rev();
        let right = grid.tree_heights[position..(start_of_row + grid.width)]
            .iter()
            .skip(1);
        let up = grid.tree_heights[column..position]
            .iter()
            .step_by(grid.width)
            .rev();
        let down = grid.tree_heights[position..]
            .iter()
            .step_by(grid.width)
            .skip(1);

        let current = grid.tree_heights[position];
        let score = score_view(current, left)
            * score_view(current, right)
            * score_view(current, up)
            * score_view(current, down);
        if score > best_score {
            best_score = score;
        }
    }
    best_score
}

fn part_1(input: &str) {
    let grid = parse(&input);
    let result = count_trees_visible_from_perimeter(&grid);
    println!("There are {} trees visible from the perimeter.", result);
}

fn part_2(input: &str) {
    let grid = parse(&input);
    let result = optimal_viewing_score_for_grid(&grid);
    println!("The optimal viewing score found was {}.", result);
}

fn main() -> Result<(), std::io::Error> {
    let input = std::fs::read_to_string("input")?;
    part_1(&input);
    part_2(&input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{score_view, visible_trees_in_sequence};

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

    #[test]
    fn test_score_view() {
        assert_eq!(score_view(5, [3].iter()), 1);
        assert_eq!(score_view(5, [5, 2].iter()), 1);
        assert_eq!(score_view(5, [1, 2].iter()), 2);
        assert_eq!(score_view(5, [3, 5, 3].iter()), 2);
    }
}
