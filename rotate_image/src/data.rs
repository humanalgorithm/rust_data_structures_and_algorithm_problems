pub struct Data {
    pub matrix: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
    };
}

pub fn data_2() -> Data {
    return Data {
        matrix: vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ],
    };
}
