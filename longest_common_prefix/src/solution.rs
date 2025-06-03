pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let (mut longest, mut index) = (String::new(), 0 as usize);
    let mut str_vecs: Vec<Vec<char>> = vec![];
    for word in strs.clone() {
        str_vecs.push(word.chars().collect());
    }

    loop {
        let current_letter = match str_vecs[0].get(index) {
            Some(letter) => *letter,
            None => return longest,
        };
        for x in 0..str_vecs.len() {
            if index >= str_vecs[x].len() || str_vecs[x][index] != current_letter {
                return longest;
            }
        }
        longest.push_str(&current_letter.to_string());
        index += 1;
    }
}
