pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![-1, 2, 1, -4],
        target: 1,
    };
}

pub fn data_2() -> Data {
    return Data {
        nums: vec![0, 0, 0],
        target: 1,
    };
}
