pub fn reverse_string(s: &mut Vec<char>) -> &mut Vec<char> {
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        let tmp = s[l];
        s[l] = s[r];
        s[r] = tmp;
        l += 1;
        r -= 1;
    }
    s
}
