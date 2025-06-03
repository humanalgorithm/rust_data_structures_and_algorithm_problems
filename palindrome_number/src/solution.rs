pub fn is_palindrome(x: i32) -> bool {
    let x_str = x.to_string();
    let mut left = 0;
    let mut right = x_str.len() - 1;
    let x_str_chars = x_str.chars().collect::<Vec<char>>();
    if right == 0 {
        return true;
    }
    while left <= right {
        if &x_str_chars[left] != &x_str_chars[right] {
            return false;
        }
        left = left + 1;
        right = right - 1;
    }
    return true;
}
