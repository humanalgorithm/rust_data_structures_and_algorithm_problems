pub fn str_str(haystack: String, needle: String) -> i32 {
    let (needle_len, haystack_len) = (needle.len(), haystack.len());
    if haystack_len == 1 {
        return 0;
    }

    let haystack_chars: Vec<char> = haystack.chars().collect();
    let needle_chars: Vec<char> = needle.chars().collect();
    let mut index = 0;

    while (index + needle_len) < haystack_len + 1 {
        let haystack_word: Vec<char> = haystack_chars[index..index + needle_len].to_vec();
        if haystack_word == needle_chars {
            return index as i32;
        }
        index += 1;
    }
    return -1;
}
