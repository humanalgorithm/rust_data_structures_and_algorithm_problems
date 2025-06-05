use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s2.len() < s1.len() {
        return false;
    }
    let mut s1_map: HashMap<char, i32> = HashMap::new();
    let mut s2_map: HashMap<char, i32> = HashMap::new();
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for letter in s1_chars.iter() {
        s1_map
            .entry(*letter)
            .and_modify(|m| *m = *m + 1)
            .or_insert(1);
    }
    for x in 0..s1_chars.len() {
        let letter = s2_chars[x];
        s2_map
            .entry(letter)
            .and_modify(|m| *m = *m + 1)
            .or_insert(1);
    }
    if s1_map == s2_map {
        return true;
    }
    let mut left = 0;
    for right in s1_chars.len()..s2_chars.len() {
        s2_map.entry(s2_chars[left]).and_modify(|m| *m = *m - 1);
        if s2_map[&s2_chars[left]] == 0 {
            s2_map.remove(&s2_chars[left]);
        }
        left += 1;
        s2_map
            .entry(s2_chars[right])
            .and_modify(|m| *m = *m + 1)
            .or_insert(1);
        if s1_map == s2_map {
            return true;
        }
    }
    return false;
}
