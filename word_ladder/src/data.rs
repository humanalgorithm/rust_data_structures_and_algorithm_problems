pub struct Data {
    pub begin_word: String,
    pub end_word: String,
    pub word_list: Vec<String>,
}

pub fn data_1() -> Data {
    return Data {
        begin_word: "hit".to_string(),
        end_word: "cog".to_string(),
        word_list: vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
            "cog".to_string(),
        ],
    };
}

pub fn data_2() -> Data {
    return Data {
        begin_word: "hit".to_string(),
        end_word: "cog".to_string(),
        word_list: vec![
            "hot".to_string(),
            "dot".to_string(),
            "dog".to_string(),
            "lot".to_string(),
            "log".to_string(),
        ],
    };
}
