pub struct Data {
    pub costs: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        costs: vec![10, 15, 20],
    };
}

pub fn data_2() -> Data {
    return Data {
        costs: vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1],
    };
}
