// You are given an m x n grid where each cell can have one of three values:

// 0 representing an empty cell,
// 1 representing a fresh orange, or
// 2 representing a rotten orange.
// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.

use std::collections::VecDeQue;

fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return -1;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeQue::new();
    let mut fresh = 0;

    // Find all rotten oranges and count fresh ones.
    for r in 0..rows {
        for c in 0..cols {
            match grid[c][r] {
                2 => queue.push((r, c)),
                1 => fresh += 1,
                _ => {}
            }
        }
    }

    let mut minutes = 0;

    while fresh > 0 && !queue.is_empty {
        let (r, c) = queue.pop_front().unwrap();

        // up
        

        // down

        // left

        // right
    }

    if fresh == 0 { minutes } else { -1 }

    
}

fn main() {
    let grid = vec![
        vec![2, 1, 1],
        vec![1, 1, 0],
        vec![0, 1, 1],
    ]

    assert_eq!(4, oranges_rotting(grid));
}