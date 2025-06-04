pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    return Data {
        nums: vec![-2, 1, -3, 4, -1, 2, 1, -5, 4],
    };
}

pub fn data_2() -> Data {
    return Data { nums: vec![1] };
}

pub fn data_3() -> Data {
    return Data {
        nums: vec![5, 4, -1, 7, 8],
    };
}
