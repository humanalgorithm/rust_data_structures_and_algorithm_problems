use std::collections::HashSet;

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut directions: Vec<(i32, i32)> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    directions.push((0, 1));
    directions.push((1, 0));
    directions.push((0, -1));
    directions.push((-1, 0));

    let (mut row, mut col, row_len, col_len) = (0, 0, matrix.len(), matrix[0].len());
    let (mut direction_ctr, mut direction) = (0, directions[0]);
    let (mut output, total_count) = (vec![], row_len * col_len);

    while visited.len() < total_count {
        if visited.get(&(row, col)).is_some() || row >= row_len || col >= col_len {
            row = (row as i32 - direction.0) as usize;
            col = (col as i32 - direction.1) as usize;
            direction_ctr += 1;
            direction = directions[direction_ctr % directions.len()];
            row = (row as i32 + direction.0) as usize;
            col = (col as i32 + direction.1) as usize;
            continue;
        }
        visited.insert((row, col));
        output.push(matrix[row][col]);
        row = (row as i32 + direction.0) as usize;
        col = (col as i32 + direction.1) as usize;
    }
    return output;
}
