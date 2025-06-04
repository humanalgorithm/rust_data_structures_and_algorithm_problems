use std::collections::HashSet;

pub fn dfs(
    s_char: &Vec<char>,
    curr_word: &mut Vec<char>,
    word_vecs: &mut Vec<Vec<char>>,
    visited: &mut HashSet<Vec<char>>,
) -> bool {
    if visited.contains(curr_word) {
        return false;
    }
    visited.insert(curr_word.to_vec());
    if curr_word == s_char {
        return true;
    }

    for x in 0..word_vecs.len() {
        let word = word_vecs[x].clone();
        if curr_word.len() + word.len() <= s_char.len()
            && s_char[curr_word.len()..curr_word.len() + word.len()] == word[0..word.len()]
        {
            let mut new_curr = [curr_word.to_vec(), word.to_vec()].concat();
            let res = dfs(s_char, &mut new_curr, word_vecs, visited);
            if res {
                return true;
            }
        }
    }
    return false;
}

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let s_char: Vec<char> = s.chars().collect();
    let mut word_vecs: Vec<Vec<char>> = Vec::new();
    let mut visited: HashSet<Vec<char>> = HashSet::new();
    for word in word_dict {
        word_vecs.push(word.chars().collect());
    }
    return dfs(&s_char, &mut vec![], &mut word_vecs, &mut visited);
}
