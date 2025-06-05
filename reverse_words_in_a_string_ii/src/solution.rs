pub fn reverse_words(s: &mut Vec<char>) -> Vec<char> {
    let mut left: usize = 0;
    let mut right: usize = s.len() - 1;

    while left < right {
        (s[left], s[right]) = (s[right], s[left]);
        (left, right) = (left + 1, right - 1);
    }

    let (mut left, mut right, mut next) = (0, 0, 0);
    while right < s.len() {
        while right < s.len() && s[right] != ' ' {
            right += 1;
            next = right;
        }
        right -= 1;
        while left < right {
            (s[left], s[right]) = (s[right], s[left]);
            (left, right) = (left + 1, right - 1);
        }
        (left, right) = (next + 1, next + 1);
    }
    s.to_vec()
}
