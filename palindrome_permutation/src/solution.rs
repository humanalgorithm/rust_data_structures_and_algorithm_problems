use std::collections::HashMap;

pub fn can_permute_palindrome(s: String) -> bool {
    let chars_s: Vec<char> = s.chars().collect();
    let mut map: HashMap<char, i32> = HashMap::new();

    for letter in chars_s {
        map.entry(letter).and_modify(|m| *m = *m + 1).or_insert(1);
    }
    let mut odd_count_chars = 0;
    for key in map.keys() {
        let value = map.get(&key).unwrap();
        if !(value % 2 == 0) {
            odd_count_chars += 1;
        }
    }
    return odd_count_chars <= 1;
}
