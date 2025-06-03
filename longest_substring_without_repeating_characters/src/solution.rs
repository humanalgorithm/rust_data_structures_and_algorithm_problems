use std::cmp;
use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut longest_len: i32 = 0;
    let str_len = chars.len();

    if str_len == 0 {
        return 0;
    }
    for i in 0..str_len {
        let mut char_set: HashSet<char> = HashSet::new();
        char_set.insert(chars[i]);
        longest_len = if longest_len == 0 { 1 } else { longest_len };
        for j in i + 1..str_len {
            if char_set.get(&chars[j]).is_some() {
                break;
            }
            longest_len = cmp::max(j as i32 - i as i32 + 1, longest_len);
            char_set.insert(chars[j]);
        }
    }
    longest_len
}
