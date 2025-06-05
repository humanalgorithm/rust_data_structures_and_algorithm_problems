use std::collections::HashSet;

pub fn search(
    matrix: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    target: i32,
    visited: &mut HashSet<(usize, usize)>,
) -> bool {
    if i >= matrix.len() || j >= matrix[0].len() {
        return false;
    }
    if matrix[i][j] == target {
        return true;
    }
    if visited.contains(&(i, j)) {
        return false;
    }
    if matrix[i][j] > target {
        return false;
    }
    visited.insert((i, j));
    if search(matrix, i + 1, j, target, visited) {
        return true;
    }
    if search(matrix, i, j + 1, target, visited) {
        return true;
    }
    return false;
}
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let res = search(&matrix, 0, 0, target, &mut visited);
    return res;
}
