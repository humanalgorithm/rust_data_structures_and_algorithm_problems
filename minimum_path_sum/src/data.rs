pub struct Data {
    pub grid: Vec<Vec<i32>>,
}

pub fn data_1() -> Data {
    return Data {
        grid: vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]],
    };
}

pub fn data_2() -> Data {
    return Data {
        grid: vec![vec![1, 2, 3], vec![4, 5, 6]],
    };
}
