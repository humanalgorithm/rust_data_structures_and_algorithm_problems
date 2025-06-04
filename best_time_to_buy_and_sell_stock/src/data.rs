pub struct Data {
    pub prices: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        prices: vec![7, 1, 5, 3, 6, 4],
    };
}

pub fn data_2() -> Data {
    return Data {
        prices: vec![7, 6, 4, 3, 1],
    };
}
