pub struct Data {
    pub board: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        board: vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]],
    };
}

pub fn data_2() -> Data {
    return Data {
        board: vec![vec![1, 1], vec![1, 0]],
    };
}
