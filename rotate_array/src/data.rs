#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
    pub k: i32,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![1, 2, 3, 4, 5, 6, 7],
        k: 3,
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![-1, -100, 3, 99],
        k: 2,
    }
}
