pub struct Data {
    pub board: Vec<Vec<char>>,
    pub word: String,
}

pub fn data_1() -> Data {
    return Data {
        board: vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        word: "ABCCED".to_string(),
    };
}

pub fn data_2() -> Data {
    return Data {
        board: vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        word: "SEE".to_string(),
    };
}

pub fn data_3() -> Data {
    return Data {
        board: vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ],
        word: "ABCB".to_string(),
    };
}
