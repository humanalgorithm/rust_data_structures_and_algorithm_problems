pub struct Data {
    pub matrix: Vec<Vec<i32>>,
    pub target: i32,
}

pub fn data_1() -> Data {
    return Data {
        matrix: vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ],
        target: 5,
    };
}

pub fn data_2() -> Data {
    return Data {
        matrix: vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ],
        target: 20,
    };
}
