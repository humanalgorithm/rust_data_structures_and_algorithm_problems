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
        matrix: vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]],
    };
}
