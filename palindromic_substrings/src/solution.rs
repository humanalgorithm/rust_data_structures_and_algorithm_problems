
pub fn count_substrings(s: String) -> i32 {
    let mut count = 0;
    let s_chars: Vec<char> = s.chars().collect();
    for i in 0..s_chars.len() {
        let mut l = i as i32;
        let mut r = l as i32;
        while l >= 0 && r < s_chars.len() as i32 && s_chars[l as usize] == s_chars[r as usize] {
            count += 1;
            l -= 1;
            r += 1;
        }
        let mut l = i as i32;
        let mut r = l as i32 + 1;
        while l >= 0 && r < s_chars.len() as i32 && s_chars[l as usize] == s_chars[r as usize] {
            count += 1;
            l -= 1;
            r += 1;
        }
    }
    return count;
}
