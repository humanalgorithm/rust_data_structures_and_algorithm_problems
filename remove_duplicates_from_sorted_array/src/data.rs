#[derive(Debug)]
pub struct Data {
    pub nums: Vec<i32>,
}

pub fn data_1() -> Data {
    Data {
        nums: vec![1, 1, 2],
    }
}

pub fn data_2() -> Data {
    Data {
        nums: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
    }
}
