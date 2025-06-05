pub struct Data {
    pub nums: Vec<i32>,
    pub k: i32,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![1, 1, 1],
        k: 2,
    };
}

pub fn data_2() -> Data {
    return Data {
        nums: vec![1, 2, 3],
        k: 3,
    };
}
