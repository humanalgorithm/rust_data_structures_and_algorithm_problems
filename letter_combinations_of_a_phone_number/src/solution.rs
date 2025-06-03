use std::collections::HashMap;
pub fn dfs(
    digits: &Vec<char>,
    map: &HashMap<char, Vec<char>>,
    index: i32,
    path: String,
    res: &mut Vec<String>,
) {
    if index >= digits.len() as i32 {
        res.push(path);
        return;
    }
    let letters = map.get(&digits[index as usize]).unwrap();
    for letter in letters {
        let new_path = format!("{}{}", path, letter);
        dfs(&digits, &map, index + 1, new_path, res.as_mut());
    }
}

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.len() == 0 {
        return vec![];
    }
    let digits = digits.chars().collect();
    let map: HashMap<char, Vec<char>> = HashMap::from([
        ('2', vec!['a', 'b', 'c']),
        ('3', vec!['d', 'e', 'f']),
        ('4', vec!['g', 'h', 'i']),
        ('5', vec!['j', 'k', 'l']),
        ('6', vec!['m', 'n', 'o']),
        ('7', vec!['p', 'q', 'r', 's']),
        ('8', vec!['t', 'u', 'v']),
        ('9', vec!['w', 'x', 'y', 'z']),
    ]);
    let mut res = vec![];
    let path = String::new();

    dfs(&digits, &map, 0, path, res.as_mut());
    return res;
}
