pub struct Data {
    pub intervals: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        intervals: vec![vec![0, 30], vec![5, 10], vec![15, 20]],
    };
}

pub fn data_2() -> Data {
    return Data {
        intervals: vec![vec![7, 10], vec![2, 4]],
    };
}
