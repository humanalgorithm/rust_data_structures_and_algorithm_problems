use std::collections::HashSet;

pub fn dfs(
    board: &Vec<Vec<char>>,
    word: &Vec<char>,
    index: usize,
    current: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> bool {
    if index == word.len() {
        return true;
    }
    if visited.get(&current).is_some() {
        return false;
    }
    if board[current.0][current.1] != word[index] {
        return false;
    }

    let mut left = false;
    let mut down = false;
    let mut right = false;
    let mut up = false;

    visited.insert(current);

    //go right
    if current.1 < board[0].len() - 1 {
        let next = (current.0, current.1 + 1);
        right = dfs(board, word, index + 1, next, visited);
    }
    //go down
    if current.0 < board.len() - 1 {
        let next = (current.0 + 1, current.1);
        down = dfs(board, word, index + 1, next, visited);
    }

    //go left
    if current.1 > 0 {
        let next = (current.0, current.1 - 1);
        left = dfs(board, word, index + 1, next, visited);
    }
    //go up
    if current.0 > 0 {
        let next = (current.0 - 1, current.1);
        up = dfs(board, word, index + 1, next, visited);
    }
    visited.remove(&current);
    return right || down || left || up;
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let word: Vec<char> = word.chars().collect();

    if board.len() == 1 && board[0].len() == 1 {
        return board[0] == word;
    }
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if dfs(&board, &word, 0, (i, j), &mut HashSet::new()) {
                return true;
            }
        }
    }
    return false;
}
