use std::collections::HashMap;

pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();
    for path in paths {
        let path_chars: Vec<char> = path.chars().collect();
        let first_space = path_chars.iter().position(|x| *x == ' ').unwrap();

        let mut this_path: Vec<char> = path_chars[0..first_space].to_vec();
        let mut files: Vec<char> = path_chars[first_space + 1..].to_vec();
        this_path.push('/');
        files.reverse();
        while !files.is_empty() {
            let mut file_name: Vec<char> = Vec::new();
            let mut file_contents: Vec<char> = Vec::new();
            loop {
                let letter = files.pop().unwrap();
                if letter == '(' {
                    loop {
                        let current = files.pop().unwrap();
                        if current != ')' {
                            file_contents.push(current);
                        } else {
                            break;
                        }
                    }
                    break;
                } else {
                    if letter != ' ' {
                        file_name.push(letter);
                    }
                }
            }
            let combined_name = [this_path.to_vec(), file_name.to_vec()].concat();
            let combined_name_str: String = combined_name[..].iter().collect();
            map.entry(file_contents.clone())
                .and_modify(|m| m.push(combined_name_str.clone()))
                .or_insert([combined_name_str].to_vec());
        }
    }
    let mut res: Vec<Vec<String>> = Vec::new();
    for key in map.keys() {
        let files: Vec<String> = map.get(key).unwrap().to_vec();
        if files.len() > 1 {
            res.push(files);
        }
    }
    return res;
}
