// You are given an m x n grid where each cell can have one of three values:

// 0 representing an empty cell,
// 1 representing a fresh orange, or
// 2 representing a rotten orange.
// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.

use std::collections::VecDeque;

fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return -1;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeque::new();
    let mut fresh = 0;

    // Find all rotten oranges and count fresh ones.
    for r in 0..rows {
        for c in 0..cols {
            match grid[r][c] {
                2 => queue.push_back((r, c)),
                1 => fresh += 1,
                _ => {}
            }
        }
    }

    let mut minutes = 0;

    while fresh > 0 && !queue.is_empty() {
        let level_size = queue.len();

        for _ in 0..level_size {
            let (r, c) = queue.pop_front().unwrap();

            // up
            if r >= 1 && grid[r-1][c] == 1 {
                grid[r-1][c] = 2;
                queue.push_back((r-1, c));
                fresh -= 1;
            }


            // down
            if r + 1 <= rows - 1 && grid[r+1][c] == 1 {
                grid[r+1][c] = 2;
                queue.push_back((r+1, c));
                fresh -= 1;
            }

            // left
            if c >= 1 && grid[r][c-1] == 1 {
                grid[r][c-1] = 2;
                queue.push_back((r, c-1));
                fresh -= 1;
            }

            // right
            if c + 1 <= cols -1 && grid[r][c+1] == 1 {
                grid[r][c+1] = 2;
                queue.push_back((r, c+1));
                fresh -= 1;
            }
        }

        minutes += 1;
    }

    if fresh == 0 { minutes } else { -1 }

    
}

fn main() {
    let grid = vec![
        vec![2, 1, 1],
        vec![1, 1, 0],
        vec![0, 1, 1],
    ];

    assert_eq!(4, oranges_rotting(grid));
}