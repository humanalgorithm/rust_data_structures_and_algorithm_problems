pub fn longest_palindrome(s: String) -> String {
    let chars_array: Vec<char> = s.chars().collect();
    let char_len = chars_array.len();
    let (mut longest_str, mut window_size) = (String::new(), char_len);
    if char_len == 1 {
        return s;
    }
    while window_size > 0 {
        let mut x = 0;
        while x + window_size < char_len {
            let mut current_str = chars_array[x].to_string();
            let (mut left, mut right) = (x, x + window_size);
            while chars_array[left] == chars_array[right] && left < right {
                (left, right) = (left + 1, right - 1);
                if left == right || left > right {
                    current_str = chars_array[x..x + window_size + 1].iter().collect();
                }
            }
            if current_str.len() > longest_str.len() {
                longest_str = current_str;
            }
            x = x + 1
        }
        window_size = window_size - 1;
        if longest_str.len() > window_size {
            break;
        }
    }
    return longest_str;
}
