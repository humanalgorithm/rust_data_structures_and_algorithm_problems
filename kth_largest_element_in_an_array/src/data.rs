pub struct Data {
    pub nums: Vec<i32>,
    pub k: i32,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![3, 2, 1, 5, 6, 4],
        k: 2,
    };
}

pub fn data_2() -> Data {
    return Data {
        nums: vec![3, 2, 3, 1, 2, 4, 5, 5, 6],
        k: 4,
    };
}
