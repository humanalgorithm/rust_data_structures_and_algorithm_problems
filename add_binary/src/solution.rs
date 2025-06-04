use std::cmp;

pub fn add_binary(a: String, b: String) -> String {
    let a_char: Vec<char> = a.chars().rev().collect();
    let b_char: Vec<char> = b.chars().rev().collect();
    let a_len = a_char.len();
    let b_len = b_char.len();

    let mut carry = 0;
    let mut output: Vec<String> = vec![];
    for i in 0..cmp::max(a_len, b_len) {
        let a_char: char = if i < a_len { a_char[i] } else { '0' };
        let b_char: char = if i < b_len { b_char[i] } else { '0' };
        let a_digit: i32 = if a_char == '1' { 1 } else { 0 };
        let b_digit: i32 = if b_char == '1' { 1 } else { 0 };

        let total = a_digit + b_digit + carry;

        carry = if total > 1 { 1 } else { 0 };
        let this_num = total % 2;
        output.push(this_num.to_string());
    }
    if carry == 1 {
        output.push(carry.to_string());
    }
    return output.iter().rev().map(|s| s.to_string()).collect();
}
