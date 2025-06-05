pub struct Data {
    pub commands: Vec<String>,
    pub values: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        commands: vec![
            "LRUCache".to_string(),
            "put".to_string(),
            "put".to_string(),
            "get".to_string(),
            "put".to_string(),
            "get".to_string(),
            "put".to_string(),
            "get".to_string(),
            "get".to_string(),
            "get".to_string(),
        ],
        values: vec![
            vec![2],
            vec![1, 1],
            vec![2, 2],
            vec![1],
            vec![3, 3],
            vec![2],
            vec![4, 4],
            vec![1],
            vec![3],
            vec![4],
        ],
    };
}
