use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    if p.len() > s.len() {
        return vec![];
    }
    let (chars_s, chars_p): (Vec<char>, Vec<char>) = (s.chars().collect(), p.chars().collect());
    let (mut p_map, mut s_map): (HashMap<char, i32>, HashMap<char, i32>) =
        (HashMap::new(), HashMap::new());

    for x in 0..chars_p.len() {
        p_map
            .entry(chars_p[x])
            .and_modify(|m| *m = *m + 1)
            .or_insert(1);
        s_map
            .entry(chars_s[x])
            .and_modify(|m| *m = *m + 1)
            .or_insert(1);
    }
    let mut left = 0;
    let mut res: Vec<i32> = if s_map == p_map { vec![0] } else { vec![] };

    for right in chars_p.len()..chars_s.len() {
        if right < chars_s.len() {
            s_map
                .entry(chars_s[right])
                .and_modify(|m| *m = *m + 1)
                .or_insert(1);
        }
        s_map
            .entry(chars_s[left])
            .and_modify(|m| *m = *m - 1)
            .or_insert(1);
        if s_map[&chars_s[left]] == 0 {
            s_map.remove(&chars_s[left]);
        }
        left += 1;
        if p_map == s_map {
            res.push(left as i32);
        }
    }
    return res;
}
