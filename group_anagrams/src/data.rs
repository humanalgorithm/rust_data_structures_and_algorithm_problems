pub struct Data {
    pub strs: Vec<String>,
}

pub fn data_1() -> Data {
    return Data {
        strs: vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ],
    };
}

pub fn data_2() -> Data {
    return Data {
        strs: vec!["".to_string()],
    };
}

pub fn data_3() -> Data {
    return Data {
        strs: vec!["a".to_string()],
    };
}
