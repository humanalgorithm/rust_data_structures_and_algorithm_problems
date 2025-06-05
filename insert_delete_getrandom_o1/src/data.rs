#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub commands: Vec<String>,
    pub values: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        commands: vec![
            "RandomizedSet".to_string(),
            "insert".to_string(),
            "remove".to_string(),
            "insert".to_string(),
            "getRandom".to_string(),
            "remove".to_string(),
            "insert".to_string(),
            "getRandom".to_string(),
        ],
        values: vec![
            vec![],
            vec![1],
            vec![2],
            vec![2],
            vec![],
            vec![1],
            vec![2],
            vec![],
        ],
    };
}
