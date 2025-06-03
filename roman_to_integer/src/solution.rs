use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let char_vec: Vec<char> = s.chars().collect();
    let mut output_num = 0;
    let hash_map = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);

    let char_len = char_vec.len();
    let mut stack: Vec<char> = vec![];

    for x in 0..char_len {
        let current_char = char_vec[x];
        let current_val = hash_map.get(&current_char).unwrap();
        let mut consumed = false;
        if !stack.is_empty() {
            let char_top_stack = stack.pop().unwrap();
            let value_prev = hash_map.get(&char_top_stack).unwrap();
            if value_prev < current_val {
                output_num += current_val - value_prev;
                consumed = true;
            } else if value_prev > current_val {
                output_num += value_prev;
            } else if current_val == value_prev {
                output_num += value_prev;
            }
        }
        if !consumed {
            stack.push(current_char);
        }
        if !consumed && x == char_len - 1 {
            output_num += current_val;
        }
    }
    return output_num;
}
