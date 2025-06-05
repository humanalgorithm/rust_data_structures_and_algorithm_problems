
pub fn decode_string(s: String) -> String {
    let s_chars: Vec<char> = s.chars().collect();
    let mut stack: Vec<char> = Vec::new();
    for x in 0..s_chars.len() {
        if s_chars[x] != ']' {
            stack.push(s_chars[x]);
        } else {
            let mut substr: Vec<char> = Vec::new();
            while *stack.last().unwrap() != '[' {
                let letter = stack.pop().unwrap();
                substr = [[letter].to_vec(), substr.to_vec()].concat();
            }
            stack.pop();
            let mut k: Vec<char> = Vec::new();
            while !stack.is_empty() && stack.last().unwrap().is_digit(10) {
                k = [[stack.pop().unwrap()].to_vec(), k.to_vec()].concat();
            }
            let str_k: String = k[..].iter().collect();
            let int_k = str_k.parse::<i32>().unwrap();
            for _ in 0..int_k {
                stack = [stack.to_vec(), substr.to_vec()].concat();
            }
        }
    }
    let res: String = stack[..].iter().copied().collect();
    return res;
}
