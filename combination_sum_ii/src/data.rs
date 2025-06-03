pub struct Data {
    pub candidates: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    return Data {
        candidates: vec![10, 1, 2, 7, 6, 1, 5],
        target: 8,
    };
}

pub fn data_2() -> Data {
    return Data {
        candidates: vec![2, 5, 2, 1, 2],
        target: 5,
    };
}
