pub fn is_anagram(s: String, t: String) -> bool {
    let mut s_chars: Vec<char> = s.chars().collect();
    let mut t_chars: Vec<char> = t.chars().collect();
    s_chars.sort();
    t_chars.sort();
    return s_chars == t_chars;
}
