pub struct Data {
    pub intervals: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        intervals: vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
    };
}

pub fn data_2() -> Data {
    return Data {
        intervals: vec![vec![1, 4], vec![4, 5]],
    };
}
