#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub commands: Vec<String>,
    pub values: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        commands: vec![
            "MinStack".to_string(),
            "push".to_string(),
            "push".to_string(),
            "push".to_string(),
            "getMin".to_string(),
            "pop".to_string(),
            "top".to_string(),
            "getMin".to_string(),
        ],
        values: vec![
            vec![],
            vec![-2],
            vec![0],
            vec![-3],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
    };
}
