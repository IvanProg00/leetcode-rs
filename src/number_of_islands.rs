//! [Number of Islands](https://leetcode.com/problems/number-of-islands)

use std::collections::{HashSet, VecDeque};

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut islands = 0;

    let mut visited = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'1' && !visited.contains(&(i, j)) {
                islands += 1;

                let mut steps = VecDeque::new();
                steps.push_front((i, j));

                while let Some(pos) = steps.pop_back() {
                    island_bfs(&grid, &mut visited, &mut steps, pos);
                }
            }
        }
    }

    islands
}

fn island_bfs(
    grid: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    steps: &mut VecDeque<(usize, usize)>,
    pos: (usize, usize),
) {
    if pos.0 >= grid.len()
        || pos.1 >= grid[0].len()
        || visited.contains(&pos)
        || grid[pos.0][pos.1] == '0'
    {
        return;
    }

    visited.insert((pos.0, pos.1));

    steps.push_front((pos.0 + 1, pos.1));
    if pos.0 > 0 {
        steps.push_front((pos.0 - 1, pos.1));
    }
    steps.push_front((pos.0, pos.1 + 1));
    if pos.1 > 0 {
        steps.push_front((pos.0, pos.1 - 1));
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0']
    ], 1)]
    #[case(vec![
        vec!['1','1','0','0','0'],
        vec!['1','1','0','0','0'],
        vec!['0','0','1','0','0'],
        vec!['0','0','0','1','1']
    ], 3)]
    fn test_num_islands(#[case] grid: Vec<Vec<char>>, #[case] expected: i32) {
        let result = num_islands(grid);
        assert_eq!(result, expected);
    }
}
