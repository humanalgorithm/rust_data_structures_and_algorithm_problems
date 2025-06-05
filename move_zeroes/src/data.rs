pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![0, 1, 0, 3, 12],
    };
}

pub fn data_2() -> Data {
    return Data { nums: vec![0] };
}
