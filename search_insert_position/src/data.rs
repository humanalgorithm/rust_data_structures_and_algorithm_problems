#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![1, 3, 5, 6],
        target: 5,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![1, 3, 5, 6],
        target: 2,
    }
}

pub fn data_3() -> Data {
    Data {
        nums: vec![1, 3, 5, 6],
        target: 7,
    }
}
