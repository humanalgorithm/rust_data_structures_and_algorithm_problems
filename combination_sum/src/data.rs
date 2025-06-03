pub struct Data {
    pub candidates: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    return Data {
        candidates: vec![2, 3, 6, 7],
        target: 7,
    };
}

pub fn data_2() -> Data {
    return Data {
        candidates: vec![2, 3, 5],
        target: 8,
    };
}

pub fn data_3() -> Data {
    return Data {
        candidates: vec![2],
        target: 1,
    };
}
