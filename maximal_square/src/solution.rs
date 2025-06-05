use std::cmp;
use std::collections::HashMap;

pub fn dfs(
    matrix: &Vec<Vec<char>>,
    cache: &mut HashMap<(usize, usize), i32>,
    i: usize,
    j: usize,
    max_square: &mut Vec<i32>,
) -> i32 {
    if i >= matrix.len() || j >= matrix[0].len() {
        return 0;
    }
    if cache.get(&(i, j)).is_some() {
        return *cache.get(&(i, j)).unwrap();
    } else {
        cache.insert((i, j), 0);
        let right = dfs(matrix, cache, i, j + 1, max_square);
        let down = dfs(matrix, cache, i + 1, j, max_square);
        let diag = dfs(matrix, cache, i + 1, j + 1, max_square);

        let min = cmp::min(cmp::min(right, down), diag);
        if matrix[i][j] == '1' {
            cache.insert((i, j), min + 1);
        }
    }
    let cache_value = *cache.get(&(i, j)).unwrap();
    max_square[0] = cmp::max(cache_value, max_square[0]);
    return cache_value;
}

pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut cache: HashMap<(usize, usize), i32> = HashMap::new();
    let mut max_square: Vec<i32> = Vec::from([0]);

    dfs(&matrix, &mut cache, 0, 0, &mut max_square);
    return max_square[0] * max_square[0];
}
