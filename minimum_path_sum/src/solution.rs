use std::cmp;

pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (row_len, col_len) = (grid.len(), grid[0].len());
    let mut side_matrix: Vec<Vec<i32>> = vec![vec![]; row_len];

    let mut start_col = 1;
    side_matrix[0].push(grid[0][0]);

    for i in 0..row_len {
        for j in start_col..col_len {
            let left_val = if j > 0 {
                side_matrix[i][j - 1]
            } else {
                i32::MAX
            };
            let above_val = if i > 0 {
                side_matrix[i - 1][j]
            } else {
                i32::MAX
            };

            let choice = cmp::min(left_val, above_val);
            side_matrix[i].push(grid[i][j] + choice);
        }
        start_col = 0;
    }
    return side_matrix[row_len - 1][col_len - 1];
}
