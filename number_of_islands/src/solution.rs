use std::collections::HashSet;

pub fn bfs(grid: &Vec<Vec<char>>, i: usize, j: usize, visited: &mut HashSet<(usize, usize)>) {
    if i > grid.len() || j > grid[0].len() {
        visited.insert((i, j));
        return;
    }
    if visited.contains(&(i, j)) {
        return;
    }

    visited.insert((i, j));
    // go left
    if j > 0 && grid[i][j - 1] == '1' {
        bfs(grid, i, j - 1, visited);
    }
    // go right
    if j < grid[0].len() - 1 && grid[i][j + 1] == '1' {
        bfs(grid, i, j + 1, visited);
    }
    // go up
    if i < grid.len() - 1 && grid[i + 1][j] == '1' {
        bfs(grid, i + 1, j, visited);
    }
    // go down
    if i > 0 && grid[i - 1][j] == '1' {
        bfs(grid, i - 1, j, visited);
    }
}
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut islands: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '1' && !visited.contains(&(i, j)) {
                bfs(&grid, i, j, &mut visited);
                islands += 1;
            }
        }
    }
    return islands;
}
