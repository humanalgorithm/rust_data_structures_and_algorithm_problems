use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
    let mut word_list = word_list;
    let mut neighbor_map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    word_list.push(begin_word.clone());

    for word in word_list {
        for x in 0..word.len() {
            let mut word_chars: Vec<char> = word.clone().chars().collect();
            word_chars[x] = '*';
            neighbor_map
                .entry(word_chars.clone())
                .and_modify(|m| m.push(word.clone()))
                .or_insert([word.clone()].to_vec());
        }
    }

    let mut q: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut res: i32 = 1;
    q.push_front(begin_word);

    while !q.is_empty() {
        for _ in 0..q.len() {
            let curr_word = q.pop_front().unwrap();
            if curr_word == end_word {
                return res;
            }
            for x in 0..curr_word.len() {
                let mut word_chars: Vec<char> = curr_word.chars().collect();
                word_chars[x] = '*';
                let neighbors = neighbor_map.get(&word_chars).unwrap();
                for neighbor in neighbors {
                    if visited.get(neighbor).is_some() {
                        continue;
                    }
                    visited.insert(neighbor.to_string());
                    q.push_back(neighbor.to_string());
                }
            }
        }
        res += 1;
    }
    return 0;
}
