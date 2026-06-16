// Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.
//An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

// TIP: turn 1s into 0s to mark visited and save memory

fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return 0;
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut islands = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                islands += 1;
                dfs(r, c, &mut grid)
            }
        }
    }

    islands
}

fn dfs(start_row: usize, start_col: usize, grid: &mut Vec<Vec<char>>) {
    let mut stack = vec![(start_row, start_col)];

    while let Some((r, c)) = stack.pop() {
        if grid[r][c] != '1' {
            continue;
        }

        grid[r][c] = '0';

        // up
        if r > 0 {
            stack.push((r - 1, c));
        }

        // down
        if r + 1 < grid.len() {
            stack.push((r + 1, c));
        }

        // left
        if c > 0 {
            stack.push((r, c - 1));
        }

        // right
        if c + 1 < grid[0].len() {
            stack.push((r, c + 1));
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
