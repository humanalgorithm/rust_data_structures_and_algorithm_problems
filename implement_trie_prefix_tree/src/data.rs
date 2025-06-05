#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub commands: Vec<String>,
    pub values: Vec<Vec<String>>,
}

pub fn data_1() -> Data {
    return Data {
        commands: vec![
            "Trie".to_string(),
            "insert".to_string(),
            "search".to_string(),
            "search".to_string(),
            "startsWith".to_string(),
            "insert".to_string(),
            "search".to_string(),
        ],
        values: vec![
            vec![],
            vec!["apple".to_string()],
            vec!["apple".to_string()],
            vec!["app".to_string()],
            vec!["app".to_string()],
            vec!["app".to_string()],
            vec!["app".to_string()],
        ],
    };
}
