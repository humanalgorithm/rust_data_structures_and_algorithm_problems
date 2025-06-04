pub struct Data {
    pub digits: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        digits: vec![1, 2, 3],
    };
}

pub fn data_2() -> Data {
    return Data {
        digits: vec![4, 3, 2, 1],
    };
}

pub fn data_3() -> Data {
    return Data { digits: vec![9] };
}
