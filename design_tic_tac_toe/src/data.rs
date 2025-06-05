#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub commands: Vec<String>,
    pub values: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        commands: vec![
            "TicTacToe".to_string(),
            "move".to_string(),
            "move".to_string(),
            "move".to_string(),
            "move".to_string(),
            "move".to_string(),
            "move".to_string(),
            "move".to_string(),
        ],
        values: vec![
            vec![3],
            vec![0, 0, 1],
            vec![0, 2, 2],
            vec![2, 2, 1],
            vec![1, 1, 2],
            vec![2, 0, 1],
            vec![1, 0, 2],
            vec![2, 1, 1],
        ],
    };
}
