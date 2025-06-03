use std::collections::HashMap;
use std::collections::HashSet;

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];
    let str_chars: Vec<char> = s.chars().collect();
    let open: HashSet<char> = HashSet::from([('('), ('['), ('{')]);
    let closing = HashMap::from([(')', '('), (']', '['), ('}', '{')]);
    for curr_char in str_chars {
        if open.contains(&curr_char) {
            stack.push(curr_char.clone());
            continue;
        }
        let top = match stack.pop() {
            Some(value) => value,
            None => return false,
        };
        if closing.get(&curr_char).unwrap() == &top {
            continue;
        } else {
            return false;
        }
    }
    if stack.is_empty() {
        return true;
    }
    false
}
