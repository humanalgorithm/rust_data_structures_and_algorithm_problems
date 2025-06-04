pub struct Data {
    pub intervals: Vec<Vec<i32>>,
    pub new_interval: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        intervals: vec![vec![1, 3], vec![6, 9]],
        new_interval: vec![2, 5],
    };
}

pub fn data_2() -> Data {
    return Data {
        intervals: vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ],
        new_interval: vec![4, 8],
    };
}
