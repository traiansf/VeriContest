#![doc = include_str!("description.md")]

pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
        let rows = strs.len();
        let cols = grid[0].len();
        let mut deleted = 0;
        let mut col: usize = 0;

        while col < cols {
            let mut bad = false;
            let mut row: usize = 1;
            while row < rows {
                if grid[row - 1][col] > grid[row][col] {
                    bad = true;
                }
                row += 1;
            }
            if bad {
                deleted += 1;
            }
            col += 1;
        }

        deleted
    }
}
