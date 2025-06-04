use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

    for item in strs {
        let mut item_chars: Vec<char> = item.chars().collect();
        item_chars.sort();
        map.entry(item_chars)
            .and_modify(|vec| vec.push(item.clone()))
            .or_insert(vec![item]);
    }
    let ret_value: Vec<Vec<String>> = map.iter().map(|m| m.1.to_vec()).collect();
    return ret_value;
}
