pub struct Data {
    pub s: String,
    pub word_dict: Vec<String>,
}

pub fn data_1() -> Data {
    return Data {
        s: "leetcode".to_string(),
        word_dict: vec!["leet".to_string(), "code".to_string()],
    };
}

pub fn data_2() -> Data {
    return Data {
        s: "applepenapple".to_string(),
        word_dict: vec!["apple".to_string(), "pen".to_string()],
    };
}

pub fn data_3() -> Data {
    return Data {
        s: "catsandog".to_string(),
        word_dict: vec![
            "cats".to_string(),
            "dog".to_string(),
            "sand".to_string(),
            "and".to_string(),
            "cat".to_string(),
        ],
    };
}
