// Given an m x n 2D binary grid grid which represents a map of '1's (land) 
// and '0's (water), return the number of islands.
// An island is surrounded by water and is formed by connecting adjacent 
// lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

// TIP: turn 1s into 0s to mark visited and save memory

fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let mut num_of_islands = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                num_of_islands += 1;
                dfs(r, c, &mut grid);
            }
        }
    }

    num_of_islands
    
}

fn dfs(r_start: usize, c_start: usize, mut grid: &mut Vec<Vec<char>>) {
    let mut stack = vec![(r_start, c_start)];

    while let Some((r, c)) = stack.pop() {
        if grid[r][c] != '1' {
            continue;
        }

        grid[r][c] = '0';

        // Up
        if r > 0 {
            dfs(r-1, c, &mut grid);
        }

        // Down
        if r + 1 < grid.len() {
            dfs(r+1, c, &mut grid);
        }

        // Left
        if c > 0 {
            dfs(r, c- 1, &mut grid);
        }

        // Right
        if c + 1 < grid[0].len() {
            dfs(r, c+1, &mut grid);
        }
    }
}

fn main() {
    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    let num_of_islands = num_islands(grid);
    println!("{}", num_of_islands);
}