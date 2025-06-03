pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![2, 7, 11, 15],
        target: 9,
    };
}

pub fn data_2() -> Data {
    return Data {
        nums: vec![3, 2, 4],
        target: 6,
    };
}

pub fn data_3() -> Data {
    return Data {
        nums: vec![3, 3],
        target: 6,
    };
}
