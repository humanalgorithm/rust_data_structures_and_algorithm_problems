#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![4, 5, 6, 7, 0, 1, 2],
        target: 0,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![4, 5, 6, 7, 0, 1, 2],
        target: 3,
    }
}

pub fn data_3() -> Data {
    Data {
        nums: vec![1],
        target: 0,
    }
}
