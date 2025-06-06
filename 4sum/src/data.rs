#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![1, 0, -1, 0, -2, 2],
        target: 0,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![2, 2, 2, 2, 2],
        target: 8,
    }
}
