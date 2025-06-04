use std::cmp;

pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1_len = word1.len() + 1;
    let word2_len = word2.len() + 1;
    let mut matrix = vec![vec![i32::MAX; word2_len]; word1_len];
    let word1: Vec<char> = word1.chars().collect();
    let word2: Vec<char> = word2.chars().collect();

    for j in (0..word2.len() + 1).rev() {
        matrix[word1.len()][j] = (word2.len() - j) as i32;
    }
    for i in (0..word1.len() + 1).rev() {
        matrix[i][word2.len()] = (word1.len() - i) as i32;
    }

    // replace: i+1, j+1
    // delete: i+1, j
    // insert: i, j+1
    for i in (0..word1.len()).rev() {
        for j in (0..word2.len()).rev() {
            if word1[i] == word2[j] {
                matrix[i][j] = matrix[i + 1][j + 1];
            } else {
                let min1 = cmp::min(matrix[i + 1][j], matrix[i][j + 1]);
                let min2 = cmp::min(min1, matrix[i + 1][j + 1]);
                matrix[i][j] = 1 + min2;
                if min2 as usize >= cmp::max(word1.len(), word2.len()) {
                    return min2;
                }
            }
        }
    }
    return matrix[0][0];
}
